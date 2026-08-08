# The persistent identifier, and the line between recording one and inventing one

Status: decided. Issue #40.

## The decision

This board records persistent identifiers. It does not mint them, and it never
writes a string in a registered namespace it does not control.

A registration is a pointer, held beside the dataset rather than inside it,
between an identifier some registration agency resolves and the content-derived
identifiers `identity.md` already gives every dataset. Where nothing has been
registered, no such pointer exists and no slot for one is filled with something
that looks like an identifier.

Who mints, and whether anything is minted at all, is entry 4 of issue #1 and is
open. Everything below works under every answer to it, including the answer that
nothing is ever registered, because the archive's own identifiers are computed
from the bytes and need no registry to be running.

## Three identifiers, and only one of them is registered

`identity.md` fixed two of them and this record adds none.

The **version identifier** is the version digest. It names exactly one version of
one dataset, it is self-verifying, and it is derived from the content.

The **concept identifier** is the version identifier of the first version,
carried forward by every later version as a declared field. It names the
measurement across its versions.

The **registered persistent identifier** is a third thing, and it is the only one
of the three this archive cannot produce on its own. It is minted by somebody
else, it resolves through somebody else's infrastructure, and what this board
holds is a record that it exists and what it points at.

Keeping them apart matters because they fail differently. A version identifier
that does not match the bytes is a wrong identifier and anybody can prove it. A
registered identifier that resolves nowhere is a wrong identifier and nobody can
prove it from the deposit, which is the reason for the section below on what is
never invented.

## Why the registered identifier is not written into the metadata document

This is the part that has to be decided before anything is registered, because
the obvious arrangement does not work.

The version identifier is the digest over the measurement digest and the metadata
digest. Write a registered identifier into `metadata.json` after registration and
the metadata digest changes, so the version identifier changes, so the identifier
that was registered against a version now points at a version that no longer has
that identifier. The registration is stale the moment it is recorded, and nothing
about the deposit says so.

It is the same circularity `identity.md` already names for a document containing
its own digest, arriving one step further out, and the same answer follows: the
pointer lives where the digests live rather than inside the bytes they cover.

Two ways around it were available and both are worse.

**Exclude the identifier field from the metadata digest.** That puts a hole in the
digest, which means a key in the metadata document that no hash covers and that
can therefore be changed without changing any identifier. The digest covering the
document's bytes as deposited, all of them, is the property that makes it worth
computing.

**Register first and accept that the pointer names the previous version.** The
recorded relationship is then wrong by exactly one version, permanently, and it
is wrong in a way that reads as correct.

So the registration record sits beside the dataset in the operator's store, keyed
by the identifiers it points at, and the deposit is never rewritten. A
registration is added, corrected and withdrawn without touching a single byte of
the measurement or of the metadata document, which is the same property
`container.md` bought for a corrected calibration.

The cost is real and lands on the depositor. Somebody holding the deposit
directory alone cannot read its registered identifier out of it, because it is not
in there. What they can read is the version identifier, which is what a citation
should carry anyway, and what the archive shows them is the citation record issue
#41 owes. A deposit that arrived carrying an identifier the depositor wrote into
their own metadata is a different case and is below.

## What a registration record holds

The identifier as the agency returned it, written with its scheme in front of it
so that a `doi:`, a `urn:` and a handle are not three strings that have to be told
apart by their shape.

The agency, and the date the registration was made.

Which level it registers, one version or the concept, and the local identifier it
points at: a version identifier for the first, a concept identifier for the
second.

The metadata that was submitted to the agency, or a reference to the exact
submission, so that a later registration can be compared against it. A
registration whose submitted metadata was not kept is one nobody can check
afterwards, and the field it most often gets wrong is the one nobody looks at
again.

Nothing in this record is personal data that is not already in the deposit. Who
performed the registration is in the operator's own record and follows
`personal-data.md`.

## What is never invented

No string in a namespace this board does not control. Not a DOI under somebody
else's prefix, not a plausible-looking suffix under a prefix nobody has
registered, and not a placeholder that differs from a real identifier only in a
digit.

No provisional identifier that is shaped like a registered one. A string that
looks like a DOI will be copied into a reference list by somebody who did not read
this document, and at that point the difference between a real identifier and a
convincing one is a broken citation in a published paper. Where a citable string
is needed before registration, it is the version identifier, which is not shaped
like anything a registry resolves and is verifiable by hand.

No empty registration record. Where nothing has been registered, there is no
record, rather than a record with an identifier field waiting to be filled in.
The two look the same in a listing and only one of them can be filled in wrongly.

## A deposit that arrives already carrying an identifier

Recorded as it arrived, and never replaced.

The depositor's identifier is recorded as an external registration naming the
agency the depositor states, pointing at the version identifier of the deposit as
it arrived. The archive does not mint a second identifier for a dataset that
already has one, because two identifiers for one dataset is how the citations of
one measurement split into two counts that neither resolves.

Where the identifier appears inside the metadata document the depositor wrote, it
stays there. It was part of the bytes that produced the metadata digest, so it
costs nothing and removing it would change the identity of a deposit in order to
tidy it. What the section above forbids is this board adding one afterwards, which
is a different act with a different consequence.

Whether the identifier resolves is not checked. Checking means a network call from
the operator's machine, the network boundary is issue #49 and is not yet drawn,
and a failure would say more about the operator's connection than about the
deposit. The identifier is recorded as the depositor asserted it, and that it is
an assertion rather than a verified fact is written into the record beside it.

## How the local scheme maps onto the two-level scheme

The two-level shape is not this board's invention. Zenodo, minting through
DataCite, already registers one identifier that always resolves to the newest
version and one per version, linked through relations in the registered metadata
rather than by arithmetic on the strings. The scheme in `identity.md` has the same
two levels, so the map is direct:

    concept identifier   ->  the identifier that resolves to the newest version
    version identifier   ->  the identifier registered for one version

and the relations between the registered identifiers carry what the local chain
already declares: that a version belongs to a concept, and which version preceded
it.

Where the two do not line up, written down rather than smoothed over.

**The concept identifier does not exist before the first version does.** It is
derived from the first version's content, so there is nothing to map a reserved
identifier onto until bytes exist. An embargoed deposit that needs a citable
identifier before it opens is the case where this bites, it is issue #60's, and
the shape that works is a registration record naming the reserved identifier with
no local identifier beside it yet, filled in when the first version is built. That
is the one registration record permitted to name no local identifier, and it is
permitted because the alternative is inventing one.

**The registered relations are declarations, exactly like the local ones.**
`identity.md` states that the version chain is checkable backwards and not
forwards: given a later version and the earlier bytes, anybody can confirm the
predecessor it names, and nothing in the earlier bytes says a later version is
legitimate. Registering the same relations with an agency does not improve that.
It publishes the same assertion somewhere else, and a reader who treats a
registered relation as stronger than a declared one has been misled by the
registry rather than informed by it.

**Permanence points the same way and stops at different edges.** An agency
undertakes that a registered identifier keeps resolving, and `identity.md` already
requires a withdrawn dataset's identifier to resolve to a record saying it was
withdrawn. Those agree. What does not follow is control: the record an agency
serves is the agency's, this board cannot make it say what its own withdrawal
record says, and a depositor promised otherwise has been promised something nobody
here can deliver. What this board commits to is entry 11 of issue #1 and issue
#74, and both are open.

## The model fields a registration needs, and the three it does not have

The mapping below is onto DataCite's metadata schema, version 4, because that is
the schema the concrete prior art registers through. It is written from that
schema's published documentation and not from a registration this board has made,
so every line of it is a claim rather than a measurement, and the first real
registration is what turns it into one.

    Identifier        the registered identifier, assigned by the agency
    ResourceType      Dataset, fixed
    Publisher         the operator running the node, from the store's own
                      configuration and not from any field of the model
    PublicationYear   the year the dataset was made available, which is not
                      measurement_date
    Date (Collected)  measurement_date, and acquisition_timestamps where present
    Version           the version identifier
    RelatedIdentifier publication_reference, as a supplement relation, and the
                      version relations above
    Size, Format      the file sizes and count from the manifest, and the
                      container this board writes
    Rights            the licence a deposit carries, which is entry 2 of issue #1
                      and issue #70, and is open

`PublicationYear` and `measurement_date` are two different years and the mapping
says so at the one point where conflating them is tempting. A scan taken in 2019
and deposited in 2026 is a 2026 publication of a 2019 measurement, and dating the
registration to the experiment would make every deposit look like it had been
citable for years.

Three properties a registration needs have no field in this model, which is
checkable rather than asserted:

    git ls-files -- 'schema/1.0/fields/*.json' | wc -l
    49
    git grep -l '"field": "\(title\|creators\|description\)"' -- 'schema/1.0/fields/*.json' ; echo "exit=$?"
    exit=1

A **title**, a **creator list** and a **description**. None of them is a
measurement, which is why the field-by-field work has not produced them: the rule
in `docs/model/README.md` asks each field for the reanalysis step that fails
without it, and no reanalysis fails for want of a title. They are needed anyway,
by the registration and by anybody reading a listing, and naming them here is what
makes the gap visible instead of leaving it to be discovered by whoever first
tries to register something.

This record does not add them. A field lands from a field issue under #21 with its
sentence and its row, and a decision record that quietly added three keys to the
schema would be the drift `layout.md` separates the two directories to prevent.
The creator list is also not only a schema question: named creators are personal
data the depositor chooses to publish, that choice is issue #58's, and a field
landed ahead of it would answer half of it in the wrong place.

The consequence, stated because it is the one that will be met first: a dataset
with no published creator cannot be registered with an agency that requires one.
It is still archived, still identified by its version identifier, and still
readable. What it does not get is a DOI, and the reason is a choice the depositor
made rather than a limit of the archive.

## What this record does not settle

Who mints, which is entry 4 of issue #1.

The citation a depositor is shown and the attribution a reuse carries, which is
issue #41.

The reserved identifier an embargoed deposit needs, which is issue #60, and the
retention and withdrawal commitments, which are entry 11 of issue #1 and issue
#74.

Where the registration record physically sits in an operator's store, which is
issue #47, and what the store's index holds, which is issue #31. This record fixes
that it sits beside the dataset rather than inside it, and not the file it is
written in.

## What would overturn this

A registration agency that requires its identifier to appear inside the deposited
metadata document. The circularity above becomes unavoidable, and the repair is a
stated exclusion from the metadata digest, which is a change to `identity.md`
rather than to this record.

A decision under entry 4 of issue #1 that this board becomes a registration
agency itself. The first sentence changes from recording to minting and
recording, and every section after it survives, because what is forbidden here is
writing a string in a namespace this board does not control rather than writing
one at all.

Evidence that depositors will not cite a version identifier and will only cite
something a registry resolves. That is the assumption behind offering the version
identifier as the citable string before registration, and if it is wrong then
registration has to happen earlier in the intake rather than after it.
