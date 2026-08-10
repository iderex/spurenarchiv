// Deliberately broken, to show the lint check refuses a clippy warning (#5).
// Not for merging.

#[test]
fn a_length_compared_against_zero() {
    let findings: Vec<String> = Vec::new();
    assert!(findings.len() == 0);
}
