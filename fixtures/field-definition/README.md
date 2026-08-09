# The fixtures that prove the field definition guard bites

`.github/workflows/field-sentence.yml` runs each of these against
`schema/meta/field-definition.schema.json` and asserts the verdict named in the
file's own name. Each is the smallest document that reaches the outcome it is
named for, which is the rule in `docs/testing.md`. How many there are is what the
workflow runs rather than a number written here.

`accepted-shortest-sentence.json` is the near miss. Its sentence is one
character, which says nothing useful, and it is accepted. That is the boundary
the guard sits on: the schema refuses a slot with no non-whitespace character in
it and refuses nothing about what the sentence says. A guard proved only against
an absent key would leave a reader thinking the second half is covered too.

`refused-blank-sentence.json` carries the key with only whitespace in it, which
is the mistake a template makes when a writer tabs past the slot.

`refused-absent-sentence.json` omits the key.

`refused-conditional-without-condition.json` is a well-formed row in every other
way whose requirement is conditional and which names no condition. It is here
because that is the requirement state the model uses most and the one whose
absence is invisible in a diff.

`refused-unmarked-publication.json` is a well-formed row that never says whether
the field travels with a copy of the dataset. It is refused rather than defaulted,
so a field cannot land here without somebody having answered the question.
`accepted-local-marking.json` beside it is the same row marked `local`, and it is
accepted, so what the refusal reaches is the key being absent rather than one of
the two answers being unwelcome. Neither says anything about whether the answer
is correct: a field marked `published` that carries something personal is refused
by nothing here, which is why the marking is a claim by whoever wrote the row and
`docs/decisions/personal-data.md` is what it is claimed against.
