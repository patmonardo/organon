# Relation Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `relation/`, not a replacement for the local Part A/B/C workbooks.
- Read it to follow the chapter-level spine from whole and parts, through force and expression, into outer and inner.
- Use the local `relation/RELATION-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which appearance becomes essential relation and then exhausts itself into actuality.
- Second question: what is the chapter's central operator?
  Answer: each side being what it is only through its other.
- Third question: where does the subfolder lead?
  Answer: to actuality.

## Authority + format lock (must persist)

- Working extraction references: `relation/relation-idea.txt`, `relation/whole-parts.txt`, `relation/force-expression.txt`, `relation/outer-inner.txt`, and `RELATION-DISTILLATION.md`
- Upstream source authority: `relation/relation-idea.txt`, `relation/whole-parts.txt`, `relation/force-expression.txt`, `relation/outer-inner.txt`
- This workbook covers the `relation/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-04 (Relation-folder pass)

Scope:

- files:
  - `relation/relation-idea.txt`
  - `relation/whole-parts.txt`
  - `relation/force-expression.txt`
  - `relation/outer-inner.txt`
  - `RELATION-DISTILLATION.md`
- pass policy: 1 marker entry + 4 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `relation/` subfolder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: whole and parts, force and expression, outer and inner, actuality threshold.

### Entry hegel-appearance-relation — Marker `Relation`

- sourceFiles:
  - `relation/relation-idea.txt`
  - `relation/whole-parts.txt`
  - `relation/force-expression.txt`
  - `relation/outer-inner.txt`
- lineSpans:
  - `relation/relation-idea.txt:1-111`
  - `relation/whole-parts.txt:2-262`
  - `relation/force-expression.txt:2-369`
  - `relation/outer-inner.txt:2-231`
- summary: The `relation/` subfolder presents essential relation as the truth of appearance, unfolding through whole and parts, force and expression, and outer and inner until actuality emerges.

Key points: (KeyPoint)

- k1. Essential relation is the truth of appearance.
- k2. Whole and parts is the first relational form.
- k3. Force and expression deepen the relation.
- k4. Outer and inner complete the chapter and turn it into actuality.

Claims: (Claim)

- c1. id: hegel-appearance-relation-c1
  - subject: relation_chapter
  - predicate: unfolds_through
  - object: whole_parts_force_expression_and_outer_inner
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `relation/relation-idea.txt:1-111`
    - `relation/whole-parts.txt:2-262`
    - `relation/force-expression.txt:2-369`
    - `relation/outer-inner.txt:2-231`

- c2. id: hegel-appearance-relation-c2
  - subject: relation_chapter
  - predicate: is
  - object: truth_of_appearance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `relation/relation-idea.txt:1-34`

- c3. id: hegel-appearance-relation-c3
  - subject: relation_chapter
  - predicate: culminates_in
  - object: actuality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `relation/outer-inner.txt:220-231`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-appearance-whole-parts
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: the chapter begins with whole and parts.
  - sourceClaimIds: [`hegel-appearance-relation-c1`, `hegel-appearance-relation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-appearance-whole-parts-c1`, `hegel-appearance-whole-parts-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-appearance-force-expression
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: the truth of whole and parts is force and expression.
  - sourceClaimIds: [`hegel-appearance-relation-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-force-expression-c1`, `hegel-appearance-force-expression-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-appearance-outer-inner
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: force and expression culminate in outer and inner.
  - sourceClaimIds: [`hegel-appearance-relation-c1`, `hegel-appearance-relation-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-appearance-outer-inner-c1`, `hegel-appearance-outer-inner-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the final appearance chapter as a single consumating movement.

### Entry hegel-appearance-whole-parts — `Relation`: whole and parts

- sourceFiles:
  - `relation/whole-parts.txt`
- lineSpans:
  - `relation/whole-parts.txt:2-262`
- summary: Whole and parts first appear as reciprocal self-subsistences, but each has its being only in the other, and their truth is mediation.

Key points: (KeyPoint)

- k1. Whole and parts first appear as self-subsistent sides.
- k2. Each conditions and presupposes the other.
- k3. Their truth is mediation, not external aggregation.

Claims: (Claim)

- c1. id: hegel-appearance-whole-parts-c1
  - subject: whole_and_parts
  - predicate: are
  - object: reciprocally_conditioning_sides
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `relation/whole-parts.txt:2-100`

- c2. id: hegel-appearance-whole-parts-c2
  - subject: truth_of_whole_and_parts
  - predicate: is
  - object: mediation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `relation/whole-parts.txt:176-262`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-force-expression
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: the truth of whole and parts is force and expression.
  - sourceClaimIds: [`hegel-appearance-whole-parts-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-force-expression-c1`]

Review outcome:

- review_pending
- notes: this entry keeps whole and parts from remaining a merely mechanical schema.

### Entry hegel-appearance-force-expression — `Relation`: force and expression

- sourceFiles:
  - `relation/force-expression.txt`
- lineSpans:
  - `relation/force-expression.txt:2-369`
- summary: Force is the higher truth of whole and parts: it passes into expression, returns into itself through that expression, and preserves itself in the movement.

Key points: (KeyPoint)

- k1. Force is the truth of whole and parts.
- k2. Force passes into expression and returns from it.
- k3. The movement preserves itself through alteration.

Claims: (Claim)

- c1. id: hegel-appearance-force-expression-c1
  - subject: force
  - predicate: is
  - object: truth_of_whole_and_parts
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `relation/force-expression.txt:2-56`

- c2. id: hegel-appearance-force-expression-c2
  - subject: force
  - predicate: unfolds_as
  - object: conditionedness_solicitation_and_infinity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `relation/force-expression.txt:57-188`
    - `relation/force-expression.txt:189-341`
    - `relation/force-expression.txt:342-369`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-outer-inner
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: force and expression culminate in the relation of outer and inner.
  - sourceClaimIds: [`hegel-appearance-force-expression-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`hegel-appearance-outer-inner-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the force chapter at the level of architectonic movement rather than isolating its examples.

### Entry hegel-appearance-outer-inner — `Relation`: outer and inner

- sourceFiles:
  - `relation/outer-inner.txt`
- lineSpans:
  - `relation/outer-inner.txt:2-231`
- summary: Outer and inner complete the relation by showing that each is immediately the other and that a thing's externality is the complete expression of what it is in itself.

Key points: (KeyPoint)

- k1. Inner and outer are immediately each the other.
- k2. Their unity is the identity of expression and essence.
- k3. What something is, it is entirely in its externality.
- k4. The chapter turns into actuality.

Claims: (Claim)

- c1. id: hegel-appearance-outer-inner-c1
  - subject: inner_and_outer
  - predicate: are
  - object: immediate_conversion_of_each_into_the_other
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `relation/outer-inner.txt:4-103`

- c2. id: hegel-appearance-outer-inner-c2
  - subject: what_something_is
  - predicate: is
  - object: entirely_in_its_externality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `relation/outer-inner.txt:185-231`

Relations: (Relation)

- r1. type: gathers
  - targetEntryId: hegel-appearance-force-expression
  - targetWorkbook: `RELATION-WORKBOOK.md`
  - note: outer and inner gathers the self-expression already present in force.
  - sourceClaimIds: [`hegel-appearance-outer-inner-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-appearance-force-expression-c2`]

Review outcome:

- review_pending
- notes: this terminal entry keeps the chapter's actuality-threshold explicit without pre-writing the actuality container.
