// A test that cannot run without a display server, and the fixture proving the
// default suite refuses one.
//
// It declares the need rather than opening a window. A window opens
// successfully on a machine that has a display, so a fixture that tried would
// pass for whoever wrote it and be refused only on the headless machine, which
// is the wrong way round for a fixture.

// suite-needs: display

#[test]
fn renders_a_spectrogram_and_wants_somewhere_to_put_it() {
    // Nothing here runs. This file is not part of any crate, and the line above
    // is what the suite refuses.
    let bins = 8;
    assert_eq!(bins, 8);
}
