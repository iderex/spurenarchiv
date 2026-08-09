# The fixture deposits that prove the manifest rules bite

Nine deposits, each a directory with `manifest.json` at its root, one dataset
directory and the files that dataset names.
`.github/workflows/deposit-manifest.yml` judges every one of them against
`docs/spec/deposit-layout.md`, and `schema/1.0/manifest.schema.json` is what
decides the manifest as a document.

Eight of the nine differ from `accepted-complete` in one thing, so what a refusal
proves is that one thing rather than the general condition of the directory.

What is judged here is fixtures in this repository. Nothing here is the validator
a depositor runs before sending anything: that is issue #32, it does not exist,
and a real deposit that breaks every rule below still lands as quietly as one
that keeps them.

`accepted-complete` is the deposit the others are compared against. Three listed
files, each present, each the length and the digest its entry states, and nothing
else in the directory.

`accepted-auxiliary-listed` adds an acquisition log at the deposit root, listed
with the `auxiliary` role. It is the near miss for the unlisted-file refusal: an
extra file is refused for being unlisted and never for being extra, and without
this fixture a rule that refused any file outside the dataset directory would
look identical to the one that is wanted.

`refused-listed-file-missing` is the partial transfer. The manifest still lists
`scan-001/delay-axis.bin` and the file did not arrive, so the deposit fails on
inspection rather than reading as a deposit that never had a delay axis.

`refused-truncated-array` is the transfer that stopped part way through a file.
The spectrogram is 192 bytes where its entry says 256. Size is compared before
digest, so this is reported as a length that does not match rather than as a
digest that does not, because truncation is what a depositor can act on.

`refused-digest-mismatch` is the same file at the right length with one bit of
one value changed. It is the near miss for the truncation refusal in the other
direction: the two failures get two messages, and a deposit whose bytes rotted
without changing length is not reported as a short file.

`refused-unlisted-file` is the direction nothing else in the design can see. A
superseded copy of the spectrogram sits beside the one that replaced it, named by
no entry. `docs/decisions/identity.md` builds the measurement digest from a
listing of roles rather than of file names, so the extra file moves no digest, and
the metadata document names the files it describes and not this one. The manifest
is the only artefact that states the file set is complete.

`refused-file-count-disagrees` states four files and lists three. Everything else
about it is `accepted-complete`.

`refused-manifest-lists-itself` carries an entry for `manifest.json`. A document
cannot carry the digest of the bytes that carry the digest, so the schema refuses
the path before anything is opened.

`refused-path-escapes-the-deposit` names `../scan-001/delay-axis.bin`. It is
refused by the schema, again before any file is read, which is the point: a reader
that resolved the path first would already have left the deposit. This deposit is
not otherwise readable, and it is not meant to be; what it exercises is the entry
and not the directory.

The metadata documents here are not conforming deposits and are not meant to be
read as examples of one. What they are is files with a length and a digest, which
is all a manifest claims about them. A complete deposit meant to be read as a
deposit belongs in `examples/`, which `docs/decisions/layout.md` keeps separate
from this directory.
