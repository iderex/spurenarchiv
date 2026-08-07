# What the archive holds about a person

Status: decided. Issue #19.

## The decision

The measurement and the people are two stores, not two sections of one file.

A dataset container holds the physics and a small, named set of attribution
fields. Everything else the archive knows about a person lives in a separate
personal record, keyed by the dataset identifier, stored apart from the
measurement bytes and never copied by any operation that copies a dataset.

The reason is that a dataset is meant to be copied. It goes to a reanalyst, into
a benchmark export, and to another node if an operator ever federates. Anything
inside it travels with every one of those copies, permanently, including copies
made by somebody who only ever wanted the array. A personal field inside the
container is therefore a personal field the archive has published, whatever the
intention was, and no later deletion reaches it.

Separation by construction makes the difference checkable instead of promised.

## The fields

Published means the field is inside the dataset container and travels with every
copy of it. Local means the field is in the personal record on the node, is never
written into a container, and never leaves the host.

| Field | Required | Where | Retention |
| --- | --- | --- | --- |
| Creator name, one per creator | required | published | as long as the dataset version exists |
| Creator ORCID | optional | published | as long as the dataset version exists |
| Creator affiliation as stated at deposit | optional | published | as long as the dataset version exists |
| Contact address for questions about the dataset | optional | published only if it is a role address the depositor supplies for that purpose | as long as the dataset version exists |
| Depositor name | required | local | while the dataset is held |
| Depositor e-mail | required | local | while the dataset is held |
| Depositor ORCID | optional | local | while the dataset is held |
| Depositor institution and its address | optional | local | while the dataset is held |
| Correspondence that settled a question about the deposit | optional | local | while the dataset is held, and it is stored as the answer rather than as the exchange wherever the answer is what mattered |
| The intake record for this deposit, including the channel it arrived on | required | local | while the dataset is held |
| The reviewer's identity and the review outcome | required | local | while the dataset is held |

The published set is four rows and it is deliberately short. A citable dataset
with no named creator is worth little to the depositor, and the depositor is the
person this archive has to persuade, so attribution is published on purpose. Nothing
else is.

A personal e-mail address is never published. The contact row is published only
where the depositor supplies an address that is a role rather than a person, and
if they supply a personal address instead it is stored local and the container
carries no contact field.

Every retention above is stated relative to the dataset. How long the archive
keeps a dataset at all is not decided in this record: that is issue #74 and it
depends on entry 11 of issue #1, which is open. This record cannot state an
absolute duration and does not pretend to.

## What happens on withdrawal

A withdrawal is a request that the dataset stop being available. What it can and
cannot undo is different for the two stores, and saying so before a depositor
deposits is part of the terms in issue #71.

The local personal record is deleted. Every row marked local above, including the
correspondence and the intake record, is removed from the node. What survives is
a note that a record existed and was deleted on a date, which carries no personal
field and exists so that the deletion itself is auditable.

The published set behaves differently, and this is the part that has to be said
plainly rather than softened. A dataset version that has already been distributed
cannot be recalled. Copies exist on other people's disks, in benchmark exports and
possibly on other nodes. Withdrawal on this node removes the dataset from
distribution and leaves a tombstone that says the version existed and was
withdrawn. Where the version was already cited, the tombstone keeps the creator
names, because a citation that silently loses its creators is a worse outcome for
everybody involved than one that resolves to a withdrawal notice. Where the
version was never distributed, the tombstone carries no creator names at all.

What the tombstone contains and how an identifier resolves after a retraction is
issue #39. This record fixes only which personal fields it may carry.

## The property

An export of a dataset contains no personal field outside the published set.

It is testable and it is meant to be tested rather than asserted. The shape of
the test is a deposit whose local personal record is filled with distinct
markers, one per local row above, exported through every export path the tool
has, with an assertion that no marker appears anywhere in the exported bytes.
Distinct markers rather than one, so that a path that leaks exactly one field is
not hidden by the other ten passing.

The check that carries this into the suite is issue #73. The reason it is a check
rather than a paragraph is that the separation is easy to lose by accident: one
convenience field added to a container to make a listing prettier is enough, and
nothing about it looks like a mistake at the time.

## Why the legal statement quotes this and not the other way round

The data protection statement in issue #72 is written for a reader and it is
written in plain words. If it is drafted first, it describes an archive nobody
has built yet, and the archive then either has to match a text written before the
design or quietly diverge from it. Deciding here and quoting there means the
statement describes what the tool actually does, and the property above is what
makes the description checkable.
