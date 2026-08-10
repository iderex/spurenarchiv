// Deliberately broken, to show the format check refuses unformatted source (#5).
// Not for merging.

#[test]
fn a_body_rustfmt_would_rewrite() {
      let value    =   1;
    assert_eq!(value, 1);
}
