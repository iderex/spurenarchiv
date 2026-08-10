// The near miss for the declaration rule. This file talks about displays,
// elevation and the network throughout, and declares a need for none of them.
//
// A guard that refused this would be enforcing a vocabulary rather than a
// constraint, and the first casualty would be the document explaining the rule
// and the test that checks it.

#[test]
fn describes_what_a_plotting_library_would_do_on_an_unconfigured_machine() {
    let opened_a_window = false;
    let asked_for_elevation = false;
    assert!(!opened_a_window && !asked_for_elevation);
}
