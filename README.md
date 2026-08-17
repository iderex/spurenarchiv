# spurenarchiv

Streaking traces are practically never published as data, only as images, so checking a published pulse duration means recovering numbers from a picture. The repository holds the raw spectrogram, delay axis, energy calibration, IR parameters, target gas and detector response, with a DOI per dataset, which is what makes independent reanalysis possible at all. The work is the data model and persuading a few groups to deposit rather than the software, so the milestone that counts is the first real dataset from an outside group. It feeds the messlatte benchmark: real traces beside synthetic ones are the only way to detect a method that works solely inside the assumptions of the simulation.

The first sentence above is a claim and is kept as one. What has actually been
counted, the searches that produced it and the much larger part it does not
cover are in [docs/landscape.md](docs/landscape.md).

Planning happens on the issue tracker first. Every decision that shapes
the architecture is written down there with its reasons before the code
that depends on it exists.

## Checking a deposit

The validator judges one dataset's metadata document against the schema version
the document declares. It runs from a single file with no runtime, no
interpreter and no package manager, so a depositor can check their own work
before sending anything.

```console
$ deposit-validator fixtures/deposit/accepted-complete-deposit.json
fixtures/deposit/accepted-complete-deposit.json: accepted against schema version 1.0
  4 field(s) carry a state rather than a value:
    detector_noise_parameters is not_applicable
      without it: The noise of a sensor with read noise and gain cannot be modelled at all, so the Poisson assumption is applied to data it does not describe and every uncertainty derived from it is wrong in an unknown direction.
    saturated_pixel_marker is not_applicable
      without it: A deposit that has already admitted a saturated pixel gives no way to find one, so a compressed peak is fitted as though it were the true one and every parameter retrieved through it is biased.
    stage_to_delay_relation is not_applicable
      without it: The conversion from stage position to optical delay cannot be redone or checked, so a scan whose factor of two for a double-passed retroreflector was left out stays wrong by two on its time axis and every duration derived from it is wrong by two.
    target_number_density is estimated
      without it: Whether space charge could have shifted and broadened the spectrum cannot be assessed at all, and a space-charge broadening left in the trace is read as a chirp.
  3 check(s) this schema version declares were not evaluated here:
    uncertainty-array-shape-matches-spectrogram: reads a file outside the metadata document, and this route is given one metadata document rather than the deposit directory it sits in
    array-file-length-matches-the-declared-shape: reads a file outside the metadata document, and this route is given one metadata document rather than the deposit directory it sits in
    marker-file-is-listed-in-the-manifest: reads a file outside the metadata document, and this route is given one metadata document rather than the deposit directory it sits in
```

A conforming deposit still gets the two paragraphs under the verdict. The first
says which fields carry a state rather than a value and what a reanalysis cannot
do without each of them, because a deposit that states its absences is valid and
is not as good as one that does not have any. The second says which checks this
version declares that the run did not make, so a run that covered less than the
whole set cannot be read as one that covered it and found nothing.

The block above is executed by the test suite and its output is compared to what
is printed here, so a README that says something the tool no longer does is a red
run rather than a thing somebody notices later.
[docs/spec/validation.md](docs/spec/validation.md) specifies what is refused and
what is not.

See [NOTICE.md](NOTICE.md) for the intended-use notice.

## License

AGPL-3.0, copyright 2026 Nils Lehnen.

The full text is in [LICENSE](LICENSE).
