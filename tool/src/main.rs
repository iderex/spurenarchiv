//! The command over the library. It parses arguments, prints, and decides an
//! exit status; every judgement is the library's, so the two surfaces issue #36
//! asks for cannot disagree about a deposit.

use std::path::PathBuf;
use std::process::ExitCode;

use spurenarchiv_validator::{validate, Report};

const USAGE: &str = "\
usage: deposit-validator <metadata.json> [<metadata.json> ...] [--schema-root <dir>] [--json]

  --schema-root <dir>   where the schema versions are, default ./schema
  --json                write the whole report as JSON instead of as prose

Exit status is 0 when every document was accepted, 1 when any was refused, and
2 when a document could not be judged at all. The status is about conformance
and never about completeness: a deposit that states a field's absence is a
conforming deposit and exits 0.
";

fn main() -> ExitCode {
    let mut documents: Vec<PathBuf> = Vec::new();
    let mut schema_root = PathBuf::from("schema");
    let mut as_json = false;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            "--json" => as_json = true,
            "--schema-root" => match arguments.next() {
                Some(directory) => schema_root = PathBuf::from(directory),
                None => {
                    eprintln!("--schema-root wants a directory after it");
                    return ExitCode::from(2);
                }
            },
            _ => documents.push(PathBuf::from(argument)),
        }
    }

    if documents.is_empty() {
        eprint!("{USAGE}");
        return ExitCode::from(2);
    }

    let mut refused = false;
    let mut unjudged = false;
    let mut judged: Vec<serde_json::Value> = Vec::new();

    for document in &documents {
        let name = document.display().to_string();
        match validate(document, &schema_root) {
            Ok(report) => {
                refused |= !report.accepted();
                if as_json {
                    judged.push(as_object(&name, &report));
                } else {
                    print(&name, &report);
                }
            }
            Err(error) => {
                unjudged = true;
                if as_json {
                    judged.push(serde_json::json!({
                        "document": name,
                        "judged": false,
                        "why": error.to_string(),
                    }));
                } else {
                    println!("{name}: not judged");
                    println!("  {error}");
                }
            }
        }
    }

    if as_json {
        match serde_json::to_string_pretty(&judged) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("the report could not be written as JSON: {error}");
                return ExitCode::from(2);
            }
        }
    }

    if unjudged {
        ExitCode::from(2)
    } else if refused {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// The same report a reader is shown, for something that is going to consume it
/// rather than read it. The verdict is written here rather than left to be
/// inferred from an empty finding list, because an empty list and a run that
/// judged nothing look the same to a caller counting entries.
fn as_object(name: &str, report: &Report) -> serde_json::Value {
    let mut object = serde_json::json!({
        "document": name,
        "judged": true,
        "accepted": report.accepted(),
    });
    if let (Some(map), Ok(serde_json::Value::Object(rest))) =
        (object.as_object_mut(), serde_json::to_value(report))
    {
        map.extend(rest);
    }
    object
}

fn print(name: &str, report: &Report) {
    if report.accepted() {
        println!(
            "{name}: accepted against schema version {}",
            report.schema_version
        );
    } else {
        println!(
            "{name}: refused against schema version {}, {} finding(s)",
            report.schema_version,
            report.findings.len()
        );
        for finding in &report.findings {
            println!("  {}: {}", finding.field, finding.problem);
            println!("    fix: {}", finding.remedy);
        }
    }

    completeness(report);

    // Printed on an accepted document as well. A run that covered less than the
    // whole set must not be readable as one that covered it and found nothing.
    if report.not_evaluated.is_empty() {
        println!("  every check this schema version declares was evaluated.");
    } else {
        println!(
            "  {} check(s) this schema version declares were not evaluated here:",
            report.not_evaluated.len()
        );
        for skipped in &report.not_evaluated {
            println!("    {}: {}", skipped.id, skipped.why);
        }
    }
}

/// What the deposit does not carry, and what that costs a reanalysis. A deposit
/// missing optional fields is valid and is not as good as one that is not, and
/// printing this on an accepted deposit is the only way to say both.
fn completeness(report: &Report) {
    if report.completeness.is_empty() {
        return;
    }
    println!(
        "  {} field(s) carry a state rather than a value:",
        report.completeness.len()
    );
    for absent in &report.completeness {
        println!("    {} is {}", absent.field, absent.state);
        println!("      without it: {}", absent.without_this_field);
    }
}
