// A test that cannot run without administrator rights, and the fixture proving
// the default suite refuses one.
//
// It never asks for the privilege. A test that requests elevation in order to
// be caught raises a consent prompt on the machine of whoever runs the suite,
// on every run rather than once, and that is the habit the constraint exists to
// break rather than a way to prove it.

// suite-needs: elevation

#[test]
fn writes_where_only_an_administrator_may_write() {
    // Nothing here runs, and nothing here elevates.
    let path = "a path under a directory an ordinary account cannot write";
    assert!(!path.is_empty());
}
