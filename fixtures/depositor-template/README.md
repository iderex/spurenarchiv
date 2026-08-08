# The fixtures that prove the template and the statement are kept in step

Each file here is a candidate `templates/depositor-details.json`, judged against
the table in `docs/privacy.md` by the same comparison that judges the real one.
A file whose name begins `accepted-` has to agree with the table and one whose
name begins `refused-` has to disagree with it, for the reason it is named for.
`.github/workflows/personal-fields.yml` runs them and reports under the check
name `The template collects what the statement declares`.

The failure this exists against does not arrive as one bad change. It arrives as
a field added to a form because somebody needed it once, at a moment when nobody
has the data protection statement open in another window. A form and a statement
that disagree are worse than either alone, because the statement is what a
depositor is asked to rely on and the form is what actually collects.

`refused-extra-field.json` is that failure exactly: the real template with one
telephone number added and no row written for it. Nothing about it looks like a
mistake, which is why a machine judges it.

`refused-missing-field.json` is the other direction, a row in the statement with
no field on the form. It is the less alarming of the two and it still has to be
refused: a statement describing a form that does not exist teaches a reader that
the rest of it was checked.

`refused-published-instead-of-local.json` moves the depositor's e-mail address
from local to published without touching the key set. It is the one that matters
most and the one a comparison of keys alone would miss, so it is here to prove
that the comparison reaches the two columns beside the key.

`accepted-reordered-keys.json` is the near miss. It is the real template with its
keys in the opposite order, and it is accepted, so the comparison is shown to
judge the set of fields rather than the order they happen to be written in.
