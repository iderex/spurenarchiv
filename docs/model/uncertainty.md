# What the archive stores so that a noise model can be rebuilt

A reconstruction without an uncertainty is a picture. The argument for archiving
raw traces is that somebody else can ask how well a method fits the data and how
far the answer moves within the noise, and that question needs a noise model
which this archive either enables or destroys.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document.

## The default case is cheap, and it is why two other fields are required

Counts plus shots per point gives Poisson errors with nothing further stored.

That is why `docs/decisions/raw-counts.md` requires counts as integers and
refuses a normalised array as a replacement, and it is worth reading the two
records together: the noise model is the reason the counts rule exists, and the
counts rule is what makes the cheap case cheap. A trace divided by its maximum
before archiving has lost the noise model, and no field defined here brings it
back.

`noise_model` is required and declares which case a deposit is in. Three answers.
The Poisson default. A detector whose noise is not Poisson. Or not characterised,
which is a statement a reanalysis can act on rather than a gap it has to guess
about.

## The cases the default does not cover

`detector_noise_parameters` is required when `noise_model` says the detector is
not Poisson, and refused as absent in that case. An imaging sensor with read
noise and gain has noise the Poisson assumption does not describe, and applying
that assumption to data it does not fit produces uncertainties that are wrong in
a direction nobody can work out afterwards.

`inter_scan_scatter` is optional. A trace averaged over repeated scans has a
scatter between them, and if it was computed it is worth more than any model
because it was measured. Averaging the scans and keeping only the mean destroys
it, which is a one-line decision in an acquisition script and is not recoverable.

`subtracted_background_variance` is optional and is the other half of the
background field in `detector.md`. A subtraction adds its own variance, and the
amount subtracted is what allows that variance to be added back. Without it every
error bar in the low-count region is too small by an amount nobody can estimate.

## A stored uncertainty carries the method that produced it

`uncertainty_array` is optional, and where it is present it names the method that
produced it. One with no method is refused, and the fixtures under
`fixtures/field-value/uncertainty-array/` prove it.

This is the rule the whole document is for. A single uncertainty array with no
statement of where it came from is the thing to avoid, because a reanalyst cannot
tell an estimated one from a measured one and the two justify very different
claims. An array of Poisson errors computed from the counts, an array propagated
through a detector noise model, an array of measured scan-to-scan scatter and an
array somebody estimated are four different objects that look identical in
storage.

## Correlations, which no per-pixel array can carry

Interpolation and smoothing introduce correlations between neighbouring pixels,
and a per-pixel uncertainty array cannot represent them at all. There is no field
here that fixes that, and inventing one would be worse than saying so.

What the archive does instead is keep the processing history that
`docs/decisions/raw-counts.md` requires, so a reanalyst can see that a smoothing
step happened and treat the per-pixel errors as the lower bound they are. That is
the reason the processing history is part of the record rather than a footnote,
and this section is the second place it earns its keep.

## What no schema here can refuse

That an uncertainty array has the same shape as the spectrogram. It is in a
separate file and is not in the metadata document at all.

That a method declared as `measured_inter_scan_scatter` really was measured. As
with the intensity in `dressing-field.md`, the field records the claim and makes
it visible; it does not verify it.

That a deposit declaring the Poisson default really carries counts rather than
something already normalised. `docs/decisions/raw-counts.md` is where that is
refused, and it is refused on the counts rather than here, which is the right
place for it: one rule in one place, and this document points at it rather than
restating it.
