//! The tracked synthetic deposit, checked against the schema, against the
//! generator that wrote it and against its own manifest.
//!
//! The point of the deposit is that it can be regenerated rather than trusted,
//! and the point of these tests is that nothing about it is trusted here
//! either: the field list comes out of the schema rather than being written
//! down, and the digests come off the files on disk rather than out of the
//! manifest that claims them.
//!
//! In the default suite and inside its three constraints: no window, no socket,
//! no privilege. It runs the generator once, into a directory under the
//! system's temporary path, and reads files already in the tree.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};
use spurenarchiv_validator::validate;

/// Where the tracked deposit is. Everything else is derived from it.
const DEPOSIT: &str = "examples/synthetic/neon-2p-streaking";

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits one level under the repository root")
        .to_path_buf()
}

fn read(path: &Path) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|error| panic!("{} is tracked: {error}", path.display()))
}

fn json(path: &Path) -> Value {
    serde_json::from_slice(&read(path))
        .unwrap_or_else(|error| panic!("{} is a JSON document: {error}", path.display()))
}

fn metadata_path() -> PathBuf {
    repository().join(DEPOSIT).join("scan-001/metadata.json")
}

/// Every key the schema version declares. Read out of the schema rather than
/// listed here, because a list here would agree with the deposit on the day it
/// was written and never afterwards.
fn declared_fields() -> BTreeSet<String> {
    let schema = json(&repository().join("schema/1.0/dataset.schema.json"));
    schema
        .get("properties")
        .and_then(Value::as_object)
        .expect("the dataset schema declares its properties")
        .keys()
        .cloned()
        .collect()
}

#[test]
fn the_example_states_every_field_the_schema_declares() {
    let document = json(&metadata_path());
    let written: BTreeSet<String> = document
        .as_object()
        .expect("the metadata document is an object")
        .keys()
        .cloned()
        .collect();
    assert_eq!(
        declared_fields(),
        written,
        "the example is the deposit that exercises the whole model, so a field the schema \
         declares and this deposit does not write is a field nothing here has ever filled in"
    );
}

#[test]
fn the_example_is_accepted_against_the_version_it_declares() {
    let report = validate(&metadata_path(), &repository().join("schema"))
        .expect("the example deposit is judgeable");
    assert!(
        report.accepted(),
        "the example deposit is refused: {:?}",
        report.findings
    );
}

#[test]
fn no_field_of_the_example_is_left_unstated() {
    // The three states that say a number is missing for a reason outside the
    // dataset. An example that carries one of them is an example teaching a
    // depositor that the gap is normal, which is the opposite of what it is
    // for. `not_applicable` is not among them: it says the quantity does not
    // arise, and for a computed dataset most of the apparatus does not.
    let unstated = ["not_measured", "not_recorded", "withheld"];
    let document = json(&metadata_path());
    let carried: Vec<String> = document
        .as_object()
        .expect("the metadata document is an object")
        .iter()
        .filter_map(|(field, record)| {
            let state = record.get("state")?.as_str()?;
            unstated
                .contains(&state)
                .then(|| format!("{field} is {state}"))
        })
        .collect();
    assert!(
        carried.is_empty(),
        "the example leaves a field unstated: {carried:?}"
    );
}

#[test]
fn every_file_in_the_deposit_is_the_one_its_manifest_names() {
    let root = repository().join(DEPOSIT);
    let manifest = json(&root.join("manifest.json"));
    let listed = manifest
        .get("files")
        .and_then(Value::as_array)
        .expect("the manifest lists its files");

    assert_eq!(
        manifest.get("file_count").and_then(Value::as_u64),
        Some(listed.len() as u64),
        "file_count and the entries disagree, so one of the two lost something"
    );

    let mut named = BTreeSet::new();
    for entry in listed {
        let path = entry
            .get("path")
            .and_then(Value::as_str)
            .expect("an entry names a path");
        named.insert(path.to_string());
        let bytes = read(&root.join(path));
        assert_eq!(
            entry.get("size").and_then(Value::as_u64),
            Some(bytes.len() as u64),
            "{path}: the manifest states a length the file does not have"
        );
        assert_eq!(
            entry.get("digest").and_then(Value::as_str),
            Some(format!("sha256:{:x}", Sha256::digest(&bytes)).as_str()),
            "{path}: the manifest states a digest the bytes do not produce. A checkout that \
             rewrote a line ending inside a listed file looks exactly like this."
        );
    }

    let mut on_disk = BTreeSet::new();
    walk(&root, &root, &mut on_disk);
    on_disk.remove("manifest.json");
    assert_eq!(
        named, on_disk,
        "the manifest and the directory hold different file sets"
    );
}

#[test]
fn regenerating_the_example_reproduces_every_byte_of_it() {
    let written = std::env::temp_dir().join("spurenarchiv-synthetic-example");
    let _ = std::fs::remove_dir_all(&written);
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_synthetic-example"))
        .arg(&written)
        .status()
        .expect("the generator is built beside this test");
    assert!(status.success(), "the generator refused to write");

    let tracked_root = repository().join(DEPOSIT);
    let mut tracked = BTreeSet::new();
    walk(&tracked_root, &tracked_root, &mut tracked);
    let mut regenerated = BTreeSet::new();
    walk(&written, &written, &mut regenerated);
    assert_eq!(
        tracked, regenerated,
        "the generator writes a different file set from the one that is tracked"
    );

    let mut differ = BTreeMap::new();
    for path in &tracked {
        let before = read(&tracked_root.join(path));
        let after = read(&written.join(path));
        if before != after {
            differ.insert(path.clone(), (before.len(), after.len()));
        }
    }
    let _ = std::fs::remove_dir_all(&written);
    assert!(
        differ.is_empty(),
        "the generator does not reproduce the tracked deposit: {differ:?}. Either the tracked \
         copy was edited by hand, or the generator changed and the deposit was not written \
         again. It can also mean this platform's exp or cos disagrees in the last bits with the \
         one the tracked copy was written on, which is the bound examples/synthetic/README.md \
         states rather than a defect in either."
    );
}

/// Every file under `root`, as a path relative to it, with `/` as the
/// separator so the comparison is the same on every platform.
fn walk(root: &Path, directory: &Path, found: &mut BTreeSet<String>) {
    let entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} can be listed: {error}", directory.display()));
    for entry in entries {
        let entry = entry.expect("a directory entry reads");
        let path = entry.path();
        if path.is_dir() {
            walk(root, &path, found);
        } else {
            let relative = path
                .strip_prefix(root)
                .expect("the walk stays under the root it started at");
            found.insert(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}
