# Being Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `being/` folder.
- Read it to follow the sphere-level architectonic from quality, through quantity, into measure.
- Its task is to preserve the high-level spine of the Sphere of Being within the Logic as a whole.
- Use the section-level readable files [quality/QUALITY-WORKBOOK.md](quality/QUALITY-WORKBOOK.md), [quantity/QUANTITY-WORKBOOK.md](quantity/QUANTITY-WORKBOOK.md), and [measure/MEASURE-WORKBOOK.md](measure/MEASURE-WORKBOOK.md) for the detailed walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the sphere-level sequence in which being unfolds as quality, quantity, and measure.
- Second question: what is the central operator of the sphere as a whole?
  Answer: determinateness first immediate, then indifferent, then reunited.
- Third question: where does the sphere lead?
  Answer: into essence.

## Authority + format lock (must persist)

- Working extraction references: `being-idea.txt`, `quality/quality-idea.txt`, `quantity/quantity-idea.txt`, `measure/measure-idea.txt`, and `BEING-DISTILLATION.md`
- Upstream source authority: `being-idea.txt`, `quality/quality-idea.txt`, `quantity/quantity-idea.txt`, `measure/measure-idea.txt`
- This workbook covers the whole `being/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Being side.
- Do not collapse the sphere-level container into one of its child sections.
- Do not duplicate the section-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (being-folder container pass)

Scope:

- files:
  - `being-idea.txt`
  - `quality/quality-idea.txt`
  - `quantity/quantity-idea.txt`
  - `measure/measure-idea.txt`
  - `BEING-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `being/` folder.
- Keep the emphasis on the sphere sequence: `quality/`, `quantity/`, `measure/`.
- Preserve the older local idea and dense chapter stacks as the detailed lower layers.

### Entry hegel-being-sphere — Marker `Being`

- sourceFiles:
  - `being-idea.txt`
  - `quality/quality-idea.txt`
  - `quantity/quantity-idea.txt`
  - `measure/measure-idea.txt`
- lineSpans:
  - `being-idea.txt:700-772`
  - `quality/quality-idea.txt:1-24`
  - `quantity/quantity-idea.txt:1-64`
  - `measure/measure-idea.txt:181-236`
- summary: The `being/` folder is the first sphere of logic and unfolds as quality, quantity, and measure, with measure carrying the sphere over into essence.

Key points: (KeyPoint)

- k1. Being is the first sphere of logic.
- k2. The first section is quality.
- k3. The second section is quantity.
- k4. The third section is measure.
- k5. The sphere culminates in essence.

Claims: (Claim)

- c1. id: hegel-being-sphere-c1
  - subject: being_sphere
  - predicate: unfolds_through
  - object: quality_quantity_and_measure
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `being-idea.txt:700-772`

- c2. id: hegel-being-sphere-c2
  - subject: being_sphere
  - predicate: culminates_in
  - object: essence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `measure/measure-idea.txt:181-236`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-being-quality-section
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: being first appears as immediate determinateness.
  - sourceClaimIds: [`hegel-being-sphere-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-being-quality-section-c1`, `hegel-being-quality-section-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-being-quantity-section
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: being next appears as indifferent determinateness.
  - sourceClaimIds: [`hegel-being-sphere-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-being-quantity-section-c1`, `hegel-being-quantity-section-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-being-measure-section
  - targetWorkbook: `BEING-WORKBOOK.md`
  - note: being completes itself as measure and passes into essence.
  - sourceClaimIds: [`hegel-being-sphere-c1`, `hegel-being-sphere-c2`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`hegel-being-measure-section-c1`, `hegel-being-measure-section-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the sphere-level `being/` surface.

### Entry hegel-being-quality-section — `Being`: quality as immediate determinateness

- sourceFiles:
  - `being-idea.txt`
  - `quality/quality-idea.txt`
- lineSpans:
  - `being-idea.txt:709-756`
  - `quality/quality-idea.txt:1-24`
- summary: Quality is the first section of being because determinateness is still immediate and one with being; it carries the sphere from being and nothing through becoming and existence to the threshold of quantity.

Key points: (KeyPoint)

- k1. Quality is immediate determinateness.
- k2. Determinateness is one with being.
- k3. The section culminates in quantity.

Claims: (Claim)

- c1. id: hegel-being-quality-section-c1
  - subject: quality_section
  - predicate: is
  - object: immediate_determinateness_of_being
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `being-idea.txt:709-756`

- c2. id: hegel-being-quality-section-c2
  - subject: quality_section
  - predicate: culminates_in
  - object: quantity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quality/quality-idea.txt:1-24`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-being-quality-folder
  - targetWorkbook: `quality/QUALITY-WORKBOOK.md`
  - note: the detailed readable surface for quality is in the section workbook.
  - sourceClaimIds: [`hegel-being-quality-section-c1`, `hegel-being-quality-section-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-being-quality-folder-c1`, `hegel-being-quality-folder-c2`]

Review outcome:

- review_pending
- notes: this entry keeps Quality at section scale.

### Entry hegel-being-quantity-section — `Being`: quantity as sublated determinateness

- sourceFiles:
  - `being-idea.txt`
  - `quantity/quantity-idea.txt`
- lineSpans:
  - `being-idea.txt:736-758`
  - `quantity/quantity-idea.txt:1-64`
- summary: Quantity is quality become negative and indifferent, so determinateness survives as magnitude, quantum, and ratio, and prepares the reunion of quality and quantity in measure.

Key points: (KeyPoint)

- k1. Quantity is sublated quality.
- k2. Determinateness becomes indifferent magnitude.
- k3. The section culminates in measure.

Claims: (Claim)

- c1. id: hegel-being-quantity-section-c1
  - subject: quantity_section
  - predicate: is
  - object: sublated_determinateness_of_being
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `being-idea.txt:736-747`

- c2. id: hegel-being-quantity-section-c2
  - subject: quantity_section
  - predicate: culminates_in
  - object: measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `quantity/quantity-idea.txt:55-64`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-being-quantity-folder
  - targetWorkbook: `quantity/QUANTITY-WORKBOOK.md`
  - note: the detailed readable surface for quantity is in the section workbook.
  - sourceClaimIds: [`hegel-being-quantity-section-c1`, `hegel-being-quantity-section-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-being-quantity-folder-c1`, `hegel-being-quantity-folder-c2`]

Review outcome:

- review_pending
- notes: this entry keeps Quantity at section scale.

### Entry hegel-being-measure-section — `Being`: measure as qualitatively determined quantity

- sourceFiles:
  - `being-idea.txt`
  - `measure/measure-idea.txt`
- lineSpans:
  - `being-idea.txt:757-772`
  - `measure/measure-idea.txt:1-236`
- summary: Measure is qualitatively determined quantity, the concrete unity of quality and quantity, and therefore the final form of being and the threshold of essence.

Key points: (KeyPoint)

- k1. Measure unites quality and quantity.
- k2. Measure is the concrete truth of being.
- k3. The section culminates in essence.

Claims: (Claim)

- c1. id: hegel-being-measure-section-c1
  - subject: measure_section
  - predicate: is
  - object: qualitatively_determined_quantity
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `being-idea.txt:757-772`
    - `measure/measure-idea.txt:1-13`

- c2. id: hegel-being-measure-section-c2
  - subject: measure_section
  - predicate: culminates_in
  - object: essence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `measure/measure-idea.txt:181-236`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-being-measure-folder
  - targetWorkbook: `measure/MEASURE-WORKBOOK.md`
  - note: the detailed readable surface for measure is in the section workbook.
  - sourceClaimIds: [`hegel-being-measure-section-c1`, `hegel-being-measure-section-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-being-measure-folder-c1`, `hegel-being-measure-folder-c2`]

- r2. type: transitions_to
  - targetEntryId: essence-threshold
  - targetWorkbook: `../essence/ESSENCE-WORKBOOK.md`
  - note: the next sphere is essence.
  - sourceClaimIds: [`hegel-being-measure-section-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: pending_cross_workbook

Review outcome:

- review_pending
- notes: this entry keeps Measure at section scale and fixes the Being-to-Essence threshold.
