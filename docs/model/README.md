# The field table, and the sentence every field has to carry

This is the governing document for the model. It states the rule a field has to
pass to be in the model at all, the three requirement states and what each one
means for the validator, where a row lives, and what refuses a row that skipped
the rule. It does not say which fields exist. That is the field-by-field issues
under #21, and each of them adds rows here rather than editing this document.

`docs/decisions/layout.md` is what puts this directory beside `schema/` rather
than inside it, and it is worth reading before adding a field: the schema is
normative for what is refused and the prose is normative for what a field means,
and neither can answer the other's question.

## The sentence

Every field in the model carries one sentence naming the reanalysis step that
fails if the field is absent.

Not a description of what the field means, which is the useless kind of
documentation and the kind that gets written when nobody can think of a reason
for the field. A field that cannot be given that sentence is not in the model.

That rule is the only thing standing between this schema and the usual outcome,
which is a metadata standard with two hundred fields of which eleven are ever
filled in. The pressure to add fields does not arrive as one bad proposal. It
arrives later, one reasonable request at a time, from depositors who want to
record something their apparatus does, and every one of those requests is easy to
grant on its own. The sentence is what a request has to pass, and it is written
down here so that refusing one is a rule being applied rather than a person being
difficult.

The sentence is part of the field's definition and lives in the row, not in a
comment beside it. `docs/decisions/schema-language.md` is where that was decided
and it names the keyword: `withoutThisField`, spelled in the casing JSON Schema's
own keywords use rather than in the casing of a deposit key, because it is a
keyword of the schema and never appears in a deposit.

## The three requirement states

Each row declares one of three, and each means something specific to the
validator rather than something for a person to interpret.

`required` means a deposit without the field is refused.

`optional` means the absence is recorded as one of the absence states in
`docs/decisions/absence.md` and the deposit stands. Absent is not the same as
zero and is not the same as not applicable, which is what that record exists to
keep apart.

`conditional` means required when another field takes a particular value, and it
is the common case in this model rather than the exotic one. An array whose axis
is a kinetic energy requires the calibration that produced it; the same array in
time of flight does not. A row in this state names its condition in
`requiredWhen`, and a row that does not is refused, because a condition that
lives only in somebody's head is a field that is required on the days people
remember.

## The unit and the convention

A row names the quantity it holds. It does not restate the unit.

`docs/decisions/conventions.md` fixes one internal representation per quantity,
so the unit follows from the quantity through that record. A table that restated
it would be a second place for a unit to be wrong, and the two would drift on the
day one of them was corrected. The value of `quantity` is the quantity as that
record names it, or the string `none` where the field is not a physical quantity
at all.

Nothing refuses a quantity that record does not name. Making the check enumerate
the quantities would put the list in a third place and drift against the record
in exactly the way this section exists to avoid, so what catches an invented
quantity is a person reading the row against the record.

## Where a row lives

One field is one file: `schema/<version>/fields/<name>.json`.

A version's dataset schema references a row's `schema` object rather than
carrying a copy of it, so the row is the one place the field is defined and the
dataset schema is where the fields are assembled and where `required` and the
conditional shapes are written.

One file per field is a choice and it has two reasons. A row is then reviewable
on its own, against the sentence that motivated it, rather than as a hunk in the
middle of a document about thirty other fields. And several field issues can land
at once without touching one file, which is not a matter of taste: two efforts
that must edit the same file are one effort, and a table in a single document
would make this whole milestone serial for no benefit to the reader.

A row carries exactly the keys the meta-schema names and no others. A key that is
not one of them is refused rather than ignored, so a misspelt `withoutThisFields`
fails instead of sitting in the file looking like it did something.

A row's `schema` describes the value the field carries and not the record around
it. `docs/decisions/absence.md` puts every field in the deposit whether or not it
has a value, so each one is a small record holding a state and, where the state
has one, the value. That record has the same shape for every field in the model,
so it is written once where a version's fields are assembled rather than copied
into thirty rows, and a row that carried its own copy would be thirty places for
the absence machinery to drift.

## What refuses a row that skipped the rule

`schema/meta/field-definition.schema.json` is a plain JSON Schema over this
board's own schema documents. A JSON Schema document is itself a JSON document,
so the schema over it is written in the same language and evaluated by the same
evaluator, which is the line `docs/decisions/schema-language.md` chose JSON
Schema on.

`.github/workflows/field-sentence.yml` applies it to every tracked row and
reports under the check name `Field definition sentence`. It says how many rows
it examined, so a run over a model with no fields yet cannot be read as a run
that covered the model and found it sound. It fails closed if the listing fails.

The meta-schema is not versioned with the format. It is a statement about this
repository's own artefacts rather than about what a depositor writes, so a
deposit never references it and a new schema version does not produce a new copy
of it.

### What the check cannot do, which is the larger half

It refuses a slot with no non-whitespace character in it. It cannot judge whether
the sentence says what this document asks for. A row whose sentence reads "the
delay axis" passes every check here and satisfies nothing else, and no reading of
the tree separates a sentence naming a reanalysis step that fails from a sentence
naming the field a second time.

That is a judgement about meaning, review is where a bad one is caught, and it is
worth saying plainly because a green check is easy to mistake for the rule being
kept. The fixture `fixtures/field-definition/accepted-shortest-sentence.json`
exists to make the boundary visible: its sentence is one character, it is
accepted, and the check asserts that it is accepted.

## What the table holds today

The count moves, so what stands here is a measurement at a commit rather than a
property of this document:

    git ls-files -- 'schema/*/fields/*.json' | wc -l
    26

Measured at `2b5a4c3`. This paragraph said `0` from the commit it landed on until
the rows arrived, and it went on saying `0` afterwards, which is the drift a
document invites the moment it writes down a number the tree decides. A reader
who wants the count now runs the command, and a reader who wants to know which
fields exist runs it without `wc` rather than looking for a list here.

The rules landed before any row did, which is what this document was owed for.
The rows arrive from the field-by-field issues in this milestone, each adding
files under `schema/<version>/fields/` rather than editing this document. That
ordering is also why the check reports how many rows it examined: a run over an
empty directory and a run over a full one print different numbers, so the first
cannot be read as the second.

## What is not settled here

Which fields exist, what they are called and what their sentences say. Each field
issue in this milestone answers that for its own fields.

How a version's dataset schema is assembled and what else it carries, including
the `deferred_checks` list that `docs/decisions/schema-language.md` requires for
the refusals a schema cannot express. That arrives with the first version's
files.

The deposit's shape on disk and the manifest, which is issue #31.
