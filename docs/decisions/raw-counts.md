# Raw counts are the archived quantity

Status: decided. Issue #15.

## The decision

The archived quantity is the counted array as the detector recorded it, together
with the number of shots behind every delay point. Processed forms may be
deposited alongside it. They never replace it, and they are never returned by a
read that asked for the measurement.

Every processing step already applied before the deposit is recorded as an
ordered list with the parameters of each step. Where a step is known to have been
applied but its parameters are not known, the step is recorded with its
parameters stated as unknown. An unknown parameter is never written as a default
value, and a step nobody can describe is recorded as an undescribed step rather
than omitted.

Where the raw array no longer exists, the deposit is still accepted, and it
declares a completeness level that says so. The level is a required field, it
travels with the dataset into every export, and it is the field a benchmark run
reads before it compares anything.

## Why the noise model decides this

Photoelectron counts are Poisson distributed. If the archive holds counts and the
shots per delay point, the uncertainty on every pixel is recoverable arithmetic:
nobody has to have estimated it, and two reanalysts get the same number. That
single property is worth more than every other argument in this record.

Normalise each delay slice and it is gone permanently. The array still looks like
data, the reconstruction still runs, and every statement it makes about how well
it fits now rests on a noise model somebody invented after the fact. Traces with
poor statistics are exactly where reconstruction methods disagree, which is
exactly where an invented noise model does its damage. A method that is
insensitive to noise and a method that is sensitive to it cannot be told apart on
data whose noise has been erased.

Interpolation onto a uniform delay grid is the second irreversible step, and it
is quieter. It moves counts between neighbouring bins and correlates their
errors. A reanalysis that assumes independent pixels is then wrong, and no
inspection of the array reveals it: an interpolated trace looks better than the
one it came from. Background subtraction is the third, because a subtracted
background can drive a bin negative or to zero and both destroy the count
statistics that made the bin readable.

Jitter correction against a fitted centre of mass is the fourth and the most
delicate, because it is often the step that made the measurement publishable. It
is not forbidden. It is recorded, with its parameters, so that a reanalyst can
decide whether to trust it and can see that it happened.

## The completeness levels

Every dataset declares exactly one level.

**counts.** The array holds detector counts as integers, the shots per delay
point are present, and no irreversible step has been applied. Poisson
uncertainties follow from the array itself.

**scaled.** The array is proportional to counts by a factor that is recorded,
either one factor for the whole array or one per delay point, and the shots per
delay point are present. The counted array is recoverable by dividing, so the
noise model survives. This level exists because some acquisition chains hand back
a gain-scaled or averaged array and the scaling is documented; it does not exist
to launder an unrecorded normalisation.

**processed.** At least one irreversible step has been applied and no counted
array survives. The deposit is accepted. The processing list records every step
that is known. A reanalysis using this dataset carries an uncertainty that was
supplied rather than derived, and the level is what says so.

A deposit at level `processed` is not a lesser citizen of the archive and it is
not hidden. It is labelled, in the container, in every listing, and in the export
to the benchmark board. What is refused is a `processed` dataset presented as
`counts`, and the check that refuses a declared level contradicted by the array
itself, integers absent where integers are claimed, or shots absent where they
are required, belongs with the validator in issues #32 and #33.

## Why refusing the processed deposits was rejected

Refusing them would mean refusing most of what has already been published,
because for a large part of the existing literature the processed trace behind
the figure is what survives. An archive that only holds the measurements taken
after it existed cannot be used to check the numbers that motivated it.

Accepting them silently is the worse option and is the one this record exists to
prevent. An archive whose entries look alike and are not is a trap set for the
reanalyst who trusted it, and it is precisely the failure this board is being
built against, reintroduced one level down.

The declared level is the third option and it is designed here rather than added
later, because a level added after there are deposits means reclassifying entries
whose depositors have moved on, from evidence nobody kept.

## Processed forms alongside the raw array

A depositor may include the processed trace they published. It is useful: it is
what a reader will compare against the figure in the paper. It is stored as a
derived form, it carries the ordered list of steps that produced it from the
archived array, and it is marked derived in the container so that no reader can
mistake it for the measurement. Where the steps that produced it cannot be
stated, it is not deposited as a derived form of that array, because a derived
form that cannot be traced back is a second measurement claiming to be the same
one.

## What would overturn this

A detector chain whose native output is genuinely not a count and cannot be
expressed as one, where forcing the `counts` level would mean inventing a
conversion. That case would add a level rather than remove the rule, and it needs
a real instrument behind it rather than a hypothetical.
