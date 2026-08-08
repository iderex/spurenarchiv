# Testing

This document is for somebody about to write a test in this repository, or about
to add a fixture for one to run against. The constraints on the default suite are
issue #7's and the fixture rules are issue #9's, and they sit in one document
because the person writing a test meets both in the same minute.

## Nothing in this tree runs a test yet

Said first, because everything below is written before the thing it governs
exists and a reader who assumes otherwise will go looking for a suite that is not
there. At the commit this document lands on:

    git ls-files -- tool/ | wc -l
    0

The tool the suite belongs to is issue #4's and the checks that run it are issue
#5's. Writing the constraints now is the point rather than an accident of
ordering: a suite that has already grown a display dependency does not lose it
again cheaply, and the same holds for a tree that has already grown the habit of
committing real measurement arrays.

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

## What refuses a violation of those three today

Nothing does. There is no suite for such a check to run inside, and the shape the
rule needs is a test that fails loudly rather than one that passes slowly. Issue
#7 holds that mechanism, together with the three fixture tests that prove it
bites: one that needs a display, one that needs elevation and one that opens a
socket to something outside loopback, each refused by the default suite rather
than skipped in silence. Until #7 closes, a test that opens a socket lands
exactly as quietly as one that does not, and this document is what a reader has
instead of a guard.

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
