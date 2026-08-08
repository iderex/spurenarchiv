# A measured trace carries no ground truth

Status: decided. Issue #20.

## The decision

No field in this model asserts the true pulse behind a measured trace. Not under
that name, not under a name that means it, and not as a field that a reader would
reasonably use as one. The reason is not caution. A measured trace has no known
pulse, because nobody generated it from one, and any pulse associated with it is
the output of a method carrying that method's assumptions.

A published reconstruction is worth archiving and is archived, as a claim, in its
own part of the record, with everything a reader needs to judge it.

A generated trace is a different case and the model keeps it different. A
synthetic dataset was produced from an input pulse, that pulse is a property of
the generator and is known exactly, and it is stored with the generator rather
than as a field on a measurement. A dataset is either measured or generated, the
two carry different fields, and the field holding an input pulse exists only on
the generated one. That is a distinction the schema can refuse a violation of,
which is worth more than a rule saying not to fill the field in.

## Where this bites, and why it has to be decided now

A benchmark scoring methods against a field called truth will score them against
whatever is in it. If what is in it is the output of one particular method, the
benchmark measures agreement with that method, and a method reproducing its
assumptions scores well for doing so. The board this archive feeds exists to
detect a method that works only inside the assumptions of the simulation, and a
truth field filled from a reconstruction would rebuild that failure on the real
data too, which is the one place it was supposed to be impossible.

It is decided now because it shapes the model rather than the software. A field
added later can be left empty. A field that was never there cannot be filled in by
somebody who assumed it was meant to be, and the empty field is the one that gets
filled with the nearest available number.

## How a published reconstruction is stored

As a reconstruction claim, in a part of the record that is not the measurement.
There may be several for one dataset, including several by the same group, and
they are a list rather than a slot, because a slot forces a choice of which one is
the answer.

Each claim records:

- The retrieved quantities as published, with their units and conventions declared
  the way every other quantity in this archive is, per `conventions.md`.
- The method, by name, and its version or the reference that pins the code.
- The settings the method was run with, including whatever the method calls its
  initial guess, its regularisation and its stopping condition, since those are
  where the assumptions live.
- Who produced it, and the publication it appeared in, with its identifier.
- The uncertainty as the authors stated it, in the form they stated it, marked
  through the absence states in `absence.md` where they stated none.
- Which array it was produced from. Whether the reconstruction ran on the archived
  array or on a differently processed form of it, because a claim derived from a
  trace that was normalised and interpolated before anybody reconstructed it is a
  claim about a different set of numbers.

A read that asked for the measurement does not return the reconstruction claims,
in the same sense `raw-counts.md` uses for derived forms. They sit beside the
measurement, they are labelled as claims in every listing and every export, and no
default in any part of this system promotes one of them to the answer.

## The naming rule

The words truth, ground truth, true pulse, actual pulse and reference pulse do not
appear as field names in the schema, in the model prose, or in the export. This
is not squeamishness about a word. A field named `reference_pulse` on a measured
dataset will be read as the answer by somebody skimming a schema, and the whole
purpose of this record is that no such reading is available.

Whether that is checked by a machine or held by review is not this record's to
decide, and issue #65 is where greppable invariants for this board are settled.
Until something reads it, this paragraph is a rule that nothing refuses, and the
schema landing without such a field is the evidence rather than this sentence.

## What a benchmark may compute from a measured trace

Each of these is a claim real data supports, and each is stated as the thing it
is rather than dressed as an error.

**Agreement between methods.** The spread of a retrieved quantity across
independent methods run on the same array. This is the closest real data comes to
the synthetic case and it is not the same thing, which the next section says
again.

**Self-consistency of one method.** The spread of a method's own output across
restarts, initial guesses and random seeds on one array. A method whose answer
moves under a change that should not matter has said something about itself that
no comparison with another method could.

**Coverage of a stated uncertainty.** Whether a method's own stated interval
contains what other methods found. A method that is confidently wrong and a method
that is honestly uncertain look identical on any measure that ignores the
interval, and this is the measure that does not.

**Residual in the trace domain.** The difference between the measured spectrogram
and the spectrogram predicted by the retrieved pulse. This is a comparison against
data rather than against an answer, and it is legitimate. It also has to be
reported with its limit attached: streaking retrieval is not unique in general, so
a small residual is consistent with a wrong pulse and does not establish a right
one.

**Stability under the measurement's own noise.** Because `raw-counts.md` keeps the
counts and the shots, the Poisson uncertainty on every pixel is arithmetic, so a
trace can be resampled within its own statistics and the method re-run. A method
whose answer moves further than the noise justifies is telling you something, and
this is the one measure on this list that is only available because of a decision
made elsewhere in this archive.

**Whether it ran at all.** Convergence, failure rate, and time taken. Unglamorous
and the most reported thing missing from method comparisons.

**Response to a declared convention.** Re-running with the delay sign flipped
should produce the mirrored answer and nothing else. A method that returns
something else has a convention baked into it, which is the defect class this
whole board is built around, and a measured trace detects it as well as a
synthetic one does.

## What a benchmark may not compute from a measured trace

**An error against any published reconstruction.** No deviation, no root mean
square difference, no accuracy, no score, under any name, computed against another
method's output. This is the single prohibition the rest of the section elaborates.

**A ranking by closeness to one method.** The same thing with the arithmetic
hidden. A table ordered by distance from a chosen reference is an error measure
whether or not the column is called one.

**A pooled figure mixing synthetic errors with real-data agreement.** Two
different claims added together produce a number that means neither. Where both
appear in one report they appear as separate columns, separately labelled, and a
single headline number over both is refused rather than qualified in a footnote.

**Consensus treated as truth.** Methods in this field share assumptions, so they
can agree and be wrong together, and the agreement then looks like confirmation.
Agreement between methods is reported as agreement between methods. A benchmark
may not compute an error against the consensus and call it an error.

## What the export has to carry

The absence of a reference pulse is a positive statement in the exported data
rather than something a consumer infers from a missing field. A measured dataset
says in the export that it has no reference pulse, and a generated one says what
its input pulse was, so that a results table cannot be built by a script that
treated the missing field as an omission. Issue #53 holds that design and this
record is what it implements.

## What would overturn this

A measured trace whose pulse has been established by a genuinely independent
route, with assumptions that do not overlap the reconstruction methods being
compared. That would still not be truth. It would be a second claim with a
stronger standing, and the repair would be a field recording the independence of
the route, not a field asserting the answer. The distinction survives, which is
why this record is written in terms of what a field asserts rather than in terms
of how good the number is.

A synthetic dataset that has been through a real detector chain, meaning a known
pulse measured by a real instrument. That case is genuinely different and the
model already holds it: the dataset is generated in its provenance and measured in
its acquisition, and if that combination turns out to need its own arm rather than
a mixture of the two, this record is the one to re-argue.
