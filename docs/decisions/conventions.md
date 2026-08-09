# Units, sign conventions, and the one place a value is converted

Status: decided. Issue #14.

## The decision

Every physical quantity in the model has one internal representation, stated
below. A deposit states the unit and the convention of every value it carries.
Nothing is inherited from a default, with one exception that is named and argued
at the end rather than left to be discovered. A value whose unit or convention is
not stated is refused, per field, and the field-by-field answer is the table in
this record rather than a general sentence.

Conversion into the internal representation happens in exactly one place in the
tool and nowhere else.

This record is about unit and convention conversion inside this repository.
`conversion-boundary.md` is about conversion from an instrument's own export
format, which happens outside this repository entirely. The two records use the
same word for different operations and this sentence exists so that a reader who
has met one does not assume the other.

## The internal representation

The internal system is SI, with one departure named below. The reason is not
purity. It is that a single system is the only condition under which a
dimensional check is possible at all: a model carrying seconds, metres,
electronvolts and watts per square centimetre at once has to track which system
each value is in before it can refuse an addition, and the tracking is the thing
that goes wrong.

**Time.** Seconds. Delays, pulse durations, times of flight, dead times and
acquisition timestamps are all seconds, and a duration is never stored in optical
cycles. A cycle is not a unit: it is a duration that depends on the dressing
wavelength, so a pulse duration in cycles is unreadable without a second field
and becomes wrong the moment that field is corrected. Femtoseconds and
attoseconds are permitted declared units on the way in and are exact multiples.

**Energy.** Electronvolts, and this is the named departure from SI. It is taken
because since the 2019 redefinition of the SI the elementary charge is exact by
definition, so the electronvolt is an exact multiple of the joule and the
departure costs no uncertainty at all. It is taken rather than declined because
every published number in this field is in electronvolts, and converting on the
way in and back out again on the way to every reader adds two crossings for no
benefit. Photon energies, kinetic energies, ionisation thresholds and energy
widths are all electronvolts.

**Length and wavelength.** Metres. Wavelengths are stored as vacuum wavelength in
metres, not as angular frequency and not as photon energy, and the other two are
derived. The conversion to angular frequency uses the speed of light, which is
exact by definition of the SI, so that crossing carries no uncertainty either.
What it does carry is a trap named in its own section below.

**Intensity.** Watts per square metre. This is the crossing almost every deposit
will make, because the field writes watts per square centimetre, and the factor
of ten thousand is exact and applied in one place. The alternative, storing what
everyone writes, was weighed and declined: it puts one quantity outside the
system every other quantity is in, which is the condition the paragraph above
says makes a dimensional check impossible.

**Number density.** Inverse cubic metres. The target density in the interaction
region is the quantity this covers, and it is here rather than left to follow
from the general rule because a quantity derived somewhere else is a quantity two
rows can derive differently. Densities in this field are written per cubic
centimetre far more often than per cubic metre, so the factor of a million is a
crossing most deposits carrying a density will make, and it is exact and applied
in the same one place as the others.

**Pressure.** Pascals. No field in the model carries a pressure today, and this
line fixes the representation ahead of the field rather than creating one. It is
written now because the field it is waiting for is predictable: a depositor who
has a backing pressure and no density has the number that most target densities
were estimated from, and the units that number is written in are millibars and
torr rather than pascals. Fixing it here costs one line. Leaving it costs a row
that arrives with its own answer, which is the drift this section exists against.

**Field direction.** A unit vector in a stated frame, together with the
definition of the vector potential given below. Directions are not stored as
angles relative to something a sentence describes.

**Counts and shots.** Integers, dimensionless, exactly as `raw-counts.md`
requires. Not a floating point number that happens to hold an integer.

**Number density.** Inverse cubic metres. The density of the target in the
interaction region is a count per volume, and it is stored as one because the
reason it is archived at all is a space-charge estimate, which needs a number per
volume and a volume. Particles per cubic centimetre is a permitted declared unit
on the way in and the factor of a million is exact.

**Pressure.** Pascals. Nothing in the model carries a pressure today, and the
quantity is named here so that the first field to carry one has no choice left to
make in the row that introduces it. A backing pressure is not a number density
written another way, because what arrives in the interaction region depends on
the nozzle, the distance and the expansion. A depositor who has the pressure and
not the density states the density as an estimate with the pressure and the
geometry as its basis, which is what `docs/model/target.md` already says about
that field.

## The sign of the delay, and why it is a subtraction rather than a convention

The internal delay is defined as a subtraction that is written down:

    delay = (arrival time of the XUV pulse) - (reference time of the dressing field)

so a positive delay means the XUV arrives after the dressing field's reference
time. The sign is then a consequence of an equation rather than a thing to
remember, and a reader who has forgotten the convention can re-derive it from
this line instead of guessing.

Both reference times need saying, and the second one is where the quiet error is.
The XUV arrival time is the centre of its intensity envelope. The dressing field's
reference time is the peak of its intensity envelope, not a peak of the field
oscillation. The two differ by up to half an optical cycle, they move relative to
each other with the carrier-envelope phase, and a deposit that used a field
maximum as its reference has a delay axis offset by an amount that depends on a
quantity most measurements do not stabilise.

A deposit declares which convention its own axis was recorded in. It is not
permitted to omit it, and a deposit whose delay sign convention is absent is
refused rather than assumed to match this one. The reason is the one the issue
gives and it is worth restating: the two conventions are mirror images, the
reconstructed chirp changes sign between them, and a default is precisely how a
wrong one enters without anybody choosing it. An axis in the other convention is
converted at the boundary and the deposit's own declaration is kept, so the
record of what the depositor said survives the conversion.

## The stage, and the factor of two

The archived delay axis is an optical delay in seconds. It is never a stage
position, and never a stage position with a unit of seconds written next to it.

A retroreflector on a translation stage sends the beam back along its own path,
so a stage movement changes the optical path by twice the movement and the
optical delay by twice the movement divided by the speed of light. A trace
archived with the factor left out is wrong by exactly two on its time axis, every
duration derived from it is wrong by two, and nothing about the array looks wrong.
Geometries where the factor is not two exist, which is the reason the relation is
declared rather than the factor being hard-coded here.

Where the depositor has stage positions, the relation is a required field: the
geometry factor, the sign, and the position that corresponds to zero delay. Both
axes are then kept, the stage positions as recorded and the optical delay as
derived, with the relation recorded as the step that produced one from the other.
A deposit whose axis is declared as a stage position and which carries no relation
is refused. A deposit that silently declares stage positions as an optical delay
is not detectable from the file, which is why the declaration is required and why
the review in issue #59 has a person looking at whether delay zero falls inside
the scanned range.

## Kinetic energy against time of flight, and where the Jacobian actually enters

Both are legitimate archived axes and a deposit declares which one it has.
Undeclared is refused. Where the axis is a kinetic energy and the instrument was
a time-of-flight spectrometer, the calibration that produced the energies is a
required field, because the conversion is not recoverable from the array.

The Jacobian is the part that is usually stated too broadly, and stating it too
broadly is how a correct file gets "corrected". Relabelling bin edges from times
of flight to kinetic energies is lossless and needs no Jacobian: the counts in a
bin belong to that bin whatever the axis is called, and the result is counts per
bin on a non-uniform energy grid. The Jacobian is needed for exactly two things.
Turning counts per bin into a density, meaning counts per electronvolt, requires
dividing by the width of each bin in the new axis. And resampling onto a uniform
energy grid redistributes counts between bins, which is an irreversible step in
the sense `raw-counts.md` uses, and it is recorded as one.

So the field a deposit must carry is not "was the Jacobian applied" but what the
value in a pixel means: counts in that bin, or counts per unit of the axis. That
is declared, it is refused if absent, and a deposit declaring a density carries
the bin widths that were used. A spectrum converted as a density without the
Jacobian has the wrong shape and the right axis labels, and this is the field that
makes the difference visible.

## Wavelength against angular frequency, and the same trap one level up

The centre of a spectrum converts between vacuum wavelength and angular frequency
by a reciprocal with an exact constant. A width does not convert the same way.
The transformation is nonlinear, so a bandwidth stated in nanometres and the same
bandwidth stated in angular frequency are related by the derivative at the centre
and not by applying the centre's formula to the width. Applying it twice, once to
each edge, is right. Applying it to the width is wrong, by an amount that is small
for a narrow band and not small for the broadband pulses this archive is about.

Widths are therefore stored in the same quantity as the axis they describe, and a
width whose quantity is not stated is refused. This is the same defect as the
Jacobian one wearing different clothes, and the two are named separately because
nobody who has met one recognises the other.

## The field, the vector potential, and the direction of the streaking shift

The archive fixes this convention rather than asking a depositor to declare it,
because it follows from a definition and a derivation rather than from a habit.
The vector potential of the dressing field is defined by

    E(t) = -dA(t)/dt

and a free electron released at time t with initial momentum p_i leaves with

    p_f = p_i - e A(t)

where e is the elementary charge as a positive number. That is the whole
convention: the momentum shift is opposite in direction to the vector potential at
the instant of release, and everything derived from the streaking trace inherits
its sign from those two lines. They are written here so that a reader can check
the sign rather than recall it, and so that a change to it is a visible change to
this record.

What the deposit declares instead is the geometry the sign is observed through,
and these are required for a streaking scan and refused if absent: the
polarisation direction of the dressing field and the detection direction, both as
vectors in one stated frame. The observed shift is the projection of the momentum
shift onto the detection direction, so a spectrometer looking the other way sees a
trace with the opposite sign, and no reanalysis can recover which from the array.

The absolute orientation of the field is only meaningful where the
carrier-envelope phase is stabilised and known. Where it is not, the deposit says
so through the absence states in issue #16 rather than through a plausible number,
and a reanalysis reads that field before it makes any claim that depends on the
absolute sign.

## Atomic units, and the constants that are not exact

Atomic units are a permitted declared unit for energy, intensity, time and
momentum on the way in. They are never the internal representation, because a
model in atomic units hides a mass and a charge inside every number and the
hiding is what this record exists against.

The conversions divide in two and the division matters. The crossings that rest
only on the elementary charge, the speed of light or the Planck constant are exact
by the definition of the SI and carry no uncertainty. The crossings that rest on a
measured constant, which is every one involving the Hartree, the Bohr radius, the
electron mass or the fine-structure constant, carry the uncertainty of that
measurement, and the numbers change between releases of the recommended values.

Those constants are therefore a pinned artefact rather than literals in the
source: the specification names one release of the recommended values, the values
live in `schema/` where the specification can be read against them, and the tool
reads them rather than carrying its own copy. A second copy of a constant in the
source is a second place for a digit to be wrong, and it is the kind of wrong
nothing detects.

## The one place conversion happens

Every value entering the model from a deposit passes through a single boundary in
the tool that takes a declared unit and a declared convention and returns the
internal representation. Every value leaving the model for an export or a display
passes through its reverse. Nothing else in the tool converts anything, so a unit
confusion has one file to review and one set of fixtures to prove.

The means chosen in `means.md` is what makes that more than an instruction. The
internal quantities are types that cannot be built from a bare number outside the
boundary, so a value that skipped the conversion is not a value the rest of the
tool can be handed. A reviewer looking for a missing conversion is looking for a
call in one module rather than for a multiplication anywhere in the tree.

The check that refuses a conversion written elsewhere, and the near miss that
proves it does not refuse the boundary's own arithmetic, belong with the
validator in issues #32 and #33.

## What is declared, what is refused, and what may be omitted

Per field, because a general answer is what the issue asked this record not to
give.

**The delay sign convention.** Must be declared. Refused if absent. No default.

**The delay axis quantity, optical delay or stage position.** Must be declared.
Refused if absent.

**The stage-to-delay relation.** Required when the axis quantity is a stage
position, and refused if absent in that case. Not applicable otherwise, and the
absence state for that is issue #16's.

**The reference point the delay is measured from.** Must be declared, out of the
envelope peak and the field maximum. Refused if absent, because the two differ by
a carrier-envelope-phase-dependent amount and no inspection of the array
distinguishes them.

**The energy axis quantity, kinetic energy or time of flight.** Must be declared.
Refused if absent.

**The energy calibration.** Required when the axis is a kinetic energy produced
from a time-of-flight measurement. Refused if absent in that case. Issue #24 is
where its own contents are settled.

**The value semantics, counts per bin or counts per unit of the axis.** Must be
declared. Refused if absent. The bin widths are required in addition when a
density is declared.

**The unit of every physical quantity.** Must be declared per field from the
fixed set the schema carries. Refused if absent. There is no default unit
anywhere in the model, including for quantities where only one unit is ever used
in practice, because "only one unit is ever used" is a statement about today.

**The quantity a width is stated in.** Must be declared. Refused if absent.

**The polarisation direction and the detection direction.** Must be declared for
a streaking scan, in one stated frame. Refused if absent.

**The sign convention relating the momentum shift to the vector potential.** Not
declared by the depositor. Fixed by the archive in the section above, and stated
in the specification so that a reader can check it.

**The release of the recommended physical constants.** May be omitted, and the
release the specification names is then used. This is the one default in the
record and it is here rather than buried: the values of these constants move
between releases by a relative amount many orders of magnitude smaller than the
uncertainty on any quantity in this model, so the difference cannot reach a
conclusion a reanalysis draws. A deposit that converted with a different release
may declare it, and the declaration is kept. If a future release ever moves one of
these constants by an amount that matters, this exception is the line to remove
and this paragraph is where to find it.

## What is not settled here

Whether an axis coordinate is a bin centre or a bin edge. It is a convention of
exactly this kind and it belongs to the spectrogram, so it is issue #22's, and it
is named here so that a reader does not conclude from its absence that it was
missed.

What the energy calibration contains and how its uncertainty is stated, which is
issue #24. What the dressing field record contains, which is issue #25. This
record fixes the units and the signs those fields are expressed in and does not
decide which fields exist.

## What would overturn this

A geometry in common use where the stage-to-delay relation is not a constant
factor at all, for example one where the relation depends on the position. The
declared relation would then have to be a description rather than a number, which
is a larger change than adjusting a factor and deserves its own argument.

A deposit population where the internal choice of electronvolts turns out to be
the one crossing that keeps going wrong, against the argument above that it is
exact and universal. That would be evidence that the number of crossings matters
more than the familiarity of the unit, and the repair would be joules internally
rather than a weaker rule about declaring.
