# Absolute Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `absolute/`, not a replacement for the local Part A/B/C workbooks.
- Read it to follow the chapter-level spine from exposition, through attribute, into mode.
- Use the local `absolute/ABSOLUTE-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the absolute is no longer externally characterized but allowed to show itself as exposition, attribute, and mode.
- Second question: what is the chapter's central operator?
  Answer: self-exposition.
- Third question: where does the subfolder lead?
  Answer: to actuality proper.

## Authority + format lock (must persist)

- Working extraction references: `absolute/absolute-idea.txt`, `absolute/exposition.txt`, `absolute/attribute.txt`, `absolute/mode.txt`, and `ABSOLUTE-DISTILLATION.md`
- Upstream source authority: `absolute/absolute-idea.txt`, `absolute/exposition.txt`, `absolute/attribute.txt`, `absolute/mode.txt`
- This workbook covers the `absolute/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-05 (Absolute-folder pass)

Scope:

- files:
  - `absolute/absolute-idea.txt`
  - `absolute/exposition.txt`
  - `absolute/attribute.txt`
  - `absolute/mode.txt`
  - `ABSOLUTE-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `absolute/` subfolder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: exposition, attribute, mode.

### Entry hegel-actuality-absolute — Marker `Absolute`

- sourceFiles:
  - `absolute/absolute-idea.txt`
  - `absolute/exposition.txt`
  - `absolute/attribute.txt`
  - `absolute/mode.txt`
- lineSpans:
  - `absolute/absolute-idea.txt:1-36`
  - `absolute/exposition.txt:1-182`
  - `absolute/attribute.txt:1-97`
  - `absolute/mode.txt:1-115`
- summary: The `absolute/` subfolder presents the absolute as self-exposition, unfolding through exposition, attribute, and mode until actuality is reached.

Key points: (KeyPoint)

- k1. The absolute chapter is governed by exposition.
- k2. It unfolds through exposition, attribute, and mode.
- k3. Mode culminates in actuality.

Claims: (Claim)

- c1. id: hegel-actuality-absolute-c1
  - subject: absolute_chapter
  - predicate: unfolds_through
  - object: exposition_attribute_and_mode
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `absolute/absolute-idea.txt:1-36`
    - `absolute/exposition.txt:1-182`
    - `absolute/attribute.txt:1-97`
    - `absolute/mode.txt:1-115`

- c2. id: hegel-actuality-absolute-c2
  - subject: absolute_chapter
  - predicate: culminates_in
  - object: actuality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `absolute/mode.txt:106-115`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-actuality-exposition
  - targetWorkbook: `ABSOLUTE-WORKBOOK.md`
  - note: the chapter begins from exposition.
  - sourceClaimIds: [`hegel-actuality-absolute-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-actuality-exposition-c1`, `hegel-actuality-exposition-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-actuality-attribute
  - targetWorkbook: `ABSOLUTE-WORKBOOK.md`
  - note: exposition yields the relative absolute as attribute.
  - sourceClaimIds: [`hegel-actuality-absolute-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-actuality-attribute-c1`, `hegel-actuality-attribute-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-actuality-mode-threshold
  - targetWorkbook: `ABSOLUTE-WORKBOOK.md`
  - note: mode is the actuality-threshold of the chapter.
  - sourceClaimIds: [`hegel-actuality-absolute-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-mode-threshold-c1`, `hegel-actuality-mode-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the absolute chapter as movement rather than as a static summary topic.

### Entry hegel-actuality-exposition — `Absolute`: exposition of the absolute

- sourceFiles:
  - `absolute/exposition.txt`
- lineSpans:
  - `absolute/exposition.txt:1-182`
- summary: Exposition rejects the externally predicated absolute and shows the absolute as the unity in which being, essence, and relation have foundered and from which their truth shines forth.

Key points: (KeyPoint)

- k1. External reflection cannot determine the absolute truly.
- k2. The absolute is identity of form and content.
- k3. Exposition belongs to the absolute itself.

Claims: (Claim)

- c1. id: hegel-actuality-exposition-c1
  - subject: exposition_of_the_absolute
  - predicate: rejects
  - object: external_predication
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `absolute/exposition.txt:1-54`
    - `absolute/exposition.txt:55-120`

- c2. id: hegel-actuality-exposition-c2
  - subject: absolute
  - predicate: is_shown_as
  - object: self_exposition_of_form_and_content
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `absolute/exposition.txt:121-182`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-attribute
  - targetWorkbook: `ABSOLUTE-WORKBOOK.md`
  - note: the first determinate shape of self-exposition is attribute.
  - sourceClaimIds: [`hegel-actuality-exposition-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`hegel-actuality-attribute-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the negative and positive expository sides together.

### Entry hegel-actuality-attribute — `Absolute`: the relative absolute

- sourceFiles:
  - `absolute/attribute.txt`
- lineSpans:
  - `absolute/attribute.txt:1-97`
- summary: Attribute is the whole content of the absolute in a still relative form, where determinateness is present but remains only reflective shine or way and manner.

Key points: (KeyPoint)

- k1. Attribute is the relative absolute.
- k2. It carries the whole content of the absolute.
- k3. Its form is still merely formal determinateness.

Claims: (Claim)

- c1. id: hegel-actuality-attribute-c1
  - subject: attribute
  - predicate: is
  - object: relative_absolute
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `absolute/attribute.txt:1-39`

- c2. id: hegel-actuality-attribute-c2
  - subject: attribute
  - predicate: remains
  - object: reflective_shine_or_way_and_manner
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `absolute/attribute.txt:40-97`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-mode-threshold
  - targetWorkbook: `ABSOLUTE-WORKBOOK.md`
  - note: attribute passes into mode as the absolute's self-externality.
  - sourceClaimIds: [`hegel-actuality-attribute-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-mode-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps attribute from hardening into a Spinozist endpoint.

### Entry hegel-actuality-mode-threshold — `Absolute`: mode and the turn to actuality

- sourceFiles:
  - `absolute/mode.txt`
- lineSpans:
  - `absolute/mode.txt:1-115`
- summary: Mode is the absolute's externality posited as externality, reflective shine as shine, and thus the absolute's own self-manifestation; in that movement the chapter culminates in actuality.

Key points: (KeyPoint)

- k1. Mode first appears as externality and contingency.
- k2. Mode is reflective shine as reflective shine.
- k3. The absolute manifests itself for itself in mode.
- k4. Thus the absolute is actuality.

Claims: (Claim)

- c1. id: hegel-actuality-mode-threshold-c1
  - subject: mode
  - predicate: is
  - object: self_externality_of_the_absolute_as_reflective_shine
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `absolute/mode.txt:1-54`
    - `absolute/mode.txt:55-97`

- c2. id: hegel-actuality-mode-threshold-c2
  - subject: absolute
  - predicate: is_as_mode
  - object: actuality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `absolute/mode.txt:98-115`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-proper
  - targetWorkbook: `ACTUALITY-PROPER-WORKBOOK.md`
  - note: the next architectonic field is actuality proper.
  - sourceClaimIds: [`hegel-actuality-mode-threshold-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-actuality-proper-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the mode chapter from remaining a mere appendix to attribute.
