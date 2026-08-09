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
| `accepted-absence-state.json` | `delay_jitter` carrying `not_measured` instead of a value | accepted |
| `refused-value-beside-an-absence-state.json` | `delay_jitter` carrying `not_measured` **and** a value | refused |
| `refused-estimated-without-basis.json` | `dressing_peak_intensity` estimated with no basis | refused, naming `basis` |
| `accepted-estimated-with-basis.json` | the same estimate, with its basis | accepted |
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
| `refused-background-unstated.json` | `detector_background` carrying a dark count rate and nothing about the subtraction | refused, naming `subtracted` |
| `accepted-detector-response-not-measured.json` | the four optional detector fields carrying `not_measured` instead of values | accepted |

The three around `delay_jitter` are the set the Done-when of issue #96 asks for,
and they are that field rather than a required one on purpose. A depositor who
did not measure the jitter reaches for the key and leaves it out, which is the
exact defect `docs/decisions/absence.md` exists against, and the near miss beside
it is the same depositor writing the state instead. The distance between the
refused file and the accepted one is one key, so what the refusal reaches is
visible rather than argued.

`refused-value-against-its-row.json` is the one that proves the assembly is not
just a list of key names. Its `noise_model` reads `poisson_from_counts`, which is
a real spelling in this model - it is one of the methods in
`schema/1.0/fields/uncertainty-array.json` - and it is not one of the three
`schema/1.0/fields/noise-model.json` admits. If the reference into the row ever
stops resolving, the schema accepts it and this file turns the run red.

The two detector files are the same field set read from both sides.
`refused-background-unstated.json` is a depositor who answered the easier
question: it carries a dark count rate, which is a property of the detector, and
says nothing about what was taken off the array, which is the thing that cannot
be added back. `accepted-detector-response-not-measured.json` is the other side,
four detector characterisations the depositor never made, each written as
`not_measured` rather than left out or filled with a plausible number. A
reanalyst reading it knows which corrections are unavailable, which is a
different statement from a deposit that looks complete.

## The numbers in them

The values are plausible for a streaking measurement and none of them is
measured. They are here to be well formed, not to be true, and nothing should
cite them. A real deposit is issue #43.
