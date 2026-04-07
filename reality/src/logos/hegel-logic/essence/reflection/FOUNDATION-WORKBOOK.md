# Foundation Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic chapter-level workbook, not a replacement for the local Part A/B/C workbooks under `foundation/`.
- Read it to follow the Foundation chapter as one architectonic movement.
- Its task is to preserve the chapter-level spine from identity, through difference and contradiction, into ground.
- Use the local `foundation/FOUNDATION-PART-*.md` files for detailed part analysis and claim granularity.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which determined reflection becomes essentiality, unfolds as identity and difference, then drives itself into contradiction and returns as ground.
- Second question: what is the central operator of the chapter?
  Answer: self-related negativity that cannot remain merely identical, merely different, or merely opposed, and therefore founders into ground.
- Third question: where does the chapter lead?
  Answer: into the Ground chapter, where essence is unfolded explicitly as ground.

## Authority + format lock (must persist)

- Working extraction references: `foundation/foundation-idea.txt`, `foundation/identity.txt`, `foundation/difference.txt`, `foundation/contradiction.txt`, and `FOUNDATION-DISTILLATION.md`
- Upstream source authority: `foundation/foundation-idea.txt`, `foundation/identity.txt`, `foundation/difference.txt`, `foundation/contradiction.txt`
- This workbook covers the Foundation chapter only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not flatten contradiction into a merely subjective inconsistency.
- Do not duplicate the detailed local Part A/B/C workbooks entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-04 (Foundation pass)

Scope:

- files:
  - `foundation/foundation-idea.txt`
  - `foundation/identity.txt`
  - `foundation/difference.txt`
  - `foundation/contradiction.txt`
  - `FOUNDATION-DISTILLATION.md`
- pass policy: 1 marker entry + 4 analytic entries

Decision:

- Continue the Hegel Essence pass with one readable artifact pair per high-level folder.
- Preserve the older Part A/B/C workbooks as detailed KG artifacts.
- Treat this file as the chapter-level architectonic surface for Foundation.
- Keep the emphasis on the chapter sequence: identity, difference, contradiction, ground.
- Hold heavier relation overlays in reserve until the readable chapter pass is stable.

### Entry hegel-foundation — Marker `Foundation`

- sourceFiles:
  - `foundation/foundation-idea.txt`
- lineSpans:
  - `foundation/foundation-idea.txt:2-30`
- summary: Foundation presents reflection as determined reflection or essentiality, unfolds that essentiality through identity, difference, and contradiction, and returns contradiction into foundation or ground.

Key points: (KeyPoint)

- k1. Determined reflection is determined essence or essentiality.
- k2. The moments of essence are reflected into themselves.
- k3. The chapter triad is identity, difference, and contradiction.
- k4. Contradiction returns into foundation.

Claims: (Claim)

- c1. id: hegel-foundation-c1
  - subject: reflection
  - predicate: is_determined_as
  - object: essentiality_or_determined_essence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `foundation/foundation-idea.txt:6-19`

- c2. id: hegel-foundation-c2
  - subject: foundation_chapter
  - predicate: unfolds_through
  - object: identity_difference_and_contradiction
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `foundation/foundation-idea.txt:21-30`

- c3. id: hegel-foundation-c3
  - subject: contradiction
  - predicate: returns_to
  - object: foundation_or_ground
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `foundation/foundation-idea.txt:29-30`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-foundation-identity
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: the chapter first fixes simple self-reference as identity.
  - sourceClaimIds: [`hegel-foundation-c1`, `hegel-foundation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`hegel-foundation-identity-c1`, `hegel-foundation-identity-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-foundation-difference
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: identity specifies itself as difference.
  - sourceClaimIds: [`hegel-foundation-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-foundation-difference-c1`, `hegel-foundation-difference-c2`]

- r3. type: unfolds_to
  - targetEntryId: hegel-foundation-contradiction
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: difference finds its truth in contradiction.
  - sourceClaimIds: [`hegel-foundation-c2`, `hegel-foundation-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-foundation-contradiction-c1`, `hegel-foundation-contradiction-c2`]

- r4. type: transitions_to
  - targetEntryId: hegel-foundation-ground-threshold
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: the chapter resolves contradiction into ground.
  - sourceClaimIds: [`hegel-foundation-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-foundation-ground-threshold-c1`]

Review outcome:

- review_pending
- notes: this marker fixes Foundation as one chapter-level movement rather than as three disconnected local parts.

### Entry hegel-foundation-identity — `Foundation`: identity as essential self-equality

- sourceFiles:
  - `foundation/identity.txt`
- lineSpans:
  - `foundation/identity.txt:2-33`
- summary: Identity is the first essentiality of foundation: essence as simple self-immediacy and self-equality, but only through the self-sublation of being and its determinacies.

Key points: (KeyPoint)

- k1. Identity is essence as simple self-identity.
- k2. This identity is the immediacy of reflection, not abstract identity.
- k3. Being and every determinateness of being have sublated themselves in themselves.
- k4. Identity is therefore the negativity of being returned into self-equality.

Claims: (Claim)

- c1. id: hegel-foundation-identity-c1
  - subject: essence
  - predicate: is
  - object: simple_self_identity_as_sublated_immediacy
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `foundation/identity.txt:4-16`

- c2. id: hegel-foundation-identity-c2
  - subject: identity
  - predicate: is_not
  - object: abstract_or_relatively_negated_identity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `foundation/identity.txt:18-24`

- c3. id: hegel-foundation-identity-c3
  - subject: identity
  - predicate: arises_from
  - object: being_and_its_determinacies_sublating_themselves_in_themselves
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `foundation/identity.txt:26-33`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: hegel-foundation
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: identity is the first stable form of essentiality announced by the chapter marker.
  - sourceClaimIds: [`hegel-foundation-identity-c1`, `hegel-foundation-identity-c3`]
  - sourceKeyPointIds: [`k1`, `k3`, `k4`]
  - targetClaimIds: [`hegel-foundation-c1`, `hegel-foundation-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-foundation-difference
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: identity cannot remain as blank equality and therefore specifies itself as difference.
  - sourceClaimIds: [`hegel-foundation-identity-c2`, `hegel-foundation-identity-c3`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`hegel-foundation-difference-c1`]

Review outcome:

- review_pending
- notes: identity is kept here as essential identity rather than understanding's abstract identity.

### Entry hegel-foundation-difference — `Foundation`: difference, diversity, and comparison

- sourceFiles:
  - `foundation/difference.txt`
- lineSpans:
  - `foundation/difference.txt:6-94`
  - `foundation/difference.txt:96-259`
- summary: Difference is reflection's own negativity and thus contains identity within itself; when it unfolds as diversity and comparison by likeness and unlikeness, external reflection destabilizes itself and drives the movement toward opposition.

Key points: (KeyPoint)

- k1. Absolute difference is self-referring and belongs to essence itself.
- k2. Difference contains identity within itself and is the whole with its own moment.
- k3. Difference unfolds into diversity, where the moments stand indifferent.
- k4. Likeness and unlikeness are products of external reflection.
- k5. External comparison destabilizes itself and cannot hold the moments apart.

Claims: (Claim)

- c1. id: hegel-foundation-difference-c1
  - subject: absolute_difference
  - predicate: is
  - object: self_referring_difference_of_essence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `foundation/difference.txt:6-47`

- c2. id: hegel-foundation-difference-c2
  - subject: difference
  - predicate: contains
  - object: identity_within_itself_and_thus_is_whole_and_moment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `foundation/difference.txt:49-94`

- c3. id: hegel-foundation-difference-c3
  - subject: diversity_and_comparison
  - predicate: are
  - object: externalized_and_self_destabilizing_forms_of_difference
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - `foundation/difference.txt:96-259`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: hegel-foundation
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: this entry gathers the middle movement of the chapter under the heading of difference.
  - sourceClaimIds: [`hegel-foundation-difference-c1`, `hegel-foundation-difference-c2`, `hegel-foundation-difference-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`hegel-foundation-c2`]

- r2. type: builds_on
  - targetEntryId: hegel-foundation-identity
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: difference is identity's own specifying negativity, not an external addition.
  - sourceClaimIds: [`hegel-foundation-difference-c1`, `hegel-foundation-difference-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-foundation-identity-c1`, `hegel-foundation-identity-c3`]

- r3. type: transitions_to
  - targetEntryId: hegel-foundation-contradiction
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: once difference can no longer remain indifferent or merely comparative, it intensifies into contradiction.
  - sourceClaimIds: [`hegel-foundation-difference-c3`]
  - sourceKeyPointIds: [`k3`, `k4`, `k5`]
  - targetClaimIds: [`hegel-foundation-contradiction-c1`]

Review outcome:

- review_pending
- notes: this synthetic entry compresses absolute difference, diversity, and likeness/unlikeness into one chapter-level middle movement.

### Entry hegel-foundation-contradiction — `Foundation`: contradiction as truth of opposition

- sourceFiles:
  - `foundation/contradiction.txt`
- lineSpans:
  - `foundation/contradiction.txt:4-130`
  - `foundation/contradiction.txt:132-210`
- summary: Contradiction is the truth of opposition because each side contains and excludes the other within itself; contradiction therefore becomes the self-excluding movement in which opposition sublates itself.

Key points: (KeyPoint)

- k1. In opposition, each side contains its other within itself.
- k2. Positive and negative are self-mediated wholes, not merely external opposites.
- k3. Each excludes itself in excluding the other.
- k4. Contradiction is already implicit in difference and explicit in opposition.
- k5. Contradiction begins to resolve itself through the self-sublation of the opposites.

Claims: (Claim)

- c1. id: hegel-foundation-contradiction-c1
  - subject: contradiction
  - predicate: is
  - object: truth_of_self_subsisting_opposition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `foundation/contradiction.txt:4-130`

- c2. id: hegel-foundation-contradiction-c2
  - subject: positive_and_negative
  - predicate: are
  - object: posited_contradiction_and_self_excluding_self_related_unities
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `foundation/contradiction.txt:41-130`

- c3. id: hegel-foundation-contradiction-c3
  - subject: contradiction
  - predicate: resolves_itself_through
  - object: self_sublation_of_the_opposites
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `foundation/contradiction.txt:132-210`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: hegel-foundation
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: contradiction is the chapter's highest explicit determination before the return to ground.
  - sourceClaimIds: [`hegel-foundation-contradiction-c1`, `hegel-foundation-contradiction-c2`, `hegel-foundation-contradiction-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`hegel-foundation-c2`, `hegel-foundation-c3`]

- r2. type: builds_on
  - targetEntryId: hegel-foundation-difference
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: contradiction is difference no longer held as indifferent or merely external.
  - sourceClaimIds: [`hegel-foundation-contradiction-c1`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-foundation-difference-c1`, `hegel-foundation-difference-c3`]

- r3. type: transitions_to
  - targetEntryId: hegel-foundation-ground-threshold
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: contradiction does not terminate in nullity alone but founders to the ground.
  - sourceClaimIds: [`hegel-foundation-contradiction-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`hegel-foundation-ground-threshold-c1`, `hegel-foundation-ground-threshold-c2`]

Review outcome:

- review_pending
- notes: the key move here is to keep contradiction as an ontological-logical operator of essence, not a merely subjective failure of consistency.

### Entry hegel-foundation-ground-threshold — `Foundation`: resolved contradiction as ground

- sourceFiles:
  - `foundation/contradiction.txt`
- lineSpans:
  - `foundation/contradiction.txt:212-293`
- summary: Contradiction resolves positively by foundering into ground; ground is essence restored as self-identical negativity, in which opposition is both removed and preserved.

Key points: (KeyPoint)

- k1. Foundering contradiction returns to foundation or ground.
- k2. Ground is simple essence, but essence as ground.
- k3. In ground, contradiction is both removed and preserved.
- k4. Ground is positive self-identity in negativity.
- k5. The chapter hands essence onward to the explicit doctrine of ground.

Claims: (Claim)

- c1. id: hegel-foundation-ground-threshold-c1
  - subject: resolved_contradiction
  - predicate: is
  - object: ground
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `foundation/contradiction.txt:212-228`
    - `foundation/contradiction.txt:267-282`

- c2. id: hegel-foundation-ground-threshold-c2
  - subject: ground
  - predicate: is
  - object: essence_as_positive_self_identity_in_its_negation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `foundation/contradiction.txt:229-293`

Relations: (Relation)

- r1. type: gathers
  - targetEntryId: hegel-foundation-contradiction
  - targetWorkbook: `FOUNDATION-WORKBOOK.md`
  - note: ground is the positive resolution and recollection of contradiction.
  - sourceClaimIds: [`hegel-foundation-ground-threshold-c1`, `hegel-foundation-ground-threshold-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`hegel-foundation-contradiction-c3`]

- r2. type: transitions_to
  - targetEntryId: groun-abs-001
  - targetWorkbook: `ground/GROUND-PART-A-WORKBOOK.md`
  - note: the next explicit field is the Ground chapter.
  - sourceClaimIds: [`hegel-foundation-ground-threshold-c1`, `hegel-foundation-ground-threshold-c2`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: pending_cross_workbook

Review outcome:

- review_pending
- notes: this threshold entry keeps foundation from looking terminal when it is really the chapter that hands essence over to ground proper.
