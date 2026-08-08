# The detector response

Between the electrons and the array is a detector that does not respond equally
to everything, and its response is baked into every pixel. None of what follows
can be inferred from a single trace, which is why each of these is a field rather
than something a careful reanalyst works out.

The rows are under `schema/1.0/fields/`, one file per field, and the rule they
follow is in `README.md` beside this document.

## The type, and where the spectrometer looks

`detector_type` is required and carries the kind and its configuration. A
time-of-flight spectrometer with a microchannel plate, a magnetic bottle and a
velocity map imaging spectrometer distort in different ways, and a reanalysis
that does not know which one it is looking at does not know which distortions to
look for.

`detection_direction` is required and is a vector in a stated frame. It is the
other half of the geometry that `dressing-field.md` starts. The observed
streaking shift is the projection of the momentum shift onto this direction, so a
spectrometer looking the other way sees a trace with the opposite sign and no
inspection of the array recovers which.

## Efficiency, saturation and dead time

Each of these is optional in the requirement sense, which means absent is
recorded as one of the states in `docs/decisions/absence.md` and the deposit
stands. Optional is not the same as unimportant. It is the model saying that a
deposit which never measured its detection efficiency is still worth archiving,
provided it says so rather than leaving a reanalyst to assume a flat response.

`detection_efficiency` is the efficiency as a function of energy, with how it was
determined. An efficiency that falls at low energy suppresses one side of the
streaking modulation and biases the retrieved field, and the bias looks like a
result.

`detector_saturation` carries the gain behaviour and whether any pixel reached
saturation. A saturated pixel cannot be told from a genuinely flat maximum, and
the maximum is where a fit puts most of its weight.

`detector_dead_time` carries the dead time and whether a pile-up correction was
already applied. At high count rates a dead-time-limited detector compresses the
strong parts of the trace, which is exactly where the signal is. Whether the
correction was applied matters as much as the number: applying it twice is as
wrong as not applying it, and only the field says which of the two a reanalysis
is starting from.

## The background, which is required and may be the value none

`detector_background` is required, and what it must state is what was subtracted.
`none` is one of the answers and it is a statement rather than an absence: it
says the arrays are as they came off the detector. What is refused is a deposit
that says nothing about the subtraction at all, and the fixtures under
`fixtures/field-value/detector-background/` prove both directions, that `none` is
accepted and that silence is not.

The reason this one is required while the three above are optional is that a
background is almost always subtracted and almost never recorded. A subtracted
background cannot be added back if the amount was never written down. Worse, a
subtraction that went slightly negative and was clipped at zero has destroyed the
Poisson statistics of the whole low-count region, and nothing in the array shows
it, so a reanalysis computing errors from counts is computing them from numbers
that are no longer counts. The field carries whether the clip happened for that
reason.

The dark count rate sits in the same field and is a different quantity from the
background that was subtracted. One is a property of the detector and the other
is a step somebody took.

## Angular acceptance

`detector_angular_acceptance` is optional and carries the half angle and how it
was determined. It decides how much of the angular distribution is averaged into
each pixel, and a streaking amplitude reduced by angular averaging is read as a
smaller field by anything that does not know the acceptance.

## What no schema here can refuse

That `energies` and `values` in the efficiency curve have the same length, which
is a comparison between two arrays.

That a deposit declaring `any_pixel_saturated` as false is telling the truth.
Saturation is visible in the array to somebody who looks, which makes this a
plausibility check for the review in issue #59 rather than something the format
can decide.

That a background declared as `none` really was not subtracted. This is the one
worth stating plainly, because the field is required precisely so that the claim
is made rather than assumed, and requiring a claim is not the same as verifying
it. What the model buys is that a reanalyst has something to disbelieve.
