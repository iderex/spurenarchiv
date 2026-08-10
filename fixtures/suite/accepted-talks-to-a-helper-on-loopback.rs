// The near miss for the network constraint. A test that opens a socket to a
// helper it started on this machine breaks none of the three constraints, and
// refusing it would be the guard reaching past what it names.
//
// Loopback is not the boundary being tested. What the constraint is about is a
// result that depends on something outside this machine, and this call cannot
// leave it.

#[test]
fn reads_from_a_helper_this_test_started() {
    let _ = std::net::TcpStream::connect("127.0.0.1:0");
}
