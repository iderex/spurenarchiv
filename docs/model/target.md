# The target

The measured quantity is a photoelectron spectrum and not the XUV pulse. What
sits between them is the atom, so the target decides what the electrons in the
trace are: which bands can exist at all, where they sit, and whether the phase a
reconstruction attributes to the field was put there by the target instead.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document.

## The species, and what else was in the jet

`target_species` is required and is the target as the depositor names it. It is
the field everything else in this document is read against: with no species there
is no shell to assign a band to, and every binding energy a reconstruction
subtracts is a guess.

`target_composition` is optional and carries the components with their fractions
where the jet was a mixture or was known to be contaminated. Water and nitrogen
in a gas line are ordinary rather than exotic, and each produces its own
photolines. A band from a contaminant looks exactly like a band from the target,
so a reanalysis that does not know the composition either leaves it unexplained
or fits it, and fitting it puts structure into the retrieved pulse that the field
never had.

Optional here means what it means everywhere in this model: absent is recorded as
one of the states in `docs/decisions/absence.md` and the deposit stands. A jet
nobody analysed is `not_measured` and a pure sample from a sealed bottle is a
`target_composition` the depositor can state or a `not_applicable`, and those are
different statements to a reanalyst chasing a band.

## The shells, and why an empty list is refused

`target_shells` carries every shell that can contribute at the photon energies
present, each with its ionisation threshold and the source of the accepted value.
Listed, rather than left for the reanalyst to look up, because the list is a
statement about what the depositor believes was in range and the lookup is not.

The threshold is in electronvolts. `docs/decisions/conventions.md` fixes that for
every ionisation threshold in the model and this document does not restate the
unit anywhere else either.

The source is there because two published values for the same threshold differ,
and a reanalysis that subtracts a different one from the same trace gets a
different kinetic energy scale. Naming the value used makes that difference
visible instead of quiet.

An empty list is refused. This is the one refusal this field has and it is worth
being clear about what it catches, because the field is `optional` in the
requirement sense and the two look like a contradiction. They are not. Whether
the key may be absent and what a value may be are separate questions.
`docs/decisions/absence.md` settles the first for every field: the key is present
and carries a value or a state. What `optional` says is that a state is allowed
here, and it is allowed on purpose, because a depositor who has not worked out
which shells could contribute should be able to say `not_measured` and deposit
rather than invent a list. What the schema refuses is the third thing, a list
that is `present` and empty, and that is a depositor who reached the field and
had nothing to put in it. Nothing rules out every shell of a target that produced
the electrons in the array, so an empty list is not a claim anybody could mean.

`fixtures/deposit/refused-empty-shell-list.json` is the deposit that proves it,
and the accepted one beside it differs by having the two shells of the target it
names.

## Density and geometry, which are about space charge

`target_number_density` is optional and carries the number density in the
interaction region with how it was obtained. `target_jet_geometry` is optional
and carries the arrangement, with the nozzle diameter and the interaction length
where they are known.

They are here for one reason. Space charge shifts and broadens a spectrum, the
broadening is smooth and increases with energy, and it mimics a chirp closely
enough that a reconstruction will report one. Nothing in the array separates the
two. What a reanalyst can do is decide whether space charge was plausible at all,
and that decision needs a density and a volume rather than a description of the
apparatus.

A density is one of the numbers in this model most likely to be `estimated`
rather than `present`, because backing pressure and nozzle geometry are what most
setups have and an in-situ measurement is what few have. That is the state to
use, with the basis `absence.md` requires beside it, rather than a number that
reads as measured.

`conventions.md` names both quantities and fixes their internal representation, a
number density in inverse cubic metres and a length in metres, so the unit
follows from the row's `quantity` through that record. Interaction length and
nozzle diameter are lengths.

`docs/decisions/conventions.md` names the quantity for both, the number density
and the lengths the nozzle diameter and the interaction length are. Nothing about
either is derived in this document.

## Resonances, and the phase that is not the pulse's

`target_resonances` is optional and carries any resonance or autoionising state
near the photon energies present, each with its energy and a reference.

This is the field that makes this document more than bookkeeping. A resonance
imposes its own phase on the electron wave packet. A reconstruction that assumes
a flat continuum has nowhere to put that phase except the pulse, so it comes back
as chirp, and the chirp is reported as a property of the light.

The empty list means something here and it means the opposite of what it means in
the shell list. `target_resonances` present and empty is a depositor saying they
checked and there was nothing in range, which is a real and useful claim. A
depositor who did not check writes `not_measured`, and a reanalyst reading that
knows the correction they are not making may be needed. Collapsing the two is how
a resonant phase reaches a published pulse duration, so the schema admits the
empty array here and refuses it in `target_shells`, for reasons that belong to
the fields rather than to a general rule about lists.

## What no schema here can refuse

That a shell list is complete. Nothing in the metadata document says which shells
were in range, and a shell left off the list is indistinguishable from a shell
that cannot contribute. What the field buys is that the depositor's belief is
written down and can be disagreed with.

That a threshold matches the source it names. The value and the citation are two
strings and comparing them is a lookup, not a schema.

That a stated composition is what was in the jet. A residual gas analyser reading
and a hopeful sentence are equally well formed here, and the review in issue #59
is where a number is looked at rather than parsed.

That the resonance list is empty because somebody checked. The state separates
checked from not checked, which is the part a machine can hold. Whether the check
was any good is a judgement, and it is the same shape as the intensity method in
`dressing-field.md`.
