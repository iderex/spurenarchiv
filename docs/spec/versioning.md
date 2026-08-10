# Schema versions, and reading a deposit made before this one

Every deposit was written against the schema version that existed the day it was
made. An archive that can only read its newest schema has quietly stopped being
an archive, so this document says what a version is, what moves it, and what a
reader does with a version it does not carry.

What a version contains is `docs/decisions/schema-language.md`, which fixed the
schema language and what moves a minor against a major. This document is the
reader's half: what a deposit declares, what is done with the declaration, and
what is refused.

## What refuses a rule in this document, and what does not

The refusals below are the library's under `tool/src/`, proved by
`tool/tests/versioning.rs`:

    cargo test --locked --offline --manifest-path tool/Cargo.toml --test versioning
    test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Nothing on this board runs that suite. `git grep -l -i -e cargo -e rustc
-e rust-toolchain -- .github/` exits 1, and the checks that would are issue #5
with that directory as its entire declared scope.

## A deposit declares its version and is read against it

The metadata document carries `schema_version`, and the reader dispatches on
that string. It never infers a version from the fields present, because a
document that is missing a key and a document written against a version that did
not have that key look identical from the fields alone, and one of the two is
wrong.

A version is a directory under `schema/`, holding the exact bytes a deposit made
against it was judged by. A later version is a new directory beside the earlier
one rather than an edit to it, which is what lets an old deposit go on being read
against what it was written against rather than against what that version has
since become.

## Old deposits are read where they are and are never migrated

Migration would rewrite a landed metadata document. `docs/decisions/identity.md`
fixes that the metadata digest is part of a dataset's version identifier, so
rewriting the document moves an identifier a paper may already cite. Reading in
place changes nothing and costs nothing, and the reader carries every shipped
version for exactly this reason.

So a deposit is never brought forward. What changes when a new version lands is
which version a new deposit is written against, and nothing else.

## What a change within a version may do

Nothing that could make a conforming deposit non-conforming, and nothing that
changes what an existing field means.

Adding an optional field is not a within-version change here, and the reason is
worth stating because it is where this model differs from most.
`docs/decisions/absence.md` requires every field the version defines to be
present in a deposit, carrying a value or carrying the state that says why it
does not. Optional means the field may be absent in state, not absent from the
file. So adding a field, however optional, changes the required key set and every
existing deposit would fail against it. That is a new version.

## What is refused, and with what

**A version this build does not carry.** Refused with the version named, and the
document is not judged at all. A reader that fell back to the nearest version it
had would judge a deposit against rules it was not written against and report
success, which is worse than refusing it.

    deposit-validator fixtures/validator/refused-future-schema-version.json
    fixtures/validator/refused-future-schema-version.json: not judged
      schema version 2.0 is not carried here; schema holds the versions this build can judge against

The exit status is 2 rather than 1, and the difference is deliberate. A deposit
that has not been found conforming and has not been found wrong is a third
outcome, and reporting it as either would be a claim the run cannot make.
`docs/spec/validation.md` carries the statuses.

**No version at all.** Refused as its own case, for the same reason one step
earlier.

**Contents that are not the declared version's.** Refused rather than read as far
as it goes. A document declaring 1.0 and carrying a key 1.0 does not define is
what a deposit written against a later version looks like from here, and it is
named for the key rather than passed over:

    fixtures/validator/refused-contents-against-another-version.json: refused against schema version 1.0, 1 finding(s)
      delay_axis_reference_frame: Additional properties are not allowed ('delay_axis_reference_frame' was unexpected)

## What one shipped version can and cannot show

`schema/` carries version 1.0 and nothing else, so this tree cannot demonstrate
against itself that a deposit made under an earlier version still reads. The
property is proved against a schema root built for the test, carrying 1.0 and a
second version that admits and requires one key 1.0 does not define. That second
version is invented for the proof and nothing about it is a proposal.

One document is then judged twice from that root, changing nothing but what it
says it is: accepted under the version that admits the key, refused under the
version that does not, and a deposit written against the earlier version stays
accepted with a later version sitting beside it.

A row judged against the real `schema/` would prove the state of this tree on the
day it ran rather than the guard, which is why the vocabulary is a fixture one.

## What is not settled here

How a depositor discovers which version to write against, and what the archive
does when a deposit arrives against a version that has been superseded but is
still carried, are questions for the intake path rather than for the reader.
Nothing here answers them.
