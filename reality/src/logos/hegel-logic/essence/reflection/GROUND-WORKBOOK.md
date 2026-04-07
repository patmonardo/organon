# Ground Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic chapter-level workbook, not a replacement for the local Part A/B/C workbooks under `ground/`.
- Read it to follow the Ground chapter as one architectonic movement.
- Its task is to preserve the chapter-level spine from absolute ground, through determinate ground and condition, into concrete existence.
- Use the local `ground/GROUND-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which essence becomes ground, unfolds the relation of ground and grounded through form and content, externalizes itself in determinate ground, and then regathers itself through condition into the fact that proceeds into concrete existence.
- Second question: what is the central operator of the chapter?
  Answer: real mediation, where essence determines itself through the self-sublation of positedness rather than merely shining in another.
- Third question: where does the chapter lead?
  Answer: into concrete existence, where the fact emerges from the unity of ground and condition.

## Authority + format lock (must persist)

- Working extraction references: `ground/ground-idea.txt`, `ground/absolute.txt`, `ground/determinate.txt`, `ground/condition.txt`, and `GROUND-DISTILLATION.md`
- Upstream source authority: `ground/ground-idea.txt`, `ground/absolute.txt`, `ground/determinate.txt`, `ground/condition.txt`
- This workbook covers the Ground chapter only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not flatten ground into a merely explanatory relation added from outside the content.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-04 (Ground pass)

Scope:

- files:
  - `ground/ground-idea.txt`
  - `ground/absolute.txt`
  - `ground/determinate.txt`
  - `ground/condition.txt`
  - `GROUND-DISTILLATION.md`
- pass policy: 1 marker entry + 4 analytic entries

Decision:

- Continue the Hegel Essence pass with one readable artifact pair per high-level folder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Treat this file as the chapter-level architectonic surface for Ground.
- Keep the emphasis on the chapter sequence: absolute ground, determinate ground, condition, concrete existence.
- Hold heavier relation overlays in reserve until the readable chapter pass is stable.

### Entry hegel-ground — Marker `Ground`

- sourceFiles:
  - `ground/ground-idea.txt`
- lineSpans:
  - `ground/ground-idea.txt:2-118`
- summary: Ground presents essence as determining itself as ground, distinguishes ground from pure reflection as real mediation, and unfolds the chapter through absolute ground, determinate ground, and condition into concrete existence.

Key points: (KeyPoint)

- k1. Essence determines itself as ground.
- k2. Ground is the true determination of essence after reflected determinations founder.
- k3. Ground is real mediation rather than mere reflective shining.
- k4. The chapter triad is absolute ground, determinate ground, and condition.
- k5. The chapter ends in the fact passing over into concrete existence.

Claims: (Claim)

- c1. id: hegel-ground-c1
  - subject: essence
  - predicate: determines_itself_as
  - object: ground
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ground/ground-idea.txt:6-44`

- c2. id: hegel-ground-c2
  - subject: ground
  - predicate: is
  - object: real_mediation_of_essence_with_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `ground/ground-idea.txt:68-99`

- c3. id: hegel-ground-c3
  - subject: ground_chapter
  - predicate: unfolds_through
  - object: absolute_ground_determinate_ground_and_condition_into_concrete_existence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `ground/ground-idea.txt:101-118`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-ground-absolute
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: the chapter first determines essence as absolute ground.
  - sourceClaimIds: [`hegel-ground-c1`, `hegel-ground-c2`, `hegel-ground-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-ground-absolute-c1`, `hegel-ground-absolute-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-ground-determinate
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: absolute ground further determines itself as determinate ground.
  - sourceClaimIds: [`hegel-ground-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-ground-determinate-c1`, `hegel-ground-determinate-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-ground-condition
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: determinate ground passes into conditioning mediation.
  - sourceClaimIds: [`hegel-ground-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-ground-condition-c1`, `hegel-ground-condition-c2`]

- r4. type: transitions_to
  - targetEntryId: hegel-ground-existence-threshold
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: the chapter culminates in the fact passing over into concrete existence.
  - sourceClaimIds: [`hegel-ground-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`hegel-ground-existence-threshold-c1`]

Review outcome:

- review_pending
- notes: this marker fixes Ground as one chapter-level movement rather than three disconnected local parts.

### Entry hegel-ground-absolute — `Ground`: absolute ground as form, matter, and content

- sourceFiles:
  - `ground/absolute.txt`
- lineSpans:
  - `ground/absolute.txt:6-320`
- summary: Absolute ground first fixes the twofold determination of ground and grounded, then unfolds the reciprocal reference of form and essence, and finally develops matter and content as moments of that same self-mediating unity.

Key points: (KeyPoint)

- k1. Ground and grounded are two moments of one essence.
- k2. The mediation of ground unites pure and determining reflection.
- k3. Form is the completed whole of reflection and is not externally added to essence.
- k4. Matter is essence as formless identity, the other of form.
- k5. Form, matter, and content are moments of one self-mediating ground.

Claims: (Claim)

- c1. id: hegel-ground-absolute-c1
  - subject: absolute_ground
  - predicate: is
  - object: twofold_determination_of_ground_and_grounded_within_one_essence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `ground/absolute.txt:6-44`

- c2. id: hegel-ground-absolute-c2
  - subject: form_and_essence
  - predicate: stand_in
  - object: absolute_reciprocal_reference_with_form_as_completed_reflection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `ground/absolute.txt:61-190`

- c3. id: hegel-ground-absolute-c3
  - subject: matter_and_content
  - predicate: emerge_as
  - object: moments_of_forms_self_mediation_and_not_independent_principles
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `ground/absolute.txt:192-320`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: hegel-ground
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: this entry gathers the first major field of the chapter under absolute ground.
  - sourceClaimIds: [`hegel-ground-absolute-c1`, `hegel-ground-absolute-c2`, `hegel-ground-absolute-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`hegel-ground-c2`, `hegel-ground-c3`]

- r2. type: builds_on
  - targetEntryId: hegel-foundation-ground-threshold
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: absolute ground unfolds the threshold result reached at the end of Foundation.
  - sourceClaimIds: [`hegel-ground-absolute-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-foundation-ground-threshold-c1`, `hegel-foundation-ground-threshold-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-ground-determinate
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: absolute ground further determines itself into determinate ground.
  - sourceClaimIds: [`hegel-ground-absolute-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`hegel-ground-determinate-c1`]

Review outcome:

- review_pending
- notes: this synthetic entry compresses the internal sequence of form/essence, form/matter, and content into one chapter-level opening movement.

### Entry hegel-ground-determinate — `Ground`: formal, real, and complete ground

- sourceFiles:
  - `ground/determinate.txt`
- lineSpans:
  - `ground/determinate.txt:4-397`
- summary: Determinate ground moves from formal sufficiency, through real ground with diverse content, to complete ground, where the externalized ground-connection tries to recover its unity.

Key points: (KeyPoint)

- k1. Formal ground doubles one content into the forms of ground and grounded.
- k2. Its sufficiency is only formal because content remains indifferent to form.
- k3. Real ground introduces diverse content and thereby realizes grounding.
- k4. This realization externalizes the ground-connection.
- k5. Complete ground regathers formal and real ground into a more adequate unity.

Claims: (Claim)

- c1. id: hegel-ground-determinate-c1
  - subject: formal_ground
  - predicate: is
  - object: formally_sufficient_doubling_of_one_content_as_ground_and_grounded
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `ground/determinate.txt:4-104`

- c2. id: hegel-ground-determinate-c2
  - subject: real_ground
  - predicate: is
  - object: realized_but_externalized_grounding_through_diverse_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `ground/determinate.txt:105-230`

- c3. id: hegel-ground-determinate-c3
  - subject: complete_ground
  - predicate: is
  - object: reasserted_identity_of_ground_and_grounded_that_gathers_formal_and_real_ground
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `ground/determinate.txt:231-397`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: hegel-ground
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: this entry gathers the middle movement of the chapter under determinate ground.
  - sourceClaimIds: [`hegel-ground-determinate-c1`, `hegel-ground-determinate-c2`, `hegel-ground-determinate-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`hegel-ground-c3`]

- r2. type: builds_on
  - targetEntryId: hegel-ground-absolute
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: determinate ground arises from the further specification of absolute ground's form-content structure.
  - sourceClaimIds: [`hegel-ground-determinate-c1`, `hegel-ground-determinate-c2`]
  - sourceKeyPointIds: [`k1`, `k3`, `k4`]
  - targetClaimIds: [`hegel-ground-absolute-c3`]

- r3. type: transitions_to
  - targetEntryId: hegel-ground-condition
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: complete ground passes over into conditioning mediation.
  - sourceClaimIds: [`hegel-ground-determinate-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`hegel-ground-condition-c1`]

Review outcome:

- review_pending
- notes: this synthetic entry compresses formal, real, and complete ground into one chapter-level middle movement.

### Entry hegel-ground-condition — `Ground`: condition and the absolutely unconditioned fact

- sourceFiles:
  - `ground/condition.txt`
- lineSpans:
  - `ground/condition.txt:4-308`
- summary: Condition first appears as the relatively unconditioned immediacy presupposed by ground, but the contradiction of ground and condition resolves into the absolutely unconditioned fact, where both are one whole of form and content.

Key points: (KeyPoint)

- k1. Condition is immediate manifold existence presupposed by ground.
- k2. Ground and condition are each relatively unconditioned and yet each is only a moment.
- k3. Their reciprocity is contradictory.
- k4. The absolutely unconditioned is the one substrate of condition and ground.
- k5. The fact in itself conditions itself and places itself as ground.

Claims: (Claim)

- c1. id: hegel-ground-condition-c1
  - subject: condition_and_ground
  - predicate: are
  - object: relatively_unconditioned_yet_internally_mediated_and_contradictory_sides
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `ground/condition.txt:4-147`

- c2. id: hegel-ground-condition-c2
  - subject: absolutely_unconditioned
  - predicate: is
  - object: one_whole_of_form_and_content_as_fact_in_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `ground/condition.txt:148-274`

- c3. id: hegel-ground-condition-c3
  - subject: absolute_fact
  - predicate: is
  - object: unity_that_conditions_itself_and_places_itself_as_ground
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `ground/condition.txt:275-308`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: hegel-ground
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: this entry gathers the chapter's final mediation under condition and the absolutely unconditioned.
  - sourceClaimIds: [`hegel-ground-condition-c1`, `hegel-ground-condition-c2`, `hegel-ground-condition-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`hegel-ground-c3`]

- r2. type: builds_on
  - targetEntryId: hegel-ground-determinate
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: condition unfolds the conditioning mediation announced by complete ground.
  - sourceClaimIds: [`hegel-ground-condition-c1`, `hegel-ground-condition-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-ground-determinate-c3`]

- r3. type: transitions_to
  - targetEntryId: hegel-ground-existence-threshold
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: the absolutely unconditioned fact now proceeds into concrete existence.
  - sourceClaimIds: [`hegel-ground-condition-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`hegel-ground-existence-threshold-c1`]

Review outcome:

- review_pending
- notes: this synthetic entry compresses relative unconditioned and absolute fact into one chapter-level closing movement before the final existence threshold.

### Entry hegel-ground-existence-threshold — `Ground`: procession of the fact into concrete existence

- sourceFiles:
  - `ground/ground-idea.txt`
  - `ground/condition.txt`
- lineSpans:
  - `ground/ground-idea.txt:113-118`
  - `ground/condition.txt:309-528`
- summary: Ground culminates when the absolutely unconditioned fact no longer stands behind its conditions but proceeds into concrete existence through its own self-positing mediation.

Key points: (KeyPoint)

- k1. The unconditioned fact is the unity of ground and condition.
- k2. The fact is absolute ground only by positing its own conditions.
- k3. In this movement, mediation rejoins itself in the fact.
- k4. The chapter passes from ground into concrete existence.

Claims: (Claim)

- c1. id: hegel-ground-existence-threshold-c1
  - subject: fact_in_itself
  - predicate: passes_over_into
  - object: concrete_existence_through_conditioning_mediation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `ground/ground-idea.txt:113-118`
    - `ground/condition.txt:309-528`

- c2. id: hegel-ground-existence-threshold-c2
  - subject: absolute_ground
  - predicate: is
  - object: facts_self_positing_unity_of_ground_and_condition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `ground/condition.txt:309-528`

Relations: (Relation)

- r1. type: gathers
  - targetEntryId: hegel-ground-condition
  - targetWorkbook: `GROUND-WORKBOOK.md`
  - note: concrete existence is the positive outcome of the chapter's final condition-ground unity.
  - sourceClaimIds: [`hegel-ground-existence-threshold-c1`, `hegel-ground-existence-threshold-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-ground-condition-c2`, `hegel-ground-condition-c3`]

Review outcome:

- review_pending
- notes: this threshold entry keeps Ground from looking terminal when the chapter is really the handoff into concrete existence.
