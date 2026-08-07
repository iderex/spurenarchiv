# Where instrument conversion lives

Status: decided. Issue #17.

## The decision

This repository does not convert instrument exports. It parses its own deposit
format and nothing else. There is no vendor format reader in this tree, and
adding one is a change that overturns this record rather than a change that
extends the tool.

The boundary is the deposit format. What a converter has to produce is published
here, as the schema and the validator a depositor can run against their own
output before they send anything.

Conversion tools live outside this board. `iderex/messstube` exists for readers
of instrument formats and is the home for them.

## Why the parsing is refused

Taking conversion on is the shortest path to a depositor saying yes, and that is
a real argument, because the first dataset from an outside group is the milestone
that counts. It was weighed and it lost.

A collection of vendor format readers is a collection of guesses about
undocumented formats. Each guess is a place where a silent misread produces a
deposit that validates, reads, exports and is wrong. A flipped delay sign, a
stage position read as a delay without the factor of two a retroreflector
introduces, a header field that means kinetic energy on one firmware version and
time of flight on the next. None of these produce an error. They produce a
plausible number.

That is the exact failure this archive exists to remove from the published
literature. Putting it inside the archive, in the one component every deposit
passes through, would be the worst possible place to put it. A wrong reanalysis
is one paper. A wrong converter is every deposit it ever touched, all of them
carrying the archive's own assurance.

Refusing it also puts the conversion where the knowledge is. The group that took
the measurement knows what their acquisition program writes. They can describe
the conversion in their methods section, which is a thing they already write, and
the description is then part of the published record rather than a guess made
here on their behalf.

The cost is honest and it is not small: the barrier goes up at the moment the
barrier matters most. The answer to that is the submission guide and the template
in issues #56 and #57, and converters in a repository whose purpose is instrument
formats, not a parser here.

## What a deposit records about its converter

Every dataset records how it was produced from whatever the instrument wrote.

- The converter's name and version, and a reference that pins the exact code:
  a release tag or a commit identifier.
- The invocation, meaning the parameters and options the converter was run with,
  where the converter reports them.
- A checksum of the source export the conversion consumed, so that a
  reconversion can be shown to have started from the same bytes.
- The date the conversion was run.

The source export's file path is not recorded. A path is one of the commonest
carriers of a person's name and of an institution's internal structure, and
nothing in a reanalysis needs it. What is needed is the checksum, and the
checksum does not carry either. The personal data rule this follows is in
`personal-data.md`.

This is the one thing this board holds about conversion, and it exists for one
purpose: when a converter is found to have misread a field, the deposits it
touched can be listed. Without the version pin that list cannot be produced, and
a discovered misread becomes an announcement that some unknown subset of the
archive is wrong.

## A deposit whose conversion cannot be described

It is accepted, and the conversion is recorded as unknown.

This is a choice rather than a default. A researcher with a decade-old trace and
no memory of the script that produced it holds data worth archiving, and refusing
it buys nothing. What is refused is the silence: `unknown` and `none` are
different statements, and a deposit that simply omits the converter block is
refused so that the depositor has to say which one is true.

`unknown` travels with the dataset. It appears in listings and in the export to
the benchmark board, next to the completeness level from `raw-counts.md`, because
a dataset whose conversion cannot be traced cannot be added to or removed from
the affected set when a converter defect is found, and anyone comparing methods
across the archive should be able to see that.

Where the depositor can describe the conversion in prose but cannot pin any code,
the prose is recorded and the converter is still `unknown`. A description is
worth having and it is not a version pin.

## What this board will parse

The deposit format, its manifest, and the metadata that goes with it. That is the
whole list. A request to accept one more instrument format is answered with a
converter in the repository named above, and if that answer is ever wrong, this
record is what has to be argued with first.

## What would overturn this

One instrument format that is openly specified, stable across versions, and
already the export format of most of the groups this archive is trying to reach.
That is a different situation from a collection of guesses, and it would deserve
its own argument. Nothing currently in view is that format.
