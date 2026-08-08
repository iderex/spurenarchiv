# How absence is expressed, and why unknown is never zero

Status: decided. Issue #16.

## The decision

Every field the model defines is present in a deposit. What varies is whether it
carries a value or a state saying why it does not, and the state is one of a
fixed set written beside the field rather than inferred from a missing key, an
empty string or a number chosen to look unlikely.

There is no sentinel. Not zero, not minus one, not an empty string, not a null,
not a date in 1970 and not a comment. A quantity that was not obtained is a state
that says so in words, and it is the only way this model has of saying it.

## The states

**present.** The field carries a value, and the value came from the measurement
or from the apparatus. A reanalysis uses it.

**estimated.** The field carries a value and the value did not come from a
measurement. A reanalysis may use it and has to carry it as an assumption rather
than as an input, and any uncertainty it quotes has to include the fact that this
number was chosen rather than measured.

**not_measured.** Nobody obtained the quantity. A reanalysis treats the quantity
as unconstrained: whatever it does about a second shell nobody looked for, an
unrecorded detector background or an unknown gas purity, it does knowing there may
be something there that no field accounts for.

**not_applicable.** The quantity has no meaning for this measurement. A
reanalysis stops looking: there is no second band to explain, no stage relation to
apply, no retardation to undo, and an analysis that adds a term for it is adding a
term for something that does not exist.

**not_recorded.** The quantity was known at the time and is now lost. A
reanalysis treats it exactly as `not_measured` for the arithmetic and differently
for everything else: the number existed, so it may be recoverable from a
laboratory notebook, a thesis or the depositor's memory, and a reanalyst who needs
it has somebody to ask.

**withheld.** The quantity exists, is known, and is not being published yet. A
reanalysis knows the archive is not the limit here and that the field will appear
later, so a result computed without it is provisional in a way a result computed
around a `not_measured` is not.

Six states, and the two pairs that look alike are the reason there are six rather
than four. `not_measured` and `not_recorded` differ in whether anybody can be
asked. `not_applicable` and `not_measured` differ in whether the missing thing is
missing from the file or from the world, and collapsing them is how a reanalysis
either invents a term for a shell that cannot contribute or omits one for a shell
nobody checked.

## Why estimated is in this list at all

It is the one state that carries a number, so on a first reading it does not
belong in a record about absence. It is here because of where a reader looks. A
reader that checks for absence by asking whether a number is present will pass
straight over an estimated peak intensity, which is the single most common soft
number in this field and one of the numbers a reconstruction is most sensitive to.
Putting it in the same slot as the absence states means that every route that
handles absence has already had to decide what to do with it, and a route that
forgot it fails to compile rather than treating it as measured.

The value it carries is not enough on its own. A field in the `estimated` state
carries the basis for the estimate as well: what it was derived from, or the
statement that it is a typical value for this apparatus. An estimate with no basis
is refused, because "estimated" with nothing behind it and `not_measured` are the
same statement written to look better.

## What is not a state

A limit is not an absence. A background known to be below a detection threshold,
or a purity stated as better than some fraction, is a value with a one-sided
interval, and it is expressed as a value with its uncertainty rather than as a
state. Making it a state would move a number into the machinery a reader consults
when there is no number, which is the one place it would stop being read.

A value the depositor doubts is not an absence either. It is `present` with an
uncertainty wide enough to say so, and where the doubt is about the convention
rather than the magnitude it is `conventions.md`'s declaration that carries it.

## The encoding

The state sits with the field, not in a structure beside it. Each field that may
be absent is written as a small record carrying its state and, where the state
carries one, its value.

A parallel structure was the alternative and it lost twice. It can disagree with
the field it describes, so a deposit can carry both a number and an entry saying
the number was not measured, and something then has to decide which wins. And it
is droppable: a writer that does not know about the parallel structure emits the
values and none of the states, and every unknown in that deposit silently becomes
a measurement. A state written with its field cannot be separated from it by a
writer that never heard of it, because such a writer cannot produce the field at
all.

The concrete syntax is issue #13's, as `container.md` says. What this record fixes
is the shape: one state per field, adjacent to the value, out of the six names
above and no others.

## Whether a key may be omitted

No. A field defined by the schema version the deposit declares is present in the
deposit, carrying a value or carrying its state.

The reason is the one the issue gives and it survives examination: a missing key
and a writer that did not know the field existed are indistinguishable, so an
omitted key is an unknown of unknown provenance, which is the state this whole
record exists to abolish. A deposit missing a key is refused and the refusal names
the key.

This does not conflict with a deposit written against an older schema. A deposit
declares its schema version, the key set is the one that version defines, and a
key added later is not missing from a deposit that never claimed to carry it. That
is issue #34's mechanism and this record depends on it: without a declared version
the rule above would refuse every deposit the day a field is added.

## What the validator refuses

Each of these is a refusal with a fixture behind it, and the fixtures belong to
issues #32 and #33. Nothing in this tree refuses anything today; the list is what
those issues owe.

A key defined by the declared schema version and absent from the deposit.

A state that is not one of the six names.

A field in a state that carries a value and carrying none, or in a state that
carries no value and carrying one. Both directions, because a `not_measured` with
a number beside it is a deposit whose writer used the state as a comment.

A field in the `estimated` state with no basis recorded.

A field in the `not_applicable` state where another field in the same dataset
makes it applicable. The energy calibration marked not applicable on a dataset
that declares a kinetic energy axis derived from a time of flight is the case this
is written for, and the conditional requirement states in issue #21 are where the
applicability rules live.

A field required by the model in any state other than `present` or `estimated`,
where the model's requirement state says the deposit cannot stand without it. That
is the line between a required field and an optional one, and it is issue #21's to
draw per field rather than this record's.

What the validator cannot refuse is the case this record is named after: a
depositor who writes zero into a slot whose state says `present`. Nothing in the
bytes distinguishes a background that was measured as zero from a background
nobody measured and somebody wrote as zero, and no reading of the array recovers
it. That is why the states exist and why they are required rather than offered,
and it is why the review in issue #59 has a person looking at the numbers. The
mechanism removes the accident. It does not remove the lie, and this record does
not claim it does.

## What travels

The state travels with the field into every listing, every export and every
reader. It is not resolved on the way out, not replaced by a blank, and not
dropped because a table looked untidy. The export to the benchmark board in issues
#52 and #53 carries it, for the same reason the completeness level in
`raw-counts.md` does: a method compared against a dataset whose detector response
was never measured is being compared under a condition, and the condition has to
reach the results table rather than stop at the archive.

## Withheld, and the decision this record does not make

Whether the archive offers embargoed deposits before the first release is entry 5
of issue #1 and is open. Issue #60 holds the design. This record decides only that
the encoding has a name for a field whose value exists and is not being published,
because a model without one forces an embargoed deposit to describe itself as
unmeasured, and that is a false statement written into a permanent record to work
around a missing state.

A field in the `withheld` state carries what lifts it, meaning a date or a named
event, for the same reason `estimated` carries its basis: without it the state is
`not_measured` with better manners.

## What would overturn this

A seventh state that a real deposit needs and that none of the six covers. The
test is the one the issue set: it earns its place only if a reanalysis does
something different when it sees it, and if it cannot be given that sentence it is
a comment rather than a state.

Evidence that requiring every key present makes the first outside deposit harder
in practice rather than in theory. The barrier is real and the template in issue
#57 is the answer to it. If the template turns out not to be the answer, the
repair is a better template, not an omitted key, because the omitted key is the
failure this record is entirely about.
