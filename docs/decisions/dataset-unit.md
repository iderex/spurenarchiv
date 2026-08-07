# What a dataset is, and what one deposit contains

Status: decided. Issue #11.

## The decision

A **dataset** is one streaking scan: one spectrogram, its delay axis, its energy
calibration, and every other field needed to read those numbers correctly. A
dataset is self-sufficient by construction. Nothing outside it is required to
interpret it.

A **deposit** is one or more datasets recorded on one apparatus in one
configuration, delivered together with a manifest that lists every file in it.
The deposit is the unit that arrives; the dataset is the unit that is cited,
exported and reanalysed.

Both levels are addressable. How an identifier is formed, and what it returns
after a correction, is not settled here; that is issues #37, #38, #39 and #40.
What this record fixes is that a dataset has an identifier of its own and is not
reachable only as an offset into something larger.

## The question that decided it

If a reanalyst downloads one unit, do they have everything the reanalysis needs,
or do they have to go and get a second file?

They have everything. That is the whole reason the scan rather than the session
is the dataset: a unit that sends the reader away for its own calibration is a
unit that will eventually be read without it, and a spectrogram read against the
wrong energy calibration is a plausible wrong answer rather than an error.

## What one deposit is required to contain

- A manifest listing every file, with a checksum per file and the total count.
- At least one dataset.
- For every dataset in it, the complete field set the model requires, including
  the calibrations, repeated in full for each dataset even where two datasets
  share the same calibration.
- The provenance of the deposit as a whole: the instrument, the place, the dates
  the scans were taken, and the software that produced the deposit.

## What one deposit may contain

- Several datasets from the same apparatus and configuration.
- Auxiliary material that is not itself a dataset: a reference spectrum, a
  photograph of the setup, a copy of the acquisition log, a published figure the
  scans stand behind. Auxiliary files are listed in the manifest and are never
  read as measurement.
- Processed forms of a dataset, subject to issue #15, which are marked as derived
  and never stand in place of the archived array.

## Duplication is deliberate, and it has a guard

One calibration measured once and used for forty scans is written forty times.
That is the cost of self-sufficiency and it is paid on purpose. Storage is not
the reason to avoid it; drift is. Two copies of one calibration inside one
deposit can differ by a digit and nothing in the numbers says which is right.

So the deposit carries a rule the validator can enforce: where two datasets in
one deposit state the same field about the same physical thing, the values are
byte-identical, or the deposit declares that the thing changed between the two
scans and when. A silent difference is refused. This is the one place where the
deposit level does work that the dataset level cannot do for itself, and it is
why the deposit exists as a level at all rather than as a shipping convenience.

The refusal and the fixture that proves it bites belong to issues #32 and #33.

## The relationships between scans

A measurement campaign is not a bag of unrelated scans, and the session is where
that survives. The deposit records the order the scans were taken in, the shared
apparatus state, and anything the depositor knows about what changed between
them. A reanalyst asking whether a feature drifts over an afternoon can answer it
from the deposit. A reanalyst asking what one scan means never has to open the
deposit at all.

## The candidates that lost

**The session as the dataset.** Every scan on one apparatus in one configuration
would be one citable unit, with the calibrations stored once. It matches how the
data leaves the laboratory, and it is the cheapest to deposit. It lost on
citation and on concealment. A paper that reanalysed one scan would cite fifty,
and the reader could not tell which. Worse, session-level metadata averages over
within-session variation: a delay-zero drift between the first and last scan
disappears into one number, and the disappearance is invisible in the file.

**Whatever the depositor sends, described by a manifest they write.** This is the
lowest barrier, and the barrier is the thing standing between this archive and
its first outside dataset, so it was the serious contender. It lost because it
makes the archive unable to answer any question across deposits, and answering
questions across deposits is most of why the archive exists. It also moves the
cost rather than removing it: every reuser then reconstructs, per deposit, what
the depositor did not have to state. The barrier is real and the answer to it is
the submission tooling in issues #56 and #57, not an undefined unit.

**One spectrogram with no grouping level at all.** Almost the decision taken, and
it lost on one point only: the relationships between scans would live nowhere,
and the cross-scan consistency guard above would have nothing to run against.

**A deposit is a session and a dataset is a scan, with the calibration stored
once at session level.** This is the hybrid, and it is the shape that looks
efficient. It lost because it breaks the property the whole decision rests on. A
dataset whose calibration lives one level up is not self-sufficient, and the
export to the benchmark board would then have to reassemble it, which is a second
implementation of the reading rules and a second place for them to be wrong.

## What would overturn this

A size ceiling that admits per-shot data, where repeating a calibration block per
dataset is no longer the negligible cost it is for integrated scans, and where
the number of datasets per deposit grows by orders of magnitude. That is not a
reason to move the calibration up a level, but it is a reason to re-argue this
record rather than assume it still holds.
