//! Reading a deposit against the version it declares.
//!
//! The archive will ship more than one schema version, and every deposit
//! already made was written against the version that existed that day. The
//! property that keeps those readable is that the reader dispatches on the
//! declaration rather than guessing from the fields present, and judges the
//! deposit against that version's own bytes.
//!
//! One version is shipped, so the tree cannot demonstrate that property against
//! itself. It is demonstrated here against a schema root built for the purpose,
//! which is a fixture vocabulary rather than this repository's store: a row
//! judged against the real `schema/` would prove the state of the tree on the
//! day it ran rather than the guard.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;
use spurenarchiv_validator::{validate, validate_text, Error};

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits one level under the repository root")
        .to_path_buf()
}

fn schema_root() -> PathBuf {
    repository().join("schema")
}

/// Every version this tree ships, read off the directory rather than listed
/// here, so a version that lands without a fixture declaring it reddens this
/// rather than passing unnoticed.
fn shipped_versions() -> BTreeSet<String> {
    std::fs::read_dir(schema_root())
        .expect("schema/ is tracked")
        .flatten()
        .filter(|entry| entry.path().join("dataset.schema.json").is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}

fn base() -> Value {
    let text = std::fs::read_to_string(
        repository().join("fixtures/deposit/accepted-complete-deposit.json"),
    )
    .expect("the accepted base fixture is tracked");
    serde_json::from_str(&text).expect("the accepted base fixture is JSON")
}

/// A deposit is read against the version it declares, and the reader says which
/// one it read. Every shipped version has to have a fixture that exercises it.
#[test]
fn the_reader_reports_the_version_it_read() {
    let shipped = shipped_versions();
    assert!(
        !shipped.is_empty(),
        "no schema version was found, so this proved nothing"
    );

    let mut exercised = BTreeSet::new();
    for directory in ["fixtures/deposit", "fixtures/validator"] {
        for entry in
            std::fs::read_dir(repository().join(directory)).expect("the directory is tracked")
        {
            let path = entry.expect("a readable directory entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(report) = validate(&path, &schema_root()) else {
                continue;
            };
            let text = std::fs::read_to_string(&path).expect("a fixture is readable");
            let declared: Value = serde_json::from_str(&text).expect("a fixture is JSON");
            assert_eq!(
                Some(report.schema_version.as_str()),
                declared.get("schema_version").and_then(Value::as_str),
                "{} was read as a version it does not declare",
                path.display()
            );
            exercised.insert(report.schema_version);
        }
    }

    assert_eq!(
        exercised, shipped,
        "a shipped version has no fixture that is read against it"
    );
}

/// A version this build does not carry is named. A reader that fell back to the
/// nearest version it had would judge a deposit against rules it was not
/// written against and report success.
#[test]
fn a_deposit_declaring_a_version_this_build_does_not_carry_is_refused_by_name() {
    let path = repository().join("fixtures/validator/refused-future-schema-version.json");
    match validate(&path, &schema_root()) {
        Err(Error::UnknownVersion(version, _)) => assert_eq!(version, "2.0"),
        other => panic!("expected the version to be named and the deposit unjudged, got {other:?}"),
    }
}

/// A deposit whose contents are not the declared version's is refused rather
/// than read as far as it goes. The fixture declares 1.0 and carries a key 1.0
/// does not define, which is what a document written against a later version
/// looks like from here.
#[test]
fn contents_that_are_not_the_declared_versions_are_refused() {
    let path =
        repository().join("fixtures/validator/refused-contents-against-another-version.json");
    let report = validate(&path, &schema_root()).expect("the declared version is carried here");
    assert_eq!(report.schema_version, "1.0");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.field == "delay_axis_reference_frame"),
        "the refusal does not name the key that does not belong to this version: {:?}",
        report.findings
    );
}

/// Copies a directory tree. Small and local because the only tree it copies is
/// a schema version, which is text files one level deep.
fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the destination can be made");
    for entry in std::fs::read_dir(from).expect("the source is readable") {
        let entry = entry.expect("a readable directory entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("the file can be copied");
        }
    }
}

/// A schema root carrying two versions: this tree's 1.0, and a 9.9 that admits
/// and requires one key 1.0 does not define.
///
/// The second version is invented here and nothing about it is a proposal. It
/// exists so the dispatch has two versions to choose between, which is the one
/// thing a tree shipping a single version cannot supply.
fn two_version_root() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("two-version-schema-root");
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(&schema_root().join("1.0"), &root.join("1.0"));
    copy_tree(&schema_root().join("1.0"), &root.join("9.9"));

    let later = root.join("9.9/dataset.schema.json");
    let text = std::fs::read_to_string(&later).expect("the copied schema is readable");
    let mut schema: Value = serde_json::from_str(&text).expect("the copied schema is JSON");
    schema["properties"]["schema_version"]["const"] = Value::String("9.9".to_string());
    schema["properties"]["delay_axis_reference_frame"] =
        serde_json::json!({ "$ref": "#/$defs/field_record" });
    schema["required"]
        .as_array_mut()
        .expect("a version requires keys")
        .push(Value::String("delay_axis_reference_frame".to_string()));
    std::fs::write(
        &later,
        serde_json::to_string_pretty(&schema).expect("the schema can be written"),
    )
    .expect("the later version can be written");

    root
}

/// One document, two declarations, two verdicts, from one schema root. This is
/// the property the whole arrangement rests on: what a deposit is judged
/// against is what it says it was written against, and never the newest thing
/// the reader happens to carry.
#[test]
fn a_deposit_is_judged_against_the_version_it_declares() {
    let root = two_version_root();

    let mut later = base();
    later["schema_version"] = Value::String("9.9".to_string());
    later["delay_axis_reference_frame"] = serde_json::json!({ "state": "not_applicable" });

    let report = validate_text(&later.to_string(), &root).expect("9.9 is carried by this root");
    assert_eq!(report.schema_version, "9.9");
    assert!(
        report.accepted(),
        "the deposit conforms to the version it declares and was refused: {:?}",
        report.findings
    );

    // The same bytes, declaring the earlier version, which does not admit the
    // key. Nothing about the document changed except what it says it is.
    let mut earlier = later.clone();
    earlier["schema_version"] = Value::String("1.0".to_string());
    let report = validate_text(&earlier.to_string(), &root).expect("1.0 is carried by this root");
    assert_eq!(report.schema_version, "1.0");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.field == "delay_axis_reference_frame"),
        "1.0 admitted a key it does not define: {:?}",
        report.findings
    );

    // And the earlier version is not reached by the later one's requirement. A
    // deposit made before 9.9 existed still reads.
    let report = validate_text(&base().to_string(), &root).expect("1.0 is carried by this root");
    assert_eq!(report.schema_version, "1.0");
    assert!(
        report.accepted(),
        "a deposit written against 1.0 was refused once a later version existed beside it: {:?}",
        report.findings
    );

    // The later version's own requirement bites, so the root is not two copies
    // of one schema and the first leg above is not a coincidence.
    let mut incomplete = base();
    incomplete["schema_version"] = Value::String("9.9".to_string());
    let report =
        validate_text(&incomplete.to_string(), &root).expect("9.9 is carried by this root");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.field == "delay_axis_reference_frame"),
        "9.9 did not require the key it was built to require: {:?}",
        report.findings
    );
}
