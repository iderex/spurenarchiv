# The digests a deposit carries, and what each one decides

Two operators hold a file each and want to know whether they hold the same
measurement. A digest answers that only if it is clear which bytes it covers, and
the obvious choice of one digest over the whole deposit answers the wrong
question: a metadata correction changes the deposit and does not change the
measurement.

`docs/decisions/identity.md` decided the shape. This document is the normative
part: the exact byte sequence each digest covers, written so that somebody with a
hashing tool and a sort can reproduce every value in a deposit without this
repository's software. That is not a convenience. It is the property the whole
construction was chosen for, and a specification that left any of the byte
sequences to be inferred would take it away.

`docs/spec/deposit-layout.md` is where the files these digests cover are
arranged, and it carries the manifest, which is a different thing from an
identifier and says so in its own words.

## The algorithm

SHA-256.

Every stored digest is written as `sha256:` followed by the digest in lowercase
hexadecimal, with no other separator and nothing after it. The prefix is part of
the stored value. It is what lets a second algorithm be added later without any
stored value becoming ambiguous about which one produced it, and a digest written
without it is not a digest this format recognises.

Where a digest appears inside a sequence another digest covers, it appears in
exactly this form, prefix included. There is no shorter spelling anywhere.

## The array digest

**Covers.** The file's bytes, all of them, in order, with nothing skipped and
nothing normalised.

**Answers.** Whether two files hold the same array.

There is no header to exclude, because `docs/decisions/container.md` put nothing
in an array file but the values. There is no normalisation before hashing, for
the same reason: the bytes are the values, so there is nothing a writer could
have laid out differently.

## The measurement digest

**Covers.** A listing built from the dataset's arrays and from nothing else.

One line per array. Each line is the array's role, a single space, the array's
length in bytes written in decimal with no leading zero, a single space, and the
array's digest. Each line is terminated by a single line feed, including the
last. The lines are sorted by role, ascending, comparing the roles as sequences
of bytes. The listing is ASCII.

    <role> <length in bytes> sha256:<hex>

**Answers.** Whether this is the same measurement.

The role rather than the file name is what appears, and that is the point of the
construction rather than a detail of it. A dataset renamed, moved between
directories, repacked or copied onto another operator's store has the same
measurement digest. A listing that named files would tie the identity of a
measurement to a directory layout, and the layout is the thing most likely to
change.

The role comes from the metadata document, which is what says which array is
which. A role appears at most once in a dataset: the listing is sorted by role,
so two lines carrying one role have no defined order between them, and two
different datasets could then produce one listing. A dataset with a repeated role
is refused.

A dataset with no arrays has no measurement digest. It is not the digest of an
empty listing, because that value would be the same for every such dataset and
would say that they are all the same measurement.

## The metadata digest

**Covers.** The metadata document's bytes as deposited, all of them.

**Answers.** Whether this is the same description of that measurement.

The document is UTF-8, with line feed line endings and no byte order mark. That
is a requirement of the format rather than a convention, and the reason is this
digest: a transfer that rewrote the line endings would otherwise change the
digest of a document nobody edited.

The metadata document does not contain its own digest. A document containing its
own digest cannot be produced, because writing the digest changes the bytes the
digest covers. So the digests live in the manifest and in the store's index. The
same argument reaches one step further out and is why a registered persistent
identifier is not written into the document either, which is
`docs/decisions/persistent-identifier.md`.

## The version digest

**Covers.** Two lines, in this order, each terminated by a single line feed:

    measurement sha256:<hex>
    metadata sha256:<hex>

The two labels are `measurement` and `metadata`, spelled exactly like that,
separated from the digest by a single space. The sequence is ASCII.

**Answers.** Whether this is the same version of the same dataset.

It moves when either half moves, which is what makes it the identifier a citation
should carry: it names one version of one dataset and nobody can produce a
different version carrying the same one.

The labels are there so that the sequence cannot be read as two anonymous
digests whose order somebody has to remember. Fixing the order and labelling the
lines are both cheap, and losing this one would be silent.

## The deposit digest

**Covers.** The version digests of the datasets in the deposit, one per line,
each terminated by a single line feed, sorted ascending by byte order.

**Answers.** Whether this is the same set of datasets, delivered together.

It exists because `docs/decisions/dataset-unit.md` made the deposit an
addressable level. It is not the identity of anything a paper cites, and a
citation carrying it is citing a delivery rather than a measurement.

## The concept identifier

Not a digest of anything on its own. It is the version digest of a dataset's
first version, carried forward by every later version as a declared field.

`identity.md` states the limit this hides and it is repeated here because it is
the one a reader is most likely to assume away: an identifier derived from
content cannot be stable across versions, because versions differ in content by
construction. So the concept identifier is content-derived when the first version
is created and is a declaration in every version after that, and the chain is
checkable backwards and not forwards.

## What a reader may not do

Compare a digest without its prefix against one with it. They are different
strings and the format has one spelling.

Recompute a digest over a normalised form of anything. There is no normalisation
step anywhere in this document, and a reader that introduced one would produce
values that agree with nothing.

Treat a matching deposit digest as a matching measurement. The deposit digest
covers a delivery. Two operators who packed the same measurement with different
neighbours hold the same measurement and different deposits.

Treat the manifest as an identifier. `deposit-layout.md` says why: the manifest
makes a damaged copy detectable, and the identifier makes a different copy
detectable. Neither stands in for the other.

## What refuses any of this

`.github/workflows/content-address.yml` recomputes every digest above from the
fixture bytes under `fixtures/content-address/` using nothing but `sha256sum`,
`sort`, `cut` and `printf`, and asserts the three properties this document
exists to give:

- a metadata correction leaves the measurement digest unchanged and moves the
  version digest
- a deposit rewritten with identical content under different file names
  reproduces every digest
- one flipped bit in one array moves the measurement digest and moves nothing
  about the metadata

The second is not a test of any hashing code. It is the test of `container.md`'s
canonicity claim, and it is the only place in the plan where that claim is
checked by a machine. If it ever fails, the repair is in `container.md` rather
than here.

The check also carries the digests of one fixture dataset as a pinned
expectation, so that a run showing the construction agreeing with itself is
distinguishable from a run showing that the same bytes produce the same digests
on a machine that has never seen this repository.

What it does not check is that any program in this repository computes these
digests. There is a program now and it computes no digest at all:

    git grep -n -i -e sha256 -e digest -- tool/src/ ; echo "exit=$?"
    exit=1

The validator in issue #32 judges one metadata document and opens nothing beside
it. The reader in issue #36 is where a program first has to agree with this
document, and until then this document and the check above are what a reader has
instead.

Nothing here refuses a deposit. The refusals a deposit meets are the validator's,
and issues #32 and #33 hold them.

## What this document does not settle

Where the digests are stored, which is the manifest in `deposit-layout.md` and
the store's index in issue #47.

What a version is, as opposed to a new dataset, and what happens on a correction,
a retraction or a withdrawal, which is `identity.md` and issue #39.

The registered persistent identifier, which is
`docs/decisions/persistent-identifier.md` and is a pointer to these values rather
than a value of this kind.

The form a person writes into a manuscript and types back in, which is
`docs/spec/identifier.md`. It is a rendering of the version digest above and not a
second identity: the digests here are what a store matches on and what a depositor
recomputes, and that document says so in its own words.
