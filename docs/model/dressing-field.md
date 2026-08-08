# The dressing field

The streaking field is half the measurement. Every reconstruction solves for the
XUV pulse given an assumed field, so these numbers are not context around the
result. They are inputs to it, and a reanalysis that cannot see them is repeating
the original analysis rather than checking it.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document. Units are the internal
representation `docs/decisions/conventions.md` fixes for each quantity.

## The wavelength, and the spectrum where it exists

`dressing_centre_wavelength` is the vacuum wavelength of the field, required.
Without it the optical period is unknown and the streaking shift cannot be
related to the vector potential at all.

`dressing_spectrum` is the spectrum itself, as wavelengths and spectral
intensities. It is optional, which means its absence is one of the states in
`docs/decisions/absence.md` and the deposit stands. It matters because a
reconstruction that assumes a single frequency for a broadband few-cycle pulse is
making an approximation whose size only the spectrum reveals, and a reanalyst
with the spectrum can bound the error where one without it can only hope it is
small.

The width of that spectrum is stored in the same quantity as the axis it
describes. `docs/decisions/conventions.md` argues that at length: a bandwidth in
nanometres and the same bandwidth in angular frequency are related by the
derivative at the centre, not by applying the centre's formula to the width, and
the difference is not small for the pulses this archive is about.

## The duration, with the measure it refers to

`dressing_pulse_duration` carries three things and is required. The value, the
measure it is stated under, and how it was determined.

The measure is in the field because a duration is not a number on its own. An
intensity full width at half maximum, an intensity root mean square width and a
field full width at half maximum are three different numbers for one pulse, and a
reconstruction that assumed one while the deposit meant another is wrong by a
factor nobody will look for.

## The intensity, which is the softest number here

`dressing_peak_intensity` is required and it carries its determination method,
not only a value. A deposit giving a value and no method is refused, and the
fixtures under `fixtures/field-value/dressing-peak-intensity/` are what prove it.

The reason is that this number is nearly always inferred rather than measured. It
comes from a focal spot size, a pulse energy and a duration, each with its own
error, or it is read off the streaking amplitude itself, which is circular when
the streaking amplitude is what is being reconstructed. Those two are not the
same claim and no reanalysis can tell them apart from a bare number.

It is also the field the absence states exist for. An intensity that was never
determined takes `not_measured`. One taken as a typical value for the apparatus
takes `estimated` and carries the basis for the estimate, which
`docs/decisions/absence.md` requires and refuses without.

## The polarisation and the geometry

`dressing_polarisation` is required and carries the polarisation state and the
direction as a vector in a stated frame. The detection direction is the other
half of the geometry and is a detector field, defined in `detector.md`, because
it is a property of where the spectrometer looks rather than of the field.

Both are needed together and neither is recoverable from the array.
`docs/decisions/conventions.md` fixes the sign convention relating the momentum
shift to the vector potential, so the archive does not ask a depositor to declare
it, but the observed shift is the projection of that momentum shift onto the
detection direction. A spectrometer looking the other way sees a trace with the
opposite sign, and a detector accepting a wide angular range averages over the
angle and sees a smaller amplitude.

## The carrier-envelope phase

`dressing_carrier_envelope_phase` is required and says whether the phase was
stabilised and how the trace relates to it: tagged shot by shot, taken at one
known phase, or averaged over a distribution.

An averaged trace from an unstabilised field is a different object from a
single-phase trace and reconstructs differently. Analysing the first as the
second yields a systematically distorted pulse, and nothing in the array says
which of the two it is. The absolute orientation of the field is only meaningful
where the phase is stabilised and known, so where it is not, the deposit says so
through this field rather than through a plausible number.

## What no schema here can refuse

That `wavelengths` and `spectral_intensities` have the same length, which is a
comparison between two arrays.

That a peak intensity declared as `focal_spot_energy_duration` really was
obtained that way. The field records what the depositor says the method was, and
a wrong claim here is exactly as well formed as a right one. What it buys is that
a reanalyst reading `streaking_amplitude` knows not to use the number to test a
reconstruction of the streaking amplitude, which is the circularity this field
exists to make visible rather than to prevent.

That the polarisation direction and the detection direction in `detector.md` are
expressed in the same frame. Both name their frame, and comparing two strings is
not the same as knowing two frames are the same one. That is a plausibility check
for the review in issue #59.
