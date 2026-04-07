# Life Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `life/`, not a replacement for the local idea workbook, compiler workbook, or the part workbooks.
- Read it to follow the chapter-level spine from living individual, through life-process, into genus.
- Use the local `life/LIFE-IDEA-WORKBOOK.md` and part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the Idea first appears immediately as life.
- Second question: what is the chapter's central operator?
  Answer: living unity maintaining itself through negativity and process.
- Third question: where does the subfolder lead?
  Answer: to cognition.

## Authority + format lock (must persist)

- Working extraction references: `life/life-idea.txt`, `life/living-individual.txt`, `life/life-process.txt`, `life/genus.txt`, and `LIFE-DISTILLATION.md`
- Upstream source authority: `life/life-idea.txt`, `life/living-individual.txt`, `life/life-process.txt`, `life/genus.txt`
- This workbook covers the `life/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Life-folder pass)

Scope:

- files:
  - `life/life-idea.txt`
  - `life/living-individual.txt`
  - `life/life-process.txt`
  - `life/genus.txt`
  - `LIFE-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `life/` subfolder.
- Preserve the older local workbook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: living individual, life-process, genus.

### Entry hegel-idea-life — Marker `Life`

- sourceFiles:
  - `life/life-idea.txt`
  - `life/living-individual.txt`
  - `life/life-process.txt`
  - `life/genus.txt`
- lineSpans:
  - `life/life-idea.txt:1-221`
  - `life/living-individual.txt:1-333`
  - `life/life-process.txt:1-181`
  - `life/genus.txt:1-158`
- summary: The `life/` subfolder presents the Idea as immediate life, unfolds living individual and life-process, and culminates in genus as the transition to cognition.

Key points: (KeyPoint)

- k1. Life is the immediate Idea.
- k2. The chapter unfolds through individual, process, and genus.
- k3. Genus transitions into cognition.

Claims: (Claim)

- c1. id: hegel-idea-life-c1
  - subject: life_chapter
  - predicate: captures
  - object: immediate_idea_of_concept_and_objectivity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `life/life-idea.txt:1-221`
    - `life/living-individual.txt:4-86`

- c2. id: hegel-idea-life-c2
  - subject: life_chapter
  - predicate: unfolds_through
  - object: living_individual_life_process_and_genus
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `life/life-idea.txt:183-221`
    - `life/living-individual.txt:1-333`
    - `life/life-process.txt:1-181`
    - `life/genus.txt:1-158`

- c3. id: hegel-idea-life-c3
  - subject: life_chapter
  - predicate: culminates_in
  - object: cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `life/life-idea.txt:209-221`
    - `life/genus.txt:144-158`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-idea-living-individual
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: the chapter begins with the living individual as immediate idea.
  - sourceClaimIds: [`hegel-idea-life-c1`, `hegel-idea-life-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-idea-living-individual-c1`, `hegel-idea-living-individual-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-idea-life-process
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: immediate life must mediate itself in process and genus.
  - sourceClaimIds: [`hegel-idea-life-c2`, `hegel-idea-life-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`hegel-idea-life-process-c1`, `hegel-idea-life-process-c2`]

Review outcome:

- review_pending
- notes: this marker fixes Life at readable chapter scale over the local triadic stack.

### Entry hegel-idea-living-individual — `Life`: immediate living individuality

- sourceFiles:
  - `life/living-individual.txt`
- lineSpans:
  - `life/living-individual.txt:1-206`
- summary: The living individual is the immediate Idea as singularized negative unity, in which concept and objectivity correspond through self-maintaining life.

Key points: (KeyPoint)

- k1. The living individual is immediate concept-objectivity unity.
- k2. This unity exists as negative self-reference.
- k3. Immediate life already contains inner contradiction.

Claims: (Claim)

- c1. id: hegel-idea-living-individual-c1
  - subject: living_individual
  - predicate: is
  - object: immediate_idea_as_singular_negative_unity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `life/living-individual.txt:4-86`

- c2. id: hegel-idea-living-individual-c2
  - subject: living_individual
  - predicate: requires
  - object: mediation_of_its_inner_contradiction
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `life/living-individual.txt:163-206`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-life-process
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: the living individual passes into life-process.
  - sourceClaimIds: [`hegel-idea-living-individual-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-life-process-c1`]

Review outcome:

- review_pending
- notes: this entry keeps immediate life tied to its necessary mediation.

### Entry hegel-idea-life-process — `Life`: process, genus, cognition-threshold

- sourceFiles:
  - `life/life-process.txt`
  - `life/genus.txt`
- lineSpans:
  - `life/life-process.txt:1-181`
  - `life/genus.txt:1-158`
- summary: Life-process mediates living contradiction through need and assimilation, while genus raises life into concrete universality and opens cognition.

Key points: (KeyPoint)

- k1. Life-process is mediation through contradiction.
- k2. Genus is concrete universality.
- k3. Life reaches the cognition-threshold.

Claims: (Claim)

- c1. id: hegel-idea-life-process-c1
  - subject: life_process
  - predicate: mediates
  - object: contradiction_need_and_reproduction
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `life/life-process.txt:30-158`

- c2. id: hegel-idea-life-process-c2
  - subject: genus
  - predicate: culminates_in
  - object: cognition_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `life/genus.txt:21-143`
    - `life/genus.txt:144-158`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: the universal withdrawn from immediate life becomes cognition.
  - sourceClaimIds: [`hegel-idea-life-process-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-cognition-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the life-to-cognition handoff explicit.
