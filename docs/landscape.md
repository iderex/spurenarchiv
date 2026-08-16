# What is already published, and how much of it there is

Issue #82. Searches run on 2026-08-08.

## What this measures, and what it does not

The readme opens with a claim: streaking traces are practically never published
as data, only as images. That claim is the premise the rest of this repository
rests on, and until now it was held because it matches what people in the field
have seen rather than because anybody counted.

This document is the count. It is a smaller count than the claim, and the
difference is stated here rather than at the bottom. It counts records that a
public identifier registry has been told are datasets and whose metadata carries
one of a set of phrases. It does not count papers, it does not count
supplementary files attached to papers, and it does not count data held on a
group's own web space or handed over on request. Those three are where most of
the material in this field actually sits, and none of them is reachable by a
command.

So the number below is a lower bound on what exists and an upper bound on what is
findable by somebody looking for it. The second of those is the one that matters
for a depositor deciding whether to bother, and it is the reason this is worth
having despite the first.

## The method

Two public interfaces, queried directly, with no account and no key.

The registry search is the DataCite REST interface, restricted to records whose
declared resource type is a dataset, with the search phrase quoted so that it
matches as a phrase rather than as a set of words. The same interface without the
type restriction gives the denominator for the ratio below, which is the useful
comparison because it is the same index, the same phrase and the same matching
rules on both sides.

The literature interface is OpenAlex. Its counts appear below and are not used to
compute anything, for a reason given in its own section.

Every command is one line and produces one number.

## The counts

Dataset-typed records, per phrase:

    curl -s 'https://api.datacite.org/dois?query=%22attosecond%20streaking%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    4

    curl -s 'https://api.datacite.org/dois?query=%22attosecond%20metrology%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    4

    curl -s 'https://api.datacite.org/dois?query=%22FROG-CRAB%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    9

    curl -s 'https://api.datacite.org/dois?query=%22streaking%20spectrogram%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    0

    curl -s 'https://api.datacite.org/dois?query=%22streaking%20trace%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    0

    curl -s 'https://api.datacite.org/dois?query=%22attosecond%20streak%20camera%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    0

The three non-zero phrases together, which is also the check that they do not
overlap, since four and four and nine come to the same seventeen:

    curl -s 'https://api.datacite.org/dois?query=%22attosecond%20streaking%22%20OR%20%22attosecond%20metrology%22%20OR%20%22FROG-CRAB%22&resource-type-id=dataset&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    17

The same phrase without the type restriction, which is the denominator:

    curl -s 'https://api.datacite.org/dois?query=%22attosecond%20streaking%22&page%5Bsize%5D=0' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['total'])"
    146

So four of one hundred and forty-six registered records mentioning the phrase are
declared to be datasets, and the remaining one hundred and forty-two are
something else, which for this registry is overwhelmingly the paper itself and
its versions.

## Seventeen records, read one at a time

The count is not the finding. What the seventeen are is the finding, and it took
opening them.

**Nine of the seventeen are crabs.** The `FROG-CRAB` phrase, which in this field
names a reconstruction method, also names a family of decapods, and every one of
its nine hits is about them: `10.6084/m9.figshare.993995` and four versions of
it, `10.6084/m9.figshare.15042777` and one version, and
`10.6084/m9.figshare.30946470` and one version. That phrase contributes nothing
and it is left in this document rather than quietly dropped, because a search
term that returns nine hits and zero relevant ones is exactly the kind of thing
that inflates a count nobody checked.

Of the remaining eight:

**`10.7910/dvn/yh6qon`**, Harvard Dataverse, 2026, "Replication Data for Single
Trajectory Delays". Numerical electron trajectory calculations, not a
measurement. Two files, of 327 and 214 bytes.

**`10.17034/dc24280a-ac0a-4815-908b-a03a9cea1df0`**, Queen's University Belfast,
2020, angular streaking in the fluorine anion. Calculation inputs, output text
and plot scripts, 154 MB. Not a measurement.

**`10.22003/xfel.eu-data-006176-00`**, European XFEL, 2023, titled "Attosecond
streaking", and **`10.22003/xfel.eu-data-008697-00`**, 2024, titled "Attosecond
angular streaking". Both are facility data records. Their registry entries carry
no description, no format list and no size, so nothing about their contents can
be said from the registry, and their landing pages were not opened. They are the
two strongest candidates in this list for holding a real measurement and they are
the two about which the least is known here.

**`10.5281/zenodo.6924099`, `10.5281/zenodo.6924100` and
`10.5281/zenodo.6925094`**, 2022, three records under the title "Atomic partial
wave meter by attosecond coincidence metrology", described as the raw data behind
the main figures. Each holds five files named `figure1.zip` through
`figure5.zip`. The archives were not opened.

**`10.34810/data2158`**, CORA, 2025, replication data for a published
measurement of vacuum-ultraviolet high-order harmonics through laser-dressed
photoionisation of an alkali metal, supplement to `10.1038/s41467-025-56759-0`.
This is the closest thing in the seventeen to what this archive is for: a real
cross-correlation measurement with a dressing field and a photoelectron signal,
deposited as tab-separated and comma-separated text. It is the one record that
was read down to file level.

## What the closest record carries

Five files: `fig_4a_data_1.tab`, `fig_4b_data_1.tab`, `fig_4c_data_1.tab` at
roughly 1.8 MB each, `Sheet1_1.csv` at 4.9 MB, and `Readme.txt` at 9,655 bytes.
The names are the finding on their own. The unit of deposit is the figure, not
the measurement.

The readme is 9,573 characters. Its structure is authorship, abstract, subject
keywords, producers, contributors, grant numbers, distributor, deposit date,
licence, related publication, and a section headed file overview which gives, per
file, the file name and its media type. Nothing else.

Against the six things the readme of this repository names:

- The raw spectrogram. Present as numbers, split across files named for the
  figures they were drawn into, with no statement of what the columns are.
- The delay axis. Not described. No convention, no sign, no zero.
- The energy calibration. Not described.
- The dressing field parameters. The abstract says the field was mid-infrared.
  There is no wavelength, no intensity, no polarisation and no field record.
- The target gas. The abstract says caesium. There is no field, no pressure, no
  purity and no unit anywhere.
- The detector response. Not described.

Two of the six are recoverable by reading English prose written for a different
purpose, and none of the six is a field. This is a good deposit by the standards
of what is around it: it is public, it is licensed, it is tied to its paper by an
identifier, and it exists at all. The gap is not carelessness. It is that nothing
asked the depositor for any of the six.

One more thing about it bears on later decisions. Its licence is CC BY-NC-ND 4.0.
The no-derivatives term is the one to notice, because converting a deposit into
another container is arguably making a derivative, so this record is not a
candidate for a worked example or a benchmark case without asking. That question
belongs to entry 3 of #1 and to #61 and is not answered here.

## The literature counts, and why nothing is computed from them

    curl -s 'https://api.openalex.org/works?search=%22attosecond%20streaking%22&per-page=1' | python -c "import sys,json;print(json.load(sys.stdin)['meta']['count'])"
    1109

That number is printed and not used. Whether the interface treats the quotes as a
phrase or as a bag of words was not established, and a search that treats them as
a bag of words returns a much larger set than the phrase would. An inflated
denominator divided into a small numerator produces a ratio that flatters the
claim under test, which is the wrong direction to be wrong in.
A ratio against a denominator whose meaning is unverified is not quoted here.

Establishing a real denominator needs an interface with checkable phrase
semantics, and it is the obvious next piece of work on this document.

## The metadata standards examined

**NeXus, with the NXmpes application definition.** The only one of the four that
reaches the content of a measurement rather than the paperwork around it: axes,
detector, instrument, sample, and a varied axis that may be a pump-probe delay.
It is also the standard `container.md` refused as a container and adopted as a
vocabulary. None of the seventeen records declares it:

    curl -s 'https://api.datacite.org/dois?query=%22attosecond%20streaking%22%20OR%20%22attosecond%20metrology%22%20OR%20%22FROG-CRAB%22&resource-type-id=dataset&page%5Bsize%5D=100' | python -c "import sys,json;d=json.load(sys.stdin);print(sum(1 for x in d['data'] if 'nexus' in json.dumps(x).lower() or 'nxmpes' in json.dumps(x).lower()),'of',d['meta']['total'])"
    0 of 17

**The DataCite metadata schema.** What every record above is registered with. It
covers who made the deposit, when, under what licence, what it relates to, and an
inventory of file formats and sizes. It reaches the outside of the files and
stops there. This is not a criticism of it; describing a measurement was never
its job.

**The structured readme the closest record uses.** Covers authorship, funding,
licensing, the related publication and a file overview of name and media type.
The same boundary as above, written out longhand.

**PhySH**, the subject keyword vocabulary that record classifies itself with.
Covers classification. Three keywords, none of which is a property of the
measurement.

That is the shape of the finding. Every standard in actual use on these records
describes the deposit. The only one that describes the measurement is not used by
any of them.

## What the readme's claim becomes

It stays a claim, and it is now marked as one where it is made.

What has been measured supports its direction and is not the same statement. What
was measured: over six phrase searches of one registry restricted to dataset-typed
records, seventeen records were returned, nine of them about crabs, five of the
remaining eight were calculations or could not be judged from their registry
entry, and the one record read down to file level carries none of the six fields
this archive is built around. What was not measured: how many streaking
measurements have been published in any form, which is the denominator the word
"never" needs.

## What is not covered

Supplementary material attached to a paper by a publisher rather than deposited
under its own identifier. This is probably where most published streaking data
actually is, it is not typed as a dataset anywhere, and no command in this
document would find it.

Data held on a group's own pages, in an institutional repository outside this
registry's index, or available on request.

Deposits that hold a streaking trace and never use any of the six phrases. The
search is over metadata text, so a record titled for its physics rather than its
method is invisible to it.

Non-English phrasing.

The contents of five records: the two facility records and the three archives of
figure data. Opening them is the cheapest next step and would change the field
coverage section above.

A denominator, for the reason in its own section.

## Repeating this

Every command above is complete and needs nothing installed beyond a fetch tool
and a JSON reader. The counts are of a live index and will move. A different
number is a result rather than a failure, and the thing to compare is not the
number but the seventeen: whether any of them has become a deposit that carries
the six fields.
