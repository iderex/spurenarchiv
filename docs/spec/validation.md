# Validating a metadata document

A depositor should find out that their deposit is wrong from their own machine,
before they send it, and the message should tell them which key is wrong, what is
wrong with it, and what to write instead. That is what this document specifies
and what `tool/` implements.

What is judged here is one dataset's metadata document. The deposit directory
around it, its manifest and its array files are `docs/spec/deposit-layout.md`,
and the checks that read them are named below under what is not evaluated.

## What refuses a rule in this document, and what does not

Said first, because a specification is easy to read as a guarantee.

Everything below is refused by the library under `tool/src/` and proved by the
suite in `tool/tests/`, run with:

    cargo test --locked --offline --manifest-path tool/Cargo.toml

Nothing on this board runs that suite. The checks that would are issue #5's, and
`.github/workflows/` is that issue's entire declared scope:

    git grep -l -i -e cargo -e rustc -e rust-toolchain -- .github/ ; echo "exit=$?"
    exit=1

So a change that breaks a refusal below lands exactly as quietly as one that does
not, unless somebody ran the suite by hand and said so. That is the state issue
#32 exists to end and it is not ended by this document.

## The schema is the check list

`docs/decisions/schema-language.md` made the schema under `schema/<version>/` the
normative artefact. The validator evaluates it rather than restating its
conditions in Rust, so there is one statement of what a deposit must be and not
two that can disagree.

That leaves the conditions JSON Schema cannot express. Those are listed by
identifier in the version's `deferred_checks` array, which is what somebody
writing a second implementation reads instead of concluding from the schema's
silence that there are none. Version 1.0 declares sixteen:

    python -c "import json;print(len(json.load(open('schema/1.0/dataset.schema.json'))['deferred_checks']))"
    16

Thirteen of them are decidable from the metadata document alone and are evaluated
here. Three read a file outside it and are not, which is the next section.

## A refusal names the field, the problem and the remedy

Three parts, always, because the person reading them is a depositor who does not
know this schema. A message that is a stack trace, or a path into an internal
representation, is a message they cannot act on, and one that says what is wrong
without saying what to write instead sends them back to the schema to guess.

    delay_jitter: "delay_jitter" is a required property
      fix: add delay_jitter, carrying a value or carrying the state that says why it does not

The field is the deposit key they would type. A missing key has no path of its
own inside the document, which is the case where naming it is easiest to get
wrong, so it is the one the suite pins by name.

The remedy is decided by the schema keyword that refused, rather than written
beside each condition, so a remedy cannot drift away from the condition it is the
remedy for.

There are no warnings. A warning nobody has to act on trains a depositor to stop
reading the output, and a field that is genuinely optional produces nothing at
all rather than a line saying so.

## Everything found in one run

A depositor who has to make eleven round trips stops after three. Every finding
is collected before anything is printed, so a document with three independent
problems produces three findings and not the first one.

`fixtures/validator/refused-three-independent-problems.json` is that case: a
missing key, a key missing from inside a record, and two fields that disagree
about the same number. It produces three findings in three fields, and the suite
would go red if the run stopped early. Measured by truncating the finding list to
one entry, which turns that test red and nothing else:

    assertion `left == right` failed: expected one finding per problem, got ["delay_jitter"]

The one exception is a repeated key. Parsers disagree about which of the two
values a repeated key leaves, so a document carrying one has no single meaning
for any later check to be about. The run reports the repeated key and stops, and
says in the same output that it stopped.

## What was not evaluated is named, on an accepted document too

A run that covered less than the whole declared set must not be readable as one
that covered it and found nothing. So every run prints the checks the version
declares that it did not evaluate, with the schema's own sentence about why a
schema cannot make them, and it prints that list whether the deposit was accepted
or refused.

Version 1.0's three are the ones that read a file outside the metadata document:
the uncertainty array's shape against the spectrogram's, the array file's length
against the declared shape, and a marker file's presence in the manifest. This
route is handed one document rather than the directory it sits in, so it cannot
answer any of the three, and it says so rather than passing over them.

The list is derived from the dispatch rather than kept beside it. An operator
deleted from the table stops running and starts being reported as not evaluated
in the same edit, so the two cannot drift into a run that covered less than it
said it did.

What that arrangement does not prove is that an operator still refuses what its
identifier names. That is one fixture per refusal with a near miss beside it, and
it is issue #33.

## The version a deposit declares

The deposit declares its schema version and is judged against that version's
bytes. A version this build does not carry is refused with the version named,
rather than read under the nearest version to hand, because a deposit judged
against a schema it was not written against is a deposit judged against the wrong
rules by a run that reported success.

A document that declares no version is refused as its own case, for the same
reason one step earlier.

What is not settled here is the rest of the reader's version behaviour, which is
issue #34 together with `docs/spec/versioning.md`.

## Exit status

    0   every document given was accepted
    1   at least one was refused
    2   at least one could not be judged at all

The third is separate on purpose. A document that is not JSON, or that names a
version this build does not carry, has not been found conforming and has not been
found wrong, and reporting either would be a claim the run cannot make.

## A question this raised for the model

The deferred checks compare `shots_per_point`, `delay_value_uncertainty` and
`acquisition_order.index_sequence` against the length of `delay_values`, and
`delay_values` is compared against the array's delay extent. Where the delay axis
is declared `bin_edge`, `delay_values` carries one more coordinate than the axis
has bins, and each of the other three then wants one entry more than there are
delay points.

That is what the specification says today and it is what the validator does. It
reads wrong: a shot count belongs to a point that was measured, and an edge is not
a point. Recorded here rather than answered, because changing it is a change to
the model in `docs/model/` and not to the validator.

## Running it where a depositor is

`docs/decisions/means.md` chose the language for this property: one downloaded
file, no runtime, no interpreter, no package manager. What has been measured is
that the built binary needs nothing else from the machine it runs on. It was
copied into an empty directory with a copy of `schema/` and two documents, and
run with the environment emptied, no `PATH` among it:

    env -i SystemRoot=C:\Windows ./deposit-validator.exe accepted-complete-deposit.json refused-three-independent-problems.json
    accepted-complete-deposit.json: accepted against schema version 1.0
    ...
    refused-three-independent-problems.json: refused against schema version 1.0, 3 finding(s)

Nothing it depends on opens a socket, which is a reading of the dependency set
rather than an observation of a run:

    cargo tree --locked --offline --manifest-path tool/Cargo.toml --edges normal --prefix none | sort -u | grep -icE 'tokio|hyper|reqwest|curl|ureq|http|socket|tls|async'
    0
    git grep -n -E 'std::net|TcpStream|UdpSocket|std::process::Command' -- tool/src/ ; echo "exit=$?"
    exit=1

What has NOT been measured is the clause issue #32 asks for in those words: a run
on a machine with nothing else installed. This one has the toolchain on it, which
is the machine least able to answer the question. A build host cannot stand in for
a depositor's laptop and the difference is the whole point of the requirement.
