# The deposit on disk

A deposit has to survive being copied, mirrored, put on a memory stick and
unpacked three years later by somebody who has none of this software. That is why
the shape on disk is part of the specification rather than an implementation
detail of whatever wrote it, and it is why this document is written to be
complete on its own. `docs/decisions/layout.md` sets that obligation: a reader can
implement a conforming parser from `docs/spec/` without opening the tool, and no
sentence here may refer to the tool for the meaning of anything.

What this document settles is the file set, the entry point, the manifest, the
checksum, and what happens to a file that is on disk and not in the manifest or in
the manifest and not on disk. What a dataset's metadata document contains is the
model in `docs/model/`. What refuses a deposit somebody else sends is the
validator in issue #32, and the next section says what does and does not refuse
one today.

## What refuses a rule in this document, and what does not

Said first, because a specification is easy to read as a guarantee.

The manifest as a document is refused by `schema/1.0/manifest.schema.json`, which
is the normative artefact `docs/decisions/layout.md` requires and which
`docs/decisions/schema-language.md` named this document as the source of. A schema
sees one document, so it decides the keys, the shape of an entry, the form of a
path and the form of a digest, and it decides nothing that needs another file
opened. The rules it cannot reach are listed by name in its `deferred_checks`
array rather than left to be inferred from its silence.

Those are decided by `.github/workflows/deposit-manifest.yml`, which reads a
deposit directory, compares it against its manifest and names the path in every
refusal. Each refusal below has a fixture under `fixtures/manifest/` that trips
it, and the accepted fixtures beside them are what stops a rule reaching past what
it names.

None of that is a validator, and the difference is the whole of what a depositor
has. That check judges fixture deposits inside this repository. There is nothing
here anybody can point at a directory of their own:

    git ls-files -- tool/ | wc -l
    0

So a real deposit that breaks every rule in this document still lands exactly as
quietly as one that keeps them, and a depositor still finds out from somebody else
rather than from their own machine. That is issue #32, with the check a depositor
runs in #57. What has changed is narrower and is worth stating exactly: every rule
below is now decidable rather than prose, and each has been observed refusing the
deposit it names.

## A deposit is a directory

`docs/decisions/container.md` fixed that a dataset is a set of files rather than
one file, with its metadata in a text document beside the arrays.
`docs/decisions/dataset-unit.md` fixed that a deposit is one or more datasets
delivered together with a manifest listing every file. A directory is what those
two decisions already describe.

An archive file is a transport form and not a deposit. A deposit may be sent as a
`tar` or a `zip`, and what is inside is the directory, unpacked before anything
reads it. The archive is not addressed, not hashed and not listed, and a reader
handed one has been handed a container around a deposit rather than a deposit.

That line is drawn rather than left open for three reasons, and the first is the
one that decides it. The comparison property is the reason `container.md` chose
this shape at all: two copies of a dataset compare as bytes because the bytes are
the values and nothing else. An archive layer puts that back. It carries
modification times, an entry order, a permission bit and, where compression is on,
a compressor's version-dependent output, so two archives of one identical
directory are routinely different files. A format that spent four arguments
getting to canonical bytes does not then wrap them in a layer that is not.

The second is the manifest. Its checksums are over files, and a file inside an
archive is not a file until something extracts it, so a manifest checked without
unpacking would be checking a copy the reader made rather than the deposit.

The third is issue #35. A reader takes one delay slice out of a large array with a
seek, which is what makes a memory ceiling a property rather than an observation.
A seek into a compressed member is not a seek.

The cost is real and is paid knowingly. A directory of many files is slower to
move across a network than one file, some transfer tools handle it worse, and a
depositor's first instinct will be to send an archive. That last one is not a
problem: sending an archive is allowed, and what arrives is unpacked.

## The file set

    <deposit>/
      manifest.json
      <dataset>/
        metadata.json
        <array files>
      <dataset>/
        ...
      <auxiliary files>

One directory per dataset, named by the depositor. Inside it, exactly one
metadata document called `metadata.json`, and the array files that document
names.

`manifest.json` sits at the root of the deposit and nowhere else. There is exactly
one.

Two names are fixed by this document and everything else is named by the
depositor. `manifest.json` is fixed because a reader has to be able to find the
entry point without being told where it is. `metadata.json` is fixed because a
dataset directory has to be recognisable as one, and because a deposit with two
candidate metadata documents in a dataset directory is a deposit where something
has to choose, which is the kind of choice that gets made differently by two
implementations.

Array file names are not fixed and are not part of any identity.
`docs/decisions/identity.md` builds the measurement digest from a listing that
carries each array's role rather than its file name, precisely so that renaming a
file or moving it between directories does not change which measurement it is. The
metadata document names the array file for each role, so the name is a pointer the
depositor writes and the digest does not depend on it.

A path in a manifest or in a metadata document is relative to the deposit root,
uses `/` as its separator, and contains no `.` or `..` component and no leading
separator. A reader that resolved `../` out of a deposit would be a reader an
untrusted deposit can steer, and this format is read on machines belonging to
people who did not write the deposit.

Path characters are restricted to ASCII letters, digits, `.`, `-` and `_`, and a
path component is not empty. That is narrower than any filesystem requires, and
the narrowness is the point: a deposit written on one system and read on another
has to arrive intact, and case-insensitive filesystems, normalisation of accented
characters and reserved device names are each a way for a path to stop being the
path that was hashed. A depositor who wants a descriptive name has the metadata
document to put it in.

Auxiliary material is anything that is not a dataset and not part of one: a
reference spectrum, a photograph of the setup, a copy of the acquisition log.
`docs/decisions/dataset-unit.md` admits it, requires it to be listed in the
manifest, and requires that it is never read as measurement. It may sit at the
deposit root or inside a dataset directory, it is listed either way, and its role
in the manifest is what keeps it out of the measurement path rather than where it
happens to sit.

## The entry point is the manifest

A reader opens `manifest.json` first, and a directory with no `manifest.json` at
its root is not a deposit. It is not a deposit with a missing manifest, and a
reader does not fall back to walking the directory for anything that looks like a
dataset.

That is the harder direction and it is the one this document is for. A deposit
that lost its manifest during a copy has to fail on inspection rather than read as
a deposit whose file list happens to be whatever survived. The fallback is the
tempting behaviour and it is the one that turns a partial transfer into a
successful read of less data than the depositor sent.

The same reasoning stops one level down. A dataset directory is reached from the
manifest, not by scanning. A reader that found datasets by looking for
`metadata.json` would find exactly the ones that arrived and never notice the one
that did not.

## The manifest

`manifest.json` is a JSON document, UTF-8 encoded, with line feed line endings and
no byte order mark. Those are the encoding requirements
`docs/decisions/identity.md` places on the metadata document, and they are the
same here for the same reason: a transfer that rewrote the line endings must not
change a file that nobody edited.

It carries the schema version it is written against, the number of files it lists,
and one entry per file. An entry carries the path, the size in bytes, the digest,
and the role that file plays. `schema/1.0/manifest.schema.json` is the normative
form of all of that, and the shape below is the same thing written to be read.
Where the two disagree the schema is what a deposit is judged against, which is
the rule `docs/decisions/layout.md` sets for every pair like this one.

    {
      "schema_version": "1.0",
      "file_count": 4,
      "files": [
        {
          "path": "scan-001/metadata.json",
          "size": 8351,
          "digest": "sha256:1e2f...",
          "role": "dataset_metadata"
        },
        {
          "path": "scan-001/spectrogram.bin",
          "size": 1310720,
          "digest": "sha256:9a04...",
          "role": "array"
        },
        {
          "path": "scan-001/delay-axis.bin",
          "size": 512,
          "digest": "sha256:c7b1...",
          "role": "array"
        },
        {
          "path": "setup-photograph.jpg",
          "size": 402113,
          "digest": "sha256:44de...",
          "role": "auxiliary"
        }
      ]
    }

The three roles are `dataset_metadata`, `array` and `auxiliary`, and every entry
carries exactly one. The role is what a reader dispatches on, so an auxiliary file
cannot become a measurement by sitting in a dataset directory and an array cannot
become auxiliary by being moved out of one.

The role in the manifest is a coarse one and is not the role in the measurement
digest. `identity.md`'s listing carries the role the model gives an array, meaning
which array it is, and that comes from the metadata document. The manifest's
`array` says only that the file holds measurement values. Two different words in
two places would have been the mistake to make here, and they are two different
questions: the manifest asks what kind of file this is, the metadata asks what
this array is.

`file_count` is required by `dataset-unit.md`, which asks for a checksum per file
and the total count. What it buys is small and worth stating exactly. It catches a
`files` array that lost entries to something that was not parsing the document,
and it makes the two numbers a reader can compare visible without counting. It
does not catch a manifest that was edited, because whoever removed an entry could
decrement the count, and it is not a defence against anybody. The digests are not
a defence against anybody either, and the next section says so.

## The manifest lists every file except itself

A manifest cannot carry its own digest. `identity.md` gives the argument for the
metadata document and it holds here without change: a document containing its own
digest cannot be produced, because writing the digest changes the bytes the digest
covers.

So `manifest.json` is not among the entries, `file_count` does not count it, and
nothing inside the deposit verifies it.

What that means has to be said plainly rather than left for a reader to work out.
The manifest is a defence against a transfer that lost or corrupted a file. It is
not a defence against somebody who edited the deposit, because the same person
could recompute every digest in it. A reader who wants to know that a deposit is
the one a paper cited compares the identifiers in `identity.md`, which are
computed from the arrays and the metadata document rather than from the manifest,
against the identifier the citation carries. The manifest makes a damaged copy
detectable. The identifier makes a different copy detectable. Those are separate
properties and this format does not let the first stand in for the second.

## The checksum algorithm

SHA-256, written as the lowercase hexadecimal digest with `sha256:` in front of
it.

That is `docs/decisions/identity.md`'s decision and it is not re-made here. The
prefix is what lets a second algorithm be added later without any stored digest
becoming ambiguous about which one produced it, and a digest with no prefix is not
a digest this format recognises. The digest covers the file's bytes, all of them,
with nothing skipped and nothing normalised, which is available only because
`container.md` put nothing in an array file but the values.

A depositor can reproduce every digest in a manifest with the tool their operating
system already ships. That was one of the reasons the algorithm was chosen, and it
is the reason a manifest is worth anything to somebody who does not have this
repository's software.

## A file in the manifest and not on disk

Refused, naming the path.

This is the partial transfer, and it is the case the manifest exists for. A
deposit that lost its metadata document during a copy must fail on inspection
rather than read as a deposit with no metadata, and a deposit that lost one array
out of forty must fail rather than read as a deposit with thirty-nine.

`fixtures/manifest/refused-listed-file-missing` is a deposit whose delay axis did
not arrive and whose manifest still lists it.

## A file whose bytes do not match

Refused, naming the path.

The size is compared before the digest. Both are in the manifest, the size is
cheap where the digest is a full read, and a size mismatch is worth naming in its
own words because truncation is what it usually is and truncation is what a
depositor can act on. A file whose size matches and whose digest does not is
reported as a digest mismatch, which is a different thing that happened.

Neither is a warning. `docs/decisions/raw-counts.md` and the validator in issue
#32 both rest on counts being the numbers the detector reported, and a file that
does not match its digest is a file whose numbers are not known to be those.

The two are a fixture each, because one message standing in for both is the
outcome this section is written against.
`fixtures/manifest/refused-truncated-array` is short by 64 bytes.
`fixtures/manifest/refused-digest-mismatch` is the right length with one bit of
one value changed.

## A file on disk and not in the manifest

Refused, naming the path.

This is the interesting direction and it is the reason the manifest lists files
rather than just carrying a checksum of the whole. An unlisted file is how an
unnoticed second copy of a supposedly corrected array survives: somebody fixes a
calibration, writes a new array beside the old one, updates the metadata to point
at the new file, and the old file sits in the directory looking exactly like part
of the deposit. Every tool that dispatches from the metadata reads the right one.
A person opening the directory sees two.

Refused rather than ignored, because ignoring it means the deposit on disk and the
deposit in the manifest are different objects and nothing ever says so. Refused
rather than warned, because a warning nobody has to act on is a line people learn
to scroll past, and this one appears in a directory listing anyway.

The walk that finds an unlisted file is over the whole deposit directory,
recursively, with no exclusions. There is no ignore list, no exemption for hidden
files and no exemption for the droppings an operating system leaves in a directory
somebody browsed.

That is strict and the cost lands on real people. A depositor who opened their
deposit directory in a file manager on one common platform will find a hidden index
file in it and their deposit will be refused for a file they did not create. The
alternative is worse and it is worse in exactly the direction this archive cares
about: an ignore list is a set of names that are invisible to the check, and a
name is a cheap thing for an unwanted file to have. A refusal that names the file
is something a depositor fixes in one command. An exemption is something nobody
revisits.

`fixtures/manifest/refused-unlisted-file` carries the superseded copy this section
describes, and `fixtures/manifest/accepted-auxiliary-listed` carries an extra file
that is listed. The second is what keeps the rule to unlisted rather than extra: a
rule refusing anything outside a dataset directory would refuse the auxiliary
material `docs/decisions/dataset-unit.md` admits, and it would pass every fixture
the first one does.

## Reading a deposit with nothing installed

`docs/decisions/container.md` requires this specification to show the read rather
than assert that it is easy, because a format whose documentation asserts that it
is easy to read, without showing the read, is a format whose first independent
implementation discovers the order convention the hard way.

The metadata document names the shape, the element type and the byte order. The
example below is a spectrogram of 8 delay points by 16 energy bins of 64-bit
floats, little-endian, written with the delay index varying slowest.

In Python, with the standard library only:

    import sys, json, array
    meta = json.load(open("scan-001/metadata.json", encoding="utf-8"))
    values = array.array("d")
    with open("scan-001/spectrogram.bin", "rb") as f:
        values.fromfile(f, 8 * 16)
    if sys.byteorder != "little":
        values.byteswap()
    row = values[3 * 16 : 4 * 16]

`array.array` gives a flat sequence and the index arithmetic is the reshape. A
reanalyst who has `numpy` will use `numpy.fromfile` and `reshape` instead and
should, but the read above needs nothing obtained from anywhere, which is the
property `container.md` asked for.

The byte order line is not decoration. It is the one place the read is wrong on a
machine that is not little-endian, and it is silent when it is wrong.

In MATLAB, with the base product only:

    meta = jsondecode(fileread("scan-001/metadata.json"));
    f = fopen("scan-001/spectrogram.bin");
    values = fread(f, 8 * 16, "float64=>double", 0, "ieee-le");
    fclose(f);
    trace = reshape(values, [16 8]);

The reshape is where the two languages disagree and where a transposed trace comes
from. The block is written with the delay index varying slowest, so the energy
index is contiguous. MATLAB's `reshape` fills its first dimension fastest, so the
shape handed to it is energy first and delay second, which is the reverse of the
order the metadata states. On a scan that is not square, getting this wrong fails
loudly. On a square scan it produces a trace that reconstructs to a plausible
wrong answer, which is why `container.md` made the storage order a field rather
than a convention.

The MATLAB snippet has not been executed. There is no MATLAB on the machine this
document was written on, which is the same gap `docs/decisions/schema-language.md`
records against its own claim that a stock MATLAB parses JSON with `jsondecode`.
Both are claims from documentation rather than measurements, both are owed a
command, and neither should be read as verified because it appears in a
specification. If a stock MATLAB turns out not to read one of these, it is this
section that was wrong.

## What this document does not settle

The metadata document's keys, which are the model in `docs/model/` and the schema
in `schema/`.

What the deposit level carries above the manifest.
`docs/decisions/schema-language.md` names `schema/1.0/deposit.schema.json` for
what `docs/decisions/dataset-unit.md` made addressable, and it is not in the tree:

    git ls-files -- 'schema/*/deposit.schema.json' | wc -l
    0

Nothing in this document depends on it. A manifest is a claim about files and the
deposit level is a claim about datasets, and the second is where a deposit says
which of its directories are datasets rather than leaving a reader to infer it
from the presence of a `metadata.json`.

How a deposit is laid out inside an operator's store, which is issue #47 and is a
different question: this document is about the deposit as it arrives and as it is
copied, and a store is free to hold it differently as long as what comes back out
is this.

The memory ceiling for reading and validating, which is issue #35, and the
identifiers a deposit and its datasets carry, which are `identity.md` and issues
#37 through #40.

Whether the archive admits single-shot data at all, which is entry 7 of issue #1
and is open. Nothing above changes with the answer, because the offset of any
element is arithmetic on its indices and a deposit of forty gigabytes has the same
shape as one of four megabytes.
