# Fixtures for the constraints on the default suite

Five files that are not compiled by anything. Each is a test somebody could
write, and `tool/tests/suite_constraints.rs` judges the text of each against
the three constraints `docs/testing.md` states.

They are not part of the crate on purpose. A test that needs elevation cannot be
admitted to the suite in order to prove that the suite refuses it, and a test
that needs a display would open a window on the machine of whoever ran it. The
whole point of the mechanism is that such a test is refused where it is written
rather than where it runs, so the fixture for it has to be a file rather than a
test case.

`refused-` names the constraint it trips. `accepted-` is a near miss: the
smallest realistic thing next to a refusal that must not be refused. There are
two of those, one for each rule. The loopback one keeps the network rule from
refusing a test that talks to a helper it started, and the other keeps the
declaration rule from refusing a file that merely writes about a display.
