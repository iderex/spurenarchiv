//! Judging one deposit's metadata document.
//!
//! The schema under `schema/<version>/` is the normative artefact, so this
//! evaluates it rather than restating its conditions in Rust. What it adds is
//! the set `docs/decisions/schema-language.md` calls deferred: the conditions
//! JSON Schema cannot express, which that record requires to stay readable from
//! `schema/` rather than disappear into an implementation.
//!
//! Two properties are held deliberately. Every finding is collected before
//! anything is printed, because a depositor who has to make eleven round trips
//! stops after three. And a deferred check this build does not evaluate is
//! named in the report as not evaluated, so a run that covered less than the
//! whole set cannot be read as one that covered it and found nothing.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use serde_json::Value;

mod deferred;

/// One reason a deposit was refused, in the three parts a depositor needs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    /// The deposit key the problem is in, or `(document)` where it is the
    /// document as a whole.
    pub field: String,
    pub problem: String,
    pub remedy: String,
}

/// A deferred check the schema declares that this run did not evaluate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEvaluated {
    pub id: String,
    pub why: String,
}

/// What one run produced.
#[derive(Debug, Clone)]
pub struct Report {
    pub schema_version: String,
    pub findings: Vec<Finding>,
    pub not_evaluated: Vec<NotEvaluated>,
}

impl Report {
    pub fn accepted(&self) -> bool {
        self.findings.is_empty()
    }
}

#[derive(Debug)]
pub enum Error {
    Read(PathBuf, std::io::Error),
    /// The bytes are not a JSON document at all, so there is nothing to judge.
    Syntax(String),
    /// No `schema_version` string, so no version to judge against.
    NoVersion,
    /// A version this tree does not carry. Naming it is the point: a reader
    /// that fell back to the nearest version it had would read the deposit
    /// under a schema it was not written against.
    UnknownVersion(String, PathBuf),
    /// The schema itself would not compile. This is a defect in this
    /// repository and never in the deposit.
    Schema(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Read(p, e) => write!(f, "cannot read {}: {e}", p.display()),
            Error::Syntax(m) => write!(f, "not a JSON document: {m}"),
            Error::NoVersion => write!(
                f,
                "the document carries no schema_version string, so there is no version to judge it against"
            ),
            Error::UnknownVersion(v, root) => write!(
                f,
                "schema version {v} is not carried here; {} holds the versions this build can judge against",
                root.display()
            ),
            Error::Schema(m) => write!(f, "the schema in this tree would not compile: {m}"),
        }
    }
}

impl std::error::Error for Error {}

/// The deferred identifiers this build evaluates, under the names the schema
/// uses for them. Read out of the dispatch rather than kept in a second list
/// beside it, so a check that stops running stops being claimed here in the
/// same edit.
///
/// What it does not say is whether an operator still refuses what its
/// identifier names. That is one fixture per refusal and it is issue #33.
pub fn deferred_checks_evaluated() -> Vec<&'static str> {
    deferred::evaluated()
}

/// Judge the metadata document at `document` against the schema version it
/// declares, looking for that version under `schema_root`.
pub fn validate(document: &Path, schema_root: &Path) -> Result<Report, Error> {
    let text =
        std::fs::read_to_string(document).map_err(|e| Error::Read(document.to_path_buf(), e))?;
    validate_text(&text, schema_root)
}

/// The same judgement over bytes already in hand, so a caller holding a
/// document does not have to put it on disk to have it judged.
pub fn validate_text(text: &str, schema_root: &Path) -> Result<Report, Error> {
    // A repeated key is judged before anything else and ends the run. Parsers
    // disagree about which of the two values a repeated key holds, so a
    // document carrying one does not have a single meaning for the rest of the
    // checks to be about.
    if let Some(key) = repeated_key(text) {
        let version = parse(text)
            .ok()
            .and_then(|v| declared_version(&v).ok())
            .unwrap_or_else(|| "unknown".to_string());
        return Ok(Report {
            schema_version: version,
            findings: vec![Finding {
                field: "(document)".to_string(),
                problem: format!("the key {key} is written more than once, and JSON parsers disagree about which of the two values that leaves"),
                remedy: format!("keep one {key} and delete the other"),
            }],
            not_evaluated: vec![NotEvaluated {
                id: "(every other check)".to_string(),
                why: "a document with a repeated key has no single meaning for a later check to be about, so the run stopped here".to_string(),
            }],
        });
    }

    let document = parse(text)?;
    let version = declared_version(&document)?;

    let version_dir = schema_root.join(&version);
    let dataset_schema_path = version_dir.join("dataset.schema.json");
    if !dataset_schema_path.is_file() {
        return Err(Error::UnknownVersion(version, schema_root.to_path_buf()));
    }

    let schema_text = std::fs::read_to_string(&dataset_schema_path)
        .map_err(|e| Error::Read(dataset_schema_path.clone(), e))?;
    let schema: Value =
        serde_json::from_str(&schema_text).map_err(|e| Error::Schema(e.to_string()))?;

    let mut findings = schema_findings(&schema, &document, &version_dir)?;
    findings.extend(deferred::run(&document, &version_dir));
    findings.sort();
    findings.dedup();

    let not_evaluated = deferred::not_evaluated(&schema);

    Ok(Report {
        schema_version: version,
        findings,
        not_evaluated,
    })
}

fn parse(text: &str) -> Result<Value, Error> {
    serde_json::from_str(text).map_err(|e| Error::Syntax(e.to_string()))
}

fn declared_version(document: &Value) -> Result<String, Error> {
    document
        .get("schema_version")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or(Error::NoVersion)
}

/// Resolves the rows a version's dataset schema references. The references are
/// relative (`fields/delay-values.json#/schema`), so they arrive here as a path
/// under the version directory.
struct VersionDirectory(PathBuf);

impl jsonschema::Retrieve for VersionDirectory {
    fn retrieve(
        &self,
        uri: &jsonschema::Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let relative = uri.path().as_str().trim_start_matches('/');
        // A reference that climbed out of the version directory would read a
        // file this version does not carry, which is the one way an old deposit
        // stops being judged against the bytes it was written against.
        if relative.split('/').any(|part| part == "..") {
            return Err(
                format!("the reference {relative} leaves the schema version directory").into(),
            );
        }
        let text = std::fs::read_to_string(self.0.join(relative))?;
        Ok(serde_json::from_str(&text)?)
    }
}

fn schema_findings(
    schema: &Value,
    document: &Value,
    version_dir: &Path,
) -> Result<Vec<Finding>, Error> {
    let validator = jsonschema::options()
        .with_retriever(VersionDirectory(version_dir.to_path_buf()))
        .build(schema)
        .map_err(|e| Error::Schema(e.to_string()))?;

    Ok(validator
        .iter_errors(document)
        .map(|error| {
            let keyword = error.schema_path().to_string();
            let problem = error.to_string();
            Finding {
                field: field_of(&error.instance_path().to_string(), &problem),
                remedy: remedy_for(&keyword, &problem),
                problem,
            }
        })
        .collect())
}

/// The deposit key a finding is about. The instance path names it for anything
/// inside a field record; a missing key has no instance path of its own, so the
/// key comes out of the message that names it.
fn field_of(instance_path: &str, problem: &str) -> String {
    let first = instance_path
        .trim_start_matches('/')
        .split('/')
        .next()
        .unwrap_or("");
    if !first.is_empty() {
        return first.to_string();
    }
    match quoted(problem) {
        Some(name) => name,
        None => "(document)".to_string(),
    }
}

fn quoted(text: &str) -> Option<String> {
    let rest = text.split_once('"')?.1;
    let (inside, _) = rest.split_once('"')?;
    if inside.is_empty() {
        None
    } else {
        Some(inside.to_string())
    }
}

/// What the depositor does about it. The keyword that refused is what decides
/// this, so a remedy cannot drift away from the condition it is the remedy for.
fn remedy_for(keyword_path: &str, problem: &str) -> String {
    let keyword = keyword_path.rsplit('/').next().unwrap_or("");
    match keyword {
        "required" => match quoted(problem) {
            Some(key) => format!(
                "add {key}, carrying a value or carrying the state that says why it does not"
            ),
            None => "add the key the message names".to_string(),
        },
        "not" => "delete the key the message names; the state this record declares does not carry one".to_string(),
        "enum" | "const" => "use one of the values the schema names for this key".to_string(),
        "type" => "write the value as the type the schema names for this key".to_string(),
        "additionalProperties" => {
            "delete the key the message names; this record admits no key beyond the ones the schema lists".to_string()
        }
        "minLength" | "pattern" => "write a sentence here rather than an empty string".to_string(),
        "minItems" => "give this list at least one entry, or declare the field's absence state instead".to_string(),
        "exclusiveMinimum" | "minimum" | "maximum" | "exclusiveMaximum" => {
            "bring the number inside the range the schema names".to_string()
        }
        _ => format!("meet the condition at {keyword_path} in the schema for this version"),
    }
}

/// The first key written twice at any level, if there is one.
fn repeated_key(text: &str) -> Option<String> {
    let mut deserializer = serde_json::Deserializer::from_str(text);
    match serde::de::DeserializeSeed::deserialize(KeysAreUnique, &mut deserializer) {
        Ok(()) => None,
        Err(e) => {
            let message = e.to_string();
            message.strip_prefix(REPEAT_MARKER).map(|rest| {
                rest.split_once(" at line")
                    .map_or(rest, |(k, _)| k)
                    .to_string()
            })
        }
    }
}

const REPEAT_MARKER: &str = "repeated key ";

struct KeysAreUnique;

impl<'de> serde::de::DeserializeSeed<'de> for KeysAreUnique {
    type Value = ();

    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(KeysAreUnique)
    }
}

impl<'de> serde::de::Visitor<'de> for KeysAreUnique {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("any JSON value")
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let mut seen = BTreeSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(serde::de::Error::custom(format!("{REPEAT_MARKER}{key}")));
            }
            map.next_value_seed(KeysAreUnique)?;
        }
        Ok(())
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while seq.next_element_seed(KeysAreUnique)?.is_some() {}
        Ok(())
    }

    fn visit_bool<E>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_i64<E>(self, _: i64) -> Result<(), E> {
        Ok(())
    }
    fn visit_u64<E>(self, _: u64) -> Result<(), E> {
        Ok(())
    }
    fn visit_f64<E>(self, _: f64) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E>(self, _: &str) -> Result<(), E> {
        Ok(())
    }
    fn visit_unit<E>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_none<E>(self) -> Result<(), E> {
        Ok(())
    }
}

/// The document's own view of a field: the value where the state carries one,
/// and nothing where it does not. A comparison against a field that states its
/// absence is not a comparison, and reporting one would refuse a deposit for
/// being honest about what it does not have.
pub(crate) fn stated<'a>(document: &'a Value, field: &str) -> Option<&'a Value> {
    let record = document.get(field)?;
    match record.get("state").and_then(Value::as_str) {
        Some("present") | Some("estimated") => record.get("value"),
        _ => None,
    }
}

pub(crate) fn length(value: &Value) -> Option<usize> {
    value.as_array().map(Vec::len)
}

/// Every value the pointer reaches, where `*` stands for each element of an
/// array. An empty pointer is the value itself.
pub(crate) fn reach<'a>(value: &'a Value, pointer: &str, out: &mut Vec<&'a Value>) {
    match pointer.split_once('/') {
        None if pointer.is_empty() => out.push(value),
        None => {
            if let Some(next) = step(value, pointer) {
                out.extend(next);
            }
        }
        Some((head, tail)) => {
            if head.is_empty() {
                reach(value, tail, out);
            } else if let Some(next) = step(value, head) {
                for item in next {
                    reach(item, tail, out);
                }
            }
        }
    }
}

fn step<'a>(value: &'a Value, token: &str) -> Option<Vec<&'a Value>> {
    if token == "*" {
        Some(value.as_array()?.iter().collect())
    } else {
        Some(vec![value.get(token)?])
    }
}

/// The rows of a version, keyed by the deposit key each one defines.
pub(crate) fn rows(version_dir: &Path) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    let Ok(entries) = std::fs::read_dir(version_dir.join("fields")) else {
        return out;
    };
    for entry in entries.flatten() {
        let Ok(text) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let Ok(row) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if let Some(field) = row.get("field").and_then(Value::as_str) {
            out.insert(field.to_string(), row);
        }
    }
    out
}
