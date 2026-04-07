# Specific Quantity Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic readable workbook for the `measure/specific-quantity/` subchapter.
- Read it to follow the chapter-level spine from specific quantum, through specifying measure, into being-for-itself in measure.
- Use the local [specific-quantity/SPECIFIC-QUANTITY-IDEA-WORKBOOK.md](specific-quantity/SPECIFIC-QUANTITY-IDEA-WORKBOOK.md) and the part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which immediate measure first appears as a thing's proper quantum and then becomes a specifying relation.
- Second question: what is the chapter's central operator?
  Answer: quantitative alteration becoming non-indifferent to quality.
- Third question: where does the subchapter lead?
  Answer: to real measure.

## Authority + format lock (must persist)

- Working extraction references: `specific-quantity/specific-quantity-idea.txt`, `specific-quantity/specific-quantum.txt`, `specific-quantity/specifying-measure.txt`, `specific-quantity/being-for-itself.txt`, and `SPECIFIC-QUANTITY-DISTILLATION.md`
- Upstream source authority: `specific-quantity/specific-quantity-idea.txt`, `specific-quantity/specific-quantum.txt`, `specific-quantity/specifying-measure.txt`, `specific-quantity/being-for-itself.txt`
- This workbook covers the `measure/specific-quantity/` subchapter only.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Specific Quantity subchapter pass)

Scope:

- files:
  - `specific-quantity/specific-quantity-idea.txt`
  - `specific-quantity/specific-quantum.txt`
  - `specific-quantity/specifying-measure.txt`
  - `specific-quantity/being-for-itself.txt`
  - `SPECIFIC-QUANTITY-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the readable-layer pattern by treating this file as the architectonic surface for the `measure/specific-quantity/` subchapter.
- Preserve the older local compiler, idea, part, and notebook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: specific quantum, specifying measure, being-for-itself in measure.

### Entry hegel-measure-specific — Marker `Specific Quantity`

- sourceFiles:
  - `specific-quantity/specific-quantity-idea.txt`
  - `specific-quantity/specific-quantum.txt`
  - `specific-quantity/specifying-measure.txt`
  - `specific-quantity/being-for-itself.txt`
- lineSpans:
  - `specific-quantity/specific-quantity-idea.txt:1-23`
  - `specific-quantity/specific-quantum.txt:1-189`
  - `specific-quantity/specifying-measure.txt:1-245`
  - `specific-quantity/being-for-itself.txt:1-192`
- summary: The `measure/specific-quantity/` subchapter presents immediate measure as specific quantum, develops it as a specifying rule, and culminates in being-for-itself in measure.

Key points: (KeyPoint)

- k1. Specific quantity is immediate measure.
- k2. A thing is not indifferent to its proper quantum.
- k3. Measure specifies external magnitude.
- k4. Being-for-itself in measure yields a complete self-subsistent unity.

Claims: (Claim)

- c1. id: hegel-measure-specific-c1
  - subject: specific_quantity_chapter
  - predicate: captures
  - object: immediate_measure_as_specific_quantum
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/specific-quantum.txt:1-189`

- c2. id: hegel-measure-specific-c2
  - subject: specific_quantity_chapter
  - predicate: unfolds_through
  - object: specifying_measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/specifying-measure.txt:1-245`

- c3. id: hegel-measure-specific-c3
  - subject: specific_quantity_chapter
  - predicate: culminates_in
  - object: real_measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/being-for-itself.txt:150-192`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-measure-specific-quantum
  - targetWorkbook: `SPECIFIC-QUANTITY-WORKBOOK.md`
  - note: the chapter first establishes specific quantum as qualitative quantity.
  - sourceClaimIds: [`hegel-measure-specific-c1`, `hegel-measure-specific-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-measure-specific-quantum-c1`, `hegel-measure-specific-quantum-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-measure-specific-bfi
  - targetWorkbook: `SPECIFIC-QUANTITY-WORKBOOK.md`
  - note: the chapter concludes with being-for-itself in measure and the real-measure threshold.
  - sourceClaimIds: [`hegel-measure-specific-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-measure-specific-bfi-c1`, `hegel-measure-specific-bfi-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the first measure subchapter at readable scale.

### Entry hegel-measure-specific-quantum — `Specific Quantity`: specific quantum and specifying measure

- sourceFiles:
  - `specific-quantity/specific-quantum.txt`
  - `specific-quantity/specifying-measure.txt`
- lineSpans:
  - `specific-quantity/specific-quantum.txt:1-189`
  - `specific-quantity/specifying-measure.txt:1-245`
- summary: Measure first appears as the proper quantum of a thing and then as the immanent rule that transforms externally imposed quantum according to qualitative determination.

Key points: (KeyPoint)

- k1. Every thing has a proper measure.
- k2. Exceeding or falling short of that measure alters quality.
- k3. Measure specifies external quantum.
- k4. The exponent is the qualitative moment of this relation.

Claims: (Claim)

- c1. id: hegel-measure-specific-quantum-c1
  - subject: specific_quantum
  - predicate: makes_explicit
  - object: quantum_as_non_indifferent_to_quality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/specific-quantum.txt:1-189`

- c2. id: hegel-measure-specific-quantum-c2
  - subject: specifying_measure
  - predicate: transforms
  - object: external_quantum_according_to_qualitative_rule
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/specifying-measure.txt:1-245`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-measure-specific-bfi
  - targetWorkbook: `SPECIFIC-QUANTITY-WORKBOOK.md`
  - note: the specified relation becomes a unity of qualities in measure.
  - sourceClaimIds: [`hegel-measure-specific-quantum-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-measure-specific-bfi-c1`]

Review outcome:

- review_pending
- notes: this entry keeps immediate measure and its specifying activity together.

### Entry hegel-measure-specific-bfi — `Specific Quantity`: being-for-itself in measure and real-measure threshold

- sourceFiles:
  - `specific-quantity/being-for-itself.txt`
- lineSpans:
  - `specific-quantity/being-for-itself.txt:1-192`
- summary: Being-for-itself in measure turns the two sides of measure into a complete self-subsistent unity, thereby producing the threshold of real measure.

Key points: (KeyPoint)

- k1. The sides of measure stand as qualities.
- k2. Their unity is a real being-for-itself.
- k3. This unity appears as self-subsistent somethings.
- k4. The next field is real measure.

Claims: (Claim)

- c1. id: hegel-measure-specific-bfi-c1
  - subject: being_for_itself_in_measure
  - predicate: yields
  - object: self_subsistent_unity_of_qualities
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/being-for-itself.txt:1-149`

- c2. id: hegel-measure-specific-bfi-c2
  - subject: being_for_itself_in_measure
  - predicate: culminates_in
  - object: real_measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `specific-quantity/being-for-itself.txt:150-192`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-measure-real
  - targetWorkbook: `REAL-MEASURE-WORKBOOK.md`
  - note: the next measure field is real measure.
  - sourceClaimIds: [`hegel-measure-specific-bfi-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-measure-real-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the handoff to Real Measure explicit.
