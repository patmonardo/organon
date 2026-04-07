# Chemism Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `chemism/`, not a replacement for the local idea workbook, compiler workbook, or the part workbooks.
- Read it to follow the chapter-level spine from the chemical object, through process, into the transition to teleology.
- Use the local `chemism/CHEMISM-IDEA-WORKBOOK.md` and part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which objectivity ceases to be indifferent to its determinateness and becomes process.
- Second question: what is the chapter's central operator?
  Answer: non-indifference, affinity, and the striving for concrete totality.
- Third question: where does the subfolder lead?
  Answer: to teleology.

## Authority + format lock (must persist)

- Working extraction references: `chemism/chemism-idea.txt`, `chemism/object.txt`, `chemism/process.txt`, and `CHEMISM-DISTILLATION.md`
- Upstream source authority: `chemism/chemism-idea.txt`, `chemism/object.txt`, `chemism/process.txt`
- This workbook covers the `chemism/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Chemism-folder pass)

Scope:

- files:
  - `chemism/chemism-idea.txt`
  - `chemism/object.txt`
  - `chemism/process.txt`
  - `CHEMISM-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `chemism/` subfolder.
- Preserve the older local workbook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: chemical object, process, transition to teleology.

### Entry hegel-object-chemism — Marker `Chemism`

- sourceFiles:
  - `chemism/chemism-idea.txt`
  - `chemism/object.txt`
  - `chemism/process.txt`
- lineSpans:
  - `chemism/chemism-idea.txt:1-17`
  - `chemism/object.txt:1-83`
  - `chemism/process.txt:1-179`
- summary: The `chemism/` subfolder presents objectivity as non-indifferent determinateness, unfolds affinity and neutralization, and drives that process toward teleology.

Key points: (KeyPoint)

- k1. Chemism is objectivity as non-indifference.
- k2. The chapter unfolds through object and process.
- k3. Neutrality is only formal.
- k4. Chemism passes into teleology.

Claims: (Claim)

- c1. id: hegel-object-chemism-c1
  - subject: chemism_chapter
  - predicate: captures
  - object: objectivity_as_non_indifferent_determinateness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `chemism/chemism-idea.txt:1-17`
    - `chemism/object.txt:1-83`

- c2. id: hegel-object-chemism-c2
  - subject: chemism_chapter
  - predicate: unfolds_through
  - object: affinity_neutralization_and_reactivated_process
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `chemism/process.txt:1-179`

- c3. id: hegel-object-chemism-c3
  - subject: chemism_chapter
  - predicate: culminates_in
  - object: transition_to_teleology
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `chemism/process.txt:156-179`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-object-chemical-object
  - targetWorkbook: `CHEMISM-WORKBOOK.md`
  - note: the chapter opens with the chemical object and its non-indifference.
  - sourceClaimIds: [`hegel-object-chemism-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-object-chemical-object-c1`, `hegel-object-chemical-object-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-object-chemical-process
  - targetWorkbook: `CHEMISM-WORKBOOK.md`
  - note: the process resolves and reopens until the higher sphere appears.
  - sourceClaimIds: [`hegel-object-chemism-c2`, `hegel-object-chemism-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-object-chemical-process-c1`, `hegel-object-chemical-process-c2`]

Review outcome:

- review_pending
- notes: this marker fixes chemism at readable chapter scale over the older part workbooks.

### Entry hegel-object-chemical-object — `Chemism`: the chemical object

- sourceFiles:
  - `chemism/object.txt`
- lineSpans:
  - `chemism/object.txt:1-83`
- summary: The chemical object is a totality whose determinateness belongs to its own nature, so that its incompleteness is an inner striving toward relation and completion.

Key points: (KeyPoint)

- k1. The object's determinateness belongs to its concept.
- k2. Its reference to another is internal, not accidental.
- k3. The object is a striving toward real totality.

Claims: (Claim)

- c1. id: hegel-object-chemical-object-c1
  - subject: chemical_object
  - predicate: is
  - object: non_indifferent_to_its_determinateness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `chemism/object.txt:1-39`

- c2. id: hegel-object-chemical-object-c2
  - subject: chemical_object
  - predicate: strives_toward
  - object: objective_totality_of_the_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `chemism/object.txt:40-83`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-object-chemical-process
  - targetWorkbook: `CHEMISM-WORKBOOK.md`
  - note: this inner contradiction becomes affinity and process.
  - sourceClaimIds: [`hegel-object-chemical-object-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-object-chemical-process-c1`]

Review outcome:

- review_pending
- notes: this entry keeps chemism grounded in the object's own non-indifference.

### Entry hegel-object-chemical-process — `Chemism`: affinity, neutrality, reactivation

- sourceFiles:
  - `chemism/process.txt`
- lineSpans:
  - `chemism/process.txt:1-179`
- summary: Chemical process begins in affinity, reaches formal neutrality, and then reactivates itself until the process sublates itself into teleology.

Key points: (KeyPoint)

- k1. Affinity and communication initiate the process.
- k2. Neutral product resolves the first contradiction only formally.
- k3. The process reactivates itself from outside neutrality.
- k4. Chemism passes beyond itself.

Claims: (Claim)

- c1. id: hegel-object-chemical-process-c1
  - subject: chemical_process
  - predicate: unfolds_through
  - object: affinity_neutralization_and_renewed_tension
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `chemism/process.txt:1-155`

- c2. id: hegel-object-chemical-process-c2
  - subject: chemical_process
  - predicate: sublates_itself_into
  - object: teleology
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `chemism/process.txt:156-179`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-object-teleology
  - targetWorkbook: `TELEOLOGY-WORKBOOK.md`
  - note: the higher truth of process is purpose.
  - sourceClaimIds: [`hegel-object-chemical-process-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-object-teleology-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the teleological handoff explicit at the end of chemism.
