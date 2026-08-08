# The fixtures that prove each digest decides what it claims to

Four datasets, each a directory holding a metadata document, its arrays, and a
`roles` file naming which array is which.
`.github/workflows/content-address.yml` recomputes every digest in
`docs/spec/content-address.md` from these bytes and asserts nine things about
them.

`base` is the dataset the other three are compared against, and it carries
`expected-digests`, which is what its three digests came out as. That file is the
difference between a run showing the construction agreeing with itself and a run
showing that the same bytes give the same digests on a machine that has never
seen this repository. It changes only when the fixture bytes change, and the
specification is what says whether that was allowed.

`corrected-metadata` differs from `base` in one digit of one calibration entry
and in nothing else. It is the commonest thing that will ever happen to an
archived dataset, and it is the case the two-digest construction exists for: the
measurement digest does not move, and the version digest does.

`rewritten` holds the same arrays and the same metadata bytes under different
file names in different directories. Every digest matches `base`, which is what
the role-rather-than-filename listing buys and what `docs/decisions/container.md`
claims about its own bytes. This is the only place that claim is checked by a
machine.

`flipped-bit` differs from `base` by one bit of one array. Its measurement digest
moves and its metadata digest does not, so a corrupted array is reported where it
happened rather than as a dataset that has generally changed.

The `roles` file is fixture scaffolding. In a deposit the role of each array
comes from the metadata document, and the field that carries it is issue #22's.
Naming the roles in a file beside the arrays lets these fixtures exercise the
listing before that field exists, and it is why `rewritten` can rename every
array and keep its identity.

The metadata documents here are not conforming deposits and are not meant to be
read as examples of one. What they are is a sequence of bytes with a known
digest. A complete deposit meant to be read as a deposit belongs in `examples/`,
which `docs/decisions/layout.md` keeps separate from this directory for exactly
that reason.
