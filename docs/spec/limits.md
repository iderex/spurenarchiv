# Limits

What reading and validating a deposit may cost in memory, and where each number
comes from.

The distinction this document is about is between a ceiling that is a property
of the format and a ceiling somebody observed once. A note saying that a
two-gigabyte deposit was validated on a machine with sixteen gigabytes proves
nothing about the next deposit. What is written below is derived from the shape
`docs/spec/deposit-layout.md`, `docs/decisions/container.md` and
`docs/decisions/identity.md` fix, and it says in each case whether anything has
been measured.

## Validation

**Peak memory during validation does not grow with the size of any array in the
deposit.**

That is the whole claim, and it is an inequality rather than a number because a
number set against today's fixtures is a number somebody raises later.

It holds because of what the format is rather than because of how a validator is
written. `container.md` writes each array as its values and nothing else, so the
offset of any element is arithmetic on its indices and nothing has to be parsed
to find one. `deposit-layout.md` puts a manifest at the deposit root listing
every file with its size and its SHA-256 digest, so what validation asks about a
file is its length, which is a directory entry, and its digest, which is a
streaming read through a fixed buffer. `identity.md` takes the measurement digest
over a listing of one line per array holding its role, its byte length and its
digest, so the identity construction is bounded by the number of arrays and never
by their size.

So validation holds three things at once: the metadata document, the manifest,
and one read buffer. None of the three grows with an array.

That says what a regression here would actually be. Not a validator that got
slower, but one that started reading an array it had no reason to read. The
array shape check is the place that will be got wrong: `deposit-layout.md` needs
an array file's byte length to match the shape and the element width the metadata
states, and that is a length read from the directory entry rather than a read of
the file. Implementing it as a read that happens to count is the regression,
and it passes every test that only checks the verdict.

### The term that makes it a constant

Two of the three things above are bounded by the format. The third is bounded by
a depositor.

A metadata document is text and `container.md` chose that partly because it has
no ceiling on how much of the model it can carry, which is the right property for
the model. It leaves validation's memory bounded by somebody else's input: a
deposit carrying a gigabyte of metadata conforms to everything written down and
would be read whole by any ordinary parser.

**A metadata document is at most 1048576 bytes.**

Where that comes from. Every key the declared schema version defines is present
in a deposit, which is `docs/decisions/absence.md`'s rule, so a document's size is
decided by the version rather than by how much a depositor chose to say. Version
1.0 defines fifty-five rows and the largest metadata document in this tree is
under twelve thousand bytes:

    git ls-tree -r --name-only HEAD -- schema/1.0/fields/ | wc -l
    git ls-tree -r -l HEAD -- fixtures/deposit examples | awk '{print $4, $5}' | sort -rn | head -3

The cap is about ninety times that. It is headroom around a document that states
every field once, not a target, and a metadata document approaching it is a
deposit to ask about in review even though nothing refuses it. The reason for
that much headroom is `processing_history` and the other rows that carry free
text: they are the keys a real deposit will make long, and a cap set at the size
of the fixtures that exist today is a cap raised in a hurry by whoever is first
blocked by it.

Nothing refuses a metadata document above the cap. The validator reads the
document whole before it knows how large it is, which is the wrong order for
this check, so refusing it is a length read before the parse rather than a
condition inside one. That is a change to the reading path in issue #36 and this
number is what it would refuse against.

### What has been measured

Nothing. No run has been made under an enforced memory limit, and the paragraphs
above are derived from the format rather than from an observation. Issue #35
holds the test, and its second half is the one that decides whether the first
means anything: a deliberately buffering implementation has to fail the same
limit, or a run that passed proves only that the limit was generous.

How that limit is enforced has a constraint from `docs/testing.md` that will
otherwise be discovered late. The default suite runs without elevation, and the
usual ways of capping a process's memory are administrative on at least one
platform this board expects to meet. A test that needs an administrative
mechanism does not get an exception; it moves to the harness in issue #8 where
its cost is visible.

There is a weaker mechanism that needs no privilege at all, and the two prove
different things, so whichever is built has to say which claim it carries. A
reader whose allocations for array data all pass through one place, with a test
asserting that place was never asked for more than the ceiling, proves the reader
did not intend to buffer. It proves nothing about the process's actual footprint.

## A streaming read

**A read of part of an array costs the slice the caller asked for plus a fixed
buffer, and does not depend on the size of the array it came out of.**

`container.md` is why that is available: the offset of any element is arithmetic
on its indices, so one delay slice comes out of a large file with a seek and a
read of that slice.

**There is no reader.** This section is a requirement on the one issue #36
builds rather than a description of anything:

    git ls-tree -r --name-only HEAD -- tool/src/ | wc -l

The crate under `tool/` today judges one metadata document against the schema
version it declares. It opens no array file, so it is not the subject of this
section, and a measurement of its memory would say nothing about a reader.

The distinction matters because the two ceilings above are different kinds of
statement and a document that ran them together would let the second borrow the
first's evidence. Validation's ceiling is derived from a format that exists. A
streaming read's ceiling is a property of an interface that does not.

## What this document does not fix

The largest array a deposit may carry. That is entry 7 of issue #1, which asks
whether the archive accepts single-shot acquisitions and with what ceiling, and
it is open. Nothing above depends on the answer: the claim is that validation's
memory does not grow with an array, which holds at any array size, and it is the
claim rather than a number precisely so that the answer to that entry cannot
invalidate it.

The size of a deposit as a whole, or the number of files in one. Validation holds
the manifest, so its memory grows with the number of entries in it rather than
with their sizes, and no bound on that count is written anywhere. A deposit with
a million auxiliary files is conforming today.

Anything about time. A streaming read of a terabyte is bounded in memory and is
not quick, and nothing here is a statement about how long any of it takes.
