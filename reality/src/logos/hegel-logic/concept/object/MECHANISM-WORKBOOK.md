# Mechanism Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `mechanism/`, not a replacement for the local idea workbook, compiler workbook, or the part workbooks.
- Read it to follow the chapter-level spine from the mechanical object, through the mechanical process, into center and law.
- Use the local `mechanism/MECHANISM-IDEA-WORKBOOK.md` and part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which objectivity first appears as a field of externally connected, self-subsistent objects.
- Second question: what is the chapter's central operator?
  Answer: external connection that still does not belong to objects as their own concept.
- Third question: where does the subfolder lead?
  Answer: toward chemism.

## Authority + format lock (must persist)

- Working extraction references: `mechanism/mechanism-idea.txt`, `mechanism/object.txt`, `mechanism/process.txt`, and `MECHANISM-DISTILLATION.md`
- Upstream source authority: `mechanism/mechanism-idea.txt`, `mechanism/object.txt`, `mechanism/process.txt`
- This workbook covers the `mechanism/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Mechanism-folder pass)

Scope:

- files:
  - `mechanism/mechanism-idea.txt`
  - `mechanism/object.txt`
  - `mechanism/process.txt`
  - `MECHANISM-DISTILLATION.md`
- pass policy: 1 marker entry + 2 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `mechanism/` subfolder.
- Preserve the older local workbook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: mechanical object, mechanical process, center and law.

### Entry hegel-object-mechanism — Marker `Mechanism`

- sourceFiles:
  - `mechanism/mechanism-idea.txt`
  - `mechanism/object.txt`
  - `mechanism/process.txt`
- lineSpans:
  - `mechanism/mechanism-idea.txt:1-43`
  - `mechanism/object.txt:1-157`
  - `mechanism/process.txt:1-455`
- summary: The `mechanism/` subfolder presents objectivity as external indifference, unfolds the mechanical object and process, and yields center and law as the truth of purely external interaction.

Key points: (KeyPoint)

- k1. Mechanism is objectivity as external indifference.
- k2. The chapter unfolds through the mechanical object and process.
- k3. Center and law are the truth generated within mechanism.
- k4. The chapter points toward chemism.

Claims: (Claim)

- c1. id: hegel-object-mechanism-c1
  - subject: mechanism_chapter
  - predicate: captures
  - object: objectivity_as_external_indifference
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `mechanism/mechanism-idea.txt:1-43`
    - `mechanism/object.txt:1-157`

- c2. id: hegel-object-mechanism-c2
  - subject: mechanism_chapter
  - predicate: unfolds_through
  - object: mechanical_object_and_mechanical_process
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `mechanism/object.txt:1-157`
    - `mechanism/process.txt:1-455`

- c3. id: hegel-object-mechanism-c3
  - subject: mechanism_chapter
  - predicate: culminates_in
  - object: center_and_law_as_transition_to_chemism
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `mechanism/process.txt:397-455`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-object-mechanical-object
  - targetWorkbook: `MECHANISM-WORKBOOK.md`
  - note: the chapter opens with the mechanical object.
  - sourceClaimIds: [`hegel-object-mechanism-c1`, `hegel-object-mechanism-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-object-mechanical-object-c1`, `hegel-object-mechanical-object-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-object-mechanical-process
  - targetWorkbook: `MECHANISM-WORKBOOK.md`
  - note: the chapter proceeds from object to process and its truth.
  - sourceClaimIds: [`hegel-object-mechanism-c2`, `hegel-object-mechanism-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-object-mechanical-process-c1`, `hegel-object-mechanical-process-c2`]

Review outcome:

- review_pending
- notes: this marker fixes mechanism at readable chapter scale over the older part workbooks.

### Entry hegel-object-mechanical-object — `Mechanism`: the mechanical object

- sourceFiles:
  - `mechanism/object.txt`
- lineSpans:
  - `mechanism/object.txt:1-157`
- summary: The mechanical object is a totality indifferent to its determinateness, so that its order, arrangement, and connection remain external to what it is.

Key points: (KeyPoint)

- k1. The object is a totality, not a mere aggregate of accidents.
- k2. Its determinateness is external to its own concept.
- k3. Determinism therefore becomes an empty regress of outside explanation.

Claims: (Claim)

- c1. id: hegel-object-mechanical-object-c1
  - subject: mechanical_object
  - predicate: is
  - object: totality_indifferent_to_its_determinateness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `mechanism/object.txt:1-76`

- c2. id: hegel-object-mechanical-object-c2
  - subject: mechanical_object
  - predicate: points_outside_itself_for
  - object: every_determination
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `mechanism/object.txt:77-157`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-object-mechanical-process
  - targetWorkbook: `MECHANISM-WORKBOOK.md`
  - note: the contradiction of external identity and externality becomes process.
  - sourceClaimIds: [`hegel-object-mechanical-object-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-object-mechanical-process-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the chapter anchored in indifferent objectivity rather than later causal overlays.

### Entry hegel-object-mechanical-process — `Mechanism`: communication, reaction, center, law

- sourceFiles:
  - `mechanism/process.txt`
- lineSpans:
  - `mechanism/process.txt:1-455`
- summary: The mechanical process communicates a universal, produces reaction and resistance, and yields center and law as the truth immanent within external interaction.

Key points: (KeyPoint)

- k1. Communication universalizes determinateness.
- k2. Reaction particularizes and resists.
- k3. The product gives center and law.
- k4. Mechanism thereby points beyond itself.

Claims: (Claim)

- c1. id: hegel-object-mechanical-process-c1
  - subject: mechanical_process
  - predicate: unfolds_through
  - object: communication_reaction_and_product
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `mechanism/process.txt:1-208`

- c2. id: hegel-object-mechanical-process-c2
  - subject: product_of_mechanism
  - predicate: yields
  - object: center_and_law_as_its_truth
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `mechanism/process.txt:209-455`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-object-chemism
  - targetWorkbook: `CHEMISM-WORKBOOK.md`
  - note: center and law prepare the negation of sheer external indifference.
  - sourceClaimIds: [`hegel-object-mechanical-process-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-object-chemism-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the end of mechanism on its immanent truth rather than only on external motion.
