# Testing

This document is for somebody about to write a test in this repository, or about
to add a fixture for one to run against. The constraints on the default suite are
issue #7's and the fixture rules are issue #9's, and they sit in one document
because the person writing a test meets both in the same minute.

## Where the suite is

Under `tool/`, run by `cargo test`, and reported on every pull request by the
check named `test`. How many files it is spread across is printed rather than
written here:

    git ls-files -- 'tool/tests/*.rs' | wc -l

This document was written before any of that existed and said so in this place,
which was the point rather than an accident of ordering: a suite that has
already grown a display dependency does not lose it again cheaply, and the same
holds for a tree that has already grown the habit of committing real measurement
arrays. Both rules below were in force before there was anything to enforce them
against.

## The three constraints on the default suite

Every test in the default suite runs with no display server, with no elevated
privileges and with no network. A test that cannot meet all three does not get an
exception. It moves to the separate integration harness in issue #8, where its
cost is visible instead of being spread across every run that anybody makes.

### No display

This board renders arrays, and the first thing a plotting library does on an
unconfigured machine is open a window. A suite that has drifted into needing one
fails on a headless machine for a reason that has nothing to do with the code
under test, and whoever reads that failure spends their time on the display
instead of on the defect.

### No elevation

A test that asks for administrator rights trains whoever runs it to grant them,
and that habit is worth more to somebody attacking the machine than anything in
this tree. The second reason is narrower and is the one that actually bites. On
at least one platform a socket bound to a non-loopback address raises a firewall
consent dialog whose subject is the executable path, so answering it settles
nothing beyond that one build directory and every new one asks again.

A test that turns out to need elevation is disclosed as skipped and moved to the
integration harness. It is never worked around by granting the privilege once and
writing the suite as though it had it.

### No network

A suite whose result depends on a remote service reports that service's
availability rather than the state of the code. A red run then means nothing
until somebody has investigated, and the investigation happens every time, which
is how a suite stops being read.

## What refuses a violation of those three

`tool/tests/suite_constraints.rs`, inside the default suite. It reads the source
of every `.rs` file under `tool/src` and `tool/tests` and refuses two things,
naming the file and the line.

A test that declares it needs a display, elevation or the network. The
declaration is one line, `suite-needs:` followed by one of those three words,
and writing it is what is refused. A word that is none of the three is refused
as well, so a misspelling cannot turn a declared need into an undeclared one.

A call site that opens a socket, unless the same line carries an address naming
this machine. A test talking to a helper it started on loopback breaks none of
the three constraints and is left alone. A call whose address is assembled
somewhere else carries no such address on the line and is refused, which is the
safe direction: an address the check cannot read is one it cannot vouch for.

### Why it reads source rather than watching a run

Two of the three cannot be proved by letting a test do the forbidden thing.

A test that asks for administrator rights in order to be caught raises a consent
prompt on the machine of whoever runs the suite, on every run rather than once,
which is the habit the constraint exists to break. A test that opens a window
succeeds on a machine that has a display, so a fixture written that way would
pass for whoever wrote it and be refused only on the headless machine, which is
the wrong way round for a fixture. So the refusal happens where such a test is
declared. Nothing in this mechanism elevates anything, opens a window, or
touches the network, and the suite's result does not depend on the machine's
network state.

The network constraint is the one where the source can carry the act itself, so
its fixture holds a real call and is refused before anything runs it.

### The fixtures

`fixtures/suite/` holds five files that no crate compiles. Three are refused,
one per constraint, and two are near misses that must not be: a test connecting
over loopback, and a file that writes about displays and elevation throughout
and declares a need for neither. The second exists because a guard that refused
it would be enforcing a vocabulary rather than a constraint, and the first thing
it would refuse is this document's own explanation of the rule.

Each rule was deleted and the suite watched. Removing the declaration rule
accepts the display fixture and the elevation fixture. Removing the socket rule
accepts the network fixture. Removing the loopback exemption refuses the
loopback near miss. All three turn `each_suite_fixture_gets_the_verdict_its_name_claims`
red and name the fixture that moved.

### What is still not refused

A socket opened through a helper in another file, or by a crate this tree
depends on. The check reads the text of this tree, and a dependency's own source
is not in it. What covers that is the lock file and the review of what is added,
which is issue #4's, and it is a weaker thing than a check.

A display opened by a library rather than by a call in this tree, for the same
reason.

Elevation attempted at run time by a test that declared nothing. There is no
run-time half of that rule at all, and there is deliberately not going to be
one: observing an elevation request means making it.

A test in the integration harness of issue #8. This check reads `tool/src` and
`tool/tests`, which is the default suite, and the harness is where a test that
cannot meet the three constraints goes so that its cost is visible.

Whether a run happened on a machine that actually had no display. That is a
property of a machine on a day rather than of the tree, and the check makes no
claim about it.

What the toolchain contributes on its own is a starting point and not a guard.
`docs/decisions/means.md` records that the default test runner opens no window,
binds no socket and asks for no privilege by itself, so a test that does any of
the three is a test somebody wrote that way. That is where the constraints start.
It is not what keeps them.

## The three fixture rules

The fixtures here are measurement arrays, which makes this repository different
from one whose fixtures are strings. Once an array is committed it is in the
history for good, and a tree that grows the habit of committing real traces
becomes an archive stored in the wrong system, cloned in full by everybody who
ever contributes a typo fix.

### A fixture is the smallest array that still exercises its property

For most refusals in this model that is a handful of bins rather than a real
scan. A fixture proving that a deposit with no declared delay sign convention is
refused does not need a realistic spectrogram behind it, and a realistic one
makes the fixture harder to read without making the refusal any more certain.

### A generated fixture carries its generator

The generator is tracked beside the fixture, under the same name with the
extension the generating means uses. Nobody should have to guess later what the
numbers in an array were supposed to represent, and a generator is the only
answer to that question that stays true when the array is regenerated.

Nothing refuses a generated fixture whose generator is missing. What catches one
is listing the fixture directory and seeing an array with nothing beside it,
which is why the two files carry the same name.

### A real fixture needs its terms established before it lands

That is a licence question rather than a preference, and it is not settled here.
Entry 2 and entry 3 of issue #1 are what decide whether this repository carries
measurement bytes at all and under what terms, and both are open. Until they are
answered, a real trace does not land in this tree.

There is a second reason to keep fixtures synthetic that has nothing to do with
licensing. A fixture cut from this board's own example deposits proves the state
of the tree on the day it ran rather than proving the guard, and the two are easy
to confuse because both are green. `docs/decisions/layout.md` draws the same line
between `fixtures/` and `examples/` from the other direction.

## The ceiling

A tracked file under `fixtures/` may be at most 65536 bytes.

The number is set in the check rather than here, so that a reader who wants the
value the machine uses can read it from the machine:

    grep -n 'CEILING_BYTES' .github/workflows/fixture-size.yml

Where it came from. A fixture of the size the rule above describes, eight delay
points by sixteen energy bins of eight-byte values, is a thousand bytes and a
little:

    python -c "print(8 * 16 * 8)"
    1024

And the ceiling holds a sixty-four by one hundred and twenty-eight array of the
same values with nothing left over:

    python -c "print(64 * 128 * 8)"
    65536

So the ceiling sits about sixty times above what a fixture here should need. That
is deliberate. A ceiling set at the size of the fixtures that exist today refuses
the next legitimate one and gets raised in a hurry by whoever is blocked, which
is how a ceiling stops meaning anything. This one is headroom around the rule
rather than a target to grow into, and a fixture approaching it is a fixture to
argue with in review even though the check passes it.

## The check

`.github/workflows/fixture-size.yml` refuses a tracked file under `fixtures/`
above the ceiling and names the file and its size. It reports under the check
name `Fixture size ceiling`.

It judges the bytes git holds rather than the bytes on disk, by reading the size
of each tracked blob, so a file that is large only in a working tree is not
refused and a file that is large in the commit cannot be talked out of it
locally. It runs on every push and on every pull request, because a fixture that
enters the history on a branch is in the history whether or not that branch is
ever merged.

It says how many files it examined. A run that walked an empty `fixtures/` and a
run that walked forty fixtures and found them all small print different numbers,
so the second cannot be mistaken for the first. It fails closed: if the listing
or a blob read fails, the run is red rather than green with nothing examined.

### What the check does not do

It does not refuse a generated fixture whose generator is missing, which is the
paragraph above under that rule. It does not judge whether a fixture is the
smallest one that exercises its property, which is a judgement about the property
and belongs to review. It does not look at the total size of `fixtures/`, only at
each file, so a thousand small fixtures pass and the argument against them has to
be made by a person.
