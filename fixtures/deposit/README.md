# Deposit fixtures

Each file here is a whole metadata document for one dataset, judged against the
schema version it declares. A whole document rather than a fragment, because the
refusals these prove are refusals of a deposit: a fragment cannot be missing a
key that the version defines, and a conditional cannot fire on a field that is
not there.

`accepted-complete-deposit.json` is the base. Every other file is that document
with one field changed, and the table says which. A reviewer reads the change
rather than the five kilobytes around it.

`.github/workflows/dataset-schema.yml` runs them. A file whose name starts with
`accepted-` has to be accepted and a file whose name starts with `refused-` has
to be refused **and** the report has to name the reason the file exists for, so a
deposit that broke in some unrelated way cannot report a guard biting that was
never reached.

## What each one differs by, and what it proves

| File | Differs from the base by | Verdict |
| --- | --- | --- |
| `accepted-complete-deposit.json` | nothing, it is the base | accepted |
| `refused-missing-key.json` | `delay_jitter` left out | refused, naming `delay_jitter` |
| `refused-sign-convention-absent.json` | `delay_sign_convention` left out | refused, naming `delay_sign_convention` |
| `accepted-absence-state.json` | `delay_jitter` carrying `not_measured` instead of a value | accepted |
| `refused-value-beside-an-absence-state.json` | `delay_jitter` carrying `not_measured` **and** a value | refused |
| `refused-estimated-without-basis.json` | `dressing_peak_intensity` estimated with no basis | refused, naming `basis` |
| `accepted-estimated-with-basis.json` | the same estimate, with its basis | accepted |
| `refused-intensity-without-method.json` | `dressing_peak_intensity` carrying a value and no `determination_method` | refused, naming `determination_method` |
| `refused-conditional-not-stated.json` | a stage-position axis with the stage relation not applicable | refused, naming the state |
| `accepted-conditional-stated.json` | a stage-position axis with the stage relation given | accepted |
| `refused-value-against-its-row.json` | `noise_model` carrying a value no row admits | refused, naming the value |
| `refused-empty-shell-list.json` | `target_shells` present and empty | refused, naming the empty list |
| `refused-energy-axis-without-calibration.json` | `energy_calibration` carrying `not_recorded` on a kinetic-energy axis | refused, naming the state |
| `accepted-flight-time-axis-without-calibration.json` | the same, on a time-of-flight axis | accepted |
| `refused-counts-without-shots.json` | `completeness_level` still `counts` and `shots_per_point` carrying `not_recorded` | refused, naming the state |
| `accepted-processed-without-shots.json` | the same, at `completeness_level` `processed` | accepted |
| `refused-unstated-processing-history.json` | `processing_history` carrying `not_recorded` | refused, naming the state |
| `accepted-processing-history-with-an-undescribed-step.json` | `processing_history` carrying a subtraction and one undescribed step | accepted |
| `refused-transposed-array.json` | `spectrogram_axis_order` reading `energy` first, with the shape swapped to match | refused, naming the order |
| `refused-saturated-without-a-marker.json` | `any_pixel_saturated` true and `saturated_pixel_marker` not applicable | refused, naming the state |
| `accepted-saturated-with-a-marker.json` | the same, with the marker given | accepted |
| `refused-sign-convention-omitted.json` | `delay_sign_convention` left out | refused, naming `delay_sign_convention` |
| `refused-sign-convention-not-recorded.json` | `delay_sign_convention` carrying `not_recorded` | refused, naming the state |
| `refused-intensity-without-a-method.json` | `dressing_peak_intensity` carrying a number and no `determination_method` | refused, naming `determination_method` |
| `refused-background-unstated.json` | `detector_background` carrying a dark count rate and no `subtracted` | refused, naming `subtracted` |
| `accepted-background-none.json` | `detector_background` with `subtracted` reading `none` | accepted |
| `accepted-efficiency-not-measured.json` | `detection_efficiency` carrying `not_measured` instead of a curve | accepted |
| `accepted-detector-response-not-measured.json` | the four optional detector characterisations carrying `not_measured` instead of values | accepted |

The three around `delay_jitter` are the set the Done-when of issue #96 asks for,
and they are that field rather than a required one on purpose. A depositor who
did not measure the jitter reaches for the key and leaves it out, which is the
exact defect `docs/decisions/absence.md` exists against, and the near miss beside
it is the same depositor writing the state instead. The distance between the
refused file and the accepted one is one key, so what the refusal reaches is
visible rather than argued.

`refused-transposed-array.json` is the deposit that would otherwise be read wrong
rather than refused. Its shape and its axis order agree with each other, so
nothing about it is malformed; what it declares is that the block was written with
the energy index varying slowest, which `docs/spec/deposit-layout.md`'s read does
not fit and `schema/1.0/fields/spectrogram-axis-order.json` does not admit. The
base beside it is the near miss, one value away, and `docs/model/spectrogram.md`
is where admitting one order is argued.

`refused-sign-convention-absent.json` is the same shape one field over, and the
field is the one `docs/decisions/conventions.md` refuses a default for. The two
sign conventions are mirror images and the retrieved chirp changes sign between
them, so a deposit that leaves the key out has to be refused rather than read as
the archive's own convention. Its near miss is the base document, which differs
from it by that key and nothing else, so what the refusal reaches is one key
wide. There is no accepted file beside it carrying an absence state, because the
row is `required` and the assembly holds a required field to the two states that
carry a value.

The two around `dressing_peak_intensity` are one field refused in two directions,
and the distinction is worth keeping in view because both files carry a number
that looks fine. `refused-estimated-without-basis.json` is a depositor calling
the number an estimate and not saying what the estimate rests on.
`refused-intensity-without-method.json` is a depositor stating the number as
measured and not saying how it was obtained, and for this field that is the
question the whole row exists for: an intensity inferred from a focal spot, a
pulse energy and a duration and an intensity read off the streaking amplitude are
the same number with different standing, and the second is circular in a
reconstruction that is solving for the streaking amplitude. Neither refusal
reaches the other, which is what makes them two files.

`refused-value-against-its-row.json` is the one that proves the assembly is not
just a list of key names. Its `noise_model` reads `poisson_from_counts`, which is
a real spelling in this model - it is one of the methods in
`schema/1.0/fields/uncertainty-array.json` - and it is not one of the three
`schema/1.0/fields/noise-model.json` admits. If the reference into the row ever
stops resolving, the schema accepts it and this file turns the run red.

The two sign-convention files are issue #23's, and they are two rather than one
because "a deposit that omits the sign convention" has two meanings that fail
differently. A deposit with no `delay_sign_convention` key is the easy case and
almost nobody will write it, since a depositor working from the template gets
every key. A deposit carrying the key in `not_recorded` is the one that will
actually arrive, because a depositor who does not know which convention their
axis was recorded in has a state that says exactly that and will reach for it.
Both have to be refused, since the two conventions are mirror images and no
inspection of the array distinguishes them, so a trace whose convention is lost
is not archivable as a streaking trace. A pair that proved only the first would
leave the guard passing the case the field exists for.

`refused-intensity-without-a-method.json` is issue #25's. The number in it is a
perfectly plausible peak intensity and the deposit is refused anyway, because a
peak intensity read off the streaking amplitude and one inferred from a focal
spot, a pulse energy and a duration are different claims, and a reanalyst
testing a reconstruction against this trace has to be able to exclude the first
automatically. The method is a value from a closed set for that reason rather
than free text.

The background files are issue #27's. `refused-background-unstated.json` is the
deposit somebody writes when they answer the easier question: it carries a dark
count rate, which is a property of the detector, and says nothing about what was
taken off this array, which is the thing that cannot be added back.
`accepted-background-none.json` beside it is the deposit that says nothing was
subtracted, as a positive fact rather than by leaving the key out, and it is
accepted. The distance between the two is one key.

`accepted-efficiency-not-measured.json` is the same issue's other half. An
efficiency curve is optional and a deposit without one still has to say so, so
the field carries `not_measured` rather than being absent or defaulting to flat.
An efficiency that falls at low energy suppresses one side of the streaking
modulation, and a reanalyst who cannot tell a flat curve from an unmeasured one
cannot tell whether that bias is in the trace.

`accepted-detector-response-not-measured.json` widens that from one field to
four. A depositor who characterised none of the detector response writes each
of the four as `not_measured` rather than leaving them out or filling in a
plausible number, and the deposit is accepted. A reanalyst reading it knows
which corrections are unavailable, which is a different statement from a
deposit that merely looks complete.

## The numbers in them

The values are plausible for a streaking measurement and none of them is
measured. They are here to be well formed, not to be true, and nothing should
cite them. A real deposit is issue #43.
