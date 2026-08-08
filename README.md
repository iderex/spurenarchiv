# spurenarchiv

Streaking traces are practically never published as data, only as images, so checking a published pulse duration means recovering numbers from a picture. The repository holds the raw spectrogram, delay axis, energy calibration, IR parameters, target gas and detector response, with a DOI per dataset, which is what makes independent reanalysis possible at all. The work is the data model and persuading a few groups to deposit rather than the software, so the milestone that counts is the first real dataset from an outside group. It feeds the messlatte benchmark: real traces beside synthetic ones are the only way to detect a method that works solely inside the assumptions of the simulation.

The first sentence above is a claim and is kept as one. What has actually been
counted, the searches that produced it and the much larger part it does not
cover are in [docs/landscape.md](docs/landscape.md).

Planning happens on the issue tracker first. Every decision that shapes
the architecture is written down there with its reasons before the code
that depends on it exists.

See [NOTICE.md](NOTICE.md) for the intended-use notice.
