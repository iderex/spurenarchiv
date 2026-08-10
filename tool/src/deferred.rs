//! The refusals the schema declares and cannot evaluate.
//!
//! `docs/decisions/schema-language.md` requires each one to be listed by
//! identifier in the version's `deferred_checks`, so that somebody writing a
//! second implementation finds them instead of concluding from the schema alone
//! that there are none. This module is the first side of that arrangement.
//!
//! The table below is the only place a deferred check is named here, and both
//! the dispatch and the report of what was not evaluated are read out of it. An
//! operator deleted from the table stops running and starts being disclosed as
//! not evaluated in the same edit, so the two cannot drift apart into a run that
//! covered less than it said it did.

use std::path::Path;

use serde_json::Value;

use crate::{length, reach, rows, stated, Finding, NotEvaluated};

/// What an operator is handed: the parsed document and the directory of the
/// version it declared, for the operators that have to read the version's rows.
type Operator = fn(&Value, &Path, &mut Vec<Finding>);

/// The deferred checks this build evaluates, under the identifiers the schema
/// uses, beside the operator that refuses them.
///
/// An entry carries more than one identifier where one operator answers for
/// both. `edge-coordinates-carry-one-more-value-than-bins` is the case in
/// version 1.0: the delay axis is the only axis whose coordinates are deposited
/// as an array, so the operator that compares those coordinates against the
/// array extent is also the one that knows whether the axis was declared by its
/// edges.
const CHECKS: &[(&[&str], Operator)] = &[
    (
        &["delay-uncertainty-length-matches-delay-values"],
        delay_uncertainty,
    ),
    (
        &[
            "delay-values-length-matches-array-delay-axis",
            "edge-coordinates-carry-one-more-value-than-bins",
        ],
        delay_axis_extent,
    ),
    (&["efficiency-curve-arms-same-length"], efficiency_curve),
    (&["dressing-spectrum-arms-same-length"], dressing_spectrum),
    (&["resolution-curve-arms-same-length"], resolution_curve),
    (&["transmission-curve-arms-same-length"], transmission_curve),
    (
        &["scan-count-agrees-between-the-two-fields-that-carry-it"],
        scan_count,
    ),
    (
        &["shots-per-point-length-matches-delay-values"],
        shots_per_point,
    ),
    (
        &["acquisition-order-is-a-permutation-of-the-delay-points"],
        acquisition_order,
    ),
    (&["bin-widths-length-matches-the-energy-extent"], bin_widths),
    (
        &["integer-field-carries-an-integer-literal"],
        integer_literals,
    ),
];

/// Judged before the document is parsed, which is why it is not an entry in the
/// table: by the time the table is handed a document the repeat is gone.
/// `validate_text` runs it and stops the run where it bites.
const BEFORE_PARSING: &[&str] = &["no-repeated-key"];

/// Every deferred identifier this build evaluates, derived from the table
/// rather than kept beside it.
pub(crate) fn evaluated() -> Vec<&'static str> {
    BEFORE_PARSING
        .iter()
        .copied()
        .chain(CHECKS.iter().flat_map(|(ids, _)| ids.iter().copied()))
        .collect()
}

pub(crate) fn run(document: &Value, version_dir: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (_, operator) in CHECKS {
        operator(document, version_dir, &mut findings);
    }
    findings
}

/// Everything the version declares as deferred that this build does not
/// evaluate, carrying the schema's own sentence about why a schema cannot.
pub(crate) fn not_evaluated(schema: &Value) -> Vec<NotEvaluated> {
    let Some(declared) = schema.get("deferred_checks").and_then(Value::as_array) else {
        return Vec::new();
    };
    let evaluated = evaluated();
    declared
        .iter()
        .filter_map(|entry| {
            let id = entry.get("id")?.as_str()?;
            if evaluated.contains(&id) {
                return None;
            }
            let cannot = entry
                .get("cannot")
                .and_then(Value::as_str)
                .unwrap_or("the schema cannot evaluate it");
            Some(NotEvaluated {
                id: id.to_string(),
                why: format!(
                    "{cannot}, and this route is given one metadata document rather than the deposit directory it sits in"
                ),
            })
        })
        .collect()
}

fn finding(field: &str, problem: String, remedy: String) -> Finding {
    Finding {
        field: field.to_string(),
        problem,
        remedy,
    }
}

fn delay_uncertainty(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    let (Some(values), Some(uncertainty)) = (
        stated(document, "delay_values").and_then(length),
        stated(document, "delay_value_uncertainty").and_then(length),
    ) else {
        return;
    };
    if values != uncertainty {
        out.push(finding(
            "delay_value_uncertainty",
            format!("it carries {uncertainty} values and delay_values carries {values}, so no delay point can be paired with its uncertainty"),
            format!("give one uncertainty per delay point, which is {values} of them"),
        ));
    }
}

/// How many bins the spectrogram declares along one of its axes.
fn axis_bins(document: &Value, axis: &str) -> Option<u64> {
    let order = stated(document, "spectrogram_axis_order")?;
    let index = order
        .as_array()?
        .iter()
        .position(|entry| entry.as_str() == Some(axis))?;
    stated(document, "spectrogram")?
        .get("shape")?
        .as_array()?
        .get(index)?
        .as_u64()
}

/// The delay coordinates against the delay axis of the array. Which of the two
/// identifiers the finding belongs to is decided by the bin convention, so one
/// wrong length is reported once rather than under both.
fn delay_axis_extent(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    let (Some(values), Some(bins)) = (
        stated(document, "delay_values").and_then(length),
        axis_bins(document, "delay"),
    ) else {
        return;
    };
    let edges = stated(document, "axis_bin_convention")
        .and_then(|c| c.get("delay"))
        .and_then(Value::as_str)
        == Some("bin_edge");
    let bins = bins as usize;
    let expected = if edges { bins + 1 } else { bins };
    if values != expected {
        let convention = if edges { "bin_edge" } else { "bin_centre" };
        out.push(finding(
            "delay_values",
            format!("it carries {values} coordinates for a delay axis of {bins} bins declared as {convention}, which wants {expected}"),
            format!("give {expected} delay coordinates, or correct the shape or the bin convention the deposit declares"),
        ));
    }
}

/// A record whose arms are read together, so one arm shorter than another is a
/// curve nobody can evaluate.
fn arms(document: &Value, field: &str, names: &[&str], out: &mut Vec<Finding>) {
    let Some(value) = stated(document, field) else {
        return;
    };
    let present: Vec<(&str, usize)> = names
        .iter()
        .filter_map(|name| value.get(name).and_then(length).map(|n| (*name, n)))
        .collect();
    let Some((_, first)) = present.first().copied() else {
        return;
    };
    for (name, count) in present.iter().skip(1) {
        if *count != first {
            let (first_name, _) = present[0];
            out.push(finding(
                field,
                format!("{name} carries {count} entries and {first_name} carries {first}, so the two cannot be read as one curve"),
                format!("give {name} and {first_name} the same number of entries"),
            ));
        }
    }
}

fn efficiency_curve(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    arms(
        document,
        "detection_efficiency",
        &["energies", "values"],
        out,
    );
}

fn dressing_spectrum(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    arms(
        document,
        "dressing_spectrum",
        &["wavelengths", "spectral_intensities"],
        out,
    );
}

fn resolution_curve(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    arms(
        document,
        "spectrometer_resolution",
        &["energies", "widths", "uncertainty"],
        out,
    );
}

fn transmission_curve(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    arms(
        document,
        "transmission_function",
        &["energies", "values", "uncertainty"],
        out,
    );
}

fn scan_count(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    let read = |field: &str| {
        stated(document, field)
            .and_then(|v| v.get("scan_count"))
            .and_then(Value::as_u64)
    };
    let (Some(combined), Some(scatter)) = (read("scan_combination"), read("inter_scan_scatter"))
    else {
        return;
    };
    if combined != scatter {
        out.push(finding(
            "inter_scan_scatter",
            format!("it says the scatter is across {scatter} scans and scan_combination says {combined} were combined, so a reanalysis cannot tell how many scans the array holds"),
            "make the two scan counts agree, or say which of them the array was built from".to_string(),
        ));
    }
}

fn shots_per_point(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    let (Some(shots), Some(points)) = (
        stated(document, "shots_per_point").and_then(length),
        stated(document, "delay_values").and_then(length),
    ) else {
        return;
    };
    if shots != points {
        out.push(finding(
            "shots_per_point",
            format!("it carries {shots} entries for {points} delay points, so the counts at some points cannot be turned back into a rate"),
            format!("give one shot count per delay point, which is {points} of them, or give a single number that holds at every point"),
        ));
    }
}

fn acquisition_order(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    let (Some(order), Some(points)) = (
        stated(document, "acquisition_order").and_then(|v| v.get("index_sequence").cloned()),
        stated(document, "delay_values").and_then(length),
    ) else {
        return;
    };
    let Some(sequence) = order.as_array() else {
        return;
    };
    let mut seen: Vec<u64> = sequence.iter().filter_map(Value::as_u64).collect();
    let numeric = seen.len() == sequence.len();
    seen.sort_unstable();
    let is_permutation =
        numeric && seen.len() == points && seen.iter().enumerate().all(|(i, v)| *v as usize == i);
    if !is_permutation {
        out.push(finding(
            "acquisition_order",
            format!("index_sequence is not each index of the {points} delay points exactly once, so the order the points were taken in cannot be recovered"),
            format!("list each index from 0 to {} exactly once", points.saturating_sub(1)),
        ));
    }
}

fn bin_widths(document: &Value, _: &Path, out: &mut Vec<Finding>) {
    let (Some(widths), Some(bins)) = (
        stated(document, "spectrogram_value_semantics")
            .and_then(|v| v.get("bin_widths"))
            .and_then(length),
        axis_bins(document, "energy"),
    ) else {
        return;
    };
    let bins = bins as usize;
    if widths != bins {
        out.push(finding(
            "spectrogram_value_semantics",
            format!("bin_widths carries {widths} widths for an energy axis of {bins} bins, so a density cannot be turned back into counts"),
            format!("give one bin width per energy bin, which is {bins} of them"),
        ));
    }
}

/// A field the schema types as an integer, written with a fractional part or an
/// exponent. JSON has one number type and `1.0` satisfies a schema that says
/// integer, so the literal is what has to be judged, and by the time a schema
/// is evaluated the literal is gone.
fn integer_literals(document: &Value, version_dir: &Path, out: &mut Vec<Finding>) {
    for (field, row) in rows(version_dir) {
        let Some(value) = stated(document, &field) else {
            continue;
        };
        let mut pointers = Vec::new();
        integer_pointers(row.get("schema").unwrap_or(&Value::Null), "", &mut pointers);
        for pointer in pointers {
            let mut reached = Vec::new();
            reach(value, &pointer, &mut reached);
            for number in reached {
                if number.is_f64() {
                    out.push(finding(
                        &field,
                        format!("a value this schema types as an integer is written as {number}, with a fractional part or an exponent"),
                        "write it as a plain integer, with no decimal point and no exponent".to_string(),
                    ));
                }
            }
        }
    }
}

/// Where a row types a value as an integer, as pointers into the field's value.
/// `*` stands for each element of an array.
fn integer_pointers(schema: &Value, at: &str, out: &mut Vec<String>) {
    let Some(object) = schema.as_object() else {
        return;
    };
    if object.get("type").and_then(Value::as_str) == Some("integer") {
        out.push(at.to_string());
    }
    if let Some(properties) = object.get("properties").and_then(Value::as_object) {
        for (name, sub) in properties {
            integer_pointers(sub, &format!("{at}/{name}"), out);
        }
    }
    for keyword in ["items", "contains"] {
        if let Some(sub) = object.get(keyword) {
            integer_pointers(sub, &format!("{at}/*"), out);
        }
    }
    for keyword in ["allOf", "anyOf", "oneOf"] {
        if let Some(branches) = object.get(keyword).and_then(Value::as_array) {
            for branch in branches {
                integer_pointers(branch, at, out);
            }
        }
    }
    for keyword in ["then", "else"] {
        if let Some(sub) = object.get(keyword) {
            integer_pointers(sub, at, out);
        }
    }
}
