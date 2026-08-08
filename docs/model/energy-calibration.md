# The energy calibration

Most streaking spectrometers measure a time of flight and almost everything
published is in kinetic energy. The conversion between the two is where a quiet,
whole-trace distortion enters, and once the array has been converted the mistake
is not recoverable from the file.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document.

## Which quantity the second axis is

`energy_axis_quantity` is required and is `time_of_flight` or `kinetic_energy`.
`docs/decisions/conventions.md` fixes that both are legitimate archived axes and
that an undeclared one is refused, and this row is where that declaration lives.

Everything else in this document hangs off it. A reanalysis that reads flight
times as energies, or converts an axis somebody already converted, gets a
spectrum whose numbers are plausible and whose shape is wrong.

## The mapping, which is required when the axis is an energy

`energy_calibration` carries the form of the mapping, its parameters, the
retardation voltage where one was applied, and its uncertainty. It is required
when `energy_axis_quantity` is `kinetic_energy`, which is the condition
`conventions.md` already declares, and refused if absent in that case.

Parameters with a stated form rather than a fitted curve nobody can reproduce.
The form says what the parameters mean, so a reader can apply the mapping again,
apply its inverse, or check it against a photoline, and none of those is possible
from a set of numbers with no equation attached.

The uncertainty is a required key inside the record and not an afterthought
beside it. A reanalysis that treats a fitted energy axis as exact reports a pulse
duration whose error bar is missing its largest term, and the term is missing
silently. A depositor who never quantified it writes `not_quantified`, which is a
statement and takes the same slot, so what is refused is the calibration that
says nothing about its own uncertainty at all.

`calibration_reference` is optional and names the species, the transition, the
accepted value used and where that value came from. It is optional because a
calibration fitted to a manufacturer's curve or to a simulated flight path has no
photoline behind it and is still a calibration. Where there is one, the accepted
value is the number the whole axis is pinned to, and two published values of the
same threshold differ, so two deposits calibrated against the two disagree by
that difference with nothing in either file to show it.

## The Jacobian flag this document does not carry

Issue #24 asks for a field recording whether the Jacobian was applied when the
histogram was converted. There is no such field and the omission is deliberate,
because the flag has a wrong answer available in the ordinary case.

`conventions.md` landed the narrower account. Relabelling bin edges from times of
flight to kinetic energies is lossless and owes no Jacobian at all: the counts in
a bin belong to that bin whatever the axis is called, and the result is counts
per bin on a non-uniform energy grid. The Jacobian is owed by exactly two
operations. Turning counts per bin into a density divides by the width of each
bin in the new axis. Resampling onto a uniform energy grid redistributes counts
between bins and is irreversible in the sense `docs/decisions/raw-counts.md`
uses.

So a correctly relabelled trace would have to declare the Jacobian not applied,
and a reader acting on that flag would apply it and corrupt a file that was
right. What carries the same information without that failure is already in the
model in two other places: what a pixel value means, which
`conventions.md` requires a deposit to declare and which belongs to the
spectrogram in issue #22, and the resample recorded as a step in
`processing_history`, which is `docs/model/provenance.md`.

## Resolution and transmission

`spectrometer_resolution` is optional and carries the instrument width as a
function of energy, with the measure it is stated in and how it was determined.
An instrument width that is not separated from the spectrum is attributed to the
pulse, and it makes the retrieved duration longer by an amount nobody downstream
can subtract.

`transmission_function` is optional and carries the transmission against energy.
A transmission that falls with energy tilts the whole trace, and a tilt across
the energy axis is what a chirp looks like, so a spectrometer property is
reported as a property of the light.

Both carry an optional per-point uncertainty, for the reason the calibration's is
required: a correction applied as though it were exact moves the central value
and leaves the error bar where it was.

Both are optional in the requirement sense, which means an absence is one of the
states in `docs/decisions/absence.md` and the deposit stands. That is the model
saying a trace whose transmission was never characterised is still worth
archiving, provided it says so rather than leaving a reanalyst to assume a flat
response.

## What no schema here can refuse

That `energies`, `widths` and `values` in the resolution and transmission records
have the same length as each other. Comparisons between two arrays, and they are
in `deferred_checks` in `schema/1.0/dataset.schema.json`.

That the parameters match the form they are declared under. `parameters` is an
object whose keys depend on the form, and a schema that enumerated the keys per
form would fix the parameter names of every calibration anyone will ever write.
What the form buys is that a reader knows which equation to read the parameters
into, and a mismatch is caught by trying it.

That a stated calibration is the one that produced the axis. The record is a
description of an arithmetic operation that happened elsewhere, and a deposit
whose axis was produced some other way is exactly as well formed.

That an axis declared as a kinetic energy really is one. This is the same shape
as the optical delay in `docs/model/delay-axis.md`, and it is in
`not_refused_anywhere` for the same reason.
