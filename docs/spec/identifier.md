# The identifier a dataset carries, and the form a paper can carry

A dataset is going to be copied between operators, mirrored, exported into a
benchmark case set and cited in a paper. Every one of those breaks an identifier
that encodes a location, and the archive then has several names for one
measurement and no way to tell which of them is the measurement.

This document defines the identifier this archive generates for itself. It is
available on a machine that has never been on a network and it needs nothing
minted by anybody. The registered persistent identifier is a different string
with a different owner, it is
`docs/decisions/persistent-identifier.md` and issue #40, and a reader who takes
the first identifier they find as the citable one will take the wrong one, which
is why the difference is the first thing written here.

## What is already decided, and is not re-decided here

`docs/decisions/identity.md` fixed that a dataset's identifier is derived from its
content rather than assigned, and `docs/spec/content-address.md` fixed the
construction: the version digest names one version of one dataset, the concept
identifier is the first version's version digest carried forward as a declared
field, and every digest is SHA-256 written as lowercase hexadecimal with
`sha256:` in front of it.

That is the identity. Nothing below replaces it, and the store, the manifest and
any comparison of two copies use the digest and never the form this document
adds.

## The problem this document exists for

The identity is 71 characters:

    printf 'sha256:%s\n' "$(printf '' | sha256sum | cut -d ' ' -f 1)" | tr -d '\n' | wc -c
    71

A string of that length cannot be typed back in from a printed page, wraps in any
two-column layout, and has no way to tell a reader that they mistyped it: one
wrong digit gives a string that matches no dataset, and failing to resolve and
detecting an error are different outcomes for somebody holding a paper.

Hexadecimal is better than the alternatives on exactly one line, which is that it
mixes no similar-looking characters, and it is the encoding a depositor's own
hashing tool produces without help. Both of those are why it is the identity and
why this document does not touch it.

## The citable form

A rendering of the version digest into 20 symbols, written in five groups of
four:

    17QQ-8RQP-PBQQ-R6WP-6G83

Twenty-four characters with the hyphens. It is derived from the digest by the
function below, the derivation runs one way only, and the digest cannot be
recovered from it.

**The alphabet** is Crockford's base 32 symbol set:

    0123456789ABCDEFGHJKMNPQRSTVWXYZ

`I`, `L`, `O` and `U` are absent. The first three are absent because a reader
copying from a printed page confuses them with `1`, `1` and `0`, and the fourth
because it is the letter that turns a random string into a word somebody has to
read out.

**Written form** is uppercase in five groups of four separated by hyphens. The
hyphens are not part of the value.

**Read form** is lenient in three ways and no others. Case is ignored. Hyphens
are ignored wherever they appear, so a form broken across a line by a typesetter
still reads. `I` and `L` are read as `1` and `O` as `0`, which is the substitution
a person makes and not one this archive invents. Anything else is not an
identifier: a trailing full stop is not stripped and not accepted, because a
reader that discarded characters it did not recognise would accept a truncation.

**The derivation**, from a version digest:

1. Take the 85 most significant bits of the digest. The digest is 64 hexadecimal
   characters; the first 21 give 84 bits and the 22nd gives the last one.
2. Encode them as 17 symbols of five bits each, most significant first.
3. Put the symbol `1` in front. That is the version of this rendering, so that a
   later rendering of the same digest is distinguishable from this one rather
   than being a string of the same shape that means something else.
4. Read those 18 symbols as a base 32 number and take it modulo 1021. Write the
   result as two more symbols, the first carrying the quotient by 32 and the
   second the remainder.

Step 4 is the whole of the error detection and 1021 is chosen rather than
convenient: it is the largest prime below 1024, so two symbols carry it, and it
is prime so that no single-symbol difference and no swap of two neighbours can
vanish under the modulus.

**Checking a typed string** is the same arithmetic. Recompute the two symbols
from the first 18 and compare. A form whose check symbols do not agree is
refused, and it is refused as a mistyped identifier rather than as an unknown
dataset, which is the difference a person needs.

## What the check symbols catch, and the proof

Every substitution of one symbol for another, anywhere in the twenty, and every
transposition of two adjacent symbols that differ.

Both are properties of the modulus rather than of a table, and neither is a
statistical claim. A substitution at position `i` changes the number by
`d * 32^(18-i)` with `0 < |d| < 32`; 1021 is prime and neither factor is
divisible by it, so the residue moves. A transposition of neighbours at `i` and
`i+1` changes it by `(a - b) * 32^(17-i) * (1 - 32)`, and 31 is not divisible by
1021 either. A substitution inside the two check symbols changes the stored value
and not the computed one. A swap of the two check symbols changes the stored
value unless they are equal, and a swap across the boundary between the payload
and the check symbols requires `33 * (a - b) ≡ 0 (mod 1021)`, which needs
`a = b`.

Measured as well as argued, over the identifiers of twenty thousand digests:

    substitutions: 12400000 accepted wrongly: 0
    transpositions inside the covered prefix: 329328 accepted wrongly: 0
    transposition across the boundary (17,18): 19390 accepted wrongly: 0
    transposition of the two check symbols (18,19): 19428 accepted wrongly: 0

What is not caught is anything larger: two errors at once, a symbol dropped
together with another inserted, or a form retyped from a different dataset
correctly. The check symbols are for a slip, not for an adversary, and nothing
here is a defence against somebody who wants a string to verify.

## What the truncation costs

Eighty-five bits of the digest are kept and 171 are discarded, so the citable
form names a dataset and does not prove one. Two datasets whose version digests
agree in their first 85 bits carry the same citable form, and the digest that
distinguishes them does not.

The number is a trade rather than a length that looked right:

    python -c "print(['%.3g' % (N*N/2**86) for N in (10**4, 10**6, 10**8)])"
    ['1.29e-18', '1.29e-14', '1.29e-10']

An archive of a hundred million datasets is four orders of magnitude larger than
this field will ever produce, and at that size the expected number of colliding
pairs is still about one in ten billion.

What follows from that is a rule rather than a reassurance. A resolver matches on
the digest. A citable form is expanded to the datasets whose version digest
begins with those 85 bits, and if that is more than one, the resolver reports the
ambiguity and the digests, and never picks one. A resolver that chose would turn
the one accepted risk of this design into a silently wrong answer, which is the
class of failure this archive exists against.

## The concept identifier's citable form

The same function applied to the concept identifier, which
`content-address.md` defines as the first version's version digest. So a paper
citing a dataset without pinning a version writes the citable form of the concept
identifier, and one pinning a version writes the citable form of that version's
version digest. The two are the same string for a first version and differ for
every version after it, which is the same relation the digests have and is not a
new fact this document introduces.

Which of the two a citation should carry is issue #41.

## What a reader may not do

Compare a citable form against a digest. They are different strings and neither
contains the other.

Recover a digest from a citable form. 171 bits are gone.

Store a citable form as the identity of anything. It is a rendering, an index
built on it is an index on a truncation, and the store's rule is
`docs/decisions/identity.md`'s.

Accept a form whose check symbols do not agree, on the grounds that the payload
looks fine. The check symbols exist because the payload always looks fine.

## What refuses any of this

`.github/workflows/content-address.yml` derives the citable form of every fixture
dataset under `fixtures/content-address/` from the version digest it recomputes
from the bytes, and asserts four things.

That `base` renders to the identifier pinned beside it in
`fixtures/content-address/base/expected-identifier`, so a run showing the
derivation agreeing with itself is distinguishable from a run showing that the
same bytes give the same identifier on a machine that has never seen this
repository.

That `base` and `rewritten` render the same identifier. Those two hold identical
content under different file names in different directories, and the check copies
the deposit to a further path before deriving it again, so the clause about a
deposit copied to a different path is measured rather than argued from the
construction.

That `corrected-metadata` and `flipped-bit` render different identifiers from
`base`, so the rendering does not lose the thing the digests exist to keep.

That every single-symbol substitution and every adjacent transposition of two
different symbols in the identifier is refused, with the three counts printed, so
a sweep that tried nothing cannot read like a sweep that tried everything.

What none of that checks is that any program in this repository derives these
identifiers. There is a program now and it computes no digest at all, so it
derives nothing this document is about:

    git grep -n -i -e sha256 -e digest -- tool/src/ ; echo "exit=$?"
    exit=1

The reader in issue #36 is where a program first has to agree with this document.

## What this document does not settle

The registered persistent identifier, which is
`docs/decisions/persistent-identifier.md` and issue #40. Who mints one at all is
entry 4 of issue #1 and is open.

What an identifier returns after a version is superseded, corrected or withdrawn,
which is `identity.md` and issue #39.

The citation string a depositor is given and what a reuser is asked to cite,
which is issue #41.

Whether a second rendering is ever added. The version symbol in front exists so
that one can be, and nothing here decides that one will.
