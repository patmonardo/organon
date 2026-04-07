# Pure Quantity Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic readable workbook for the `quantity/quantity/` subchapter.
- Read it to follow the chapter-level spine from pure quantity, through magnitude, into limiting quantity.
- Use the local [quantity/QUANTITY-IDEA-WORKBOOK.md](quantity/QUANTITY-IDEA-WORKBOOK.md) and the part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which quantity first appears as the unity of continuity and discreteness before explicit quantitative limit is posited.
- Second question: what is the chapter's central operator?
  Answer: continuity and discreteness held together as one quantity.
- Third question: where does the subchapter lead?
  Answer: to quantum.

## Authority + format lock (must persist)

- Working extraction references: `quantity/quantity-idea.txt`, `quantity/pure-quantity.txt`, `quantity/magnitude.txt`, `quantity/limiting-quantity.txt`, and `PURE-QUANTITY-DISTILLATION.md`
- Upstream source authority: `quantity/quantity-idea.txt`, `quantity/pure-quantity.txt`, `quantity/magnitude.txt`, `quantity/limiting-quantity.txt`
- This workbook covers the `quantity/quantity/` subchapter only.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Pure Quantity subchapter pass)

Scope:

- files:
  - `quantity/quantity-idea.txt`
  - `quantity/pure-quantity.txt`
  - `quantity/magnitude.txt`
  - `quantity/limiting-quantity.txt`
  - `PURE-QUANTITY-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the readable-layer pattern by treating this file as the architectonic surface for the `quantity/quantity/` subchapter.
- Preserve the older local compiler, idea, part, and notebook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: pure quantity, continuous and discrete magnitude, limiting quantity.

### Entry hegel-quantity-pure — Marker `Pure Quantity`

- sourceFiles:
  - `quantity/quantity-idea.txt`
  - `quantity/pure-quantity.txt`
  - `quantity/magnitude.txt`
  - `quantity/limiting-quantity.txt`
- lineSpans:
  - `quantity/quantity-idea.txt:1-4`
  - `quantity/pure-quantity.txt:1-67`
  - `quantity/magnitude.txt:1-59`
  - `quantity/limiting-quantity.txt:1-47`
- summary: The `quantity/quantity/` subchapter presents pure quantity as the unity of continuity and discreteness, and culminates in limiting quantity as the threshold of quantum.

Key points: (KeyPoint)

- k1. Pure quantity is sublated being-for-itself.
- k2. Quantity unifies continuity and discreteness.
- k3. Limiting quantity produces quantum.

Claims: (Claim)

- c1. id: hegel-quantity-pure-c1
  - subject: pure_quantity_chapter
  - predicate: captures
  - object: quantity_as_unity_of_continuity_and_discreteness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/pure-quantity.txt:1-67`
    - `quantity/magnitude.txt:1-59`

- c2. id: hegel-quantity-pure-c2
  - subject: limiting_quantity
  - predicate: yields
  - object: quantum
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/limiting-quantity.txt:1-47`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-quantity-pure-magnitude
  - targetWorkbook: `PURE-QUANTITY-WORKBOOK.md`
  - note: the chapter first fixes quantity as continuity and discreteness.
  - sourceClaimIds: [`hegel-quantity-pure-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-quantity-pure-magnitude-c1`, `hegel-quantity-pure-magnitude-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-quantity-pure-limit
  - targetWorkbook: `PURE-QUANTITY-WORKBOOK.md`
  - note: pure quantity becomes quantum by receiving explicit limit.
  - sourceClaimIds: [`hegel-quantity-pure-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quantity-pure-limit-c1`, `hegel-quantity-pure-limit-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the first quantity subchapter at readable scale.

### Entry hegel-quantity-pure-magnitude — `Pure Quantity`: continuity, discreteness, magnitude

- sourceFiles:
  - `quantity/pure-quantity.txt`
  - `quantity/magnitude.txt`
- lineSpans:
  - `quantity/pure-quantity.txt:1-67`
  - `quantity/magnitude.txt:1-59`
- summary: Pure quantity first appears as continuity, but continuity already contains discreteness, and each side is the whole quantity as magnitude.

Key points: (KeyPoint)

- k1. Continuity is self-same reference holding plurality.
- k2. Discreteness is equally a moment of quantity.
- k3. Continuous and discrete magnitude are both the whole quantity.

Claims: (Claim)

- c1. id: hegel-quantity-pure-magnitude-c1
  - subject: continuity
  - predicate: contains
  - object: discreteness_as_its_own_moment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/pure-quantity.txt:1-67`

- c2. id: hegel-quantity-pure-magnitude-c2
  - subject: magnitude
  - predicate: presents
  - object: continuity_and_discreteness_as_the_same_quantity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/magnitude.txt:1-59`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quantity-pure-limit
  - targetWorkbook: `PURE-QUANTITY-WORKBOOK.md`
  - note: the whole quantity becomes quantum when its one is posited as explicit limit.
  - sourceClaimIds: [`hegel-quantity-pure-magnitude-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quantity-pure-limit-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the opening unity of quantity intact.

### Entry hegel-quantity-pure-limit — `Pure Quantity`: limiting quantity and quantum-threshold

- sourceFiles:
  - `quantity/limiting-quantity.txt`
- lineSpans:
  - `quantity/limiting-quantity.txt:1-47`
- summary: Limiting quantity gathers discrete plurality under an excluding one, producing determinate quantity or quantum.

Key points: (KeyPoint)

- k1. The one becomes enclosing limit.
- k2. Quantity becomes determinate existence.
- k3. The result is quantum.

Claims: (Claim)

- c1. id: hegel-quantity-pure-limit-c1
  - subject: limiting_quantity
  - predicate: posits
  - object: one_as_enclosing_limit
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/limiting-quantity.txt:1-31`

- c2. id: hegel-quantity-pure-limit-c2
  - subject: limiting_quantity
  - predicate: culminates_in
  - object: quantum
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/limiting-quantity.txt:32-47`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quantity-quantum
  - targetWorkbook: `QUANTUM-WORKBOOK.md`
  - note: the next quantity field is quantum.
  - sourceClaimIds: [`hegel-quantity-pure-limit-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-quantity-quantum-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the handoff to quantum explicit.
