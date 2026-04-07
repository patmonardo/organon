# Actuality Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic container workbook for the whole `actuality/` folder.
- Read it to follow the folder-level architectonic from `absolute/`, through `actuality/`, into `substance/`.
- Its task is to preserve the high-level spine of the actuality band within the Doctrine of Essence.
- Use the chapter-level files [ABSOLUTE-WORKBOOK.md](ABSOLUTE-WORKBOOK.md), [ACTUALITY-PROPER-WORKBOOK.md](ACTUALITY-PROPER-WORKBOOK.md), and [SUBSTANCE-WORKBOOK.md](SUBSTANCE-WORKBOOK.md) for the detailed readable walk-through.

## Quick orientation

- First question: what is being fixed here?
  Answer: the folder-level sequence in which the absolute first manifests itself, then differentiates itself as actuality-possibility-necessity, and finally becomes substance, causality, and reciprocity.
- Second question: what is the central operator of the folder as a whole?
  Answer: actuality as the achieved identity of essence and existence.
- Third question: where does the folder lead?
  Answer: out of essence and into the concept.

## Authority + format lock (must persist)

- Working extraction references: `actuality-idea.txt`, `absolute/absolute-idea.txt`, `actuality/actuality-idea.txt`, `substance/substance-idea.txt`, and `ACTUALITY-DISTILLATION.md`
- Upstream source authority: `actuality-idea.txt`, `absolute/absolute-idea.txt`, `actuality/actuality-idea.txt`, `substance/substance-idea.txt`
- This workbook covers the whole `actuality/` folder as a container surface.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not collapse the folder-level container into one of its child chapter-clusters.
- Do not duplicate the chapter-level workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-05 (actuality-folder container pass)

Scope:

- files:
  - `actuality-idea.txt`
  - `absolute/absolute-idea.txt`
  - `actuality/actuality-idea.txt`
  - `substance/substance-idea.txt`
  - `ACTUALITY-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the whole `actuality/` folder.
- Give each major child folder its own chapter-level artifact pair.
- Keep the emphasis on the folder sequence: `absolute/`, `actuality/`, `substance/`.
- Because the folder and its middle child share the same name, use `ACTUALITY-PROPER-*` for the child chapter pair.

### Entry hegel-actuality-folder — Marker `Actuality`

- sourceFiles:
  - `actuality-idea.txt`
  - `absolute/absolute-idea.txt`
  - `actuality/actuality-idea.txt`
  - `substance/substance-idea.txt`
- lineSpans:
  - `actuality-idea.txt:1-53`
  - `absolute/absolute-idea.txt:1-36`
  - `actuality/actuality-idea.txt:1-105`
  - `substance/substance-idea.txt:1-64`
- summary: The `actuality/` folder unfolds as one high-level movement from the absolute's self-exposition, through actuality-possibility-necessity, into substance, causality, and reciprocity.

Key points: (KeyPoint)

- k1. Actuality is the unity of essence and concrete existence.
- k2. The section begins with the chapter of the absolute.
- k3. It deepens through actuality proper as modality and necessity.
- k4. It culminates in substance and opens the concept.

Claims: (Claim)

- c1. id: hegel-actuality-folder-c1
  - subject: actuality_folder
  - predicate: unfolds_through
  - object: absolute_actuality_proper_and_substance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `actuality-idea.txt:1-53`

- c2. id: hegel-actuality-folder-c2
  - subject: actuality_folder
  - predicate: mediates
  - object: from_essence_completed_to_concept_threshold
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `actuality-idea.txt:1-53`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-actuality-absolute-folder
  - targetWorkbook: `ACTUALITY-WORKBOOK.md`
  - note: the folder begins with the absolute as self-exposition.
  - sourceClaimIds: [`hegel-actuality-folder-c1`, `hegel-actuality-folder-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-actuality-absolute-folder-c1`, `hegel-actuality-absolute-folder-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-actuality-proper-folder
  - targetWorkbook: `ACTUALITY-WORKBOOK.md`
  - note: the middle chapter develops actuality, possibility, and necessity.
  - sourceClaimIds: [`hegel-actuality-folder-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-proper-folder-c1`, `hegel-actuality-proper-folder-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-actuality-substance-folder
  - targetWorkbook: `ACTUALITY-WORKBOOK.md`
  - note: the folder culminates in absolute relation as substance.
  - sourceClaimIds: [`hegel-actuality-folder-c1`, `hegel-actuality-folder-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-actuality-substance-folder-c1`, `hegel-actuality-substance-folder-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the top-level `actuality/` surface as container artifact and resolves the naming collision explicitly.

### Entry hegel-actuality-absolute-folder — `Actuality`: the `absolute/` opening field

- sourceFiles:
  - `absolute/absolute-idea.txt`
  - `absolute/mode.txt`
- lineSpans:
  - `absolute/absolute-idea.txt:1-36`
  - `absolute/mode.txt:1-115`
- summary: The `absolute/` subfolder presents the absolute as self-exposition and culminates in actuality as the absolute's manifestation for itself.

Key points: (KeyPoint)

- k1. The absolute chapter rejects external predication.
- k2. It unfolds through exposition, attribute, and mode.
- k3. Mode culminates in actuality.

Claims: (Claim)

- c1. id: hegel-actuality-absolute-folder-c1
  - subject: absolute_subfolder
  - predicate: captures
  - object: absolute_as_self_exposition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `absolute/absolute-idea.txt:1-36`

- c2. id: hegel-actuality-absolute-folder-c2
  - subject: absolute_subfolder
  - predicate: turns_toward
  - object: actuality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `absolute/mode.txt:106-115`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-proper-folder
  - targetWorkbook: `ACTUALITY-WORKBOOK.md`
  - note: the mode of the absolute yields actuality proper.
  - sourceClaimIds: [`hegel-actuality-absolute-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-proper-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-actuality-absolute
  - targetWorkbook: `ABSOLUTE-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Absolute workbook.
  - sourceClaimIds: [`hegel-actuality-absolute-folder-c1`, `hegel-actuality-absolute-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-actuality-absolute-c1`, `hegel-actuality-absolute-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the absolute chapter tied to its movement into actuality rather than treating it as static theology.

### Entry hegel-actuality-proper-folder — `Actuality`: the `actuality/` middle field

- sourceFiles:
  - `actuality/actuality-idea.txt`
  - `actuality/contingency.txt`
  - `actuality/absolute-necessity.txt`
- lineSpans:
  - `actuality/actuality-idea.txt:1-105`
  - `actuality/contingency.txt:1-243`
  - `actuality/absolute-necessity.txt:1-232`
- summary: The `actuality/` subfolder develops actuality proper through contingency, relative necessity, and absolute necessity until substance emerges as the next determination.

Key points: (KeyPoint)

- k1. The middle chapter develops actuality, possibility, and necessity.
- k2. It moves from formal to real to absolute necessity.
- k3. Absolute necessity turns into substance.

Claims: (Claim)

- c1. id: hegel-actuality-proper-folder-c1
  - subject: actuality_proper_subfolder
  - predicate: captures
  - object: modality_and_necessity_of_actuality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/actuality-idea.txt:1-105`

- c2. id: hegel-actuality-proper-folder-c2
  - subject: actuality_proper_subfolder
  - predicate: turns_toward
  - object: substance
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/absolute-necessity.txt:223-232`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-substance-folder
  - targetWorkbook: `ACTUALITY-WORKBOOK.md`
  - note: absolute necessity yields substance as the next determination.
  - sourceClaimIds: [`hegel-actuality-proper-folder-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-substance-folder-c1`]

- r2. type: unfolds_to
  - targetEntryId: hegel-actuality-proper
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Actuality Proper workbook.
  - sourceClaimIds: [`hegel-actuality-proper-folder-c1`, `hegel-actuality-proper-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-actuality-proper-c1`, `hegel-actuality-proper-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the naming clear by mapping the child `actuality/` folder to `ACTUALITY-PROPER-*`.

### Entry hegel-actuality-substance-folder — `Actuality`: the `substance/` consummating field

- sourceFiles:
  - `substance/substance-idea.txt`
  - `substance/relation-substantiality.txt`
  - `substance/reciprocity-action.txt`
- lineSpans:
  - `substance/substance-idea.txt:1-64`
  - `substance/relation-substantiality.txt:2-180`
  - `substance/reciprocity-action.txt:2-164`
- summary: The `substance/` subfolder is the chapter of absolute relation, unfolding through substantiality, causality, and reciprocity until the concept emerges.

Key points: (KeyPoint)

- k1. Absolute relation is the truth of necessity.
- k2. Substance unfolds through substantiality, causality, and reciprocity.
- k3. Reciprocity opens the concept.

Claims: (Claim)

- c1. id: hegel-actuality-substance-folder-c1
  - subject: substance_subfolder
  - predicate: captures
  - object: absolute_relation_as_substance_causality_and_reciprocity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `substance/substance-idea.txt:1-64`

- c2. id: hegel-actuality-substance-folder-c2
  - subject: substance_subfolder
  - predicate: culminates_in
  - object: concept_threshold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/reciprocity-action.txt:133-164`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-actuality-substance
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: the detailed readable surface for this subfolder is the Substance workbook.
  - sourceClaimIds: [`hegel-actuality-substance-folder-c1`, `hegel-actuality-substance-folder-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-actuality-substance-c1`, `hegel-actuality-substance-c2`, `hegel-actuality-substance-c3`]

Review outcome:

- review_pending
- notes: this entry fixes the final actuality chapter-cluster as the completion of essence.
