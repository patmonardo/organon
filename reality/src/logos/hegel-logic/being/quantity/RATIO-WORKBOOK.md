# Ratio Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic readable workbook for the `quantity/ratio/` subchapter.
- Read it to follow the chapter-level spine from direct ratio, through inverse ratio, into the ratio of powers.
- Use the local [ratio/RATIO-IDEA-WORKBOOK.md](ratio/RATIO-IDEA-WORKBOOK.md) and the part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which quantity finds its determinateness in another quantum and thereby returns into qualitative determination.
- Second question: what is the chapter's central operator?
  Answer: self-reference through externality.
- Third question: where does the subchapter lead?
  Answer: to measure.

## Authority + format lock (must persist)

- Working extraction references: `ratio/ratio-idea.txt`, `ratio/ratio.txt`, `ratio/inverse.txt`, `ratio/powers.txt`, and `RATIO-DISTILLATION.md`
- Upstream source authority: `ratio/ratio-idea.txt`, `ratio/ratio.txt`, `ratio/inverse.txt`, `ratio/powers.txt`
- This workbook covers the `quantity/ratio/` subchapter only.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Ratio subchapter pass)

Scope:

- files:
  - `ratio/ratio-idea.txt`
  - `ratio/ratio.txt`
  - `ratio/inverse.txt`
  - `ratio/powers.txt`
  - `RATIO-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the readable-layer pattern by treating this file as the architectonic surface for the `quantity/ratio/` subchapter.
- Preserve the older local compiler, idea, part, and notebook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: direct ratio, inverse ratio, powers.

### Entry hegel-quantity-ratio — Marker `Ratio`

- sourceFiles:
  - `ratio/ratio-idea.txt`
  - `ratio/ratio.txt`
  - `ratio/inverse.txt`
  - `ratio/powers.txt`
- lineSpans:
  - `ratio/ratio-idea.txt:1-81`
  - `ratio/ratio.txt:1-110`
  - `ratio/inverse.txt:1-268`
  - `ratio/powers.txt:1-140`
- summary: The `quantity/ratio/` subchapter presents quantity as determinateness in another quantum, develops direct and inverse ratio, and culminates in powers and the transition to measure.

Key points: (KeyPoint)

- k1. Ratio gives quantity its determinateness in another quantum.
- k2. Direct ratio gives the immediate relation.
- k3. Inverse ratio makes the qualitative moment explicit.
- k4. Powers return quantity into itself.
- k5. Ratio culminates in measure.

Claims: (Claim)

- c1. id: hegel-quantity-ratio-c1
  - subject: ratio_chapter
  - predicate: captures
  - object: quantity_as_self_reference_through_externality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/ratio-idea.txt:1-49`
    - `ratio/ratio.txt:1-110`

- c2. id: hegel-quantity-ratio-c2
  - subject: inverse_ratio
  - predicate: makes_explicit
  - object: qualitative_limit_in_the_exponent
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/inverse.txt:1-268`

- c3. id: hegel-quantity-ratio-c3
  - subject: ratio_of_powers
  - predicate: culminates_in
  - object: measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/ratio-idea.txt:67-81`
    - `ratio/powers.txt:1-140`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-quantity-ratio-direct-inverse
  - targetWorkbook: `RATIO-WORKBOOK.md`
  - note: the chapter first develops direct and inverse ratio.
  - sourceClaimIds: [`hegel-quantity-ratio-c1`, `hegel-quantity-ratio-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-quantity-ratio-direct-inverse-c1`, `hegel-quantity-ratio-direct-inverse-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-quantity-ratio-powers
  - targetWorkbook: `RATIO-WORKBOOK.md`
  - note: the chapter culminates in powers and the measure threshold.
  - sourceClaimIds: [`hegel-quantity-ratio-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`hegel-quantity-ratio-powers-c1`, `hegel-quantity-ratio-powers-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the third quantity subchapter at readable scale.

### Entry hegel-quantity-ratio-direct-inverse — `Ratio`: direct and inverse ratio

- sourceFiles:
  - `ratio/ratio.txt`
  - `ratio/inverse.txt`
- lineSpans:
  - `ratio/ratio.txt:1-110`
  - `ratio/inverse.txt:1-268`
- summary: Direct ratio gives the immediate relational form of quantum, while inverse ratio makes the exponent into the negative limit governing the reciprocal determination of the sides.

Key points: (KeyPoint)

- k1. Direct ratio gives a shared determinateness through the exponent.
- k2. The sides remain incomplete quanta.
- k3. Inverse ratio makes reciprocal alteration explicit.
- k4. The exponent becomes a qualitative limit.

Claims: (Claim)

- c1. id: hegel-quantity-ratio-direct-inverse-c1
  - subject: direct_ratio
  - predicate: is
  - object: immediate_quantitative_relation_centered_on_the_exponent
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/ratio.txt:1-110`

- c2. id: hegel-quantity-ratio-direct-inverse-c2
  - subject: inverse_ratio
  - predicate: makes_explicit
  - object: reciprocal_limitation_and_qualitative_moment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/inverse.txt:1-268`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-quantity-ratio-powers
  - targetWorkbook: `RATIO-WORKBOOK.md`
  - note: the full return of quantity into itself occurs in the ratio of powers.
  - sourceClaimIds: [`hegel-quantity-ratio-direct-inverse-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-quantity-ratio-powers-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the direct/inverse band compact while preserving the conceptual shift.

### Entry hegel-quantity-ratio-powers — `Ratio`: powers and measure-threshold

- sourceFiles:
  - `ratio/powers.txt`
- lineSpans:
  - `ratio/powers.txt:1-140`
- summary: In the ratio of powers, quantum determines its own otherness and so becomes self-identical in externality; this is quantity's return into quality, and therefore the threshold of measure.

Key points: (KeyPoint)

- k1. Powers make quantum self-identical in its otherness.
- k2. Quantity returns into quality.
- k3. The truth of quantity is measure.

Claims: (Claim)

- c1. id: hegel-quantity-ratio-powers-c1
  - subject: ratio_of_powers
  - predicate: posits
  - object: quantum_as_self_determining_in_its_otherness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/powers.txt:1-94`

- c2. id: hegel-quantity-ratio-powers-c2
  - subject: ratio_of_powers
  - predicate: culminates_in
  - object: measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ratio/powers.txt:95-140`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: measure-threshold
  - targetWorkbook: `../measure/MEASURE-WORKBOOK.md`
  - note: the next section is measure.
  - sourceClaimIds: [`hegel-quantity-ratio-powers-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: pending_cross_workbook

Review outcome:

- review_pending
- notes: this threshold entry keeps the quantity-to-measure passage explicit.
