// Deliberately broken, to show the build check refuses a warning (#5).
// Not for merging.

#[test]
fn a_binding_nobody_reads() {
    let unread_binding = 1;
    assert!(true);
}
