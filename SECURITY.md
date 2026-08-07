# Security policy

## Reporting a vulnerability

Use GitHub's private vulnerability reporting on this repository: open the
Security tab and choose "Report a vulnerability". The report stays private
between you and the maintainers until a fix is published or the report is
closed.

That is the route this project asks for. Please do not open a public issue for a
vulnerability, and please do not send the details to a personal address: a public
issue publishes the problem before there is a fix, and a mail thread puts the
report on infrastructure this project does not control.

If you cannot use private reporting for any reason, open a public issue that says
only that you have a security report and nothing about its content, and a private
channel will be arranged.

## What is in scope

The parts of this project that read bytes somebody else produced, or that decide
what leaves an operator's machine. Specifically:

The deposit parsers, meaning everything that reads a deposit: the container, the
manifest, the metadata and the arrays. A deposit is a file from a stranger, and
the validator is the component whose whole purpose is to read one. Memory safety,
resource exhaustion driven by a crafted deposit, a path in a manifest that
escapes the directory it is unpacked into, and any route by which reading a
deposit executes something are all in scope.

The federation receive path, meaning anything that accepts a deposit or a query
from another node. Federation is off unless an operator turns it on, and that
makes it a smaller surface rather than a lesser one, because an operator who
turns it on is trusting this code with an open door.

The personal-field marking, meaning the mechanism that decides which fields are
published and which stay on the host. A defect that lets a field marked local
reach an export, a federation payload or any other outbound path is a security
defect and is treated as one rather than as a data-modelling mistake. The rule it
implements is in `docs/decisions/personal-data.md`.

None of these three exists yet. Every tracked file in this repository is a
document or a workflow definition, and there is no implementation to attack.
Check that rather than take it from this file:

    git ls-tree -r --name-only HEAD

So there is currently no code in scope for this policy. The scope is named before
the code exists on purpose, so that the components most worth attacking are known
to be in scope on the day they land rather than assessed afterwards.

The repository's own automation is also in scope: the workflow definitions, and
anything that could let a pull request from outside this repository gain write
access or read a secret.

## What is out of scope

- An operator's own deployment: the machine, the account it runs under, the
  network it sits on, and any configuration choice they made. What this project
  owes there is documentation, and a gap in the documentation is a normal issue.
- A vulnerability in a dependency with no demonstrated path through this
  project's code. Report those upstream. If you can show a path through this
  project, that is in scope and is worth reporting here.
- A deposit that is scientifically wrong. A wrong calibration or an invented
  measurement is a data-integrity problem and a review problem, and neither the
  validator nor this policy claims to catch it.
- Reports produced only by running a scanner, with no described impact. A tool's
  output is a starting point rather than a finding.
- Anything about a publicly hosted instance of this software. This project builds
  software an operator runs and does not run a public service.

There is no bug bounty and no payment.

## What to expect

An acknowledgement is aimed at within 14 days. This is a small project and that
is a statement of intent rather than a guarantee: nothing here promises a
response time it could be held to, and pretending otherwise would be worse than
saying so.

If 30 days pass with no acknowledgement at all, treat the report as not received
and say so publicly without describing the vulnerability, so that the silence is
visible.

After a report is acknowledged, what happens is the ordinary process of this
repository. The problem becomes an issue saying what is wrong, what the evidence
is and what a fix has to achieve, and the fix lands as a pull request against it.
Where the fix is a refusal, it ships with a fixture proving it refuses the case
that was reported.

## Credit and your details

If you want to be credited, say so and how you want to be named. If you say
nothing, you are not named.

Whatever the report carries about you is personal data and is held under the
rules in `docs/decisions/personal-data.md`: it stays on the host, it is not
published, and it is not written into anything a dataset carries.

## Versions

There is no release yet, so there is no supported-version table. Once releases
exist this section states which ones receive fixes. Until then the answer is the
default branch.
