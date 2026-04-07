# Objectivity Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `object/` folder.
- Read it to follow the folder-level architectonic from `mechanism/`, through `chemism/`, into `teleology/`.
- Its task is to preserve the high-level spine of the objectivity band within the Doctrine of the Concept.
- Use the chapter-level files [MECHANISM-WORKBOOK.md](MECHANISM-WORKBOOK.md), [CHEMISM-WORKBOOK.md](CHEMISM-WORKBOOK.md), and [TELEOLOGY-WORKBOOK.md](TELEOLOGY-WORKBOOK.md) for the detailed readable walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the folder-level sequence in which the concept first exists as external objectivity, then overcomes indifferent externality, and finally returns as purposive unity.
- Second question: what is the central operator of the folder as a whole?
  Answer: the progressive immanence of the concept within externality.
- Third question: where does the folder lead?
  Answer: into the Idea.

## Authority + format lock (must persist)

- Working extraction references: `object-idea.txt`, `mechanism/mechanism-idea.txt`, `chemism/chemism-idea.txt`, `teleology/teleology-idea.txt`, and `OBJECTIVITY-DISTILLATION.md`
- Upstream source authority: `object-idea.txt`, `mechanism/mechanism-idea.txt`, `chemism/chemism-idea.txt`, `teleology/teleology-idea.txt`
- This workbook covers the whole `object/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not collapse the folder-level container into one of its child chapter-clusters.
- Do not duplicate the chapter-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (objectivity-folder container pass)

Scope:

- files:
  - `object-idea.txt`
  - `mechanism/mechanism-idea.txt`
  - `chemism/chemism-idea.txt`
  - `teleology/teleology-idea.txt`
  - `OBJECTIVITY-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `object/` folder.
- Give each major child folder its own chapter-level artifact pair at the folder root.
- Keep the emphasis on the folder sequence: `mechanism/`, `chemism/`, `teleology/`.
- Preserve the older local coordination workbooks as denser legacy support rather than replacing them.

### Entry hegel-objectivity-folder — Marker `Objectivity`

- sourceFiles:
  - `object-idea.txt`
  - `mechanism/mechanism-idea.txt`
  - `chemism/chemism-idea.txt`
  - `teleology/teleology-idea.txt`
- lineSpans:
  - `object-idea.txt:1-383`
  - `mechanism/mechanism-idea.txt:1-43`
  - `chemism/chemism-idea.txt:1-17`
  - `teleology/teleology-idea.txt:1-376`
- summary: The `object/` folder unfolds as one high-level movement from external objectivity in mechanism, through non-indifferent process in chemism, to purpose as the truth of objectivity in teleology.

Key points: (KeyPoint)

- k1. Objectivity is the concept in immediate external existence.
- k2. Mechanism is the first field of objectivity.
- k3. Chemism internalizes difference into process.
- k4. Teleology returns the concept as the truth of objectivity.
- k5. The folder culminates in the Idea.

Claims: (Claim)

- c1. id: hegel-objectivity-folder-c1
  - subject: objectivity_folder
  - predicate: unfolds_through
  - object: mechanism_chemism_and_teleology
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `object-idea.txt:1-383`

- c2. id: hegel-objectivity-folder-c2
  - subject: objectivity_folder
  - predicate: mediates
  - object: from_external_objectivity_to_idea_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `object-idea.txt:1-383`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-objectivity-mechanism-folder
  - targetWorkbook: `OBJECTIVITY-WORKBOOK.md`
  - note: the folder begins with objectivity as external indifference.
  - sourceClaimIds: [`hegel-objectivity-folder-c1`, `hegel-objectivity-folder-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-objectivity-mechanism-folder-c1`, `hegel-objectivity-mechanism-folder-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-objectivity-chemism-folder
  - targetWorkbook: `OBJECTIVITY-WORKBOOK.md`
  - note: chemism negates the indifferent externality of mechanism.
  - sourceClaimIds: [`hegel-objectivity-folder-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-objectivity-chemism-folder-c1`, `hegel-objectivity-chemism-folder-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-objectivity-teleology-folder
  - targetWorkbook: `OBJECTIVITY-WORKBOOK.md`
  - note: teleology gives the concept back as the truth of objectivity.
  - sourceClaimIds: [`hegel-objectivity-folder-c1`, `hegel-objectivity-folder-c2`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`hegel-objectivity-teleology-folder-c1`, `hegel-objectivity-teleology-folder-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the top-level `object/` surface as a true container artifact.

### Entry hegel-objectivity-mechanism-folder — `Objectivity`: the `mechanism/` opening field

- sourceFiles:
  - `mechanism/mechanism-idea.txt`
- lineSpans:
  - `mechanism/mechanism-idea.txt:1-43`
- summary: The `mechanism/` subfolder gives objectivity first as a plurality of self-subsistent objects whose unity is only inner law or external arrangement.

Key points: (KeyPoint)

- k1. Mechanism is objectivity as external indifference.
- k2. Objects are totalities standing outside one another.
- k3. Mechanism turns toward chemism.

Claims: (Claim)

- c1. id: hegel-objectivity-mechanism-folder-c1
  - subject: mechanism_subfolder
  - predicate: captures
  - object: external_indifference_of_objectivity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `mechanism/mechanism-idea.txt:1-43`

- c2. id: hegel-objectivity-mechanism-folder-c2
  - subject: mechanism_subfolder
  - predicate: turns_toward
  - object: chemism
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `mechanism/mechanism-idea.txt:1-43`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-objectivity-chemism-folder
  - targetWorkbook: `OBJECTIVITY-WORKBOOK.md`
  - note: the contradiction of external indifference drives the chapter into chemism.
  - sourceClaimIds: [`hegel-objectivity-mechanism-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-objectivity-chemism-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-object-mechanism
  - targetWorkbook: `MECHANISM-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Mechanism workbook.
  - sourceClaimIds: [`hegel-objectivity-mechanism-folder-c1`, `hegel-objectivity-mechanism-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-object-mechanism-c1`, `hegel-object-mechanism-c2`, `hegel-object-mechanism-c3`]

Review outcome:

- review_pending
- notes: this entry keeps the first objective field at chapter scale.

### Entry hegel-objectivity-chemism-folder — `Objectivity`: the `chemism/` field of process

- sourceFiles:
  - `chemism/chemism-idea.txt`
- lineSpans:
  - `chemism/chemism-idea.txt:1-17`
- summary: The `chemism/` subfolder gives the object as non-indifferent to its determinateness, so that its relation to others becomes affinity, tension, and process.

Key points: (KeyPoint)

- k1. Chemism is objectivity as non-indifferent determinateness.
- k2. Objects seek completion through process.
- k3. Chemism turns toward teleology.

Claims: (Claim)

- c1. id: hegel-objectivity-chemism-folder-c1
  - subject: chemism_subfolder
  - predicate: captures
  - object: non_indifferent_objectivity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `chemism/chemism-idea.txt:1-17`

- c2. id: hegel-objectivity-chemism-folder-c2
  - subject: chemism_subfolder
  - predicate: turns_toward
  - object: teleology
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `chemism/chemism-idea.txt:1-17`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-objectivity-teleology-folder
  - targetWorkbook: `OBJECTIVITY-WORKBOOK.md`
  - note: chemism sublates itself into the higher sphere of teleology.
  - sourceClaimIds: [`hegel-objectivity-chemism-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-objectivity-teleology-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-object-chemism
  - targetWorkbook: `CHEMISM-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Chemism workbook.
  - sourceClaimIds: [`hegel-objectivity-chemism-folder-c1`, `hegel-objectivity-chemism-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-object-chemism-c1`, `hegel-object-chemism-c2`, `hegel-object-chemism-c3`]

Review outcome:

- review_pending
- notes: this entry keeps chemism at the scale of tension and process rather than only chemical examples.

### Entry hegel-objectivity-teleology-folder — `Objectivity`: the `teleology/` field of purpose

- sourceFiles:
  - `teleology/teleology-idea.txt`
- lineSpans:
  - `teleology/teleology-idea.txt:1-376`
- summary: The `teleology/` subfolder gives the concept back as purpose, subordinates mechanism and chemism, and carries objectivity to the threshold of the Idea.

Key points: (KeyPoint)

- k1. Teleology is purpose as the concept in free concrete existence.
- k2. Mechanism and chemism are subordinated within teleology.
- k3. Teleology culminates in the Idea-threshold.

Claims: (Claim)

- c1. id: hegel-objectivity-teleology-folder-c1
  - subject: teleology_subfolder
  - predicate: captures
  - object: concept_as_purpose_in_objectivity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `teleology/teleology-idea.txt:1-376`

- c2. id: hegel-objectivity-teleology-folder-c2
  - subject: teleology_subfolder
  - predicate: culminates_in
  - object: idea_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `teleology/teleology-idea.txt:1-376`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-object-teleology
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Teleology workbook.
  - sourceClaimIds: [`hegel-objectivity-teleology-folder-c1`, `hegel-objectivity-teleology-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-object-teleology-c1`, `hegel-object-teleology-c2`, `hegel-object-teleology-c3`]

Review outcome:

- review_pending
- notes: this entry fixes the teleological completion of objectivity without pre-writing the Idea layer.
