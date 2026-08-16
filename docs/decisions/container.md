# The container a deposited dataset is written in

Status: decided. Issue #12.

## The decision

A dataset is a set of files and not one file. Its metadata is a text
document, and each array it carries is a separate file holding nothing but the
array's values, one element after another in a stated order, with the shape, the
element type and the byte order stated in the metadata and not in a header on
the bytes.

The metadata sits beside the arrays, not inside them. That is the second half of
this decision and it carries as much weight as the first.

The concrete syntax of the metadata document, and how a schema version is stated
in it, is issue #13. What this record fixes is that the metadata is text, that it
is one document per dataset, and that it is a separate file from every array it
describes. How the files of a deposit are arranged in directories, and what the
manifest that lists them looks like, is issue #31. What this record fixes there
is only that a dataset's files are named by its metadata and that each array file
is reachable from the document that describes it.

The container is described normatively in `docs/spec/`, in prose complete enough
that somebody can write a conforming reader without opening this repository's
source. That obligation is `layout.md`'s and this record is what makes it
cheap: a format whose specification is longer than its reader is a format nobody
implements twice.

## An array and its axes, held without loss

The array is written as its values and nothing else. An integer array of counts
is stored as integers and stays exactly the integers the detector reported, which
is what `raw-counts.md` requires and what no lossy or floating point round trip
can promise. The element type is named in the metadata out of a small fixed set,
the byte order is named, and the validator refuses a deposit that leaves either
of them to be inferred.

The axes are arrays too, written the same way in their own files. A delay axis is
not a start, a step and a count, because a scan that was not taken on a uniform
grid then has to be forced onto one to be recorded, and `raw-counts.md` names
interpolation onto a uniform grid as an irreversible step. Storing the coordinate
of every point costs a few kilobytes against a spectrogram of megabytes and
removes the pressure entirely.

The storage order is stated and never assumed, and the specification carries the
two-line read for each of the two readers this record commits to, because this is
where the loss actually happens. The values run in one nested order, and the two
languages that have to read them disagree about which order their own reshape
means: a block written with the delay index varying slowest, handed to a
column-major reshape, comes back transposed. On a scan that is not square that
fails loudly. On a square scan it produces a trace that reconstructs to a
plausible wrong answer, which is issue #22's failure and the reason the order is
a field and not a convention.

## A metadata record large enough for the model

The metadata document holds every field the model in issue #21 defines, and the
model is not small: the delay axis and how it was calibrated, the energy
calibration and its provenance, the dressing field, the target, the detector
response, the acquisition record, the processing list from `raw-counts.md`, the
converter block from `conversion-boundary.md`, and an absence state per field
from issue #16. A text document has no ceiling on how much of that it can carry
and no key length limit, which is more than one of the rejected candidates can
say.

It is also the part that gets corrected. A wrong energy calibration entry, a
delay sign recorded against the wrong convention, a target purity nobody wrote
down at the time: these are the corrections issue #18 has to make possible, they
are all metadata, and none of them is a correction to the measurement.

## Readable from Python and MATLAB with nothing installed

Both requirements are met by the shape and not by a library. The array is a
flat block, so it is one call in each language to read it and one to give it its
shape, and neither call comes from a package the reader has to obtain. The
metadata is text, so at worst it is readable by eye and at best it is one call
too, and how well that goes depends on the syntax issue #13 chooses. That is the
one criterion this record hands to #13 rather than deciding: whichever syntax it
picks, a reader with a stock installation of either language has to be able to
parse it, and a syntax that needs a package on the reanalyst's machine fails a
requirement that was set here.

The specification carries the snippets. Not as a courtesy: a format whose
documentation asserts that it is easy to read, without showing the read, is a
format whose first independent implementation discovers the order convention the
hard way.

## Comparable as bytes

Two copies of a dataset compare as bytes because the bytes are the values and
nothing else. There is no library version in the file, no creation timestamp, no
chunk layout, no allocation order and no free space left behind by an edit. Write
the same array twice with two different programs on two different machines and
the files are identical, which is what makes a checksum in a manifest mean what a
depositor thinks it means.

This is also what issue #18 needs in order to hash the array and the metadata
separately and get an identity that survives a metadata correction. A container
whose measurement bytes are rewritten every time a calibration typo is fixed
cannot offer that, and it is the single line on which the strongest candidate
lost.

## Twenty years

An open specification and more than one independent implementation is the stated
bar, and the shape chosen clears it in the way that needs the least trust: the
number of independent implementations of a flat block of little-endian
two's-complement integers is not a property of anybody's ecosystem. It is a
`fread`. If every tool named in this repository is gone, the array is recoverable
from the specification with a hex editor and patience, and the metadata is
recoverable by reading it.

That is a lower bar than it sounds and it is the right one. Durability here is
not a promise that some organisation keeps maintaining a library. It is the
absence of anything that has to be maintained.

## Streaming, and the size ceiling that is not yet set

Whether the archive ever admits single-shot data is entry 7 of issue #1 and is
open. This record does not decide it and does not need to, because a flat block
is the one shape where the question does not change the format. The offset of any
element is arithmetic on its indices, so a reader takes one delay slice out of a
terabyte file with a seek, and a writer emits it a slice at a time without ever
holding the whole array. Issue #35's memory ceiling is a property the reader can
hold instead of one it observes, for the same reason.

What a large deposit would still need is a transfer path and a chunked storage
decision, and those are real work that this record does not do. What it does is
leave entry 7 answerable either way without the container being the thing that
has to change.

## NXmpes, and NeXus underneath it

Refused as the container. Adopted as a vocabulary to check the model against, and
named as the export the archive should eventually be able to write.

The case for it is strong and it was not dismissed. NXmpes is an existing
application definition for multidimensional photoemission in which the varied
axis may be a pump-probe delay, so this board would be extending a standard
rather than inventing one, and it comes from the instrument and beamline world,
which is a large part of the population this archive is trying to reach. A
depositor who already writes NeXus would have had nothing to learn.

It lost on four things, in this order.

The measurement bytes and the metadata live in one file, so every metadata
correction rewrites the file that holds the measurement. Issue #18 asks for an
identity for the measurement array that a metadata correction does not move, and
in a single-file container that identity does not exist to be given.

The bytes are not canonical. The same logical content written by two versions of
the library, or by the same version after an in-place edit, is not the same
sequence of bytes, so a checksum stops being a statement about the data and
becomes a statement about the writer. That is fatal for a manifest whose whole
job is to let two copies be compared, and for the content addressing in issue
#38.

The metadata cannot be reviewed. The metadata is where the errors are, this
board's review path in issue #59 has a person reading a deposit, and a container
no diff can display turns that reading into an operation that requires the tool,
which is the dependency `layout.md` exists to prevent.

The library is large and it is C, and it sits between a depositor and the check
they run before sending anything. That is the smallest of the four and it is not
nothing.

The fit of the definition itself was the thing checked last, and it is a smaller
objection than the four above: NXmpes is built for photoemission generally, this
measurement's fields for the dressing field, the delay sign convention and the
shots behind each point are not the ones it was designed around, and a fit like
that has to be checked field by field rather than assumed from the name. That
check is worth doing anyway, as a way of finding fields this model is missing,
and issue #82 is where the existing standards get examined. Adopting the
vocabulary where it fits costs nothing and buys a reanalyst who already knows one
of the two.

A NeXus export is a derived form in the sense `raw-counts.md` already defines.
It is produced from the archived dataset, it never stands in place of it, and a
read that asked for the measurement does not return it.

## The other candidates that lost

**Everything as text, arrays included.** Maximally inspectable and requires no
specification at all for the array. It lost on size and on exactness. A real
spectrogram becomes tens of megabytes of digits, every read is a parse, and a
floating point value written as text and read back is a round trip somebody has
to prove is lossless rather than one that obviously is.

**netCDF.** The classic form is a genuinely simple documented binary format with
independent readers, and the modern form is HDF5 underneath and inherits every
objection above. Either way it arrives with the CF conventions, whose axes and
vocabulary were built for geoscience, and a convention set that does not fit gets
ignored, which leaves the cost of adopting it and none of the benefit.

**FITS.** The closest of the standard formats to what was chosen and the one with
the best durability argument of any candidate here, since it is a text header
followed by a binary block and has been readable for decades. It lost on the
header. The keyword and card format was designed for a metadata record far
smaller than this model, and carrying the model in it means either fighting the
format or adopting an extension convention, at which point the durability
argument is being made about a format nobody is actually using. Its big-endian
default and its astronomical conventions are the smaller half of the objection.

**Zarr.** The nearest living relative of this decision: a directory, JSON
metadata beside chunked binary arrays, an open specification, and streaming built
in. It lost on two lines. Chunking and optional compression mean the bytes are
not canonical for a given array, so the comparison property above goes, and a
rechunked copy of identical values is a different set of files. And its reach
into the second required reader is uncertain: whether a stock MATLAB installation
reads it today is not measured in this record, and a container whose second
mandatory reader is a question mark is not the one to build an archive on. If
both of those change, this is the candidate to re-argue, because the shape is
already right.

**ASDF.** A YAML tree with binary blocks in one file, from astronomy, and again
very close in spirit. It lost on the single file, for the same reason NXmpes did:
the metadata and the measurement share a file, so a correction to one rewrites
the other. Its implementations are also concentrated in one language, which is
the twenty-year line rather than a comment on the quality of the code.

## Beside rather than embedded, and the drift that buys

Embedded metadata cannot drift from its array and that is a real property to give
up. It is given up on purpose, because everything this archive has to do with
metadata is a thing that is easier when the metadata is a separate file: correct
it without touching the measurement, review it in a diff, hash it separately from
the array, and replace it in a new version while the array's identity holds.

The drift is answered rather than accepted. The metadata document names each
array file it describes and carries that file's checksum, so a metadata document
paired with the wrong array is a mismatch the validator refuses rather than a
silent misreading, and the manifest in issue #31 catches a file that went missing
between them. That is a check with a fixture behind it under issues #32 and #33,
not a paragraph asking people to keep files together.

## What would overturn this

A measurement of what the depositor population actually writes, showing that
NeXus files already exist for these scans in numbers that make refusing them the
thing standing between this archive and its deposits. Issue #82 is the
measurement. That would not answer the four objections above, and the shape of
the repair would be an accepted NeXus intake that is converted at the boundary
into this container, with the original kept as what arrived, rather than this
record being reversed.

A Zarr ecosystem in which both required readers are stock and the byte layout for
a given array is canonical. The shape was already right and only those two lines
were wrong.

An element type that a flat block cannot express without inventing an encoding.
Nothing in the model as it stands is one, and a detector chain that produced one
would deserve its own argument rather than a quiet extension to this record.
