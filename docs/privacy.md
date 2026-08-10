# What this archive holds about you

This is written for the person the data is about rather than for a lawyer, and
it describes what the software does rather than what anybody intends. Where a
sentence is enforced by a check, the check is named. Where nothing enforces it,
the sentence says so. Both kinds are in here and the second kind is more common
today than the first.

The design this describes is `docs/decisions/personal-data.md`, and it was
decided before this document was written. That order is deliberate: a statement
drafted first describes an archive nobody has built, and the archive then either
follows a text written ahead of it or quietly diverges from it.

## The sentence everything else is a detail of

Personal data stays on the host the operator runs, and it leaves that host only
when the operator deliberately publishes a dataset or federates with another
node.

Nothing in this archive sends anything anywhere on its own.

## The two stores, and why there are two

The measurement and the people are two stores rather than two sections of one
file.

A dataset container holds the physics and a short, named set of attribution
fields. Everything else the archive knows about a person lives in a separate
personal record on the operator's node, keyed by the dataset identifier, and no
operation that copies a dataset copies it.

The reason is that a dataset is meant to be copied. It goes to a reanalyst, into
a benchmark export, and to another node if an operator ever federates. Anything
inside it travels with every one of those copies, permanently, including copies
made by somebody who only wanted the array. A personal field inside the container
is a personal field this archive has published, whatever anybody meant, and no
later deletion reaches it.

So the difference is a matter of which store a field is in, which is checkable,
rather than a matter of care, which is not.

## What the template asks for

These are the fields a depositor is asked to supply, and the template is the only
place the archive asks for any of them. `Published` means the field is inside the
dataset container and travels with every copy of it. `Local` means the field is in
the personal record on the node, is never written into a container, and never
leaves the host.

| Key | What it is | Required | Where | Why it is asked for |
| --- | --- | --- | --- | --- |
| `creator_name` | The name of each person credited for the dataset | required | published | A citable dataset with no named creator is worth little to the people who made it, and attribution is most of what a depositor gets in return |
| `creator_orcid` | A creator's ORCID | optional | published | It is the one identifier that survives a name change and a move between institutions, which is what makes a credit findable years later |
| `creator_affiliation` | A creator's affiliation as stated at deposit | optional | published | It says where the work was done at the time, which a later affiliation does not |
| `contact_address` | An address for questions about the dataset | optional | published only where it is a role address supplied for that purpose | A reanalyst with a question about a calibration has somebody to ask, which is the difference between a dataset that gets reused and one that gets abandoned |
| `depositor_name` | The person sending the deposit | required | local | The archive has to know who it is talking to about this deposit, and who to go back to when a field turns out to be wrong |
| `depositor_email` | How to answer the depositor | required | local | Every intake ends in an answer, including a refusal, and an arrival nobody can answer is how a deposit gets lost |
| `depositor_orcid` | The depositor's ORCID | optional | local | It disambiguates the depositor from another person with the same name in later correspondence |
| `depositor_institution` | The depositor's institution and its address | optional | local | It is what an operator needs to establish that the deposit comes from where it says, and to reach the group if the depositor moves on |

Four rows are published and four are local, and the published four are the
attribution. Nothing else about a person is ever put in a container.

A personal e-mail address is never published. `contact_address` is published only
where the depositor supplies an address that belongs to a role rather than to a
person, and where they supply a personal address instead it is stored local and
the container carries no contact field at all.

Everything above except the two required published rows and the two required
local rows is optional. The default for anything that is not attribution and not
the ability to answer the depositor is optional, and a blank is an answer.

## What the archive records without asking

Three things exist in the personal record that the template does not collect,
because the archive produces them rather than the depositor.

The intake record for the deposit, including the channel it arrived on, which is
`docs/decisions/intake.md`'s and exists so that an arrival leaves a trace.

Correspondence that settled a question about the deposit, stored as the answer
rather than as the exchange wherever the answer is what mattered.

The identity of the person who reviewed the deposit, and the outcome.

All three are local. None of them is ever written into a container, and none of
them is published or exported.

## What is never asked for

This is the part a depositor cannot verify by looking at a form, which is why it
is written down.

The archive does not ask for a date of birth, a gender, a personal postal
address, a personal telephone number, a staff or student number, a nationality, a
photograph, a curriculum vitae, a funding or grant reference tied to a person, or
anything about anybody who is not a creator or the depositor.

It runs no telemetry. There is no usage reporting, no crash reporting, no update
check and no analytics, so there is no place for an address, a hostname or a
machine identifier to be collected from.

It has no accounts, so there is no password, no session and no login record.

The cheapest way to protect a detail is not to collect it, and the list above is
that principle applied rather than a promise about how carefully something is
looked after.

## Retention

Every retention here is stated relative to the dataset, and that is the honest
form rather than a shortened one.

The published attribution is kept as long as the dataset version exists, because
it is part of what a citation resolves to.

Every local field, meaning the depositor's details, the correspondence, the
intake record and the reviewer's identity, is kept while the dataset is held.

How long a dataset is held at all is not settled. It is issue #74 and it depends
on entry 11 of issue #1, which is open, so this document cannot give an absolute
number and does not invent one. When that is answered, the number belongs in
`docs/retention.md` and this section will point at it rather than restate it.

## Withdrawal, and the part that cannot be undone

A withdrawal is a request that a dataset stop being available.

The local personal record is deleted. Every local field above, including the
correspondence and the intake record, is removed from the node. What survives is
a note that a record existed and was deleted on a date, carrying no personal
field, so that the deletion itself can be audited.

The published set is different, and this has to be said plainly rather than
softened. A dataset version that has already been distributed cannot be recalled.
Copies exist on other people's disks, in benchmark exports and possibly on other
nodes. Withdrawal on this node removes the dataset from distribution and leaves a
record saying the version existed and was withdrawn. Where the version was
already cited, that record keeps the creator names, because a citation that
silently loses its creators is worse for everybody than one that resolves to a
withdrawal notice. Where the version was never distributed, it carries no creator
names at all.

So this archive can stop distributing a dataset and it cannot unpublish somebody
else's copy of one. Nobody can. A statement implying otherwise would be worth
less than this paragraph.

## An operator's own obligations

Somebody running this archive for a group has obligations to the people whose
names are in it, and this software does not discharge them.

What it gives them to work with: the two stores are separate by construction, so
answering what is held about a person is a question about one record rather than
a search; the local record is deletable on its own without touching a
measurement; and the deletion leaves an auditable note. What it does not give
them is a lawful basis, a register of processing, or an answer to their own
institution, and no software could.

## What enforces any of this, and what does not

Said in its own section, because a document like this is easy to read as a
guarantee.

**The template collects nothing outside the list above.** Enforced.
`.github/workflows/personal-fields.yml` compares the key set of
`templates/depositor-details.json` against the table in this document and fails
on any difference in either direction, so a field added to the template without a
row here reds the check, and so does a row here with no field in the template.

**No personal field outside the published set reaches an export or a
federation.** Not enforced. There is no export, no federation and no marking in
the schema for a check to read. Issue #73 holds the mechanism and issue #52 holds
the export. Until they land, this is a property of a design rather than of a
running program, and the difference matters.

**No network call is made when a deposit is read.** Not enforced. There is a
reader now and nothing in it opens a socket, but that is a reading of the source
rather than a check that would refuse one being added to it:

    git grep -n -E 'std::net|TcpStream|UdpSocket' -- tool/src/ ; echo "exit=$?"
    exit=1

Issue #49 holds the boundary and the check that would refuse an undeclared call
site.

**No telemetry.** Not enforced and, for now, true by the same absence: there is
no program to send anything. It becomes a claim needing a check on the day there
is one, which is issue #49's.

**The two stores stay separate.** Not enforced. It is a decision recorded in
`docs/decisions/personal-data.md` and the check that would carry it into the
suite is issue #73. The failure mode it is written against is small and ordinary:
one convenience field added to a container to make a listing prettier, which does
not look like a mistake at the time.

## Where to ask

Questions about a deposit go to the operator who runs the node holding it. This
repository is the software rather than an archive, and nobody here can answer a
question about data on somebody else's host.

`SECURITY.md` is the route for a defect in the software itself, including one
that would expose a personal field.
