# The acquisition record

Two spectrograms with identical arrays can be different measurements, and the
difference is in how they were taken. None of it is visible in the array, all of
it changes what the array means, and one of the entries below is the difference
between a chirp and a drift.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document.

## The completeness level, which the rest of this depends on

`completeness_level` is required and is one of `counts`, `scaled` or
`processed`. `docs/decisions/raw-counts.md` defines the three and states that
every dataset declares exactly one, and this row is where that field enters the
model. It sits in this document rather than in one of its own because the
condition below is written against it and a condition against a field that does
not exist is not a condition.

The level is what a benchmark run reads before it compares anything. A method
tested against a `processed` dataset is being tested against uncertainties that
were supplied rather than derived, and that has to reach the results table
instead of stopping here.

## Shots per point, which is conditional on the level

`shots_per_point` is a count or an array of counts, one per delay point, and it
is required when `completeness_level` is `counts` or `scaled`.

The condition is not new work in this milestone. `raw-counts.md` already made the
shots part of what those two levels mean, on the ground that counts plus shots
make the Poisson uncertainty on every pixel recoverable arithmetic, so that
nobody has to have estimated it and two reanalysts get the same number. The
schema writes the same condition it declares.

An array rather than a number is the ordinary case rather than the exotic one.
Shots per point drift with the source across a scan and are often set by a
counter that stops on a target rather than on a shot count.

The refusal that matters is the contradiction rather than the omission. A
deposit declaring `counts` and leaving the shots unstated is claiming both that
the counting statistics survived and that the quantity they are computed from
was never recorded, and those cannot both be true.
`fixtures/deposit/refused-counts-without-shots.json` is that deposit.
`fixtures/deposit/accepted-processed-without-shots.json` is the same file at a
level that makes no such claim, and it stands with the same field unstated,
which is what shows the refusal reaches the contradiction and not the absence.

## The order the points were visited

`acquisition_order` is optional and carries the kind of order, and the visiting
sequence itself where the depositor has it.

This is the entry this document exists for. A scan taken monotonically from
negative to positive delay puts any slow drift onto the delay axis as a linear
term, and a linear term on the delay axis is a chirp. A randomised or interleaved
order spreads the same drift into noise. The two arrays are indistinguishable and
the two conclusions are not.

Absent here has to be one of the states rather than a plausible default, and
which state matters more here than almost anywhere else in the model.
`docs/decisions/absence.md` separates them: a visiting order that was known at
the time and is now lost is `not_recorded`, and a reanalyst reading that has
somebody to ask, which for a scan taken last year is often enough. An order
nobody ever recorded is `not_measured`. A default of monotonic would be a
guess at exactly the fact that decides whether the retrieved chirp is real.

## Repeats and how they were combined

`scan_combination` is optional and carries the number of scans, how they were
combined, and the span the combination covers.

An averaged trace looks like a single measurement. Scans taken hours apart under
a delay zero that moved are broader than any of them, the broadening is in the
delay direction, and it is read as duration. The span is there so a reanalyst can
see how much time the combination reaches across without reconstructing it from
timestamps.

`inter_scan_scatter` carries a `scan_count` of its own, because the scatter is
computed across a number of scans and that number belongs with the statistic. So
the count is in two fields, and a deposit can state two different numbers. No
schema compares two values, so this is in `deferred_checks` in
`schema/1.0/dataset.schema.json` as
`scan-count-agrees-between-the-two-fields-that-carry-it`, and it is the
validator's in issue #32.

## Monitoring and timestamps

`monitoring_channels` is optional and lists what else was recorded during the
scan, each with what it was recorded per: a shot, a delay point or a scan. A
pulse energy or a reference spectrum per point is what lets a reanalyst normalise
a source drift out instead of assuming there was none, and the assumption is the
one that gets made when the field is empty.

`acquisition_timestamps` is optional and carries the start instant and, where
they exist, the offsets of the delay points from it. The offsets are seconds, as
`docs/decisions/conventions.md` fixes for every duration in this model. The start
is a calendar instant rather than a duration, so it is written as a UTC timestamp
and the record's rule about seconds does not reach it.

They earn their place together: with a monitor per point and an offset per point,
a feature that tracks the time of day rather than the delay can be recognised as
one. With neither, it cannot.

## What no schema here can refuse

That `shots_per_point`, given as an array, has one entry per delay point. It is a
comparison between two arrays and it is in `deferred_checks` as
`shots-per-point-length-matches-delay-values`.

That `acquisition_order.index_sequence` is each delay index exactly once. Same
kind of comparison, same list, as
`acquisition-order-is-a-permutation-of-the-delay-points`.

That a declared `randomised` order was randomised. The field records what the
depositor says the acquisition program did, and a wrong claim here is as well
formed as a right one. What it buys is that the claim exists and can be asked
about.

That a monitoring channel named here has a file behind it that means what its
name says. The file is not in the metadata document and neither is its content.
