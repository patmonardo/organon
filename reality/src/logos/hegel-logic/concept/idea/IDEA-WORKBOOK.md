# Idea Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `idea/` folder.
- Read it to follow the folder-level architectonic from `life/`, through `cognition/`, into the `speculation/` folder's absolute-idea stack.
- Its task is to preserve the high-level spine of the Idea band within the Doctrine of the Concept.
- Use the chapter-level files [LIFE-WORKBOOK.md](LIFE-WORKBOOK.md), [COGNITION-WORKBOOK.md](COGNITION-WORKBOOK.md), and [ABSOLUTE-IDEA-WORKBOOK.md](ABSOLUTE-IDEA-WORKBOOK.md) for the detailed readable walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the folder-level sequence in which the adequate concept first appears as life, then doubles itself as cognition, and finally gathers itself as absolute idea.
- Second question: what is the central operator of the folder as a whole?
  Answer: the progressive liberation of concept and objectivity into explicit self-knowing unity.
- Third question: where does the folder lead?
  Answer: into nature.

## Authority + format lock (must persist)

- Working extraction references: `idea-idea.txt`, `life/life-idea.txt`, `cognition/cognition-idea.txt`, `speculation/absolute-idea.txt`, `speculation/method-advance.txt`, and `IDEA-DISTILLATION.md`
- Upstream source authority: `idea-idea.txt`, `life/life-idea.txt`, `cognition/cognition-idea.txt`, `speculation/absolute-idea.txt`, `speculation/method-advance.txt`
- This workbook covers the whole `idea/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not collapse the folder-level container into one of its child chapter-clusters.
- Do not duplicate the chapter-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (idea-folder container pass)

Scope:

- files:
  - `idea-idea.txt`
  - `life/life-idea.txt`
  - `cognition/cognition-idea.txt`
  - `speculation/absolute-idea.txt`
  - `speculation/method-advance.txt`
  - `IDEA-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `idea/` folder.
- Give each major child chapter its own readable artifact pair at the folder root.
- Keep the emphasis on the folder sequence: `life/`, `cognition/`, and the `speculation/` folder as the local stack for the Absolute Idea.
- Preserve the older local coordination and part workbooks as denser legacy support rather than replacing them.

### Entry hegel-idea-folder — Marker `Idea`

- sourceFiles:
  - `idea-idea.txt`
  - `life/life-idea.txt`
  - `cognition/cognition-idea.txt`
  - `speculation/absolute-idea.txt`
  - `speculation/method-advance.txt`
- lineSpans:
  - `idea-idea.txt:1-371`
  - `life/life-idea.txt:1-221`
  - `cognition/cognition-idea.txt:1-531`
  - `speculation/absolute-idea.txt:1-296`
  - `speculation/method-advance.txt:1-735`
- summary: The `idea/` folder unfolds as one high-level movement from life as immediate Idea, through cognition as the Idea in judgment, to the Absolute Idea and its method, ending in the Idea's free release into nature.

Key points: (KeyPoint)

- k1. The Idea is the adequate concept.
- k2. Life is the first field of the Idea.
- k3. Cognition doubles the Idea into subjective and objective sides.
- k4. Absolute Idea reconciles truth and good.
- k5. The Idea culminates in the transition to nature.

Claims: (Claim)

- c1. id: hegel-idea-folder-c1
  - subject: idea_folder
  - predicate: unfolds_through
  - object: life_cognition_and_absolute_idea
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `idea-idea.txt:1-371`

- c2. id: hegel-idea-folder-c2
  - subject: idea_folder
  - predicate: mediates
  - object: from_adequate_concept_to_nature_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `idea-idea.txt:261-371`
    - `speculation/method-advance.txt:1-735`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-idea-life-folder
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: the folder begins with the Idea in immediate life.
  - sourceClaimIds: [`hegel-idea-folder-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-idea-life-folder-c1`, `hegel-idea-life-folder-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-idea-cognition-folder
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: cognition is the Idea in judgment and doubling.
  - sourceClaimIds: [`hegel-idea-folder-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-cognition-folder-c1`, `hegel-idea-cognition-folder-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-idea-absolute-folder
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: the local `speculation/` stack carries the Absolute Idea and method.
  - sourceClaimIds: [`hegel-idea-folder-c1`, `hegel-idea-folder-c2`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`hegel-idea-absolute-folder-c1`, `hegel-idea-absolute-folder-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the top-level `idea/` surface as a true container artifact.

### Entry hegel-idea-life-folder — `Idea`: the `life/` opening field

- sourceFiles:
  - `life/life-idea.txt`
- lineSpans:
  - `life/life-idea.txt:1-221`
- summary: The `life/` subfolder gives the Idea first as immediate unity of concept and objectivity in the living individual, life-process, and genus.

Key points: (KeyPoint)

- k1. Life is the immediate Idea.
- k2. Life unfolds through individual, process, and genus.
- k3. Life turns toward cognition.

Claims: (Claim)

- c1. id: hegel-idea-life-folder-c1
  - subject: life_subfolder
  - predicate: captures
  - object: immediate_idea_of_concept_and_objectivity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `life/life-idea.txt:1-221`

- c2. id: hegel-idea-life-folder-c2
  - subject: life_subfolder
  - predicate: turns_toward
  - object: cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `life/life-idea.txt:209-221`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-cognition-folder
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: genus withdraws into the concept that exists for itself as cognition.
  - sourceClaimIds: [`hegel-idea-life-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-cognition-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-idea-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Life workbook.
  - sourceClaimIds: [`hegel-idea-life-folder-c1`, `hegel-idea-life-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-idea-life-c1`, `hegel-idea-life-c2`, `hegel-idea-life-c3`]

Review outcome:

- review_pending
- notes: this entry keeps Life at chapter scale above the local triadic stack.

### Entry hegel-idea-cognition-folder — `Idea`: the `cognition/` field of judgment and reconciliation

- sourceFiles:
  - `cognition/cognition-idea.txt`
- lineSpans:
  - `cognition/cognition-idea.txt:1-531`
- summary: The `cognition/` subfolder gives the Idea in judgment, doubled into subjective and objective concept, and unfolds through the idea of the true and the idea of the good.

Key points: (KeyPoint)

- k1. Cognition is the Idea in judgment.
- k2. The chapter unfolds through truth and good.
- k3. Cognition turns toward the Absolute Idea.

Claims: (Claim)

- c1. id: hegel-idea-cognition-folder-c1
  - subject: cognition_subfolder
  - predicate: captures
  - object: idea_as_judgmental_doubling
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition/cognition-idea.txt:1-41`

- c2. id: hegel-idea-cognition-folder-c2
  - subject: cognition_subfolder
  - predicate: turns_toward
  - object: absolute_idea
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition/cognition-idea.txt:1-531`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-absolute-folder
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: the practical Idea passes into the Absolute Idea.
  - sourceClaimIds: [`hegel-idea-cognition-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-absolute-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-idea-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Cognition workbook.
  - sourceClaimIds: [`hegel-idea-cognition-folder-c1`, `hegel-idea-cognition-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-idea-cognition-c1`, `hegel-idea-cognition-c2`, `hegel-idea-cognition-c3`]

Review outcome:

- review_pending
- notes: this entry keeps Cognition at chapter scale over the denser true/good files.

### Entry hegel-idea-absolute-folder — `Idea`: the `speculation/` stack as Absolute Idea

- sourceFiles:
  - `speculation/absolute-idea.txt`
  - `speculation/method-advance.txt`
- lineSpans:
  - `speculation/absolute-idea.txt:1-296`
  - `speculation/method-advance.txt:1-735`
- summary: The `speculation/` folder carries the third chapter as Absolute Idea, where theoretical and practical Idea are unified and method is grasped as the concept's own self-movement.

Key points: (KeyPoint)

- k1. The Absolute Idea unifies the theoretical and practical Idea.
- k2. Method is the self-movement of the concept.
- k3. The Absolute Idea freely releases itself into nature.

Claims: (Claim)

- c1. id: hegel-idea-absolute-folder-c1
  - subject: absolute_idea_stack
  - predicate: captures
  - object: unity_of_theoretical_and_practical_idea
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `speculation/absolute-idea.txt:1-96`

- c2. id: hegel-idea-absolute-folder-c2
  - subject: absolute_idea_stack
  - predicate: culminates_in
  - object: transition_to_nature
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `speculation/absolute-idea.txt:240-296`
    - `speculation/method-advance.txt:1-735`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-idea-absolute
  - targetWorkbook: `ABSOLUTE-IDEA-WORKBOOK.md`
  - note: the detailed readable surface for this chapter is the Absolute Idea workbook.
  - sourceClaimIds: [`hegel-idea-absolute-folder-c1`, `hegel-idea-absolute-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-idea-absolute-c1`, `hegel-idea-absolute-c2`, `hegel-idea-absolute-c3`]

Review outcome:

- review_pending
- notes: this entry preserves the folder name `speculation/` while exposing the doctrinal chapter name Absolute Idea at the readable root level.
