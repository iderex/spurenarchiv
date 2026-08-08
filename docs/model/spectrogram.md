# The spectrogram

The array is the reason the archive exists, and it is the part most often
published in a form nobody can use. A figure has no numbers. A figure exported as
a colour image has numbers that went through a colour map. What is needed is the
array itself, and the array on its own is not enough to read it: a block of bytes
with no shape is not a measurement, and a block with a shape and no statement of
which index runs over delay is a measurement a reader has to guess at.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document. The units are the internal
representation `docs/decisions/conventions.md` fixes for each quantity and are
not restated here.

`docs/decisions/container.md` already fixed what an array file holds, which is
the values and nothing else, with the shape, the element type and the byte order
in the metadata rather than in a header on the bytes. This document is where
those become fields, and where the conventions that are not recoverable from the
values are settled.

## The array, and the four things needed to read it

`spectrogram` names the file, the shape, the element type and the byte order.

It is one record rather than four fields because none of the four is usable
without the other three. A shape with no element type does not say how many bytes
a pixel is. An element type with no byte order is the one place a read goes wrong
on a machine of the other endianness, and it is silent when it does, which is
`docs/spec/deposit-layout.md`'s own sentence about the same line. A file name with
neither is a pointer to bytes nobody can divide up.

The element type comes from a small fixed set. That is `container.md`'s
requirement rather than this document's preference, and the reason it is a set
rather than a free string is that a reader implements the set: a deposit naming
a type nobody has implemented is a deposit that does not read, and it is better
refused when it is written than discovered when it is cited. An integer array of
counts stays integers, which is what `docs/decisions/raw-counts.md` requires and
what no floating point round trip can promise.

The shape is two lengths, in the order the next section fixes. Two rather than
any number, because `docs/decisions/dataset-unit.md` fixes a dataset as one
streaking scan carrying one spectrogram, and this row types that array as two
dimensional. A measurement that varied a third parameter is not something version
1.0 expresses, and admitting a third index would be a change to what an existing
deposit means rather than an addition to it, which is issue #34's question and
not this document's.

## The axis order, and why one order rather than a declaration of any

`spectrogram_axis_order` says which index runs over delay and which over the
energy or time axis. It is written slowest-varying index first, and version 1.0
admits `["delay", "energy"]` and nothing else.

The first half of that is `container.md`'s: the order is a field rather than a
convention, so a reader reads it instead of assuming it. The second half is this
document's, and it is the part worth arguing.

A transposed trace is not detectable from the values. On a scan that is not
square the reshape fails loudly, which is the harmless case. On a square scan it
produces a trace that reconstructs to a plausible wrong answer, and square scans
exist. So the only moment a transposition can be caught is the moment it is
declared, and a format that admits both orders never catches it: both
declarations are legal, and the deposit that swapped its axes by accident and
adjusted the declaration to match is indistinguishable from the one that meant
it.

Admitting one order costs a depositor whose acquisition program wrote the other
one array rewrite. That is a reordering of the same values with no loss and no
step recorded in the processing history, because nothing about the numbers
changes. What admitting both would cost is larger and lands on every reader
rather than on one depositor: two paths through every implementation, of which
the second is exercised by whichever deposits happen to use it, and
`docs/spec/deposit-layout.md`'s worked read would be correct for some deposits
and quietly wrong for others.

The order this fixes is the one that specification already carries. Its Python
and MATLAB snippets are written against a block with the delay index varying
slowest, so the schema now refuses what the specification already required, and a
deposit written the other way is refused by name instead of read by a snippet
that does not fit it.

The word `energy` names the second axis whatever `energy_axis_quantity` says it
holds. That field already covers a flight-time axis under the same name, and
adding a second spelling here so that a time-of-flight deposit could write
`["delay", "time"]` would make the axis order two strings for one index and give
a reader a case to handle for no information.

## What one value is

`spectrogram_value_semantics` carries two answers, and they are separate
questions that get collapsed.

The first is the quantity in a pixel: counts, a count rate, a mean over the shots
at that point, or arbitrary units left over from an acquisition program. The
distinction that carries the most is between counts and everything else, because
counts are the only one of the four that brings its own statistics with it.
`docs/decisions/raw-counts.md` is where that is argued and
`completeness_level` is where a deposit says whether the array is still counts at
all. This field is not a second copy of that. The level says what survived; this
says what one number means, and a deposit at `counts` whose values are means over
shots is a deposit those two fields disagree about, which is a thing a reanalyst
can see rather than a thing they inherit.

The second is what the value is per: the bin, or a unit of the axis.
`docs/decisions/conventions.md` requires that answer and requires the bin widths
alongside it when a density is declared, because turning counts per bin into
counts per electronvolt is a division by the width of each bin, and a spectrum
divided by nothing has the wrong shape and the right axis labels. The widths sit
inside this record rather than in a field of their own so that the row refuses a
density with no widths, and refuses widths beside a per-bin value in the same
breath. A width beside a value it did not divide is a number a later reader will
use.

`arbitrary_units` is admitted rather than refused. A trace whose absolute scale
was lost in an acquisition program is still a trace worth archiving, and the
alternative to admitting it is a depositor who writes `counts` because the form
would not take the truth. What it costs is stated in the row's sentence and in
`raw-counts.md`: the counting statistics are gone and a benchmark run against
such a dataset is comparing a method against uncertainties somebody supplied.

## Bin centres against bin edges

`axis_bin_convention` says, per axis, whether a coordinate labels the middle of a
bin or its edge. `docs/decisions/conventions.md` named this question, declined to
answer it there and handed it here by name, so that its absence from that record
would not read as an oversight.

It is per axis rather than one answer for both because the two axes acquire their
coordinates differently. Delay points are stage positions the scan visited, which
are centres by construction. Energy bins that came from a digitiser's time bins
have edges, and relabelling those edges from times of flight to kinetic energies
is the lossless conversion `conventions.md` describes. A single field for both
would force one of the two to be restated as the other, which is a conversion
performed to satisfy a form.

Getting it wrong is a half-bin shift. That is not a rounding error on this
measurement: it moves the retrieved delay zero by half a step, and a delay zero
is what a retrieved offset is quoted against when two measurements are compared.
Nothing in the array distinguishes the two conventions, which is why the answer
has to be carried rather than inferred.

An axis declared as `bin_edge` carries one more coordinate than that axis has
bins. That is a comparison between an array's length and a number elsewhere in
the document, so it is not something this schema performs, and it is in
`deferred_checks` under the id `edge-coordinates-carry-one-more-value-than-bins`.

## A pixel that was never measured

`unmeasured_pixel_marker` says how the array marks a pixel that carries no
measurement, and it is required rather than optional.

`docs/decisions/absence.md` puts the rule one level up: an unknown is not a zero.
Inside an array the same defect has no absence state to carry it, because a
pixel is a number and there is nowhere in it to write a state. So the statement
moves to the metadata, and it has to be there whether or not the deposit has any
such pixel, because "every pixel was measured" and "nobody said" are the two
answers a reanalyst most needs told apart. A fit handed an unmarked gap treats it
as a measurement of zero and pulls every parameter that depends on the region it
sits in.

Three answers are admitted. Every pixel was measured. A sentinel value marks the
ones that were not, and the sentinel is written down. Or a companion file carries
a mask, named the way any other array file is named.

The sentinel may be the string `not_a_number` as well as a number. That is not a
convenience: JSON has no way to write a NaN, a NaN is a sentinel a floating point
array can already carry, and a format that could not express it would be telling
those depositors to invent a numeric sentinel that then has to be told apart from
a real reading. The spelling is a string because the alternative is a document
that is not JSON.

An interrupted scan is what this field is for. A scan stopped and resumed has a
block of delay points that were never taken, and whether the array carries zeros
or a sentinel in them is a property of the program that wrote it rather than
something a reader can work out.

## A pixel that saturated

`saturated_pixel_marker` says how the array marks a pixel at or above saturation.
It is required when `detector_saturation` is stated and its `any_pixel_saturated`
is true, and it takes an absence state otherwise.

The condition is what makes it worth having. `detector_saturation` already
carries whether any pixel reached saturation, and a deposit that has answered yes
has admitted the problem and given a reanalyst no way to find it. Requiring the
marker unconditionally would put a field with nothing to say in every deposit
that never saturated, and `README.md` beside this document is explicit that a
field which cannot be given a reason to exist is not in the model.

`not_marked` is one of the answers, and admitting it is deliberate. A depositor
whose array marks nothing has to be able to say so; the alternative is a form
that only accepts one answer, which is a form that gets a false one. What the
condition buys is not that every saturated deposit carries a mask. It is that a
deposit which admitted saturation cannot leave the question of where blank.

`at_or_above_saturation_level` is admitted because it is the honest description
of the common case: the pixels are at the level `detector_saturation` already
carries and are found by comparing against it. It says the marker is the
saturation level rather than a separate value, so a reanalyst knows the
comparison is the whole method and is not looking for a mask that was never
written.

A clipped peak looks like a flat top. It is the failure mode that biases every
fit through it and the one that looks most like data, which is why the field's
sentence is about what a fit does with it rather than about what the pixel is.

## What is not in the spectrogram

The axis coordinates. The delay values are `delay_values` and belong to
`delay-axis.md`, and the energy axis comes from `energy_calibration` and belongs
to `energy-calibration.md`. This document fixes what a coordinate on either axis
means, not what the coordinates are.

The uncertainty array, which is `uncertainty.md`'s. Its shape has to match this
array's and that comparison is already in `deferred_checks`.

Anything that was done to the array. A scan that was drift-corrected, resampled,
background-subtracted or averaged records that in `processing_history`, and
`raw-counts.md` is where the rule about irreversible steps is argued. An array
that was quietly repaired is one a reanalysis cannot undo, and no field in this
document is a place to record a repair.

## What no schema here can refuse

These are in the sense `docs/decisions/schema-language.md` sets out, and they are
named so that a second implementer finds them instead of concluding from the
schema that the list is complete. This section is not the list either. The
entries this change adds to `deferred_checks` and `not_refused_anywhere` in
`schema/1.0/dataset.schema.json` are more than the ones argued here, and that
document is where a reader counts them rather than in a paragraph that drifts
against it.

That the declared axis order matches the bytes. A depositor who wrote the block
transposed and declared the required order is not detectable from anything in the
deposit, and this is the residual that survives fixing the order rather than an
argument against fixing it. What the fixed order removes is the transposition
that announces itself; what stays is the one that does not.

That the coordinates really are centres where centres are declared. Half a bin is
not visible in the numbers.

That `delay_values` has as many entries as the delay side of `shape`. It is a
comparison between an array and a value, which no JSON Schema expresses. Its
`deferred_checks` entry already existed and gave its reason as reading a file
outside the metadata document, which was true while the shape was only in the
array file. The shape is in the document now, so that reason is corrected here
along with the change that made it wrong. The same sentence in `delay-axis.md`
says the array is not in the metadata document at all, and it is now out of date
for the same reason; that file is issue #23's and the correction is left there
rather than made from here.

That the array file holds as many elements as the shape and the element type say
it does. That one does read a file outside the metadata document.

The first two of those are in `not_refused_anywhere`, because they are judgements
about whether a declaration is true and no route will ever make them. The last
two are in `deferred_checks`, because they are refusals a validator performs and
issues #32 and #33 are where they land.
