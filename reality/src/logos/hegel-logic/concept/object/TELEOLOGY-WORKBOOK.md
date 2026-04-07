# Teleology Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `teleology/`, not a replacement for the local idea workbook, compiler workbook, or the part workbooks.
- Read it to follow the chapter-level spine from subjective purpose, through the means, into realized purpose.
- Use the local `teleology/TELEOLOGY-IDEA-WORKBOOK.md` and part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the concept explicitly returns within objectivity as purpose.
- Second question: what is the chapter's central operator?
  Answer: purpose realizing itself through and beyond external mediation.
- Third question: where does the subfolder lead?
  Answer: to the Idea-threshold.

## Authority + format lock (must persist)

- Working extraction references: `teleology/teleology-idea.txt`, `teleology/subjective.txt`, `teleology/means.txt`, `teleology/realized.txt`, and `TELEOLOGY-DISTILLATION.md`
- Upstream source authority: `teleology/teleology-idea.txt`, `teleology/subjective.txt`, `teleology/means.txt`, `teleology/realized.txt`
- This workbook covers the `teleology/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Teleology-folder pass)

Scope:

- files:
  - `teleology/teleology-idea.txt`
  - `teleology/subjective.txt`
  - `teleology/means.txt`
  - `teleology/realized.txt`
  - `TELEOLOGY-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `teleology/` subfolder.
- Preserve the older local workbook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: subjective purpose, means, realized purpose.

### Entry hegel-object-teleology — Marker `Teleology`

- sourceFiles:
  - `teleology/teleology-idea.txt`
  - `teleology/subjective.txt`
  - `teleology/means.txt`
  - `teleology/realized.txt`
- lineSpans:
  - `teleology/teleology-idea.txt:1-376`
  - `teleology/subjective.txt:1-164`
  - `teleology/means.txt:1-180`
  - `teleology/realized.txt:1-606`
- summary: The `teleology/` subfolder presents purpose as the truth of objectivity, unfolds subjective purpose and means, and culminates in realized purpose as the threshold of the Idea.

Key points: (KeyPoint)

- k1. Teleology is the truth of mechanism and chemism.
- k2. The chapter unfolds through subjective purpose, means, and realized purpose.
- k3. External purposiveness is inadequate on its own.
- k4. Teleology culminates in inner purposiveness and the Idea-threshold.

Claims: (Claim)

- c1. id: hegel-object-teleology-c1
  - subject: teleology_chapter
  - predicate: captures
  - object: concept_as_purpose_in_free_concrete_existence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `teleology/teleology-idea.txt:1-376`
    - `teleology/subjective.txt:1-164`

- c2. id: hegel-object-teleology-c2
  - subject: teleology_chapter
  - predicate: unfolds_through
  - object: subjective_purpose_means_and_realized_purpose
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `teleology/subjective.txt:1-164`
    - `teleology/means.txt:1-180`
    - `teleology/realized.txt:1-606`

- c3. id: hegel-object-teleology-c3
  - subject: teleology_chapter
  - predicate: culminates_in
  - object: inner_purposiveness_and_idea_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `teleology/realized.txt:380-606`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-object-subjective-purpose
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: the chapter begins with purpose still standing over against presupposed objectivity.
  - sourceClaimIds: [`hegel-object-teleology-c1`, `hegel-object-teleology-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-object-subjective-purpose-c1`, `hegel-object-subjective-purpose-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-object-means
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: subjective purpose externalizes itself in the means.
  - sourceClaimIds: [`hegel-object-teleology-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-object-means-c1`, `hegel-object-means-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-object-realized-purpose
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: the truth of teleology appears in realized purpose.
  - sourceClaimIds: [`hegel-object-teleology-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-object-realized-purpose-c1`, `hegel-object-realized-purpose-c2`]

Review outcome:

- review_pending
- notes: this marker fixes teleology at readable chapter scale over the older part workbooks.

### Entry hegel-object-subjective-purpose — `Teleology`: subjective purpose

- sourceFiles:
  - `teleology/subjective.txt`
- lineSpans:
  - `teleology/subjective.txt:1-164`
- summary: Subjective purpose is the concept that has recovered itself within objectivity but still confronts an external world as presupposition.

Key points: (KeyPoint)

- k1. Purpose is the concept in free concrete existence.
- k2. It is not mere force or cause.
- k3. It still stands over against presupposed objectivity.

Claims: (Claim)

- c1. id: hegel-object-subjective-purpose-c1
  - subject: subjective_purpose
  - predicate: is
  - object: concept_as_free_self_determining_activity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `teleology/subjective.txt:1-80`

- c2. id: hegel-object-subjective-purpose-c2
  - subject: subjective_purpose
  - predicate: presupposes
  - object: external_objectivity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `teleology/subjective.txt:81-164`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-object-means
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: because purpose is finite, it needs the means.
  - sourceClaimIds: [`hegel-object-subjective-purpose-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-object-means-c1`]

Review outcome:

- review_pending
- notes: this entry keeps purpose's initial subjectivity explicit.

### Entry hegel-object-means — `Teleology`: the means and the cunning of reason

- sourceFiles:
  - `teleology/means.txt`
  - `teleology/realized.txt`
- lineSpans:
  - `teleology/means.txt:1-180`
  - `teleology/realized.txt:1-213`
- summary: The means is the object penetrated by purpose, and in its mediated action the cunning of reason lets externality bear the work while purpose preserves itself through it.

Key points: (KeyPoint)

- k1. The means is the middle term of purpose.
- k2. The means is objectivity subordinated to purpose.
- k3. The cunning of reason preserves purpose through mediation.

Claims: (Claim)

- c1. id: hegel-object-means-c1
  - subject: means
  - predicate: is
  - object: middle_term_of_purpose
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `teleology/means.txt:1-102`

- c2. id: hegel-object-means-c2
  - subject: cunning_of_reason
  - predicate: consists_in
  - object: purpose_preserving_itself_by_sending_the_means_into_externality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `teleology/realized.txt:43-84`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-object-realized-purpose
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: the truth of mediation appears only in realized purpose.
  - sourceClaimIds: [`hegel-object-means-c1`, `hegel-object-means-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`hegel-object-realized-purpose-c1`, `hegel-object-realized-purpose-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the means from appearing as a merely instrumental aside.

### Entry hegel-object-realized-purpose — `Teleology`: realized purpose and the Idea-threshold

- sourceFiles:
  - `teleology/realized.txt`
- lineSpans:
  - `teleology/realized.txt:214-606`
- summary: Realized purpose exposes the inadequacy of merely external purposiveness, sublates the means as external mediation, and yields inner purposiveness as the threshold of the Idea.

Key points: (KeyPoint)

- k1. Mere external purposiveness yields only means and perishable products.
- k2. The truth of teleology is inner purposiveness.
- k3. Realized purpose is concept identical with objectivity.
- k4. Teleology passes into the Idea.

Claims: (Claim)

- c1. id: hegel-object-realized-purpose-c1
  - subject: external_purposiveness
  - predicate: proves_to_be
  - object: inadequate_when_taken_by_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `teleology/realized.txt:214-379`

- c2. id: hegel-object-realized-purpose-c2
  - subject: realized_purpose
  - predicate: is
  - object: concept_identical_with_objectivity_and_threshold_of_idea
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `teleology/realized.txt:380-606`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: idea-threshold
  - targetWorkbook: `../idea/IDEA-WORKBOOK.md`
  - note: inner purposiveness is the threshold of the Idea.
  - sourceClaimIds: [`hegel-object-realized-purpose-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: pending_cross_workbook


Review outcome:

- review_pending
- notes: this entry keeps the handoff to the Idea explicit without inventing the next folder layer.
