# Thing Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `thing/`, not a replacement for the local Part A/B/C workbooks.
- Read it to follow the chapter-level spine from thinghood, through matter, into dissolution.
- Use the local `thing/THING-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which concrete existence first appears as thinghood, externalizes itself in property and matter, and dissolves into appearance.
- Second question: what is the chapter's central operator?
  Answer: the failure of immediate thinghood to remain self-subsistent.
- Third question: where does the subfolder lead?
  Answer: to the world of appearance.

## Authority + format lock (must persist)

- Working extraction references: `thing/thing-idea.txt`, `thing/thing.txt`, `thing/matter.txt`, `thing/dissolution.txt`, and `THING-DISTILLATION.md`
- Upstream source authority: `thing/thing-idea.txt`, `thing/thing.txt`, `thing/matter.txt`, `thing/dissolution.txt`
- This workbook covers the `thing/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-04 (Thing-folder pass)

Scope:

- files:
  - `thing/thing-idea.txt`
  - `thing/thing.txt`
  - `thing/matter.txt`
  - `thing/dissolution.txt`
  - `THING-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `thing/` subfolder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: thing and properties, matter, dissolution.

### Entry hegel-appearance-thing — Marker `Thing`

- sourceFiles:
  - `thing/thing-idea.txt`
  - `thing/thing.txt`
  - `thing/matter.txt`
  - `thing/dissolution.txt`
- lineSpans:
  - `thing/thing-idea.txt:1-209`
  - `thing/thing.txt:211-642`
  - `thing/matter.txt:2-152`
  - `thing/dissolution.txt:2-111`
- summary: The `thing/` subfolder presents concrete existence first as thinghood, then as material constitution, and finally as dissolution into appearance.

Key points: (KeyPoint)

- k1. Concrete existence first determines itself as thing.
- k2. Thinghood is externalized in properties and matters.
- k3. Dissolution reveals appearance as the truth of the thing.

Claims: (Claim)

- c1. id: hegel-appearance-thing-c1
  - subject: thing_chapter
  - predicate: unfolds_through
  - object: thinghood_matter_and_dissolution
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `thing/thing.txt:211-238`
    - `thing/matter.txt:2-40`
    - `thing/dissolution.txt:2-20`

- c2. id: hegel-appearance-thing-c2
  - subject: thing_chapter
  - predicate: culminates_in
  - object: appearance_as_truth_of_concrete_existence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `thing/dissolution.txt:97-111`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-appearance-thing-property
  - targetWorkbook: `THING-WORKBOOK.md`
  - note: the chapter begins from thinghood, property, and reciprocal action.
  - sourceClaimIds: [`hegel-appearance-thing-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`hegel-appearance-thing-property-c1`, `hegel-appearance-thing-property-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-appearance-matter
  - targetWorkbook: `THING-WORKBOOK.md`
  - note: property passes over into matter.
  - sourceClaimIds: [`hegel-appearance-thing-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-appearance-matter-c1`, `hegel-appearance-matter-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-appearance-dissolution-threshold
  - targetWorkbook: `THING-WORKBOOK.md`
  - note: the chapter culminates in the dissolution of the thing.
  - sourceClaimIds: [`hegel-appearance-thing-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-dissolution-threshold-c1`, `hegel-appearance-dissolution-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the chapter-level path without redoing all local detail work.

### Entry hegel-appearance-thing-property — `Thing`: thinghood, property, reciprocal action

- sourceFiles:
  - `thing/thing.txt`
- lineSpans:
  - `thing/thing.txt:211-642`
- summary: The thing first divides into thing-in-itself and external concrete existence, then unfolds through property and reciprocal action, showing that thinghood is inseparable from external determination.

Key points: (KeyPoint)

- k1. The thing is concrete existence reflected into immediacy.
- k2. Thing-in-itself and external concrete existence first divide.
- k3. Property and reciprocal action reveal thinghood through external reference.

Claims: (Claim)

- c1. id: hegel-appearance-thing-property-c1
  - subject: thing
  - predicate: is
  - object: concrete_existent_reflected_immediacy
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `thing/thing.txt:211-238`
    - `thing/thing.txt:240-303`

- c2. id: hegel-appearance-thing-property-c2
  - subject: thinghood
  - predicate: externalizes_itself_in
  - object: property_and_reciprocal_action
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `thing/thing.txt:436-642`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-matter
  - targetWorkbook: `THING-WORKBOOK.md`
  - note: property hardens into self-subsistent matter.
  - sourceClaimIds: [`hegel-appearance-thing-property-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-matter-c1`]

Review outcome:

- review_pending
- notes: this entry compresses the whole Part A movement to its chapter function.

### Entry hegel-appearance-matter — `Thing`: constitution out of matters

- sourceFiles:
  - `thing/matter.txt`
- lineSpans:
  - `thing/matter.txt:2-152`
- summary: The property's self-subsistence turns into matter or stuff, and the thing becomes only the external `this` or `also` of indifferent materials.

Key points: (KeyPoint)

- k1. Property passes into self-subsistent matter.
- k2. The thing loses inward determinative power.
- k3. The thing as `this` becomes merely a quantitative collection.

Claims: (Claim)

- c1. id: hegel-appearance-matter-c1
  - subject: property
  - predicate: passes_into
  - object: self_subsistent_matter
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `thing/matter.txt:2-40`
    - `thing/matter.txt:78-114`

- c2. id: hegel-appearance-matter-c2
  - subject: thing_as_this
  - predicate: is_reduced_to
  - object: quantitative_collection_or_also
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `thing/matter.txt:114-152`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-dissolution-threshold
  - targetWorkbook: `THING-WORKBOOK.md`
  - note: once the thing is only a collection, its dissolution follows.
  - sourceClaimIds: [`hegel-appearance-matter-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-appearance-dissolution-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the material chapter from becoming only a chemistry excursus.

### Entry hegel-appearance-dissolution-threshold — `Thing`: dissolution into appearance

- sourceFiles:
  - `thing/dissolution.txt`
- lineSpans:
  - `thing/dissolution.txt:2-111`
- summary: The thing as merely quantitative collection is absolutely alterable and porous; its self-contradictory mediation reveals appearance as the truth of concrete existence.

Key points: (KeyPoint)

- k1. The collected thing is absolutely dissoluble.
- k2. The matters interpenetrate through porosity.
- k3. The thing is self-contradictory mediation.
- k4. The truth of the thing is appearance.

Claims: (Claim)

- c1. id: hegel-appearance-dissolution-threshold-c1
  - subject: thing
  - predicate: is
  - object: absolutely_dissoluble_and_porous
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `thing/dissolution.txt:2-47`
    - `thing/dissolution.txt:48-89`

- c2. id: hegel-appearance-dissolution-threshold-c2
  - subject: truth_of_thinghood
  - predicate: is
  - object: appearance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `thing/dissolution.txt:91-111`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-appearance-world
  - targetWorkbook: `WORLD-WORKBOOK.md`
  - note: the next architectonic field is the world of appearance.
  - sourceClaimIds: [`hegel-appearance-dissolution-threshold-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-appearance-world-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps thinghood from appearing as a self-standing ontology when it is really the first undoing of concrete existence.
