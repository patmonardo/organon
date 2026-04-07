# Subjectivity Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `subject/` folder.
- Read it to follow the folder-level architectonic from `concept/`, through `judgment/`, into `syllogism/`.
- Its task is to preserve the high-level spine of the subjectivity band within the Doctrine of the Concept.
- Use the chapter-level files [CONCEPT-WORKBOOK.md](CONCEPT-WORKBOOK.md), [JUDGMENT-WORKBOOK.md](JUDGMENT-WORKBOOK.md), and [SYLLOGISM-WORKBOOK.md](SYLLOGISM-WORKBOOK.md) for the detailed readable walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the folder-level sequence in which the concept first articulates itself inwardly, then divides itself as judgment, and finally restores itself as syllogism.
- Second question: what is the central operator of the folder as a whole?
  Answer: the concept's movement from immediate inward unity to explicit mediation.
- Third question: where does the folder lead?
  Answer: out of subjectivity and into objectivity.

## Authority + format lock (must persist)

- Working extraction references: `subject-idea.txt`, `concept/concept-idea.txt`, `judgment/judgment-idea.txt`, `syllogism/syllogism-idea.txt`, and `SUBJECTIVITY-DISTILLATION.md`
- Upstream source authority: `subject-idea.txt`, `concept/concept-idea.txt`, `judgment/judgment-idea.txt`, `syllogism/syllogism-idea.txt`
- This workbook covers the whole `subject/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not collapse the folder-level container into one of its child chapter-clusters.
- Do not duplicate the chapter-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (subjectivity-folder container pass)

Scope:

- files:
  - `subject-idea.txt`
  - `concept/concept-idea.txt`
  - `judgment/judgment-idea.txt`
  - `syllogism/syllogism-idea.txt`
  - `SUBJECTIVITY-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `subject/` folder.
- Give each major child folder its own chapter-level artifact pair at the folder root.
- Keep the emphasis on the folder sequence: `concept/`, `judgment/`, `syllogism/`.
- Preserve the older `SUBJECT-IDEA-WORKBOOK.md` as a denser legacy support rather than replacing it.

### Entry hegel-subjectivity-folder — Marker `Subjectivity`

- sourceFiles:
  - `subject-idea.txt`
  - `concept/concept-idea.txt`
  - `judgment/judgment-idea.txt`
  - `syllogism/syllogism-idea.txt`
- lineSpans:
  - `subject-idea.txt:1-53`
  - `concept/concept-idea.txt:1-70`
  - `judgment/judgment-idea.txt:1-451`
  - `syllogism/syllogism-idea.txt:1-151`
- summary: The `subject/` folder unfolds as one high-level movement from the concept as such, through judgment, into syllogism as the completed subjective concept.

Key points: (KeyPoint)

- k1. Subjectivity is the concept in its first formal band.
- k2. The subjectivity band first articulates itself as concept.
- k3. It then divides itself as judgment.
- k4. It culminates in syllogism and passes into objectivity.

Claims: (Claim)

- c1. id: hegel-subjectivity-folder-c1
  - subject: subjectivity_folder
  - predicate: unfolds_through
  - object: concept_judgment_and_syllogism
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `subject-idea.txt:1-53`

- c2. id: hegel-subjectivity-folder-c2
  - subject: subjectivity_folder
  - predicate: mediates
  - object: from_immediate_concept_to_objectivity_threshold
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `subject-idea.txt:1-53`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-subjectivity-concept-folder
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: the folder begins with the concept as such.
  - sourceClaimIds: [`hegel-subjectivity-folder-c1`, `hegel-subjectivity-folder-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-subjectivity-concept-folder-c1`, `hegel-subjectivity-concept-folder-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-subjectivity-judgment-folder
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: the middle field is judgment as the concept's self-division.
  - sourceClaimIds: [`hegel-subjectivity-folder-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subjectivity-judgment-folder-c1`, `hegel-subjectivity-judgment-folder-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-subjectivity-syllogism-folder
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: the folder culminates in syllogism as explicit mediation.
  - sourceClaimIds: [`hegel-subjectivity-folder-c1`, `hegel-subjectivity-folder-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-subjectivity-syllogism-folder-c1`, `hegel-subjectivity-syllogism-folder-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the top-level `subject/` surface as a true container artifact.

### Entry hegel-subjectivity-concept-folder — `Subjectivity`: the `concept/` opening field

- sourceFiles:
  - `concept/concept-idea.txt`
- lineSpans:
  - `concept/concept-idea.txt:1-70`
- summary: The `concept/` subfolder gives the concept first as universality, particularity, and singularity, and turns that immediate self-articulation toward judgment.

Key points: (KeyPoint)

- k1. The concept first articulates itself as universality, particularity, and singularity.
- k2. Difference is internal to the concept.
- k3. Singularity turns the concept into judgment.

Claims: (Claim)

- c1. id: hegel-subjectivity-concept-folder-c1
  - subject: concept_subfolder
  - predicate: captures
  - object: first_self_articulation_of_the_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `concept/concept-idea.txt:1-64`

- c2. id: hegel-subjectivity-concept-folder-c2
  - subject: concept_subfolder
  - predicate: turns_toward
  - object: judgment
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `concept/concept-idea.txt:66-70`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subjectivity-judgment-folder
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: singularity turns the concept into judgment.
  - sourceClaimIds: [`hegel-subjectivity-concept-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subjectivity-judgment-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-subject-concept
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Concept workbook.
  - sourceClaimIds: [`hegel-subjectivity-concept-folder-c1`, `hegel-subjectivity-concept-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-subject-concept-c1`, `hegel-subject-concept-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the immediate concept chapter tied to its judgment threshold.

### Entry hegel-subjectivity-judgment-folder — `Subjectivity`: the `judgment/` field of diremption

- sourceFiles:
  - `judgment/judgment-idea.txt`
- lineSpans:
  - `judgment/judgment-idea.txt:1-451`
- summary: The `judgment/` subfolder gives the concept as self-division into subject and predicate, and drives that unstable unity toward syllogistic mediation.

Key points: (KeyPoint)

- k1. Judgment is the concept's self-diremption.
- k2. Subject and predicate stand as self-subsisting sides.
- k3. The contradiction of the copula drives judgment toward syllogism.

Claims: (Claim)

- c1. id: hegel-subjectivity-judgment-folder-c1
  - subject: judgment_subfolder
  - predicate: captures
  - object: concept_as_self_division
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:1-178`

- c2. id: hegel-subjectivity-judgment-folder-c2
  - subject: judgment_subfolder
  - predicate: turns_toward
  - object: syllogism
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:229-451`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subjectivity-syllogism-folder
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: the contradiction of judgment drives it into syllogism.
  - sourceClaimIds: [`hegel-subjectivity-judgment-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subjectivity-syllogism-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-subject-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Judgment workbook.
  - sourceClaimIds: [`hegel-subjectivity-judgment-folder-c1`, `hegel-subjectivity-judgment-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-subject-judgment-c1`, `hegel-subject-judgment-c2`, `hegel-subject-judgment-c3`]

Review outcome:

- review_pending
- notes: this entry keeps judgment at chapter-cluster scale rather than enumerating all four local judgment bands.

### Entry hegel-subjectivity-syllogism-folder — `Subjectivity`: the `syllogism/` field of restored mediation

- sourceFiles:
  - `syllogism/syllogism-idea.txt`
- lineSpans:
  - `syllogism/syllogism-idea.txt:1-151`
- summary: The `syllogism/` subfolder restores the unity broken in judgment through explicit mediation and culminates in the passage into objectivity.

Key points: (KeyPoint)

- k1. Syllogism restores the concept through mediation.
- k2. It unfolds through existence, reflection, and necessity.
- k3. It passes into objectivity.

Claims: (Claim)

- c1. id: hegel-subjectivity-syllogism-folder-c1
  - subject: syllogism_subfolder
  - predicate: captures
  - object: concept_restored_as_explicit_mediation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism/syllogism-idea.txt:1-113`

- c2. id: hegel-subjectivity-syllogism-folder-c2
  - subject: syllogism_subfolder
  - predicate: culminates_in
  - object: objectivity_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism/syllogism-idea.txt:115-151`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-subject-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Syllogism workbook.
  - sourceClaimIds: [`hegel-subjectivity-syllogism-folder-c1`, `hegel-subjectivity-syllogism-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-subject-syllogism-c1`, `hegel-subject-syllogism-c2`, `hegel-subject-syllogism-c3`]

Review outcome:

- review_pending
- notes: this entry fixes the end of subjectivity as the objectivity-threshold.
