//! The fixtures in this repository, judged by the validator.
//!
//! The deposit fixtures already carry their verdict in their names, so they are
//! evidence that costs nothing to reuse: a change that makes the validator
//! refuse a conforming deposit, or accept one the schema names, turns this red
//! without anybody writing a new fixture.
//!
//! The deferred checks get their proof here instead of in a fixture file. Each
//! one is a one-value edit of the accepted base, which is the one-character
//! mistake somebody actually makes, and the edit is in the test rather than in
//! `fixtures/deposit/` because those files are judged by a route that evaluates
//! the schema alone and would have to call every one of these accepted.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use spurenarchiv_validator::{deferred_checks_evaluated, validate, validate_text, Error, Report};

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits one level under the repository root")
        .to_path_buf()
}

fn schema_root() -> PathBuf {
    repository().join("schema")
}

fn base() -> Value {
    let text = std::fs::read_to_string(
        repository().join("fixtures/deposit/accepted-complete-deposit.json"),
    )
    .expect("the accepted base fixture is tracked");
    serde_json::from_str(&text).expect("the accepted base fixture is JSON")
}

fn judge(document: &Value) -> Report {
    validate_text(&document.to_string(), &schema_root()).expect("the document is judgeable")
}

/// Every tracked deposit fixture, judged against the verdict its name claims.
#[test]
fn each_deposit_fixture_gets_the_verdict_its_name_claims() {
    let directory = repository().join("fixtures/deposit");
    let mut examined = 0;
    for entry in std::fs::read_dir(&directory).expect("fixtures/deposit is tracked") {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a fixture name")
            .to_string();
        let report = validate(&path, &schema_root())
            .unwrap_or_else(|e| panic!("{name} could not be judged at all: {e}"));
        if name.starts_with("accepted-") {
            assert!(
                report.accepted(),
                "{name} exists to be accepted and was refused: {:?}",
                report.findings
            );
        } else if name.starts_with("refused-") {
            assert!(
                !report.accepted(),
                "{name} exists to be refused and was accepted"
            );
        } else {
            panic!("{name} says neither accepted nor refused, so nothing decides its verdict");
        }
        examined += 1;
    }
    // A run that found no fixtures is a broken test and not a clean tree.
    assert!(
        examined > 0,
        "no deposit fixtures were found under {directory:?}"
    );
}

/// The one property the command's usefulness rests on: eleven problems are
/// eleven round trips only if the run stops at the first.
#[test]
fn three_independent_problems_produce_three_findings() {
    let path = repository().join("fixtures/validator/refused-three-independent-problems.json");
    let report = validate(&path, &schema_root()).expect("the fixture is judgeable");
    let fields: Vec<&str> = report.findings.iter().map(|f| f.field.as_str()).collect();
    assert_eq!(
        report.findings.len(),
        3,
        "expected one finding per problem, got {fields:?}"
    );
    for field in [
        "delay_jitter",
        "dressing_peak_intensity",
        "inter_scan_scatter",
    ] {
        assert!(
            fields.contains(&field),
            "no finding named {field}; got {fields:?}"
        );
    }
    for finding in &report.findings {
        assert!(!finding.problem.is_empty(), "a finding with no problem");
        assert!(!finding.remedy.is_empty(), "a finding with no remedy");
    }
}

/// One deferred check, as the identifier the schema gives it, the single edit
/// that trips it, and the deposit key the finding has to name.
type Case = (&'static str, Box<dyn Fn(&mut Value)>, &'static str);

/// Each deferred check, tripped by one edit of a document that is otherwise
/// accepted. The base being accepted is what makes each of these a proof that
/// the check bit rather than that something else did.
#[test]
fn each_deferred_check_bites_on_one_edit() {
    assert!(judge(&base()).accepted(), "the base fixture is accepted");

    let cases: Vec<Case> = vec![
        (
            "integer-field-carries-an-integer-literal",
            Box::new(|d: &mut Value| d["scan_combination"]["value"]["scan_count"] = json!(12.0)),
            "scan_combination",
        ),
        (
            "delay-uncertainty-length-matches-delay-values",
            Box::new(|d: &mut Value| {
                d["delay_value_uncertainty"]["value"] = json!([2e-16, 2e-16, 2e-16, 2e-16])
            }),
            "delay_value_uncertainty",
        ),
        (
            "delay-values-length-matches-array-delay-axis",
            Box::new(|d: &mut Value| d["spectrogram"]["value"]["shape"] = json!([6, 16])),
            "delay_values",
        ),
        (
            "edge-coordinates-carry-one-more-value-than-bins",
            Box::new(|d: &mut Value| {
                d["axis_bin_convention"]["value"]["delay"] = json!("bin_edge")
            }),
            "delay_values",
        ),
        (
            "efficiency-curve-arms-same-length",
            Box::new(|d: &mut Value| {
                d["detection_efficiency"]["value"]["values"] = json!([0.31, 0.29, 0.26])
            }),
            "detection_efficiency",
        ),
        (
            "dressing-spectrum-arms-same-length",
            Box::new(|d: &mut Value| {
                d["dressing_spectrum"]["value"]["spectral_intensities"] = json!([0.12, 0.71, 1.0])
            }),
            "dressing_spectrum",
        ),
        (
            "resolution-curve-arms-same-length",
            Box::new(|d: &mut Value| {
                d["spectrometer_resolution"]["value"]["uncertainty"] = json!([0.03, 0.05])
            }),
            "spectrometer_resolution",
        ),
        (
            "transmission-curve-arms-same-length",
            Box::new(|d: &mut Value| {
                d["transmission_function"]["value"]["values"] = json!([1.0, 0.94, 0.86])
            }),
            "transmission_function",
        ),
        (
            "scan-count-agrees-between-the-two-fields-that-carry-it",
            Box::new(|d: &mut Value| d["inter_scan_scatter"]["value"]["scan_count"] = json!(11)),
            "inter_scan_scatter",
        ),
        (
            "shots-per-point-length-matches-delay-values",
            Box::new(|d: &mut Value| {
                d["shots_per_point"]["value"] = json!([2000, 2000, 2000, 2000])
            }),
            "shots_per_point",
        ),
        (
            "acquisition-order-is-a-permutation-of-the-delay-points",
            Box::new(|d: &mut Value| {
                d["acquisition_order"]["value"]["index_sequence"] = json!([3, 0, 4, 1, 1])
            }),
            "acquisition_order",
        ),
        (
            "bin-widths-length-matches-the-energy-extent",
            Box::new(|d: &mut Value| {
                d["spectrogram_value_semantics"]["value"] = json!({
                    "quantity_per_pixel": "counts",
                    "per": "unit_of_axis",
                    "bin_widths": [0.42, 0.44, 0.47, 0.51, 0.42, 0.44, 0.47, 0.51, 0.42, 0.44, 0.47, 0.51, 0.42, 0.44, 0.47]
                })
            }),
            "spectrogram_value_semantics",
        ),
    ];

    for (id, edit, field) in cases {
        let mut document = base();
        edit(&mut document);
        let report = judge(&document);
        let fields: Vec<&str> = report.findings.iter().map(|f| f.field.as_str()).collect();
        assert_eq!(
            report.findings.len(),
            1,
            "{id}: one edit should produce one finding, got {fields:?}"
        );
        assert_eq!(report.findings[0].field, field, "{id}: wrong field named");
        assert!(
            !report.findings[0].remedy.is_empty(),
            "{id}: the finding carries no remedy"
        );
    }
}

/// The near miss for the two length checks that share one comparison. Declaring
/// the delay axis by its edges is not a defect; carrying the wrong number of
/// them is, and the distance between the two is one coordinate.
///
/// The other four fields move with it, and that is not tidying. The deferred
/// checks the schema declares compare `shots_per_point`, the uncertainty and
/// `index_sequence` against the length of `delay_values` and not against the
/// number of bins, so under an edge-declared axis each of them wants one more
/// entry than there are delay points. That reads wrong and it is what the
/// specification says; docs/spec/validation.md records it as a question for the
/// model rather than answering it here.
#[test]
fn bin_edges_are_accepted_when_there_is_one_more_of_them_than_bins() {
    let mut document = base();
    document["axis_bin_convention"]["value"]["delay"] = json!("bin_edge");
    document["delay_values"]["value"] = json!([-5e-15, -3e-15, -1e-15, 1e-15, 3e-15, 5e-15]);
    document["delay_value_uncertainty"]["value"] =
        json!([2e-16, 2e-16, 2e-16, 2e-16, 2e-16, 2e-16]);
    document["shots_per_point"]["value"] = json!([2000, 2000, 2000, 2000, 2000, 2000]);
    document["acquisition_order"]["value"]["index_sequence"] = json!([3, 0, 4, 1, 5, 2]);
    let report = judge(&document);
    assert!(
        report.accepted(),
        "an edge-declared axis with one more coordinate than bins is refused: {:?}",
        report.findings
    );
}

/// A repeated key ends the run, and says so rather than reporting a clean set of
/// other checks over a document that has no single meaning.
#[test]
fn a_repeated_key_is_refused_and_stops_the_run() {
    let text = r#"{"schema_version": "1.0", "delay_jitter": {"state": "not_measured"}, "delay_jitter": {"state": "present", "value": 1.0}}"#;
    let report = validate_text(text, &schema_root()).expect("the document parses");
    assert!(!report.accepted());
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.problem.contains("delay_jitter")),
        "the finding does not name the repeated key: {:?}",
        report.findings
    );
    assert!(
        !report.not_evaluated.is_empty(),
        "a run that stopped early reported nothing as unevaluated"
    );
}

/// The identifiers version 1.0 declares it cannot evaluate, read from the
/// schema rather than listed here, because a list in a test drifts against the
/// artefact that decides it.
fn declared_deferred() -> BTreeSet<String> {
    let text = std::fs::read_to_string(schema_root().join("1.0/dataset.schema.json"))
        .expect("version 1.0 is tracked");
    let schema: Value = serde_json::from_str(&text).expect("the schema is JSON");
    schema
        .get("deferred_checks")
        .and_then(Value::as_array)
        .expect("version 1.0 declares its deferred checks")
        .iter()
        .map(|entry| {
            entry
                .get("id")
                .and_then(Value::as_str)
                .expect("a deferred check carries an id")
                .to_string()
        })
        .collect()
}

/// What the report says about what it did not do. A run that covered less than
/// the whole declared set must not read as one that covered it.
///
/// Every identifier the schema declares is accounted for exactly once: either
/// this build evaluates it or the report names it as not evaluated. Deleting an
/// entry from the dispatch moves its identifier from one side to the other and
/// leaves this green, which is the point of deriving both from the same table;
/// what turns it red is a check that stops being run and goes on being claimed.
#[test]
fn every_declared_check_is_either_evaluated_or_named_as_not_evaluated() {
    let report = judge(&base());

    let evaluated: BTreeSet<String> = deferred_checks_evaluated()
        .into_iter()
        .map(str::to_string)
        .collect();
    let disclosed: BTreeSet<String> = report
        .not_evaluated
        .iter()
        .map(|skipped| skipped.id.clone())
        .collect();

    assert!(
        evaluated.is_disjoint(&disclosed),
        "a check is claimed as evaluated and reported as not evaluated: {:?}",
        evaluated.intersection(&disclosed).collect::<Vec<_>>()
    );
    assert_eq!(
        evaluated
            .union(&disclosed)
            .cloned()
            .collect::<BTreeSet<_>>(),
        declared_deferred(),
        "the run accounts for a different set of checks than version 1.0 declares"
    );
    assert!(
        !report.not_evaluated.is_empty(),
        "an accepted deposit still owes the reader the checks that were not evaluated"
    );
    for skipped in &report.not_evaluated {
        assert!(
            !skipped.why.is_empty(),
            "{} is reported as unevaluated with no reason",
            skipped.id
        );
    }
}

/// A claim to evaluate an identifier the schema does not declare is a
/// misspelling, and it leaves the check the schema does declare reported as not
/// evaluated while the dispatch reads as though it were covered.
#[test]
fn every_check_this_build_claims_is_one_the_schema_declares() {
    let declared = declared_deferred();
    for id in deferred_checks_evaluated() {
        assert!(
            declared.contains(id),
            "this build claims to evaluate {id}, which version 1.0 does not declare"
        );
    }
}

/// The three parts of a message, across every fixture that produces one. A
/// depositor who does not know this schema needs the key, what is wrong with it
/// and what to do; a remedy that repeats the problem back is none of the three.
#[test]
fn every_finding_names_a_field_a_problem_and_a_remedy() {
    let directory = repository().join("fixtures/deposit");
    let mut examined = 0;
    for entry in std::fs::read_dir(&directory).expect("fixtures/deposit is tracked") {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let report = validate(&path, &schema_root()).expect("a fixture is judgeable");
        for finding in &report.findings {
            let where_ = format!("{} -> {finding:?}", path.display());
            assert!(!finding.field.trim().is_empty(), "no field in {where_}");
            assert!(!finding.problem.trim().is_empty(), "no problem in {where_}");
            assert!(!finding.remedy.trim().is_empty(), "no remedy in {where_}");
            assert_ne!(
                finding.problem, finding.remedy,
                "the remedy repeats the problem in {where_}"
            );
            examined += 1;
        }
    }
    assert!(
        examined > 0,
        "no findings were examined, so this proved nothing"
    );
}

/// A missing key is reported under the key a depositor would type, and the
/// remedy names it too, so the sentence can be acted on without opening the
/// schema. The key has no instance path of its own, which is the case where
/// naming it is easiest to get wrong.
#[test]
fn a_missing_key_is_reported_under_its_own_name() {
    let mut document = base();
    document
        .as_object_mut()
        .expect("the deposit is an object")
        .remove("delay_jitter")
        .expect("the accepted base carries delay_jitter");

    let report = judge(&document);
    let named: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.field == "delay_jitter")
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one finding about the removed key was expected, and the run reported {:?}",
        report.findings
    );
    assert!(
        named[0].remedy.contains("delay_jitter"),
        "the remedy does not name the key to add: {}",
        named[0].remedy
    );
}

/// A version this build does not carry is named rather than read under the
/// nearest one to hand.
#[test]
fn an_unknown_schema_version_is_named_rather_than_guessed() {
    match validate_text(r#"{"schema_version": "9.9"}"#, &schema_root()) {
        Err(Error::UnknownVersion(version, _)) => {
            assert_eq!(version, "9.9");
            let error = Error::UnknownVersion(version, schema_root());
            assert!(
                error.to_string().contains("9.9"),
                "the sentence a depositor reads does not name the version: {error}"
            );
        }
        other => panic!("a version that is not in the tree cannot be judged, got {other:?}"),
    }
}

/// No declared version is refused as its own case. Judging such a document
/// against whichever version happens to be newest is the same defect as
/// approximating an unknown one, one step earlier.
#[test]
fn a_document_that_declares_no_version_is_not_judged() {
    match validate_text(
        r#"{"delay_jitter": {"state": "not_measured"}}"#,
        &schema_root(),
    ) {
        Err(Error::NoVersion) => {}
        other => panic!("expected a refusal naming the missing version, got {other:?}"),
    }
}
