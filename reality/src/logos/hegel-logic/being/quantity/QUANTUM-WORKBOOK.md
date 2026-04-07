# Quantum Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic readable workbook for the `quantity/quantum/` subchapter.
- Read it to follow the chapter-level spine from number, through extensive and intensive magnitude, into quantitative infinity.
- Use the local [quantum/QUANTUM-IDEA-WORKBOOK.md](quantum/QUANTUM-IDEA-WORKBOOK.md) and the part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which quantity first becomes explicit limit, displays its contradictions, and passes into ratio.
- Second question: what is the chapter's central operator?
  Answer: limit that is indifferent and yet self-transcending.
- Third question: where does the subchapter lead?
  Answer: to ratio.

## Authority + format lock (must persist)

- Working extraction references: `quantum/quantum-idea.txt`, `quantum/quantum.txt`, `quantum/number.txt`, `quantum/infinity.txt`, and `QUANTUM-DISTILLATION.md`
- Upstream source authority: `quantum/quantum-idea.txt`, `quantum/quantum.txt`, `quantum/number.txt`, `quantum/infinity.txt`
- This workbook covers the `quantity/quantum/` subchapter only.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Quantum subchapter pass)

Scope:

- files:
  - `quantum/quantum-idea.txt`
  - `quantum/quantum.txt`
  - `quantum/number.txt`
  - `quantum/infinity.txt`
  - `QUANTUM-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the readable-layer pattern by treating this file as the architectonic surface for the `quantity/quantum/` subchapter.
- Preserve the older local compiler, idea, part, and notebook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: number, extensive and intensive quantum, infinity.

### Entry hegel-quantity-quantum — Marker `Quantum`

- sourceFiles:
  - `quantum/quantum-idea.txt`
  - `quantum/quantum.txt`
  - `quantum/number.txt`
  - `quantum/infinity.txt`
- lineSpans:
  - `quantum/quantum-idea.txt:1-23`
  - `quantum/quantum.txt:1-304`
  - `quantum/number.txt:1-136`
  - `quantum/infinity.txt:1-339`
- summary: The `quantity/quantum/` subchapter presents quantity as explicit limit, develops number and degree, and culminates in quantitative infinity as the threshold of ratio.

Key points: (KeyPoint)

- k1. Quantum is quantity with explicit limit.
- k2. Number gives its first complete determinateness.
- k3. Extensive and intensive quantum are identical in truth.
- k4. Quantitative infinity restores qualitative determination.

Claims: (Claim)

- c1. id: hegel-quantity-quantum-c1
  - subject: quantum_chapter
  - predicate: captures
  - object: explicit_limit_as_number_and_degree
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/number.txt:1-136`
    - `quantum/quantum.txt:1-304`

- c2. id: hegel-quantity-quantum-c2
  - subject: quantum_chapter
  - predicate: unfolds_through
  - object: quantitative_infinite_progress
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/infinity.txt:1-339`

- c3. id: hegel-quantity-quantum-c3
  - subject: quantum_chapter
  - predicate: culminates_in
  - object: ratio
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/infinity.txt:200-339`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-quantity-quantum-number
  - targetWorkbook: `QUANTUM-WORKBOOK.md`
  - note: the chapter first fixes number and explicit limit.
  - sourceClaimIds: [`hegel-quantity-quantum-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-quantity-quantum-number-c1`, `hegel-quantity-quantum-number-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-quantity-quantum-infinite
  - targetWorkbook: `QUANTUM-WORKBOOK.md`
  - note: the chapter then turns contradiction into infinity and ratio.
  - sourceClaimIds: [`hegel-quantity-quantum-c2`, `hegel-quantity-quantum-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-quantity-quantum-infinite-c1`, `hegel-quantity-quantum-infinite-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the second quantity subchapter at readable scale.

### Entry hegel-quantity-quantum-number — `Quantum`: number, degree, alteration

- sourceFiles:
  - `quantum/number.txt`
  - `quantum/quantum.txt`
- lineSpans:
  - `quantum/number.txt:1-136`
  - `quantum/quantum.txt:1-304`
- summary: Number first gives quantum its complete determinateness, while extensive and intensive magnitude show that quantum's limit is internally alterable and self-contradictory.

Key points: (KeyPoint)

- k1. Number unifies unit and amount.
- k2. Degree is simple quantitative determinateness.
- k3. Extensive and intensive quantum are the same determination.
- k4. Alteration belongs to quantum itself.

Claims: (Claim)

- c1. id: hegel-quantity-quantum-number-c1
  - subject: number
  - predicate: gives
  - object: complete_quantitative_determinateness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/number.txt:1-136`

- c2. id: hegel-quantity-quantum-number-c2
  - subject: extensive_and_intensive_quantum
  - predicate: reveal
  - object: quantum_as_essentially_alterable
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/quantum.txt:1-304`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quantity-quantum-infinite
  - targetWorkbook: `QUANTUM-WORKBOOK.md`
  - note: alteration drives quantum into the infinite process.
  - sourceClaimIds: [`hegel-quantity-quantum-number-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-quantity-quantum-infinite-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the core contradiction of quantum explicit.

### Entry hegel-quantity-quantum-infinite — `Quantum`: bad infinity, infinity of quantum, ratio-threshold

- sourceFiles:
  - `quantum/infinity.txt`
- lineSpans:
  - `quantum/infinity.txt:1-339`
- summary: The bad infinite is the repetition of quantum's self-surpassing, but the truth of this process is the infinity of quantum, in which externality becomes its own qualitative determination and ratio emerges.

Key points: (KeyPoint)

- k1. The infinite progress repeats quantum's contradiction.
- k2. Bad infinity is not the truth of quantum.
- k3. The truth is the infinity of quantum.
- k4. Infinity culminates in ratio.

Claims: (Claim)

- c1. id: hegel-quantity-quantum-infinite-c1
  - subject: bad_quantitative_infinity
  - predicate: is
  - object: repetitive_self_surpassing_of_quantum
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/infinity.txt:1-199`

- c2. id: hegel-quantity-quantum-infinite-c2
  - subject: infinity_of_quantum
  - predicate: culminates_in
  - object: ratio
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `quantum/infinity.txt:200-339`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quantity-ratio
  - targetWorkbook: `RATIO-WORKBOOK.md`
  - note: the next quantity field is ratio.
  - sourceClaimIds: [`hegel-quantity-quantum-infinite-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-quantity-ratio-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the infinity-to-ratio handoff explicit.
