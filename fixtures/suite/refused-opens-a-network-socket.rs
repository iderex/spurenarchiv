// A test that reaches a host which is not this machine, and the fixture proving
// the default suite refuses one.
//
// This is the constraint where the source carries the act rather than a
// declaration, so the call is the real one. It is refused before anything runs
// it, which is why holding it here costs no packet and no network state.
//
// The address is in the range set aside for documentation, so a copy of this
// file that somehow did run would reach nothing.

#[test]
fn asks_a_registry_whether_a_newer_version_exists() {
    let _ = std::net::TcpStream::connect("198.51.100.7:9");
}
