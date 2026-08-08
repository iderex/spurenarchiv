# The delay axis

The delay axis is where the pulse duration comes from, and it is the axis most
often reconstructed from a stage position by whoever plots the figure. Everything
this document defines exists because some part of that reconstruction is not
recoverable from the array once it has been left out.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document. The units are the internal
representation `docs/decisions/conventions.md` fixes for each quantity and are
not restated here.

## The values are explicit, one per column

`delay_values` is an array of delays, one entry per column of the spectrogram. It
is not a start and a step.

Real scans are not uniform. Stages have backlash. Some scans are taken with a
fine region near zero and a coarse one outside it, because that is where the
information is. A scan interrupted and resumed has a gap in it. A start and a
step describes none of that and silently regularises all three, and the
regularised axis looks exactly like a good one.

The array is the delays themselves. Whether the coordinate is a bin centre or a
bin edge is a convention of the same kind and belongs to the spectrogram, which
is issue #22.

## What quantity the numbers are, before anything else

`delay_axis_quantity` says whether the numbers are optical delays or stage
positions. It is declared and never defaulted, and it is first because everything
below reads differently depending on the answer.

A deposit that silently declares stage positions as optical delays is not
detectable from the file. Nothing in the array looks wrong, and the trace is
wrong by the geometry factor on its whole time axis. That is why the declaration
is required rather than inferred, and why the review in issue #59 has a person
checking whether delay zero falls inside the scanned range.

## The sign, the reference point and the zero

Three separate fields, and they are separate because each one can be right while
another is wrong.

`delay_sign_convention` says which subtraction the depositor's axis was recorded
under. `docs/decisions/conventions.md` fixes the internal one as the XUV arrival
time minus the dressing field's reference time, so the archive's own sign is a
consequence of an equation rather than a thing to remember. A deposit in the
other convention is converted at the boundary and its own declaration is kept, so
the record of what the depositor said survives the conversion. Absent is refused.
The two conventions are mirror images, the retrieved chirp changes sign between
them, and a default is precisely how a wrong one enters without anybody choosing
it.

`delay_reference_point` says which feature of the dressing field the delay is
measured from, the peak of its intensity envelope or a maximum of the field
oscillation. The two differ by up to half an optical cycle, they move relative to
each other with the carrier-envelope phase, and a deposit that used a field
maximum has an axis offset by an amount that depends on a quantity most
measurements do not stabilise. Absent is refused.

`delay_zero_definition` is not either of those. It says what physical condition
the value zero corresponds to and how that condition was located, as two
sentences the depositor writes rather than a value from a fixed set. A zero
fitted from the centre of the streaking modulation and a zero taken from a
cross-correlation are not the same point, and a reanalysis that quotes a retrieved
offset against another measurement is comparing two numbers that were referenced
differently unless this field says otherwise.

## The stage relation, and the factor of two

`stage_to_delay_relation` is required when `delay_axis_quantity` is
`stage_position` and refused as absent in that case. Where the axis is already an
optical delay it takes the `not_applicable` state, which is one of the six in
`docs/decisions/absence.md` and is a different statement from not having been
measured.

It carries three things. The geometry factor, the sign, and the stage position
that corresponds to zero delay. The factor is declared rather than fixed at two,
because a retroreflector on a translation stage gives two and geometries where it
is not two exist. What a trace looks like when the factor was left out is the
point: it is wrong by exactly two on its time axis, every duration derived from
it is wrong by two, and nothing about the array is unusual.

Where the depositor has stage positions, both axes are kept, the positions as
recorded and the optical delay as derived, with the relation recorded as the step
that produced one from the other. That is the same rule
`docs/decisions/raw-counts.md` applies to every irreversible step.

## The two uncertainties, which are different quantities

`delay_value_uncertainty` is the uncertainty on each axis value. It shifts the
trace.

`delay_jitter` is the shot-to-shot timing jitter between the XUV pulse and the
dressing field. It blurs the trace. It carries the value and how it was
determined, because a jitter quoted from a specification and a jitter measured on
the day are different claims.

Collapsing the two loses the thing that matters most about this axis. Jitter
cannot be separated from a genuinely longer pulse by any reanalysis of a single
averaged trace, and that is the systematic behind measured durations disagreeing
between laboratories. A deposit that records it lets a reanalysis say how much of
a retrieved duration might be the apparatus. A deposit that does not still says
so, through the absence states, rather than through a plausible number.

Both are optional in the requirement sense, which means absent is recorded as a
state and the deposit stands. Neither may be silently omitted, because
`docs/decisions/absence.md` puts every field in the deposit whether or not it
carries a value.

## What is not in this axis

A scan that was drift-corrected or resampled onto a regular grid records that in
the processing history, not in the axis. An axis that has been quietly repaired
is an axis a reanalysis cannot undo, and `docs/decisions/raw-counts.md` is where
the rule about irreversible steps is argued.

## What no schema here can refuse

Three of them, in the sense `docs/decisions/schema-language.md` sets out, and
they are named so that a second implementer finds them instead of concluding from
the schema that the list is complete.

That `delay_value_uncertainty` has the same length as `delay_values`. It is a
comparison between two arrays and no JSON Schema expresses it.

That `delay_values` has the same length as the spectrogram's delay axis. The
array is in a separate file and is not in the metadata document at all.

That an axis declared as an optical delay really is one. Nothing in the file
distinguishes it from a stage position that was labelled optimistically, which is
the failure the declaration exists to make somebody state rather than one the
declaration removes.
