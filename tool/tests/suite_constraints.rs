//! The three constraints `docs/testing.md` states, refused rather than
//! described.
//!
//! Every test in the default suite runs with no display server, with no
//! elevated privileges and with no network. Until this file existed, a test
//! that broke one of the three landed exactly as quietly as one that did not.
//!
//! ## Why the refusal reads source rather than watching a run
//!
//! Two of the three cannot be proved by letting a test do the forbidden thing.
//!
//! A test that asks for administrator rights in order to be caught raises a
//! consent prompt on the machine of whoever runs the suite, on every run rather
//! than once, which is the habit the constraint exists to break. A test that
//! opens a window succeeds on a machine that has a display, so a fixture
//! written that way would pass for the person writing it and be refused only on
//! the headless machine, which is the wrong way round for a fixture.
//!
//! So the refusal happens where such a test is declared. A test that needs a
//! display or elevation says so on a line of its own, and saying so is what is
//! refused. Nothing here elevates anything, opens a window, or touches the
//! network, and the suite's own result does not depend on the machine's network
//! state.
//!
//! The network constraint is the one where the source carries the act itself
//! rather than a declaration, so the fixture for it holds a real call and is
//! refused before anything runs it.
//!
//! ## Loopback is not the boundary
//!
//! The constraint is about a socket to something outside this machine. A test
//! talking to a helper it started on loopback breaks none of the three, so a
//! call site carrying a loopback address is left alone. The direction of the
//! exemption is deliberate: a call whose address is assembled at run time
//! carries no loopback literal and is refused, because an address this file
//! cannot read is an address it cannot vouch for.
//!
//! ## What this vocabulary is written out of fragments for
//!
//! A checker that carries the strings it searches for refuses its own source,
//! and the repair somebody reaches for is an exemption for the checker's file,
//! which is the one file an exemption should never cover. Assembling each
//! needle from pieces keeps the literal out of this text without keeping it out
//! of the comparison.

use std::path::{Path, PathBuf};

/// One thing in a source file that the default suite does not admit.
#[derive(Debug, PartialEq, Eq)]
struct Refusal {
    line: usize,
    /// `display`, `elevation` or `network`.
    constraint: &'static str,
    detail: String,
}

/// The marker a test declares a need with. One line, one word after it.
const DECLARATION: &str = concat!("suite-", "needs:");

/// The three words a declaration may carry.
const DECLARED: [&str; 3] = ["display", "elevation", "network"];

/// Call sites that put a socket on the wire. Not a general list of everything
/// that can reach the network, which is the residual `docs/testing.md` states:
/// a call reached through a helper in another file is invisible here.
fn socket_call_sites() -> [String; 4] {
    [
        concat!("TcpStream", "::connect").to_string(),
        concat!("TcpListener", "::bind").to_string(),
        concat!("UdpSocket", "::bind").to_string(),
        concat!("UdpSocket", "::connect").to_string(),
    ]
}

/// Addresses that name this machine. A call site carrying one of these is
/// talking to something the test itself started.
fn loopback_literals() -> [String; 3] {
    [
        concat!("127.", "0.0.1").to_string(),
        concat!("[:", ":1]").to_string(),
        concat!("local", "host").to_string(),
    ]
}

fn refusals(source: &str) -> Vec<Refusal> {
    let sites = socket_call_sites();
    let loopback = loopback_literals();
    let mut found = Vec::new();

    for (index, line) in source.lines().enumerate() {
        let number = index + 1;

        if let Some(rest) = line.split(DECLARATION).nth(1) {
            let word = rest.split_whitespace().next().unwrap_or("");
            match DECLARED.iter().find(|spelling| **spelling == word) {
                Some(constraint) => found.push(Refusal {
                    line: number,
                    constraint,
                    detail: format!(
                        "this test declares that it needs {word}, and the default suite admits no test that does"
                    ),
                }),
                // A declaration nobody can read is refused rather than ignored.
                // The alternative is that a misspelling silently turns a
                // declared need into an undeclared one.
                None => found.push(Refusal {
                    line: number,
                    constraint: "declaration",
                    detail: format!(
                        "a declaration naming {word:?}, which is none of display, elevation or network"
                    ),
                }),
            }
        }

        if let Some(site) = sites.iter().find(|site| line.contains(site.as_str())) {
            if !loopback
                .iter()
                .any(|address| line.contains(address.as_str()))
            {
                found.push(Refusal {
                    line: number,
                    constraint: "network",
                    detail: format!(
                        "{site} to an address this line does not name as loopback, so the suite's result would depend on something outside this machine"
                    ),
                });
            }
        }
    }

    found
}

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits one level under the repository root")
        .to_path_buf()
}

/// Every `.rs` file under `root`, deepest first, in a stable order.
fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", directory.display()));
        for entry in entries {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// The claim `docs/testing.md` makes about this suite, made by a check rather
/// than by the document.
#[test]
fn no_test_in_the_default_suite_needs_a_display_elevation_or_the_network() {
    let crate_root = repository().join("tool");
    let mut examined = 0;
    let mut refused = Vec::new();

    for path in rust_files(&crate_root.join("src"))
        .into_iter()
        .chain(rust_files(&crate_root.join("tests")))
    {
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        for refusal in refusals(&source) {
            refused.push(format!(
                "{}:{} {}",
                path.display(),
                refusal.line,
                refusal.detail
            ));
        }
        examined += 1;
    }

    assert!(
        refused.is_empty(),
        "the default suite does not admit these:\n{}",
        refused.join("\n")
    );
    // A run that walked nothing and a run that walked the suite print different
    // numbers, so the second cannot be mistaken for the first.
    assert!(
        examined > 0,
        "no source files were found under {}",
        crate_root.display()
    );
}

/// The three fixture tests, and the two near misses that show the refusal is
/// not reaching past what it names.
///
/// None of the five is compiled. A test that needs elevation cannot be admitted
/// to the suite in order to prove it is refused by the suite, which is the
/// circle this fixture directory exists to break.
#[test]
fn each_suite_fixture_gets_the_verdict_its_name_claims() {
    let directory = repository().join("fixtures/suite");
    let mut examined = 0;

    for path in rust_files(&directory) {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a fixture name")
            .to_string();
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        let found = refusals(&source);

        if let Some(rest) = name.strip_prefix("refused-") {
            assert_eq!(
                found.len(),
                1,
                "{name} exists to trip exactly one constraint, and tripped {}: {found:?}",
                found.len()
            );
            let constraint = found[0].constraint;
            assert!(
                rest.contains(constraint),
                "{name} was refused for {constraint}, which its name does not claim"
            );
        } else if name.starts_with("accepted-") {
            assert!(
                found.is_empty(),
                "{name} exists to be accepted and was refused: {found:?}"
            );
        } else {
            panic!("{name} says neither accepted nor refused, so nothing decides its verdict");
        }
        examined += 1;
    }

    assert!(
        examined > 0,
        "no suite fixtures were found under {}",
        directory.display()
    );
}
