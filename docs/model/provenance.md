# Provenance

Two questions, and the second one is the one that decides whether a reanalysis
is honest. Where did the numbers come from, and what has already been done to
them.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document.

Attribution is not here. Who made the measurement, and what the archive holds
about them, is `docs/decisions/personal-data.md` and issue #41. This document is
about the apparatus, the place, the dates, the code and the arithmetic.

## Where and when

`instrument` and `facility` are optional and are free text at whatever
granularity the depositor can give. `measurement_date` is optional and is a date
written `YYYY-MM-DD`.

They earn their place together and for one reason: they are what lets two
deposits be recognised as the same apparatus, the same beamline or the same week
of beamtime. A systematic that ran through a campaign shows up in every deposit
from it, and without these three it is read as several independent measurements
agreeing with each other, which is the strongest-looking evidence there is and in
that case worthless.

`facility` is a laboratory or a beamline, not a person and not a postal address.
`personal-data.md` is where the line is drawn and what is on the wrong side of
it.

`publication_reference` is optional and is a list of identifiers with their type
rather than a sentence. A reference the archive can resolve is what connects a
trace to the number it stands behind, and a reanalysis that disagrees with that
number has to be able to find it. A list, because one trace can appear in more
than one paper.

## The converter, which is required and may be unknown

`converter` is required and carries the block `docs/decisions/conversion-boundary.md`
fixes: the name and version, a reference that pins the exact code, the invocation
where the converter reports one, a checksum of the source export, and the date
the conversion ran.

The source export's file path is not among them and there is no field for it. A
path is one of the commonest carriers of a person's name and of an institution's
internal structure, nothing in a reanalysis needs it, and the checksum does the
job a path is usually reached for.

The whole point of the block is a list nobody can produce afterwards. When a
converter is found to have misread a field, the deposits it touched have to be
listable. Without the version pin that list cannot be produced and the discovery
turns into an announcement that some unknown part of the archive is wrong.

`unknown` is one of the accepted values of `conversion`, and it is a statement
rather than an absence. A researcher with a decade-old trace and no memory of the
script that produced it holds data worth archiving. What is refused is the
silence: the key is required, so the depositor has to say which of the two is
true. A depositor who can describe the conversion in prose but cannot pin any
code writes the prose in `description` and leaves `conversion` as `unknown`,
because a description is worth having and it is not a version pin.

## The processing history, and why silence is refused

`processing_history` is required and is an ordered list of every step already
applied to the array, each with its parameters. Ordered, because the order
changes the result. A list of applied steps rather than a description, because a
reanalyst has to decide step by step which ones are reversible and account for
the rest.

`docs/decisions/raw-counts.md` fixes what a step looks like when it is not fully
known. A step whose parameters were not recorded is written with its parameters
as `unknown`, never as a default value, because a default is a number somebody
will use. A step nobody can describe at all is written as an undescribed step
rather than left out, so the list length is right even when its contents are not.

The empty list is the strongest claim in this document. It says the array is
exactly what the detector produced, and it has to be something a depositor states
rather than the value a form has when nobody filled it in. That is why this field
is `required` in the requirement sense: the states are refused here, so a deposit
whose processing history is `not_recorded` or `not_measured` does not stand. It
is the one place in this model where an absence state is not an acceptable
answer, and it is not an oversight in the machinery. Both honest answers are
already expressible as values. A depositor who knows nothing was done writes the
empty list. A depositor who knows something was done and nothing about it writes
one undescribed step with unknown parameters. Silence is neither of those, and a
silent processing history read by a reanalyst looks exactly like a raw trace.

`fixtures/deposit/refused-unstated-processing-history.json` proves the refusal
and `fixtures/deposit/accepted-processing-history-with-an-undescribed-step.json`
is the deposit beside it that does the honest thing with the same ignorance.

## What no schema here can refuse

That the processing list is complete. A step somebody forgot and a step that
never happened are the same bytes. The empty list is a claim a reanalyst can
disbelieve, and that is the whole of what the field buys.

That the checksum in the converter block is the checksum of the export the
conversion actually consumed. It is a string here, and the file it names is not
in the deposit at all.

That a code reference resolves to anything. A tag can be deleted and a commit can
live in a repository nobody outside the group can read. The field records what
the depositor pinned, and whether the pin still points anywhere is a question
about the world.

That an instrument or a facility is named the same way twice. Two deposits from
one beamline written with two spellings will not be recognised as one beamline by
anything here, which is the cost of free text and is why these are the fields a
later identifier scheme would replace rather than extend.
