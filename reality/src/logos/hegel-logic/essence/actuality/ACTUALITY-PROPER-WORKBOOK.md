# Actuality Proper Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for the child `actuality/` chapter, named `ACTUALITY-PROPER-*` to avoid collision with the root container files.
- Read it to follow the chapter-level spine from contingency, through relative necessity, into absolute necessity.
- Use the local `actuality/ACTUALITY-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which actuality proper differentiates itself as possibility, actuality, and necessity and culminates in substance.
- Second question: what is the chapter's central operator?
  Answer: modality turning into necessity.
- Third question: where does the subfolder lead?
  Answer: to substance.

## Authority + format lock (must persist)

- Working extraction references: `actuality/actuality-idea.txt`, `actuality/contingency.txt`, `actuality/relative-necessity.txt`, `actuality/absolute-necessity.txt`, and `ACTUALITY-PROPER-DISTILLATION.md`
- Upstream source authority: `actuality/actuality-idea.txt`, `actuality/contingency.txt`, `actuality/relative-necessity.txt`, `actuality/absolute-necessity.txt`
- This workbook covers the child `actuality/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-05 (Actuality-proper folder pass)

Scope:

- files:
  - `actuality/actuality-idea.txt`
  - `actuality/contingency.txt`
  - `actuality/relative-necessity.txt`
  - `actuality/absolute-necessity.txt`
  - `ACTUALITY-PROPER-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the child `actuality/` subfolder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Use `ACTUALITY-PROPER-*` as the stable root-level filename for this chapter pair.
- Keep the emphasis on the chapter sequence: contingency, relative necessity, absolute necessity.

### Entry hegel-actuality-proper — Marker `Actuality Proper`

- sourceFiles:
  - `actuality/actuality-idea.txt`
  - `actuality/contingency.txt`
  - `actuality/relative-necessity.txt`
  - `actuality/absolute-necessity.txt`
- lineSpans:
  - `actuality/actuality-idea.txt:1-105`
  - `actuality/contingency.txt:1-243`
  - `actuality/relative-necessity.txt:1-242`
  - `actuality/absolute-necessity.txt:1-232`
- summary: The child `actuality/` subfolder presents actuality proper as the chapter of contingency, relative necessity, and absolute necessity, culminating in substance.

Key points: (KeyPoint)

- k1. Actuality proper differentiates itself through modality.
- k2. The chapter unfolds through contingency, relative necessity, and absolute necessity.
- k3. Absolute necessity yields substance.

Claims: (Claim)

- c1. id: hegel-actuality-proper-c1
  - subject: actuality_proper_chapter
  - predicate: unfolds_through
  - object: contingency_relative_necessity_and_absolute_necessity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `actuality/actuality-idea.txt:1-105`
    - `actuality/contingency.txt:1-243`
    - `actuality/relative-necessity.txt:1-242`
    - `actuality/absolute-necessity.txt:1-232`

- c2. id: hegel-actuality-proper-c2
  - subject: actuality_proper_chapter
  - predicate: culminates_in
  - object: substance
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/absolute-necessity.txt:223-232`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-actuality-contingency
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: the chapter begins from contingency.
  - sourceClaimIds: [`hegel-actuality-proper-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-actuality-contingency-c1`, `hegel-actuality-contingency-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-actuality-relative-necessity
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: formal necessity deepens into real or relative necessity.
  - sourceClaimIds: [`hegel-actuality-proper-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-actuality-relative-necessity-c1`, `hegel-actuality-relative-necessity-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-actuality-absolute-necessity-threshold
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: the chapter culminates in absolute necessity as the substance-threshold.
  - sourceClaimIds: [`hegel-actuality-proper-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-absolute-necessity-threshold-c1`, `hegel-actuality-absolute-necessity-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker stabilizes the file naming and the chapter movement at the same time.

### Entry hegel-actuality-contingency — `Actuality Proper`: formal actuality, possibility, necessity

- sourceFiles:
  - `actuality/contingency.txt`
- lineSpans:
  - `actuality/contingency.txt:1-243`
- summary: Contingency develops the formal relation of actuality, possibility, and necessity, showing actuality and possibility immediately converting into one another and thereby already containing necessity.

Key points: (KeyPoint)

- k1. The actual is formally possible.
- k2. Possibility is only possibility and points beyond itself.
- k3. Contingency is the immediate unity of actual and possible.
- k4. This restlessness already yields formal necessity.

Claims: (Claim)

- c1. id: hegel-actuality-contingency-c1
  - subject: contingency
  - predicate: is
  - object: immediate_conversion_of_actuality_and_possibility
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/contingency.txt:1-125`
    - `actuality/contingency.txt:126-210`

- c2. id: hegel-actuality-contingency-c2
  - subject: contingency
  - predicate: contains
  - object: formal_necessity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/contingency.txt:211-243`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-relative-necessity
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: formal necessity deepens into real or relative necessity.
  - sourceClaimIds: [`hegel-actuality-contingency-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-actuality-relative-necessity-c1`]

Review outcome:

- review_pending
- notes: this entry keeps contingency from being read as mere arbitrariness rather than formal self-conversion.

### Entry hegel-actuality-relative-necessity — `Actuality Proper`: real actuality and relative necessity

- sourceFiles:
  - `actuality/relative-necessity.txt`
- lineSpans:
  - `actuality/relative-necessity.txt:1-242`
- summary: Relative necessity gathers actuality, possibility, and necessity into a content-filled relation of circumstances and conditions, but remains relative because it still begins from presupposed actuality.

Key points: (KeyPoint)

- k1. Real actuality preserves itself in content.
- k2. Real possibility is the totality of conditions.
- k3. Relative necessity is content-filled but still presuppositional.

Claims: (Claim)

- c1. id: hegel-actuality-relative-necessity-c1
  - subject: real_actuality
  - predicate: is
  - object: contentful_self_preserving_actuality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/relative-necessity.txt:1-46`

- c2. id: hegel-actuality-relative-necessity-c2
  - subject: relative_necessity
  - predicate: is
  - object: necessity_beginning_from_presupposed_conditions
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `actuality/relative-necessity.txt:47-242`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-absolute-necessity-threshold
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: relative necessity reflects into absolute necessity.
  - sourceClaimIds: [`hegel-actuality-relative-necessity-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-absolute-necessity-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the chapter's middle movement tied to conditions and content rather than only to modality vocabulary.

### Entry hegel-actuality-absolute-necessity-threshold — `Actuality Proper`: absolute necessity into substance

- sourceFiles:
  - `actuality/absolute-necessity.txt`
- lineSpans:
  - `actuality/absolute-necessity.txt:1-232`
- summary: Absolute necessity is necessity that includes contingency within itself, self-grounds itself entirely, and in its self-exposition becomes substance.

Key points: (KeyPoint)

- k1. Absolute necessity is self-grounding.
- k2. Contingency is internal to it.
- k3. Its blind collapse is the absolute's own exposition.
- k4. Absolute necessity becomes substance.

Claims: (Claim)

- c1. id: hegel-actuality-absolute-necessity-threshold-c1
  - subject: absolute_necessity
  - predicate: is
  - object: unity_of_necessity_and_contingency
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `actuality/absolute-necessity.txt:1-118`
    - `actuality/absolute-necessity.txt:119-196`

- c2. id: hegel-actuality-absolute-necessity-threshold-c2
  - subject: absolute_necessity
  - predicate: becomes
  - object: substance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `actuality/absolute-necessity.txt:197-232`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-substance
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: the next architectonic field is substance.
  - sourceClaimIds: [`hegel-actuality-absolute-necessity-threshold-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-actuality-substance-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the chapter from terminating in blind necessity rather than passing into absolute relation.
