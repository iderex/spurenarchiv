# The fixtures that prove a field's own schema refuses what it names

Each directory here is named after a row under `schema/<version>/fields/`, and
each file in it is a value for that field. A file whose name begins `accepted-`
has to validate against the row's `schema`, and one whose name begins `refused-`
has to fail it. `.github/workflows/field-value.yml` runs them and reports under
the check name `Field value fixtures`.

A fixture directory that names no row fails the run rather than being skipped, so
a row renamed without its fixtures reds the check instead of leaving them judging
nothing.

The refusals proved here are the ones the model would otherwise lose most
quietly, and each has its near miss one key away. How many there are is a count
the directory decides rather than a number this file keeps:

    git ls-files -- 'fixtures/field-value/*/*.json' | wc -l
    15

`dressing-peak-intensity` is the softest number in a streaking paper. It is
inferred from a focal spot size, a pulse energy and a duration, or read off the
streaking amplitude itself, which is circular when the streaking amplitude is
what is being reconstructed. The refused fixture carries a perfectly plausible
intensity and no method.

`detector-background` has to say what was subtracted, and the value `none` is a
statement rather than an absence. The two accepted fixtures are one of each, so
the check proves that stating nothing was subtracted is accepted and that
declining to say is not. The refused one carries a dark count rate and nothing
about the subtraction, which is what a deposit looks like when somebody answers
the easier question.

`uncertainty-array` has to carry the method that produced it. A reanalyst cannot
tell an estimated uncertainty from a measured one, and the two justify very
different claims. The refused fixture names the file and not the method.

`spectrogram-value-semantics` has to carry the bin widths when it declares a
density, because counts per electronvolt is counts per bin divided by the width
of each bin and a spectrum divided by nothing has the wrong shape and the right
axis labels. The two refusals are one in each direction: a density with no
widths, and widths sitting beside a value that was never divided by them. The
second is the one that would otherwise be read as harmless, and a width beside a
value it did not divide is a number a later reader will use.

`unmeasured-pixel-marker` has to write its sentinel down. The refused fixture
declares that a sentinel marks the pixels nobody measured and does not say what
the sentinel is, which leaves a reanalyst comparing against a value they guessed.
Its accepted neighbour carries `not_a_number`, which is the sentinel most
floating point arrays already use and the one JSON cannot write as a number. The
other refusal is a mask file carrying a sentinel as well, because a marker that
answers twice is a marker two readers resolve differently.
