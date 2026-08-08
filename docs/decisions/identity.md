# How a dataset is identified, versioned and corrected

Status: decided. Issue #18.

## The decision

A dataset's identifier is derived from its content. Nothing is assigned, nothing
is minted, nothing has to be looked up, and no registry has to be running for an
identifier to exist. Two copies of the same measurement on two operators' disks
carry the same identifier because they carry the same bytes, and a copy that has
been altered carries a different one whether or not anybody meant it to.

There are two digests per dataset, computed separately and never combined into
one, because they answer two questions that a correction pulls apart: which
measurement is this, and which version of it am I holding.

Everything below rests on the container decision in `container.md`, which is what
makes a byte digest mean anything at all. A container whose bytes change when the
writer's library version changes cannot be identified by its content, and that
was one of the four reasons the strongest candidate there lost.

## The digest, and the bytes each one covers

The hash is SHA-256, written as lowercase hexadecimal with the algorithm name in
front of it, so that a second algorithm can be added later without any stored
digest becoming ambiguous about which one produced it. It was chosen because a
depositor already has it: the tool that computes it ships with every operating
system this archive expects to meet, so a depositor can reproduce any digest in
this record without installing anything.

**Per array file.** The digest covers the file's bytes, all of them, with nothing
skipped and nothing normalised. There is no header to exclude because
`container.md` put nothing in the file but the values.

**The measurement digest.** The digest covers a listing built from the arrays and
nothing else. One line per array, holding the role the model gives that array, its
length in bytes and its digest, separated by single spaces, sorted by role,
terminated by a line feed, in ASCII. The measurement digest is the digest of that
listing.

The role rather than the file name is what appears, and that is the whole point of
the construction. A dataset renamed, moved between directories, repacked or copied
onto a different operator's store has the same measurement digest, which is what
issue #37 asks for. A listing that named files would tie the identity of a
measurement to a directory layout, and the layout is the thing most likely to
change.

**The metadata digest.** The digest covers the metadata document's bytes as
deposited. The document is UTF-8 with line feed endings and no byte order mark,
and that is a requirement of the format rather than a convention, because a
transfer that rewrote the line endings would change the digest of a document
nobody edited.

The metadata document does not contain its own digest. A document containing its
own digest cannot be produced, so the digests live in the manifest and in the
store's index, and issue #31 is where the manifest's own shape is settled.

**The version digest.** The digest covers two lines, the measurement digest and
the metadata digest, each labelled, in that order, line feed terminated. It
identifies exactly one version of one dataset and changes when either half
changes.

**The deposit digest.** The digest covers the sorted list of the version digests
of the datasets in the deposit, one per line. It exists because `dataset-unit.md`
made the deposit an addressable level, and it is not the identity of anything a
paper cites.

Each of these can be recomputed by hand from the files, with a hashing tool and a
sort. That is a deliberate property: an identifier a depositor cannot verify
without this repository's software is an identifier they have to trust.

## Why the measurement and the metadata are hashed separately

A metadata correction is the commonest thing that will ever happen to an archived
dataset. A calibration entry with a transposed digit, a delay sign recorded
against the wrong convention, a target purity that turns up in a notebook two
years later: all of them change the metadata document and none of them touches a
single count the detector recorded.

If one digest covered both, every one of those corrections would produce a
different identifier for the same measurement, and an archive would have no way to
say that the array in front of it is the array a paper reanalysed. Hashing them
separately means the measurement digest is stable across every metadata
correction, and a reanalysis that quotes the measurement digest is quoting the
numbers rather than the paperwork around them.

## The two levels, and the one thing content addressing cannot do

The version identifier is the version digest. It resolves to exactly one version,
it is self-verifying, and nobody can produce a different version carrying the same
one.

The concept identifier, meaning the identifier that always resolves to the newest
version of a measurement, is the version identifier of the first version, carried
forward by every later version as a declared field.

That second sentence hides a limit and this record states it rather than letting a
reader discover it. An identifier derived from content cannot be stable across
versions, because versions differ in content by construction; that is what makes
them versions. So the concept identifier is content-derived at the moment the
first version is created and is a declaration in every version after that. Version
two names its predecessor's version identifier and names the first version's, and
those names are assertions by whoever built version two rather than properties of
its bytes.

The chain is checkable backwards and not forwards. Given version two and the bytes
of version one, anybody can confirm that the predecessor it names is real and is
what it says. Nothing in version one's bytes says that version two exists or is
legitimate, and nothing prevents somebody producing a file that claims to be a
version of a measurement it has nothing to do with. What answers that is the
store's own record of what it ingested and when, not arithmetic on a hash, and an
archive that suggested otherwise would be claiming an assurance it does not have.

The alternative was an assigned concept identifier, a random string minted once
and recorded, which is the shape a well-known repository already uses. It was
declined for one reason: it requires something to do the assigning, and the first
version of a dataset then cannot be identified until it has been through that
something. Deriving it from the first version's content means a depositor can
compute their own dataset's identifier on their own laptop before anybody has
seen it, and the archive agrees with them rather than telling them what it is.

A persistent identifier registered outside this board, a DOI or anything like it,
is a third thing and it is recorded as a pointer to a version identifier and to a
concept identifier. Who mints one, and whether anything is minted at all, is entry
4 of issue #1 and issue #40, and both are open. The scheme in this record works
under every answer to them, including the answer that nothing is minted, because
nothing here needs a registry to be running.

## What makes a new version rather than a new dataset

One question decides it. Is this a description of the same photons arriving at the
same detector on the same occasion?

If it is, it is a new version. A recalibrated energy axis, a corrected delay sign,
a detector response supplied two years later, a converter defect repaired against
the same source export, a transcription error found in the target pressure: all
new versions of one measurement.

If it is not, it is a new dataset, and this includes the case that looks most like
a version. A scan repeated the next morning with every setting identical is a
different measurement, it has different counts in it, and archiving it as a
version of the first would make a reanalysis comparing them think it was comparing
one thing with itself.

The case in between is reprocessing, and `raw-counts.md` already answered it: a
processed form produced from the archived array is a derived form stored inside
the dataset, carrying the ordered list of steps that made it, and it is not a
version. A version replaces what the archive says about a measurement. A derived
form adds something beside it. Treating a reprocessing as a version would mean the
newest version of a dataset is the one furthest from the detector, which is the
opposite of what an archive of raw counts is for.

A version records what changed and why, and it records the version it came from.
A version whose change cannot be described is not accepted, for the same reason
`conversion-boundary.md` refuses a derived form whose steps cannot be stated.

## Correction, retraction and withdrawal, which are three things

**A correction** is a new version. The earlier version stays retrievable by its
own version identifier, so a paper that reanalysed it remains checkable, and the
concept identifier now resolves to the corrected one.

**A retraction** says that a version is wrong and should not be used for anything.
The bytes stay retrievable, because a paper that used them is only checkable if
they are, and the retraction record travels with them so that nobody retrieves
them without meeting it.

**A withdrawal** takes the bytes out of distribution. It is the one that costs
something irreversible and it is the one this section is mostly about.

## What a withdrawn dataset's identifier returns

It resolves. It does not return a missing page, an error, or silence, because the
papers pointing at it do not stop existing when the dataset does.

It returns a record holding: the identifier asked for and which level it was, the
fact that the dataset has been withdrawn, the date, the reason in plain words, the
versions affected and whether every version or only some, what to use instead
where there is something, and the digests the withdrawn versions had. The digests
stay in the record on purpose. Somebody holding a copy of the bytes can then
establish that what they hold is what was withdrawn, which is the question they
will actually have.

It does not return the measurement, the metadata document or any field out of it.

Who may withdraw a dataset, and what this board commits to a depositor about
permanence, is entry 11 of issue #1 and issue #74. Both are open, and this record
does not answer either. What it fixes is that whatever the answer is, the
identifier keeps resolving afterwards and the record above is what it returns,
because that property has to be designed into the store before there is anything
in it rather than added once a withdrawal is being asked for.

## What would overturn this

A finding that SHA-256 should not be the algorithm. The construction survives it:
the algorithm name is written in front of every stored digest for exactly this
reason, and the repair is a second algorithm alongside rather than a rewrite of
this record.

A container change that makes the array bytes non-canonical. That would remove the
ground this whole record stands on, and it would be `container.md` that had been
overturned first.

Evidence that depositors cannot compute the measurement digest by hand from the
specification. The construction was chosen so that they can, and if the sorting
and labelling turn out to be a source of error rather than a source of
verification, then a simpler listing is the repair and the property to protect is
that the depositor can still reproduce the number.
