# How a deposit reaches an archive

Status: decided. Issue #83.

## The decision

The route is pull. The depositor publishes the deposit at a location they control
and can keep available for as long as the transfer takes, and they send the
archive that location and the checksums from their manifest. The operator fetches
it, checks it, and records the fetch.

Nothing enters the store that did not pass an intake. The intake is one door and
it leaves a record whichever way it ends, including when it ends in a refusal.

The staging area the fetched bytes land in is not a store. It has no index, it is
not readable through the reader, and it holds bytes for a bounded time. That
boundary is the point of this whole record, and the failure it is designed
against is the one where files accumulate somewhere with no validation, no
manifest and no retention, because that is what happens when the route is
undecided and somebody has data to send today.

## Why pull

The depositor keeps control until the last moment. Up to the fetch they can
replace the files, and after it they can take their copy down, which matters to a
group depositing before their paper is out.

There is no service to run, to secure, to keep available or to defend. This board
builds software an operator runs. An upload endpoint would make it hosted
software, which is a different project with a different threat model, and whether
anything is hosted publicly at all is entry 12 of issue #1 and is open.

Personal data does not travel through infrastructure nobody here controls. A
deposit sent as a mail attachment puts the measurement, the depositor's address
and whatever they wrote in the covering note onto a mail server, in a form that
is copied, backed up and retained by rules this archive does not set.

It fits a world where the persistent identifier comes from somewhere the
depositor already publishes. Who mints that identifier is entry 4 of issue #1 and
is open, but pull is the route that stays sensible under every answer to it.

## The channel of last resort

A group with nowhere to publish exists and is not a hypothetical, and refusing
them would defeat the purpose. Where a depositor cannot publish, the operator
agrees a transfer channel with them directly.

That is a change of transport and nothing else. The bytes land in the same
staging area, pass the same checks in the same order, and produce the same intake
record, and the record names the channel. What is not permitted is a second door:
no path exists by which bytes reach the store without an intake record, and a
channel agreed by two people does not become one.

## The rejected routes

**An upload endpoint.** The most convenient thing for the depositor and the
reason it lost is above: it is a service, with availability, authentication,
abuse and a hosting decision that has not been made. It is also the route that
most easily becomes the archive by accident, since an endpoint that accepts bytes
before it validates them is a directory of unvalidated deposits by another name.

**A file sent by whatever means the two parties already use, with the archive
taking no position.** Cheapest of all. It lost because taking no position is the
failure mode itself: no manifest, no checksum, no record of what arrived or when,
and personal data through channels nobody chose. It survives as the channel of
last resort above, with a position taken.

**A version control or transfer protocol built for large files.** Precise,
resumable, checksummed by construction, and it asks a depositor to install and
learn something before they can give the archive anything. The first outside
dataset is the milestone that counts, and this is the option most likely to lose
it. If the size ceiling ever admits per-shot data this deserves re-arguing,
because at that size a resumable protocol stops being ceremony.

## What a deposit passes between arrival and the store

In this order. The first failure stops the sequence and the rest are not
attempted, so a report says how far it got.

1. The manifest is present, is well formed, and lists every file that arrived
   with no file left over and none missing.
2. Every file's checksum matches its manifest entry, and the manifest's own
   checksums match the ones the depositor sent separately from the bytes. The
   second comparison is what makes the first mean anything: a manifest fetched
   from the same place as the files is checked against itself.
3. The deposit validates against the schema, with no error. Warnings do not stop
   the intake and are recorded in the report and shown to the depositor.
4. The declared completeness level from `raw-counts.md` is consistent with the
   arrays themselves. Integer counts where `counts` is claimed, shots per delay
   point present where the level requires them, a recorded factor where `scaled`
   is claimed.
5. The container carries no personal field outside the published set named in
   `personal-data.md`.
6. The terms the deposit is offered under are declared. Which terms are
   acceptable is issue #71 and is not decided here; that a deposit states them is
   decided here, because a deposit whose terms are unstated cannot be
   redistributed and therefore cannot enter an archive whose purpose is
   redistribution.
7. A person reads it. Axes monotonic and in the direction they claim, delay zero
   inside the scanned range, energy calibration producing energies that could
   belong to the stated target, the provenance block describing an apparatus that
   could have taken this measurement. No check in the list above replaces this
   one, and this record does not pretend the earlier six add up to it. What a
   review looks at and what a refusal says is issue #59.

Only after all seven does the deposit enter the store. The store ingest is issue
#48 and is a separate operation from the intake on purpose: a deposit that passed
intake and has not yet been ingested is in a known state, and a half-ingested one
is not.

## What is recorded about an intake, including a refused one

Every intake writes a record, and a refusal writes one too. An arrival that
leaves no trace is how a deposit gets lost and how a depositor gets no answer.

The record holds the date of arrival, the route and the channel, the location the
bytes came from where there was one, the checksums as received, the manifest as
received, the report from each check in the sequence above and how far the
sequence got, the outcome, and the identity of the person who performed the
review where the sequence reached it.

An intake record is local. It is never published and never exported. A public
record that a named group's deposit was refused is a harm to them out of all
proportion to anything the archive gains, and the depositor is told the reason
directly instead.

On a refusal the staged bytes are deleted once the depositor has been told and
has had a stated period to respond, and the record says when they were deleted.
The record survives the bytes. The retention of the record follows the personal
data rules, which tie it to a dataset that in this case never existed, so a
refused intake's record is kept for a period the operator states in their own
retention policy and is deleted with the rest of the local personal data if the
depositor asks.

## Where an embargoed deposit waits

In the store, not in the intake path.

An embargoed deposit passes all seven checks and is ingested like any other, with
its visibility state set so that it is not distributed. It is a validated,
manifested, indexed dataset that is not yet published, which is a state the store
can hold and audit.

The alternative, leaving it in staging until the embargo lifts, is the exact
failure this record is written against. It would put the deposits with the
longest residence time in the one place with no index and no validated state, and
it would mean the archive's most sensitive holdings sit outside its own
guarantees.

Whether embargo is offered at all before the first release is entry 5 of issue #1
and is open, and issue #60 holds the design. This record does not decide that. It
decides that if the answer is yes, the waiting happens in the store.

## Which parts of the intake path hold personal data

The depositor's identity and their address, which is how the archive answers
them. The channel, since an address on any channel is a personal field. The
covering note or correspondence that came with the deposit. The reviewer's
identity. The source location where it contains a person's name, which happens
more often than expected on a personal or group web space.

All of it is local, all of it is in the personal record described in
`personal-data.md`, and none of it is written into a container. The source
location is stored for as long as the intake record is kept and is not published,
for the same reason the source path of a conversion is not recorded at all in
`conversion-boundary.md`.
