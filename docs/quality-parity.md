# Quality parity with the target gate

The target is the merge gate of `iderex/jellyfin-plugin-sso`. Naming a target
rather than inventing a standard here is deliberate: a gate assembled from first
principles ends up matching whatever this board found easy, and the point of
parity is to be held to something that was not chosen for convenience.

This document is the map. It has one row per control on that gate, what this
board does about it, and a line of reasoning wherever the two differ. It does not
build any of those controls. Each row that names an issue is a control this board
has no check for, and that issue is where it lands.

## What is in force here, before anything below is read

Nothing on this board's own gate requires a status check. Read out of the
ruleset rather than remembered:

    gh api repos/iderex/spurenarchiv/rulesets --jq '.[] | "\(.id) \(.name)"'
    20528564 gate
    gh api repos/iderex/spurenarchiv/rulesets/20528564 \
      --jq '{enforcement, bypass: .bypass_actors, rules: [.rules[].type],
             required: [.rules[] | select(.type=="required_status_checks")
                        | .parameters.required_status_checks[].context]}'
    {"bypass":[],"enforcement":"active","required":[],"rules":["deletion","non_fast_forward","pull_request"]}

So every check named in the map's right-hand column runs on a pull request and
none of them stops a merge. A row saying a control is adopted is a statement that
the check exists and runs, and it is not a statement that the merge waits for it.
Issue #63 is where the checks in this tree become required on the default branch,
and until it lands the parity claimed here is parity of coverage rather than
parity of enforcement.

That distinction is the one this document is most likely to be misread on, which
is why it is at the top rather than in a footnote.

## What the target gate requires, derived rather than remembered

    gh api repos/iderex/jellyfin-plugin-sso/rulesets --jq '.[] | "\(.id) \(.name)"'
    18802863 Protect main and 5.0
    gh api repos/iderex/jellyfin-plugin-sso/rulesets/18802863 \
      --jq '[.rules[] | select(.type=="required_status_checks")
             | .parameters.required_status_checks[].context]'
    ["build","ABI floor build","Package (JPRM) / Build package","Package (JPRM) / Generate SBOM",
     "CodeQL","Analyze (csharp)","DCO sign-off","Deterministic PR-hygiene checks",
     "Enforce greppable invariants","Reject Trojan Source Unicode","Audit workflows (zizmor)",
     "prettier","dependency-review"]

Run on 2026-08-09. That set moves. Re-run the commands rather than citing this
document, and where the output has moved, this map is what is out of date.

## How to read a row

**Adopted** means this board takes the same control. **Adapted** means the same
property is enforced differently, because the language or the artefact differs.
**Dropped** means the control does not apply here, and the row says why rather
than saying that it is not relevant.

The right-hand column names the check in this repository that implements the row.
Where it names an issue instead of a check, there is no check: the control is one
this board is taking and has not built. Those rows are what is owed, not what
runs, and no reading of this table should turn one into the other.

## The map

| Control on the target gate | State here | What implements it |
| --- | --- | --- |
| `DCO sign-off` | adopted | `.github/workflows/dco.yml`, check `DCO sign-off` |
| `Deterministic PR-hygiene checks` | adopted | `.github/workflows/pr-hygiene.yml`, check `Deterministic PR-hygiene checks` |
| `Reject Trojan Source Unicode` | adopted | `.github/workflows/unicode-guard.yml`, check `Reject Trojan Source Unicode` |
| `Audit workflows (zizmor)` | adopted | `.github/workflows/zizmor.yml`, check `Audit workflows (zizmor)` |
| `dependency-review` | adopted | `.github/workflows/dependency-review.yml`, check `dependency-review` |
| `Enforce greppable invariants` | adopted | nothing yet, issue #65 |
| `build` | adapted | nothing yet, issues #5 and #6 |
| `prettier` | adapted | nothing yet, issue #5 |
| `CodeQL` | adapted | nothing yet, issue #66 |
| `Analyze (csharp)` | dropped | there is no C# here |
| `ABI floor build` | adapted | nothing yet, issue #34 |
| `Package (JPRM) / Build package` | adapted | nothing yet, issues #76 and #4 |
| `Package (JPRM) / Generate SBOM` | adopted | nothing yet, issue #67 |

## The reasoning, per deviation

`Enforce greppable invariants` is adopted rather than adapted because the control
transfers unchanged: it is a lint over the tree's own text for patterns a
reviewer would have to remember, and this board has invariants of that kind
already, including the ones its own documents state about paths, spellings and
the shape of a record. What it lints is different; what it is is the same. Issue
#65 is where the invariants for this board are chosen, and choosing them is the
work rather than running the linter.

`build` is adapted because there is nothing to compile. On the target board that
one context carries the restore, the build, the test run and a coverage bar in
one job. Here the equivalent property is that the tree's own checks all run and
all pass, which is issue #6's single gate command, and the checks it runs are
issue #5's. The coverage bar inside that job is not part of this row and is
treated separately below.

`prettier` is adapted rather than adopted because the target board runs it over
`js, html, md, css, scss` and this tree carries none of the first, third or
fourth of those. What corresponds is a formatter over the file kinds this board
does carry, which are Markdown, JSON and YAML, and the property is the one that
made the control worth having: a formatting question is settled by a tool rather
than in review, so a diff never carries a whitespace argument. Issue #5 holds the
format check.

`CodeQL` is adapted because the control is static analysis for the language the
artefact is written in, and the language here is fixed by
`docs/decisions/means.md`. Whether the analyser is CodeQL or something else is
issue #66's to answer against that record rather than something this map decides,
and naming the tool here would have been the map deciding an issue that belongs
to somebody reading the standpoint.

`Analyze (csharp)` is dropped. It is the same CodeQL run reported under its
matrix language, and there is no C# in this tree:

    git ls-files | grep -c '\.cs$'
    0

A row that dropped it as not relevant would be saying nothing, so the reason is
the measurement rather than the judgement. If this board ever grows a C# surface,
the row that returns is `CodeQL` above and not this one.

`ABI floor build` is the row where the target's control and this board's have the
same shape and a different subject, which is what adapted is for. That board
builds against the oldest host it claims to support, so a change that quietly
raised the floor fails before a user finds out. The compatibility surface here is
not an application binary interface; it is the deposit format, and the
corresponding promise is that a deposit written against an older schema version
still reads. That is issue #34, and it is a stronger obligation rather than a
weaker one: an unreadable old deposit is a citation that stopped resolving, where
a raised ABI floor is a plugin that fails to load and says so.

`Package (JPRM) / Build package` is adapted. The target packages a plugin for a
host that installs it; this board's artefact is a binary an operator runs, which
is issue #76, and the reproducibility that makes a package worth anything is
issue #4. Both are named because a package that is not reproducible does not
carry the property this control exists for.

`Package (JPRM) / Generate SBOM` is adopted with no adaptation, because a bill of
materials is a statement about a dependency set and this board will have one the
moment it has a lock file. Issue #67 holds it together with the provenance
attestation.

## The controls beside the required set

The target board also runs controls that are not required to merge. They are in
this map because leaving them out would make parity look cheaper than it is, and
they are marked as what they are.

| Control | Required there | State here | What implements it |
| --- | --- | --- | --- |
| Supply-chain scoring, `Scorecard analysis` | no | adopted | `.github/workflows/scorecard.yml`, check `Scorecard analysis`, and it does not run on a pull request |
| A coverage bar on the surface that takes security decisions | yes, inside `build` | adapted | nothing yet, issue #68 |
| Mutation testing | no | adapted | nothing yet, issue #69 |
| Coverage-guided fuzzing | no | adapted | nothing yet, issue #33 |
| An end-to-end harness | no | adapted | nothing yet, issue #8 |

Scorecard here is adopted and its triggers are narrower than the map's other
adopted rows, which is worth saying rather than leaving in the table. It runs on
a push to the default branch, on a schedule and when the ruleset changes, and not
on a pull request:

    git grep -n -A6 '^on:' -- .github/workflows/scorecard.yml

So a change that lowered this board's score is scored after it lands rather than
before, and the same is true on the target board. That is the control as it
exists in both places and not a gap this map is reporting.

The coverage bar is the one deviation that can be reasoned in the other
direction, and the issue that opened this programme said so. That board bars the
surface that decides authentication. The surface here that carries the same
weight is the one deciding whether a deposit is accepted and what its numbers
mean, which is issue #68. It is listed as not required there and required here
because on that board the bar sits inside a job that is required, and a bar this
board sets on its validator should sit inside whatever the equivalent is rather
than beside it.

Mutation testing is adapted rather than adopted because the target runs it over
source and the thing to mutate here is the set of refusals: a validator that
still accepts everything it should refuse after a rule is deleted has a rule that
was never load-bearing. Issue #69 holds it.

Coverage-guided fuzzing is adapted to the same subject. The input this board
takes from a stranger is a deposit, not a login, and the property is that a
malformed one is refused rather than crashing a reader or steering it out of the
deposit directory, which `docs/spec/deposit-layout.md` already requires of a
path. Issue #33 is where a refusal gets a fixture, and the fuzzing belongs with
it rather than in a row of its own.

The end-to-end harness maps onto issue #8, which is already written as a separate
target that refuses to report success when its precondition is missing. That is
the same property the target board's harness has and the reason both are kept out
of the default suite.

## What this document does not settle

Whether any of it is required to merge, which is issue #63 and the paragraph at
the top.

Whether the target gate is the right target. It is the one issue #62 named, and
changing it is a decision rather than an edit to this table.

Any control the target board adds after the date above. The commands are here so
that a later reader derives the set rather than trusting this one, and a row
missing from this table is this table being out of date rather than the control
being dropped.
