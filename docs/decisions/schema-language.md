# The schema language, and how a schema version is stated

Status: decided. Issue #13.

## The decision

The metadata document of a deposit is JSON. The schema that refuses a malformed
one is JSON Schema, draft 2020-12, and it lives in `schema/` as the normative
artefact `layout.md` names.

Those are two decisions and they are taken together on purpose. A schema language
that cannot be applied to the syntax a deposit is written in is a translation
layer, and the translation is a place for the format to stop being what the
document says it is.

The per-field sentence the model requires is a keyword in the schema and not
a comment beside it, and an empty one is refused by a schema over the schema.
That is the line this record was decided on and it is set out in full below.

`container.md` fixed that the metadata is text, one document per dataset, in a
file separate from every array. It handed exactly one criterion here: a reader
with a stock installation of Python and of MATLAB has to be able to parse the
syntax without obtaining a package. This record answers that criterion first,
because it eliminates more candidates than anything else in it.

## Why JSON is the syntax

Measured for Python, at the version on the machine this record was written on:

    python -c "import sys; print(sys.version.split()[0]); [print(m, m in sys.stdlib_module_names) for m in ('json','tomllib','yaml')]"
    3.14.6
    json True
    tomllib True
    yaml False

So a reanalyst with a stock Python reads a JSON document and a TOML document and
does not read a YAML one. That is the criterion applied, and it removes YAML
without any argument about its syntax being needed.

MATLAB is the half this record cannot measure. `jsondecode` is documented as a
built-in function of the base product, which is why JSON survives the criterion
and TOML does not, and there is no MATLAB on the machine this was written on, so
that is a claim and not a measurement. It is the one line of this record that
is owed a command. The specification's read snippets are where it gets paid,
because `container.md` requires the specification to show the read in both
languages instead of asserting that it is easy, and those snippets land with the
deposit layout in issue #31. If a stock MATLAB turns out not to read JSON, it is
this paragraph that was wrong and not issue #31's procedure.

The second reason is smaller and it is not nothing. The metadata document is
hashed as bytes, in the construction `identity.md` fixes, and it is read by a
person in a diff during the review in issue #59. A syntax whose parsers are
everywhere and whose text is line-oriented serves both, and JSON's absence of an
include mechanism, an anchor mechanism and a document-level alias means the bytes
in front of the reviewer are the whole document.

## The keys, and what this record fixes about them

Keys are lower case with underscores. The reason is that the six absence states
in `absence.md` are already written that way, a document mixing two casings gives
a writer a choice to get wrong, and a key that differs from another only by
casing is the kind of defect nothing catches.

Which keys exist is issue #21's and the field-by-field issues under it. This
record fixes only the shape of a key and the one key that has to exist before any
of them can be written, which is the version below.

## The sentence every field carries, and what refuses an empty one

Issue #21 requires each field to carry one sentence naming the reanalysis step
that fails if the field is absent, and the issue is explicit that the sentence is
part of the field's definition and not documentation beside it. Issue #13
asked whether the schema language can refuse an empty one, or whether a separate
check has to and that has to be stated.

JSON Schema can, and this is why it was chosen over the candidate that expresses
cross-field conditions better. A JSON Schema document is itself a JSON document,
so a schema over it is written in the same language, needs no second tool, and is
evaluated by the same evaluator.

The keyword is `withoutThisField`, on the object that defines a field. It is
spelled in the casing JSON Schema's own keywords use, not the casing of
a deposit key, because it is a keyword of the schema and never appears in a
deposit, and a reader who meets it in the wrong place should be able to tell.

`schema/meta/field-definition.schema.json` is a plain JSON Schema over this
board's schema documents. It requires `withoutThisField` on every field
definition, requires its value to be a string, and refuses one that is empty or
that carries no non-whitespace character. Being a schema over schemas rather than
a vocabulary declaration is deliberate: `$vocabulary` tells a third-party
evaluator that the keyword is not decorative, which is worth declaring as well,
but it is not what makes the slot non-empty. What makes it non-empty is an
ordinary schema with `required` in it.

The meta-schema is not versioned with the format. It is a statement about this
repository's own artefacts and not about what a depositor writes, so a
deposit never references it and a new schema version does not produce a new copy
of it.

What no schema refuses is whether the sentence says what the issue asks for. A
field whose slot reads "the delay axis" satisfies every check above and satisfies
nothing else, and no reading of the tree separates a sentence naming a reanalysis
step that fails from a sentence naming the field again. That is a judgement, the
review is where a bad one is caught, and it is worth saying because the check is
easy to mistake for the rule.

Nothing in this tree refuses anything today. The check that applies the
meta-schema is what issue #21 asks for in its own words, and it is owed there
rather than here.

## Cross-field conditions, and the ones that cannot be written in a schema

The common case in this model is conditional requirement: a field required only
when another field takes a particular value. `conventions.md` has several, the
energy calibration required when the energy axis is a kinetic energy produced
from a time of flight being the one it argues at length.

Draft 2020-12 expresses that with `if`, `then` and `else` applied to a
subschema, with `dependentRequired` for the simpler shape where the presence of
one key requires another, and with `allOf` over a list of such conditions so that
each one can be read on its own. It is verbose. Verbosity is the right cost here,
because each condition is then a separate object that a reviewer can read against
the sentence in `docs/model/` that motivated it, rather than one expression doing
several things.

The applicability rules `absence.md` hands to issue #21, where a field in the
`not_applicable` state is refused because another field makes it applicable, are
conditions of exactly this shape and are written the same way.

Three kinds of refusal cannot be written in JSON Schema at all, and naming them
matters more than the ones that can, because `layout.md` makes the schema
normative for what is refused. A condition that compares two values
arithmetically. A condition that depends on the contents of an array, which in
this format is not in the document at all but in a separate file. And a condition
that depends on a file outside the document, such as the array file's length
matching the shape the metadata states, or its checksum matching.

Those are refusals of the validator, and this record does not let them be
invisible in the normative artefact. A schema version carries a
`deferred_checks` array listing each one by identifier, with the prose reference
that specifies it. The array is data and not a JSON Schema keyword, so it
constrains nothing and is not evaluated. What it does is keep the complete
refusal set readable from `schema/`, so that somebody writing a second
implementation from the specification finds the conditions the schema cannot
evaluate instead of concluding from the schema alone that there are none. The
checks themselves belong to issues #32 and #33 with a fixture each.

## The version string, and when it moves

The metadata document carries `schema_version` as a string, two non-negative
integers separated by a full stop, neither with a leading zero. `"1.0"` is the
first. It is a string and not a number because `1.10` and `1.1` are the same
JSON number and are different schema versions.

`schema/1.0/` holds that version's files. `dataset.schema.json` is the schema for
one dataset's metadata document, `deposit.schema.json` for the deposit level that
`dataset-unit.md` made addressable, and `manifest.schema.json` for the manifest
issue #31 settles the shape of. A later version is a new directory beside it and
never an edit to an existing one, which is what lets a reader validate an old
deposit against the exact bytes it was written against. That is the mechanism
issue #34 needs and it is fixed here so that #34 is about the reader rather than
about the storage.

The recommended physical constants `conventions.md` requires as a pinned artefact
are `schema/constants/<release>.json`, named by the release they carry. They are
not schema-versioned, because they are data the specification names rather than a
statement about what a deposit may contain.

What the two numbers mean is a statement about a reader and not about validity.
An old deposit never becomes invalid, because it declares its own version and is
validated against that version's files, so a rule phrased in terms of validity
would say nothing.

The minor number moves when a reader written for an earlier minor of the same
major still reads a deposit correctly for every key it knows. Adding a key is the
ordinary case. Adding an allowed unit or an allowed enumeration member is another,
provided no existing member's meaning moves.

The major number moves when that is not true. A key removed or renamed. A key's
type changed. A key's unit, convention or sign changed. The meaning of a value
changed, including an enumeration member that keeps its spelling and stops meaning
what it meant. A constraint on an existing key narrowed, because a document that
was conforming under the old constraint and is not under the new one is the same
break seen from the writing side.

A reader offered a major it does not carry refuses and names the version. It does
not fall back to the nearest version it has, because a deposit read under a
schema it was not written against is precisely the silent convention confusion
this archive exists against. A reader offered a higher minor of a major it does
carry may read it, and reports the keys it did not recognise instead of dropping
them, for the reason `absence.md` gives about writers that drop what they do not
know.

Inside one version, nothing changes once a release names that version. Before
then the files are a draft and may be corrected. After, a mistake in a landed
version is repaired by a new version and the wrong one stays in the tree, because
a deposit was written against those bytes and a reader has to be able to find
them. The check that refuses an edit to a released version's files is owed and
nothing refuses it today.

The software version and the benchmark export version are two other numbers, they
move for their own reasons, and issue #81 is where a reader is told which of the
three moved in a release. This record does not restate their rules.

## What JSON costs, and what pays for each cost

**No comments.** A depositor cannot annotate their own file. This is smaller than
it looks in this model, because the things a comment would carry have fields:
`absence.md` requires an `estimated` field to carry the basis for its estimate and
a `withheld` field to carry what lifts it, and `raw-counts.md` requires the
processing history. A note in a comment is a fact in a place nothing reads, and
where a fact matters enough to write it matters enough to have a field.

**An integer that is written as a float.** JSON has one number type. JSON Schema's
`"type": "integer"` refuses `1.5` and accepts `1.0`, so a shots-per-point of
`1.0` passes a schema that says integer, and `conventions.md` requires counts and
shots to be integers and not floating point numbers that happen to hold one.
The schema states `"type": "integer"` because it removes the larger half, and the
remainder is a deferred check in the sense above: the validator refuses a numeric
literal for an integer field that carries a fractional part or an exponent. The
element type of the arrays themselves is unaffected, because the arrays are not
in this document at all and `container.md` names their type from a fixed set.

**A duplicate key.** JSON parsers disagree about a repeated key, some taking the
last, some the first, some refusing. For a document that is hashed as bytes and
reviewed by eye, two readers disagreeing about what it says is worse than either
answer. A deposit whose metadata document repeats a key at any level is refused,
and this is a deferred check because a schema is evaluated against a parsed
document and the duplicate is gone by then.

**A big integer.** JSON's number syntax has no bound and several parsers hold
numbers as doubles, so an integer above two to the fifty-third does not survive
every reader. Nothing in this model is expected to reach it. The bound is stated
in the specification and never assumed, so that a field which ever does reach it
is a change somebody makes deliberately.

None of these is a reason to prefer a syntax the reanalyst's stock installation
cannot read. They are stated because each is a place where a valid-looking
document means two things, which is the defect class this repository is about, and
because a record that lists only what a choice buys is a record that will be
believed on the day it should be argued with.

## The candidates that lost

**A NeXus definition, in NXDL.** It would keep one artefact rather than two, which
is the argument issue #13 raised, and it is the right argument against the wrong
starting point. `container.md` refused NeXus as the container for four reasons
that have nothing to do with schemas, so an NXDL definition here would describe a
file this archive does not write, and a schema that does not describe the deposit
cannot be what refuses a bad one. Its per-field slot is a documentation element
with no requirement that it be present or non-empty, so the one thing this record
needed most from a schema language it does not offer.

**CUE.** The serious one, and it lost on less than it deserved. It expresses
cross-field conditions directly rather than through nested `if` and `then`, it
unifies constraints from several places without the `allOf` scaffolding, and a
non-empty annotation is a type in it rather than a schema over a schema, so it
answers issue #13's central question more cleanly than the winner does. It lost
on reach. The normative artefact would then be readable only through a tool, which
is the property `layout.md` exists to prevent, and a depositor's stock Python and
stock MATLAB read neither the schema nor a deposit written in CUE's syntax. The
means question `means.md` sets is the second half. CUE's specification and its
reference implementation are one project written in Go, so a Rust validator either
carries that runtime beside it or implements the language a second time, and
neither is the self-contained single executable `means.md` chose Rust in order to
hand a depositor. Whether a usable Rust evaluator for the language exists today is
not measured in this record, and a name search of the crate registry is not that
measurement. If one does exist, the reach objection above is still the one that
decided this, and it is not answered by a library.

**YAML with a JSON Schema over it.** Comments, less punctuation, and a schema
language that already applies to it once it is parsed. It lost on the measurement
at the top of this record: `yaml` is not in the Python standard library, so the
reanalyst needs a package, which is the criterion `container.md` set. Its implicit
typing is the second reason and it would matter even if the first went away. A
version written unquoted becomes a number, a country code becomes a boolean in
older parsers, and a time written with colons becomes an integer in some. A
repository built against silent convention confusion does not adopt a syntax whose
types depend on which parser read it.

**TOML.** Measured above as stock in the Python that read it, with comments, with
unambiguous integers and floats, and with a spelling of a date that does not
surprise anybody. It lost on two lines. There is no stock TOML reader in
the second required language, so the criterion that removed YAML removes it too.
And the shape fits badly: `absence.md` makes every field a small record carrying a
state and, where the state has one, a value, so the model is uniformly two levels
deeper than it looks, and TOML's array-of-tables syntax at that depth is harder to
read than the braces it was chosen to avoid. It also has no schema language of its
own, so the schema would be JSON Schema over a TOML-to-JSON mapping, and the
mapping is a second place for the format to be defined.

**XML with XML Schema or RELAX NG.** The most expressive of the candidates for
structure, with mature validators and decades of use in exactly the standards
bodies this field's instruments come from. It lost on the same criterion. The
Python standard library parses XML and does not validate against an XML Schema, so
a reanalyst validating a deposit needs a package, and the stock MATLAB story is
worse than JSON's rather than better. The document is also markedly heavier to
read in a diff, which is where issue #59 puts a person.

**A hand-written specification with the validator as the only implementation.**
Named in issue #13 as what happens by default. It is refused by `layout.md` in
advance and in its own words: a format defined by an implementation is the failure
that record's directory boundary exists to prevent, one level down from the
failure this whole archive exists to remove from published results.

## What this record does not settle

Which fields exist, what they are called and what their sentences say. That is
issue #21 and the field-by-field issues under it.

The manifest's contents, the directory arrangement of a deposit and the array file
naming, which are issue #31. This record names the file the manifest's schema goes
in and nothing about what it holds.

How a reader carries several schema versions at once and how it selects one, which
is issue #34. This record fixes the string it selects on and the storage that
makes selection possible.

Whether a Python-facing reader is written by this board, which is entry 6 of issue
#1 and open. Nothing above depends on the answer. The criterion applied here is
about a stranger's stock installation and it is a requirement of the format
whether or not this board ever ships a package.

## What would overturn this

A measurement showing that a stock MATLAB installation does not read JSON. That is
the one claim in this record that is not backed by a command, it is named as such
above, and if it comes back negative then the criterion that removed three
candidates was applied with one side unmeasured and every one of them is back in
the argument.

A cross-field condition that this model genuinely needs, that JSON Schema cannot
express, and that is common enough that most of the refusal set ends up in the
deferred list. The point at which the schema stops being the thing that refuses is
the point at which `layout.md`'s normative artefact is a document with a table of
contents, and the repair is a language that can express the conditions rather than
a longer list.

A depositor population that writes the metadata by hand and finds JSON's
punctuation the thing standing between them and a deposit. The template in issue
#57 is the answer to that and it should be tried first. If the template does not
answer it, the repair is a syntax with a stock reader in both languages, which is
a smaller set than the candidates above and may be empty.
