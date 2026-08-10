// Deliberately broken, to show the test check refuses a failing suite (#5).
// Not for merging.

#[test]
fn a_verdict_that_is_not_the_one_asserted() {
    let got = "accepted".to_string();
    let want = "refused".to_string();
    assert_eq!(got, want);
}
