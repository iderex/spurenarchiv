//! The README example, executed.
//!
//! A README example is the most reliably outdated artefact in a repository,
//! because nothing runs it and everybody reads it. Here the commands and the
//! output are taken out of the document rather than copied into this file, so
//! editing the README to say something the tool no longer does turns this red
//! instead of standing until somebody notices.
//!
//! This is in the default suite and stays inside its three constraints: it opens
//! no window, binds no socket and asks for no privilege. It runs one process,
//! which is the binary this build produced, against files already in the tree.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits one level under the repository root")
        .to_path_buf()
}

/// One command from the README and the output printed beneath it.
struct Example {
    command: Vec<String>,
    expected: String,
}

/// Every `console` block in the README, read as a shell transcript: a line
/// opening with `$ ` is a command and everything under it until the next such
/// line is what that command printed.
fn examples(readme: &str) -> Vec<Example> {
    let mut found = Vec::new();
    let mut inside = false;
    let mut current: Option<Example> = None;

    for line in readme.lines() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.trim_start().starts_with("```") {
            if inside {
                found.extend(current.take());
            }
            inside = line.trim() == "```console";
            continue;
        }
        if !inside {
            continue;
        }
        match line.strip_prefix("$ ") {
            Some(command) => {
                found.extend(current.take());
                current = Some(Example {
                    command: command.split_whitespace().map(str::to_string).collect(),
                    expected: String::new(),
                });
            }
            None => {
                if let Some(example) = current.as_mut() {
                    example.expected.push_str(line);
                    example.expected.push('\n');
                }
            }
        }
    }
    found.extend(current);
    found
}

#[test]
fn the_readme_example_prints_what_the_readme_says_it_prints() {
    let root = repository();
    let readme = std::fs::read_to_string(root.join("README.md")).expect("README.md is tracked");
    let examples = examples(&readme);

    // A run that found no example would pass having executed nothing, which is
    // the green result this test exists to make impossible.
    assert!(
        !examples.is_empty(),
        "no console example was found in README.md, so this test proved nothing"
    );

    for example in &examples {
        let (name, arguments) = example
            .command
            .split_first()
            .expect("a command line has a command on it");
        // The README names the command an operator types. What runs is the
        // binary this build produced, because a test that ran whatever was on
        // the machine would report that machine's state and not this tree's.
        assert_eq!(
            name, "deposit-validator",
            "README.md runs a command this test does not know how to execute"
        );

        let output = Command::new(env!("CARGO_BIN_EXE_deposit-validator"))
            .args(arguments)
            .current_dir(&root)
            .output()
            .expect("the command runs");

        let printed = String::from_utf8(output.stdout)
            .expect("the command prints text")
            .replace("\r\n", "\n");
        let expected = example.expected.replace("\r\n", "\n");

        if printed.trim_end() != expected.trim_end() {
            let mut lines = printed.lines().zip(expected.lines()).enumerate();
            let first = lines.find(|(_, (was, says))| was != says);
            match first {
                Some((n, (was, says))) => panic!(
                    "README.md and the command disagree at line {} of the block:\n  README: {says}\n  ran:    {was}",
                    n + 1
                ),
                None => panic!(
                    "README.md and the command disagree in length: the README shows {} line(s) and the run printed {}",
                    expected.lines().count(),
                    printed.lines().count()
                ),
            }
        }
    }
}
