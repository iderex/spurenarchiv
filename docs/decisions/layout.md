# Repository layout, and the line between the specification and the tool

Status: decided. Issue #3.

## The decision

This repository holds two things that age at different rates, and the layout puts
a directory boundary between them so that the difference can be checked by
looking rather than by reading.

The specification is what a group deposits against and what somebody has to be
able to read in fifteen years. It is prose and a machine-readable schema, and it
is complete on its own. A reader can implement a conforming parser from it
without opening the source of the tool, and no sentence in it may refer to the
tool for the meaning of anything.

The tool is software and it will be rewritten. It is the validator, the reader,
the store, the export and whatever else the milestones add. It sits under one
top-level directory, `tool/`, and nothing outside that directory is the tool.

The boundary is a directory rather than a convention because the failure it
prevents is written one sentence at a time. "As the validator accepts" is a
cheap thing to write into a specification and it is the moment the format stops
being defined and becomes whatever the current implementation happens to do,
which is the failure this archive exists to remove from published results, one
level down.

## Which artefact is normative

The schema is normative for what a well-formed deposit is. Where the schema and
the tool disagree, the tool is wrong, and that holds in both directions: a
deposit the schema accepts and the tool refuses is a defect in the tool, and a
deposit the schema refuses and the tool accepts is the same defect facing the
other way. Neither is a reason to edit the schema. A change to the schema is a
change to the format and goes through its own decision and its own version.

Where the prose in `docs/spec/` and `docs/model/` disagrees with the schema, the
schema is normative for what is refused and the prose is normative for what a
field means. Those are different questions and neither document can answer the
other's. A field the prose describes and the schema does not carry is a decision
that has not been implemented. A field the schema carries and the prose does not
describe should not have landed, because the sentence saying what a reanalysis
cannot do without the field is part of that field's definition, and issue #21 is
where that rule is written.

The decision records in `docs/decisions/` are normative for nothing about a
deposit. They record why a thing was chosen. Where a record and the schema
disagree, one of the two is out of date and finding out which is the work.

## The top-level directories

`docs/` holds everything written for a person to read.

`docs/decisions/` holds one file per decision, named after the thing decided
rather than after the issue that decided it, in lower case with hyphens and the
`.md` suffix. Each file opens with a status line naming the issue that produced
it. A landed decision is not edited into saying something different. It is
overturned by a later record that names it and says what changed, so that
somebody in two years can see that a position moved and on what evidence.

`docs/model/` holds the field-by-field model: for each field, its unit, its
requirement state and the sentence naming the reanalysis step that fails without
it.

`docs/spec/` holds the normative prose that is not about a single field: the
deposit on disk, the identifier, versions and corrections, the store, the network
boundary, federation, the export and the limits.

The documents directly under `docs/` are the ones addressed to a particular
reader rather than to the format: installing, submitting, the terms a depositor
grants, the review a deposit passes, testing, citation and releasing. A document
lands there when its audience is a person doing a task and its content would be
wrong to treat as part of the format.

`schema/` holds the machine-readable schema and nothing else. It is the normative
artefact named above, so nothing that is not read by a machine goes in it, and
nothing in it depends on a file under `tool/`.

`fixtures/` holds the inputs the test suite runs against. A fixture exists to
make one property bite and is the smallest thing that does so. It is not a
deposit, it is never cited, and it is never read as an illustration of the
format.

`examples/` holds complete deposits meant to be read as deposits: what a
depositor is shown, what the worked reanalysis runs against. An example is never
used to prove that a guard bites. A guard proved against an example proves the
state of this tree on the day it ran, and the two are easy to confuse because
both are green.

That is the whole distinction between the two directories, and it is a
distinction of purpose rather than of size. A small file in `examples/` is still
an example and a large one in `fixtures/` is still a fixture, though issue #9
holds a ceiling that makes the second rare.

`templates/` holds what a depositor fills in before they send anything.

`tool/` holds the source of the software, its build configuration and its tests.
Its internal structure is whatever the language chosen in issue #2 requires and
is not settled here.

`.github/` holds the repository's own automation.

Alongside these sit the files that convention puts at the root of a repository
and nowhere else, because a reader and a packaging tool both look for them there:
the readme, the notice, the security policy, the contributing guide, the licence,
the citation file and the changelog.

## The rule that decides where a new file goes

One question, asked in this order, and the first answer that fits is the answer.

1. Does it change what a conforming deposit is? It is specification. `schema/`
   if a machine reads it, `docs/model/` if it defines a field, `docs/spec/`
   otherwise.
2. Does it record a decision and the reasons for it? `docs/decisions/`.
3. Is it read by a person in order to do something? `docs/`.
4. Is it an input a test runs against? `fixtures/`.
5. Is it a complete deposit meant to be read as one? `examples/`.
6. Is it something a depositor fills in? `templates/`.
7. Does the software need it to run or to build? `tool/`.
8. Does it configure this repository's own automation? `.github/`.

If no answer fits, the file has no home yet. That is a decision to make in
`docs/decisions/` before the file lands, not a directory to invent while landing
it, because a directory invented in passing is one nobody can state the rule for
afterwards.

## What the tree holds today

Every directory above except `docs/` and `.github/` is declared here before it
exists. Nothing in the tree sits outside the rule, and that is checkable rather
than asserted, at the commit this record lands on:

    git ls-tree -r --name-only HEAD
    .github/workflows/dco.yml
    .github/workflows/dependency-review.yml
    .github/workflows/scorecard.yml
    .github/workflows/unicode-guard.yml
    .github/workflows/zizmor.yml
    NOTICE.md
    README.md
    SECURITY.md
    docs/decisions/conversion-boundary.md
    docs/decisions/dataset-unit.md
    docs/decisions/intake.md
    docs/decisions/layout.md
    docs/decisions/personal-data.md
    docs/decisions/raw-counts.md

The five workflow files are rule 8. The three root files are the convention named
above. The six decision records are rule 2, and their names follow the naming
rule in this document.

A directory that is declared and empty is not created as a placeholder. It
appears when the first file that belongs in it lands, and the listing above is
the evidence that the tree and this document agree today rather than a promise
that they will.

## What this record does not settle

The internal structure of `tool/`, which follows from the language and toolchain
in issue #2.

The schema language, the file names inside `schema/` and how a schema version is
stated, which is issue #13, and the container format a deposit arrives in, which
is issue #12.

Whether `examples/` may ever hold measurement bytes at all. That is entry 3 of
issue #1 and it is open. This record fixes where such bytes would go and what
they would be called if the answer is yes. It does not decide the answer, and the
directory existing is not the answer being given quietly.

The fixture size ceiling and the check behind it, which is issue #9.

## The candidates that lost

**Source at the root with the documents beside it.** What most repositories do,
and it costs nothing to adopt. It lost because the separation this whole record
exists for would then live in a paragraph, and a paragraph is not something a
directory listing can be checked against. It is also the layout in which the
first convenience import from the specification into the tool, or the reverse,
passes review because nothing about it looks wrong.

**The specification in a repository of its own.** The strongest form of the
separation and the one that makes it impossible to cross by accident. It lost on
the pairing problem: a schema change and the validator change that implements it
would land in two places with nothing making them arrive together, so every
reader would have to know which version of one goes with which version of the
other, and that is a thing tracked by hand until the day it is not. The boundary
this record draws is weaker and it is inside one history, where a single change
can move both sides and be reviewed as one thing.

**The schema under `docs/`.** Tempting because the schema and the prose are read
together. It lost because it makes the normative artefact look like
documentation, and the artefact a depositor has to write against should not be
reachable only by walking through prose that is not normative.

**One directory per component, with each component's documents inside it.** It
keeps a change to one component in one place. It lost because a depositor reading
the format would then be walking the tool's tree to find it, which is the reverse
of the property this record is buying.

## What would overturn this

A toolchain that cannot build from a subdirectory, or that can only at a cost
paid on every command an operator runs. That is a constraint on this record and
it belongs to issue #2. If it turns out to hold, the tool's root moves and this
record is overturned by a later one that says so, rather than stretched to cover
a layout it did not choose.

A decision that this repository carries measurement bytes at a scale git is the
wrong tool for, which is entry 3 of issue #1. That would not move the boundary
between the specification and the tool, but it would put the deposits somewhere
this document does not describe, and the rule above would then be answering a
question it was not written for.
