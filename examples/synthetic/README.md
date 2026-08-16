# The synthetic deposit

`neon-2p-streaking/` is a complete deposit that nothing measured. It exists so
that the model has been filled in once, end to end, before a real trace is asked
to go through it, and so that the test suite and anybody reading the format have
a deposit to point at.

It is not a measurement and it says so in its own fields and not by sitting
somewhere different. `converter` names the program that computed it, the whole
apparatus carries `not_applicable` because there is no apparatus, and
`dressing_spectrum` carries `estimated` because that curve was derived from the
stated envelope rather than recorded.

## What is in it

    neon-2p-streaking/
      manifest.json
      scan-001/
        metadata.json
        spectrogram.bin
        uncertainty.bin

The spectrogram is 41 delay points by 24 energy bins of 32-bit signed integers,
little-endian, delay varying slowest. The uncertainty array is the same shape in
64-bit floats. Both are read the way `docs/spec/deposit-layout.md` shows.

## The model

A single photoline, streaked by a linearly polarised dressing field, detected
along the polarisation. Every number below is in the metadata document and the
generator reads them back out of it before it computes anything, so the deposit
and the arithmetic cannot disagree.

The line sits at the centre of the stated energy axis, at 68.0 eV, and has a
full width at half maximum of 1.6 eV. Its centre is displaced at delay t by

    dE(t) = A * exp(-4 ln2 (t / w)^2) * cos(omega t)

with `omega` the carrier angular frequency of the 800 nm dressing field, `w` the
envelope width of the vector potential, which is the stated 5 fs intensity width
times the square root of two, and `A` the streaking amplitude

    A = sqrt(8 * E0 * Up),   Up[eV] = 9.33e-14 * I[W/cm^2] * (lambda[um])^2

The counts in each pixel are drawn from a Poisson distribution whose mean is the
line profile scaled by the shots at that delay point, so `completeness_level` of
`counts` and a Poisson noise model are true of the array rather than only
claimed about it. The uncertainty array is the square root of the counts.

What that gives, for the values in the deposit:

```console
$ python -c "import math; I=1e12; lam=8e-7; tau=5e-15; Up=9.33e-14*I*(lam*1e6)**2; print('%.6f eV'%Up); print('%.4f eV'%math.sqrt(8*68.0*Up)); print('%.4f fs'%(lam/299792458.0*1e15)); print('%.4f fs'%(tau*math.sqrt(2)*1e15))"
0.059712 eV
5.6994 eV
2.6685 fs
7.0711 fs
```

So the line swings about 5.7 eV either side of 68 eV at the peak of the
envelope, on a carrier period of 2.67 fs sampled every 0.3 fs.

Nothing runs that block. `tool/tests/readme.rs` executes the console blocks in
the repository's own `README.md` and reads no other file, so the four numbers
above are a calculation somebody has to redo rather than one a red suite would
catch. The two blocks further down are different: the suite makes the same
comparison the first one does, and recomputes the digest the second one prints.

### The approximations, stated

The streak is the classical shift of a slow electron in the dressing field with
no depletion, no continuum-continuum phase and no angular averaging. The ionising
pulse is treated as instantaneous, so the finite duration of the pulse that made
the photoline contributes nothing beyond the 1.6 eV width above. The dressing
field is a single carrier frequency under a Gaussian envelope, and the spectrum
in the deposit is that pulse's transform limit rather than anything the
computation used.

None of this is a streaking simulation anybody should reconstruct against as
though it were physics. It is a deposit shaped like a measurement, with an
answer that is known.

## Regenerating it

```console
$ cargo run --locked --offline --manifest-path tool/Cargo.toml --bin synthetic-example -- examples/synthetic/neon-2p-streaking
wrote examples/synthetic/neon-2p-streaking
$ git status --porcelain examples/synthetic
```

An empty second output is the deposit reproducing itself. The suite makes the
same comparison into a temporary directory rather than over the tracked copy, in
`tool/tests/synthetic_example.rs`, so a change to the generator that no longer
produces what is tracked turns the suite red rather than waiting for somebody to
run the command.

The digests in the manifest are reproducible with the tool an operating system
already ships:

```console
$ sha256sum neon-2p-streaking/scan-001/spectrogram.bin
89d6fe06034056ee7657202367ce02dc6a1cb3b61e0554bf23844b50410e1343 *neon-2p-streaking/scan-001/spectrogram.bin
```

### What the reproduction was measured on, and what it was not

Two runs on one machine, Windows on x86-64, produce identical bytes, which is
the command above. Whether a machine with a different mathematical library
produces the same bytes has not been measured and nothing here promises it. The
array passes through `exp` and `cos`, which are not required to agree to the
last bit between platforms, and a difference there reaches the bytes through the
rounding of one count. That is unlikely per pixel and it is not impossible, and
the suite would report it as a difference rather than hide it.

## What the model could not say

Writing this deposit is how gaps in the model are found, and it found three.
None of them is repaired here, because each is a change to `schema/` and to
`docs/model/` rather than to an example.

There is no field for the ionising pulse. The model carries the dressing pulse
in six fields and carries nothing at all about the pulse that produces the
photoelectron: no photon energy, no bandwidth, no duration. A reanalyst reading
this deposit cannot say where the unstreaked line should sit, and the generator
cannot take that number out of the deposit either, which is why the line is
placed at the centre of the stated energy axis and the width is a constant in
the generator rather than a field.

There is no way for `converter` to say that nothing was converted. The block is
required, it requires a checksum of the source export, and a computed dataset
has no source export. The deposit writes a sentence into that slot instead of a
digest, which is legal against the schema and is the wrong shape.

There is no way to say how to read the uncertainty array. `uncertainty_array`
carries a method, a file and a sentence, and no element type or byte order,
while `spectrogram` carries both. The file here is 64-bit little-endian floats
and the only place that is written is the sentence and this paragraph.

A fourth thing is not a gap in the model but a limit of this example. The
generator's pseudo-random seed and the peak counts per shot are constants in its
source, because the deposit describes a measurement and neither belongs to one.

## Why it is here rather than under fixtures

`docs/decisions/layout.md` draws the line. A fixture exists to make one property
bite and proves the guard; an example is a complete deposit read as a deposit and
proves the state of the tree on the day it ran. This deposit is judged by the
suite for what it is, which is a deposit that conforms and states every field,
and no refusal in this repository is proved against it.
