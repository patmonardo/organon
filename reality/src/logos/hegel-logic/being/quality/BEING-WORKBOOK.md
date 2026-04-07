# Being Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic readable workbook for the `quality/being/` subchapter.
- Read it to follow the chapter-level spine from pure being, through nothing, into becoming.
- Use the local [being/BEING-IDEA-WORKBOOK.md](being/BEING-IDEA-WORKBOOK.md) and the part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which pure immediacy is shown to be empty and therefore true only as becoming.
- Second question: what is the chapter's central operator?
  Answer: the immediate vanishing of being and nothing into one another.
- Third question: where does the subchapter lead?
  Answer: to existence.

## Authority + format lock (must persist)

- Working extraction references: `being/being-idea.txt`, `being/being.txt`, `being/nothing.txt`, `being/becoming.txt`, and `BEING-DISTILLATION.md`
- Upstream source authority: `being/being-idea.txt`, `being/being.txt`, `being/nothing.txt`, `being/becoming.txt`
- This workbook covers the `quality/being/` subchapter only.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Being subchapter pass)

Scope:

- files:
  - `being/being-idea.txt`
  - `being/being.txt`
  - `being/nothing.txt`
  - `being/becoming.txt`
  - `BEING-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the readable-layer pattern by treating this file as the architectonic surface for the `quality/being/` subchapter.
- Preserve the older local compiler, idea, part, and notebook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: being, nothing, becoming.

### Entry hegel-quality-being — Marker `Being`

- sourceFiles:
  - `being/being-idea.txt`
  - `being/being.txt`
  - `being/nothing.txt`
  - `being/becoming.txt`
- lineSpans:
  - `being/being-idea.txt:1-4`
  - `being/being.txt:1-19`
  - `being/nothing.txt:1-21`
  - `being/becoming.txt:1-110`
- summary: The `quality/being/` subchapter presents pure being and pure nothing as equally empty immediacy, and establishes becoming as their truth and transition into existence.

Key points: (KeyPoint)

- k1. Pure being is empty immediacy.
- k2. Pure nothing is the same emptiness.
- k3. Becoming is the truth of both.
- k4. Becoming passes into existence.

Claims: (Claim)

- c1. id: hegel-quality-being-c1
  - subject: being_chapter
  - predicate: captures
  - object: pure_immediacy_as_being_nothing_and_becoming
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/being.txt:1-19`
    - `being/nothing.txt:1-21`
    - `being/becoming.txt:1-110`

- c2. id: hegel-quality-being-c2
  - subject: becoming
  - predicate: culminates_in
  - object: existence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/becoming.txt:100-110`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-quality-being-pure-poles
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: the chapter begins from the empty poles of being and nothing.
  - sourceClaimIds: [`hegel-quality-being-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-quality-being-pure-poles-c1`, `hegel-quality-being-pure-poles-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-quality-being-becoming
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: the truth of the poles is becoming and the existence-threshold.
  - sourceClaimIds: [`hegel-quality-being-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-quality-being-becoming-c1`, `hegel-quality-being-becoming-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the first quality subchapter at readable scale.

### Entry hegel-quality-being-pure-poles — `Being`: pure being and pure nothing

- sourceFiles:
  - `being/being.txt`
  - `being/nothing.txt`
- lineSpans:
  - `being/being.txt:1-19`
  - `being/nothing.txt:1-21`
- summary: Pure being and pure nothing are equally empty immediacies, so their apparent opposition collapses in their very purity.

Key points: (KeyPoint)

- k1. Pure being is empty immediacy.
- k2. Pure nothing is the same emptiness.
- k3. Their difference vanishes in purity.

Claims: (Claim)

- c1. id: hegel-quality-being-pure-poles-c1
  - subject: pure_being
  - predicate: is
  - object: empty_immediacy
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/being.txt:1-19`

- c2. id: hegel-quality-being-pure-poles-c2
  - subject: pure_nothing
  - predicate: is
  - object: the_same_empty_immediacy
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/nothing.txt:1-21`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quality-being-becoming
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: the collapse of the poles gives becoming.
  - sourceClaimIds: [`hegel-quality-being-pure-poles-c1`, `hegel-quality-being-pure-poles-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quality-being-becoming-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the opening abstraction clean and brief.

### Entry hegel-quality-being-becoming — `Being`: becoming and existence-threshold

- sourceFiles:
  - `being/becoming.txt`
- lineSpans:
  - `being/becoming.txt:1-110`
- summary: Becoming is the immediate vanishing of being and nothing into one another, and in its quiescent result it becomes existence.

Key points: (KeyPoint)

- k1. Becoming is the unity of being and nothing.
- k2. The moments vanish into one another.
- k3. Becoming collapses into a quiescent result.
- k4. That result is existence.

Claims: (Claim)

- c1. id: hegel-quality-being-becoming-c1
  - subject: becoming
  - predicate: is
  - object: immediate_vanishing_of_being_and_nothing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/becoming.txt:1-72`

- c2. id: hegel-quality-being-becoming-c2
  - subject: becoming
  - predicate: passes_into
  - object: existence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `being/becoming.txt:73-110`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quality-existence
  - targetWorkbook: `EXISTENCE-WORKBOOK.md`
  - note: the next quality field is existence.
  - sourceClaimIds: [`hegel-quality-being-becoming-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-quality-existence-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the handoff to existence explicit.
