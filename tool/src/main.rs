//! The command over the library. It parses arguments, prints, and decides an
//! exit status; every judgement is the library's, so the two surfaces issue #36
//! asks for cannot disagree about a deposit.

use std::path::PathBuf;
use std::process::ExitCode;

use spurenarchiv_validator::{validate, Report};

const USAGE: &str = "\
usage: deposit-validator <metadata.json> [<metadata.json> ...] [--schema-root <dir>]

  --schema-root <dir>   where the schema versions are, default ./schema

Exit status is 0 when every document was accepted, 1 when any was refused, and
2 when a document could not be judged at all.
";

fn main() -> ExitCode {
    let mut documents: Vec<PathBuf> = Vec::new();
    let mut schema_root = PathBuf::from("schema");
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                return ExitCode::SUCCESS;
            }
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
    for document in &documents {
        match validate(document, &schema_root) {
            Ok(report) => {
                refused |= !report.accepted();
                print(&document.display().to_string(), &report);
            }
            Err(error) => {
                unjudged = true;
                println!("{}: not judged", document.display());
                println!("  {error}");
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
