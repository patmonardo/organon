# Existence Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic readable workbook for the `quality/existence/` subchapter.
- Read it to follow the chapter-level spine from determinate existence, through finitude, into infinity.
- Use the local [existence/EXISTENCE-IDEA-WORKBOOK.md](existence/EXISTENCE-IDEA-WORKBOOK.md) and the part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which determinate being becomes the full field of something, finitude, and infinity.
- Second question: what is the chapter's central operator?
  Answer: negation becoming internal to something and then affirmative in true infinity.
- Third question: where does the subchapter lead?
  Answer: to being-for-itself.

## Authority + format lock (must persist)

- Working extraction references: `existence/existence-idea.txt`, `existence/constitution.txt`, `existence/alternating-infinity.txt`, `existence/affirmative-infinity.txt`, and `EXISTENCE-DISTILLATION.md`
- Upstream source authority: `existence/existence-idea.txt`, `existence/constitution.txt`, `existence/alternating-infinity.txt`, `existence/affirmative-infinity.txt`
- This workbook covers the `quality/existence/` subchapter only.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Existence subchapter pass)

Scope:

- files:
  - `existence/existence-idea.txt`
  - `existence/constitution.txt`
  - `existence/alternating-infinity.txt`
  - `existence/affirmative-infinity.txt`
  - `EXISTENCE-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the readable-layer pattern by treating this file as the architectonic surface for the `quality/existence/` subchapter.
- Preserve the older local compiler, idea, part, and notebook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: determinate existence, finitude, infinity.

### Entry hegel-quality-existence — Marker `Existence`

- sourceFiles:
  - `existence/existence-idea.txt`
  - `existence/constitution.txt`
  - `existence/alternating-infinity.txt`
  - `existence/affirmative-infinity.txt`
- lineSpans:
  - `existence/existence-idea.txt:1-53`
  - `existence/constitution.txt:1-410`
  - `existence/alternating-infinity.txt:1-288`
  - `existence/affirmative-infinity.txt:1-427`
- summary: The `quality/existence/` subchapter presents determinate being as existence, develops finitude and infinity, and culminates in the transition to being-for-itself.

Key points: (KeyPoint)

- k1. Existence is determinate being.
- k2. The chapter unfolds through something, finitude, and infinity.
- k3. The chapter culminates in being-for-itself.

Claims: (Claim)

- c1. id: hegel-quality-existence-c1
  - subject: existence_chapter
  - predicate: captures
  - object: determinate_being_as_finitude_and_infinity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/existence-idea.txt:1-27`
    - `existence/constitution.txt:1-410`

- c2. id: hegel-quality-existence-c2
  - subject: existence_chapter
  - predicate: unfolds_through
  - object: bad_infinite_and_true_infinite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/alternating-infinity.txt:1-288`
    - `existence/affirmative-infinity.txt:1-427`

- c3. id: hegel-quality-existence-c3
  - subject: existence_chapter
  - predicate: culminates_in
  - object: being_for_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/existence-idea.txt:28-53`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-quality-existence-finite
  - targetWorkbook: `EXISTENCE-WORKBOOK.md`
  - note: the chapter first fixes determinate existence as finite being.
  - sourceClaimIds: [`hegel-quality-existence-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-quality-existence-finite-c1`, `hegel-quality-existence-finite-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-quality-existence-infinite
  - targetWorkbook: `EXISTENCE-WORKBOOK.md`
  - note: finitude drives into true infinity and the next field.
  - sourceClaimIds: [`hegel-quality-existence-c2`, `hegel-quality-existence-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quality-existence-infinite-c1`, `hegel-quality-existence-infinite-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the second quality subchapter at readable scale.

### Entry hegel-quality-existence-finite — `Existence`: constitution, limit, finitude

- sourceFiles:
  - `existence/constitution.txt`
- lineSpans:
  - `existence/constitution.txt:1-410`
- summary: The sphere of something, constitution, and limit shows existence as finite determinateness whose very stability is undermined from within.

Key points: (KeyPoint)

- k1. Determination and constitution pass into one another.
- k2. The finite preserves itself only precariously.
- k3. Finitude points beyond itself.

Claims: (Claim)

- c1. id: hegel-quality-existence-finite-c1
  - subject: finite_existence
  - predicate: is
  - object: determinate_being_structured_by_constitution_and_limit
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/constitution.txt:1-410`

- c2. id: hegel-quality-existence-finite-c2
  - subject: finitude
  - predicate: points_toward
  - object: infinity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/existence-idea.txt:13-20`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quality-existence-infinite
  - targetWorkbook: `EXISTENCE-WORKBOOK.md`
  - note: the finite drives into infinity.
  - sourceClaimIds: [`hegel-quality-existence-finite-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quality-existence-infinite-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the finite field compact and transition-oriented.

### Entry hegel-quality-existence-infinite — `Existence`: bad infinite, true infinite, being-for-itself threshold

- sourceFiles:
  - `existence/alternating-infinity.txt`
  - `existence/affirmative-infinity.txt`
- lineSpans:
  - `existence/alternating-infinity.txt:1-288`
  - `existence/affirmative-infinity.txt:1-427`
- summary: The bad infinite alternates externally with the finite, but the affirmative infinite sublates that alternation and opens the threshold to being-for-itself.

Key points: (KeyPoint)

- k1. The bad infinite remains opposed to the finite.
- k2. The true infinite is affirmative unity.
- k3. True infinity opens being-for-itself.

Claims: (Claim)

- c1. id: hegel-quality-existence-infinite-c1
  - subject: bad_infinite
  - predicate: is
  - object: endless_alternation_with_the_finite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/alternating-infinity.txt:1-288`

- c2. id: hegel-quality-existence-infinite-c2
  - subject: affirmative_infinite
  - predicate: culminates_in
  - object: being_for_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `existence/affirmative-infinity.txt:1-427`
    - `existence/existence-idea.txt:46-53`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quality-bfs
  - targetWorkbook: `BEING-FOR-ITSELF-WORKBOOK.md`
  - note: the next quality field is being-for-itself.
  - sourceClaimIds: [`hegel-quality-existence-infinite-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quality-bfs-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the infinity-to-being-for-itself handoff explicit.
