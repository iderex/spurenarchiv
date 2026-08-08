# The implementation language and the toolchain

Status: decided. Issue #2.

## The decision

The core, the validator, the reader, the store and the export are written in
Rust, built with Cargo, against a toolchain version pinned in a tracked file and
a tracked `Cargo.lock` that the build is required to obey. The manifest sits at
`tool/Cargo.toml`, so the lock file and the build configuration are inside the
directory `layout.md` says is the tool, and nothing about the build sits at the
repository root.

The named parts of the toolchain are Cargo for the build and the dependency lock,
`rustfmt` for the format check, and Clippy for the lint check with warnings
treated as errors. The test runner is the one Cargo already carries. Which exact
version is pinned, and how the pin is written, is issue #4 and is deliberately
not fixed in this record: a version number written here would be a second place
for the pin to live and the two would drift.

Whether a Python-facing reader is added alongside this is entry 6 of issue #1. It
is open, it is not settled here, and this record is written so that the answer
either way does not disturb it.

## Conventions the compiler can refuse

This is the line the decision was made on. The defect class this archive exists
against is a silent convention confusion that produces a plausible number, and
the cheapest place to remove it is before the program runs.

Rust distinguishes types that share a representation. A delay, a kinetic energy
and a time of flight are three newtypes over the same floating point number, and
adding one to another does not compile. There is no implicit widening and no
implicit conversion between them, so every crossing between two conventions is a
call somebody wrote on purpose and a reviewer can find by name.

The stronger half is the part that survives a change made two years from now. A
convention that can take several values is an enumeration, matching on it is
exhaustive, and adding a variant turns every site that handles the old set into a
compile error until it is dealt with. The absence states in issue #16 and the
completeness levels in `raw-counts.md` are both of this shape, and both are
things this board expects to extend. A means where a new variant falls through a
default branch is a means where extending the model silently changes what the
existing code does to data it has already accepted.

The third part is absence. There is no null and no zero value that arrives on its
own. A field that may be missing is an `Option`, an unmeasured quantity cannot be
read as a number without the reader saying what to do when it is not there, and a
struct cannot be constructed with a field left out. Issue #16 exists because
unknown gets written as zero. In a means whose uninitialised number is zero, that
defect is the default behaviour of the language and every guard against it is a
convention somebody has to keep.

## The validator on a machine with nothing on it

A depositor checks their own deposit before sending it, and the thing they run
has to be one file they downloaded. Rust compiles to a native executable with no
runtime to install, no interpreter, no package manager and no dependency tree on
the target machine, and the release artefacts in issue #76 are the platforms this
compiles for rather than the platforms an interpreter happens to exist on.

The cost sits on this board rather than on the depositor, which is the trade being
made: cross-compilation, one build per named platform, and a test matrix that has
to keep every one of them working. Which platforms are named is entry 9 of issue
#1 and is open.

## Arrays are the payload

The container format is issue #12 and is not decided here, so this line is
answered for both directions it can go.

Where the container is a plain typed block of numbers with its shape and type
stated in a text document beside it, the array path needs no library at all: a
length-checked read of little-endian IEEE-754 or two's-complement values out of a
memory-mapped file is in the standard library and the arithmetic on top of it is
`ndarray`, which is the ecosystem's array crate and carries no C dependency.

Where the container is HDF5, the path is the `hdf5` crate binding the C library,
and the cost is a C library on the build machine and a decision about whether it
is vendored or found. That cost is real, it is the same cost every language pays
for HDF5 except the ones whose ecosystem hides it, and it is a cost of the
container rather than of the language. What this record fixes is that neither
outcome requires anybody here to hand-write a format parser, which was the line
issue #2 asked about.

## A build that is the same twice

`Cargo.lock` is tracked, it names an exact version and a hash for every
dependency in the graph, and the build runs in the mode that refuses to resolve
anything the lock does not already name rather than quietly updating it. A drift
between the manifest and the lock then fails at build time and names the file,
instead of appearing months later as a difference nobody can explain.

The reverse direction matters as much and is easy to lose. A build that rewrites
the lock as a side effect leaves the working tree dirty and the rewritten pins
are the evidence for what drifted, so the locked mode is the normal mode for
every route rather than an extra flag somebody remembers on the release build.

Rust's compilation is deterministic given a fixed toolchain version, a fixed
dependency set and a fixed set of paths, and the remaining sources of difference
are known ones with known switches: absolute paths compiled into the binary, and
the build environment. That is a claim about the general shape rather than a
measurement of this tree, because this tree has nothing to build yet. The
measurement is issue #4's, which asks for two builds from a clean clone compared
byte for byte with the comparison recorded, and if it comes back negative it is
this record's fourth line that failed rather than issue #4's procedure.

## Tests without a display, without elevation, and the harnesses on top

The default test runner needs no display server, no elevated account and no
network, so the constraints in issue #7 are the state a Rust suite starts in
rather than something recovered later. Nothing in the toolchain opens a window,
binds a socket or asks for a privilege on its own, and a test that does either is
a test somebody wrote that way.

Property testing is `proptest`, which is a dependency rather than part of the
toolchain, and that is the same posture as every language here except one.

Coverage-guided fuzzing is the one line where the answer is worse than the
alternative and the record says so rather than rounding it up. Go carries fuzzing
in its default toolchain and its test runner. In Rust the common harness has
historically needed a second toolchain channel, which fights the single pin this
record just chose, and the alternative harnesses are third-party. This record
does not assert what the current requirement is, because nothing here has run it.
Issue #8 is where the harness lands and is where the requirement gets measured
against a command, and if a second channel turns out to be needed, that is a cost
this decision accepts with its eyes open rather than a surprise.

## The candidates that lost

**Go.** The serious one, and it lost on less than the framing in issue #2
suggested. Go does refuse arithmetic between two defined types over the same
underlying number, so the first half of the convention argument is not a
difference between them. It lost on the other two halves. A `switch` over a
convention has no exhaustiveness requirement, so adding an absence state or a
completeness level compiles everywhere and changes behaviour at every site that
had a default branch. And the zero value is the defect issue #16 is about,
promoted to a language feature: an unset delay sign is a delay sign of zero, an
unset intensity is zero watts per square centimetre, and no reading of the struct
tells the two apart. Against that it offers a simpler build, faster compiles, a
cross-compilation story that is genuinely better than Rust's, and fuzzing in the
toolchain. Those are real and they are worth less here than a compiler that
refuses to let an unknown enter as a number.

**Python.** It has the one argument no other candidate has: the depositors and
the reanalysts are already in it, and a reader they can import is inside their
existing scripts the day they hear about the archive. That argument is real and
it is not answered by dismissing the language. It lost as the means for the core
because it cannot refuse a unit confusion before the program runs, because the
self-contained validator becomes a bundling exercise with a per-platform
packaging tool between the depositor and the check, and because a runtime and a
dependency tree on a stranger's machine is exactly the barrier the validator
exists to remove. It is not refused as an interface: entry 6 of issue #1 is open
and a reader written against the container's published specification is a
different question from the language the core is built in.

**C++.** It has the array and HDF5 libraries in hand and the mature numerical
ecosystem, which is the strongest opening of the four. It lost on nearly every
other line at once. There is no single dependency lock and no single build tool,
so the reproducible build in issue #4 becomes a build-system project rather than
a flag. The convention safety it offers has to be built out of templates and
maintained by discipline, with implicit conversions working against it by
default. The memory safety cost falls on a parser reading untrusted deposit bytes
from strangers, which is the worst place to spend it. And the toolchain question
becomes a matrix of compilers and standard libraries rather than one pinned
version.

## What would overturn this

A container decision in issue #12 that can only be read through a library with no
usable Rust binding, where the work of writing one is larger than the whole
validator. That is the one dependency this record has on a decision not yet made,
and it is named so that the reader can check it rather than discover it.

A measurement from issue #4 showing that two builds from a clean clone at one
commit are not byte-identical and cannot be made so. The fifth line above is the
one this record would have got wrong, and the correction is a different means
rather than a weaker claim about this one.

A depositor population that turns out to want to modify the tool rather than run
it. This record trades a compiler that refuses mistakes for a language fewer
physicists write, and the trade is right while the tool is something groups run
and wrong if it becomes something groups fork.
