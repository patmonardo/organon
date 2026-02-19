# Ground Part A (TopicMap) Workbook (V2)

Part: `A. ABSOLUTE GROUND`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `absolute.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries must follow complete sentence groups (no mid-sentence start/end).

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Working template

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-19 (bootstrap + narrow focus)

Scope:

- file: `absolute.txt`
- active section range: lines `3-536` (`a. Form and essence`, `b. Form and matter`, `c. Form and content`)
- excluded for this pass: none

Decision:

- Initialize workbook with strict contract lock.
- Execute narrow/deep pass section-by-section (`a`, then `b`, then `c`).
- Keep relation modeling minimal until first-order claims stabilize for each section.
- Complete first-order claim extraction for all three sections before relation expansion.
- Keep each entry to a stable triad of core claims (`c1..c3`) for this phase.
- Reserve higher-order refinement for `Relations` in subsequent passes; do not decompose claim internals yet.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay consistently across Ground Part A/B/C in this session.

### Entry grd-abs-a-001 — Form and essence as absolute reciprocal grounding

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/absolute.txt`
- lineStart: 3
- lineEnd: 203

Summary:

Section `a` establishes absolute ground as the reciprocal determination of form and essence, where each is only through the other's sublating self-reference.

Key points: (KeyPoint)

- k1. Ground begins from immediate existence as positedness that presupposes non-posited ground.
- k2. Essence-as-ground is intrinsically twofold (ground and grounded), and their unity is not an abstract identity but mediated negativity.
- k3. Mediation in ground unifies pure and determining reflection, and this yields form as determinate positedness distinguished from simple essence.
- k4. Ground is the absolute reciprocal reference of form and essence: essence is substrate only as self-determined negativity and so also a moment of form.
- k5. Form is completed reflection and absolute negativity; it does not externally determine essence but sublates its own distinction and returns to identical essence.

Claims: (Claim)

- c1. id: grd-abs-a-001-c1
  - subject: absolute_ground_beginning
  - predicate: is_determined_as
  - object: posited_immediacy_presupposing_non_posited_ground
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [5-20] immediate existence is positedness that presupposes ground; ground is essence self-identical in negativity.

- c2. id: grd-abs-a-001-c2
  - subject: mediation_of_ground
  - predicate: is
  - object: unity_of_pure_and_determining_reflection_generating_form_against_essence
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [60-69] mediation is unity of pure and determining reflection; determinations are distinguished from simple identity and constitute form against essence.

- c3. id: grd-abs-a-001-c3
  - subject: form_essence_relation_in_absolute_ground
  - predicate: is
  - object: reciprocal_self_grounding_where_form_is_absolute_negativity_and_identity_with_essence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [132-138] explicit statement of absolute reciprocal connecting reference of form and essence.
    - [139-166] form as completed reflection/absolute negativity; form has essence in its own identity and essence has absolute form.
    - [179-190] form determines essence only by sublating its own distinction; form is contradiction and thereby ground as self-identical negation.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-ctr-c-003
  - note: specifies how resolved contradiction as ground is internally articulated as form/essence reciprocity.
  - sourceClaimIds: [`grd-abs-a-001-c2`, `grd-abs-a-001-c3`]
  - sourceKeyPointIds: [`k3`, `k4`, `k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-abs-a-002
  - note: section `a` culminates by determining formless identity as matter, opening section `b`.
  - sourceClaimIds: [`grd-abs-a-001-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`grd-abs-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `a`; triadic claim structure is intentionally retained for cross-pass stability.

### Entry grd-abs-a-002 — Form and matter as reciprocal presupposition and restored unity

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/absolute.txt`
- lineStart: 204
- lineEnd: 458

Summary:

Section `b` develops form and matter as reciprocally presupposing moments whose apparent externality is sublated into one self-mediating unity of essence as ground.

Key points: (KeyPoint)

- k1. Matter is essence as simple identity determined as the other of form and substrate of form-determinations.
- k2. The abstraction to matter is not external stripping but form's own self-reduction to simple identity.
- k3. Form and matter mutually presuppose each other yet neither is the unilateral ground of the other.
- k4. Their reciprocal determination is one movement of self-mediation through non-being, where form and matter each bear contradiction.
- k5. The result is restored but posited unity: materially subsistent form, formed matter, and transition to content.

Claims: (Claim)

- c1. id: grd-abs-a-002-c1
  - subject: matter
  - predicate: is_determined_as
  - object: essence_as_abstract_identity_other_of_form_and_its_substrate
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [206-215] matter is simple, distinctionless identity as the other of form and proper substrate.
    - [225-236] abstraction yielding matter is form's own self-reduction, not external removal.

- c2. id: grd-abs-a-002-c2
  - subject: form_and_matter_relation
  - predicate: is
  - object: reciprocal_presupposition_without_unilateral_grounding
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [237-271] each presupposes the other; each is groundless with respect to the other.
    - [272-297] matter as passive and form as active remain internally related, requiring informing/materialization.

- c3. id: grd-abs-a-002-c3
  - subject: movement_of_form_and_matter
  - predicate: culminates_in
  - object: posited_unity_of_in_itself_and_positedness_as_content_transition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [298-312] reciprocal determination is one movement and restoration of original identity.
    - [319-324] unity of form and matter is absolute self-determining ground.
    - [408-409] result is unity of in-itself and positedness.
    - [447-457] restored unity is determinate substrate (formed matter) and concludes as content.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-abs-a-001
  - note: concretizes section `a`'s form/essence reciprocity as form/matter mediation.
  - sourceClaimIds: [`grd-abs-a-002-c2`, `grd-abs-a-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`, `k5`]
  - targetClaimIds: [`grd-abs-a-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-abs-a-003
  - note: section `b` terminates in content, opening section `c`.
  - sourceClaimIds: [`grd-abs-a-002-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`grd-abs-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `b`; triadic claim structure retained and relation expansion deferred.

### Entry grd-abs-a-003 — Form and content as determinate ground

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/absolute.txt`
- lineStart: 459
- lineEnd: 536

Summary:

Section `c` determines content as the unity of form and matter that both stands over against form and internally contains the ground-connection, yielding determinate ground as twofold determinateness.

Key points: (KeyPoint)

- k1. Form's prior self-identical moments (ground, subsistence, matter) become form-determinations when form stands over against content.
- k2. Content is first the essential unity of form and matter yet, as posited unity, stands indifferent over against form.
- k3. Content is second the identity in which positedness returns to unity, so content has ground-connection as essential form.
- k4. The content of ground is ground returned into self-unity, now as informed identity where opposition-determinations are posited as negated.
- k5. Ground thereby becomes determinate ground with twofold determinateness: of form and of content.

Claims: (Claim)

- c1. id: grd-abs-a-003-c1
  - subject: content
  - predicate: is_determined_as
  - object: unity_of_form_and_matter_that_as_posited_stands_indifferent_over_against_form
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [477-487] content is unity of form and matter, yet as determinate/posited unity stands over against form and is indifferent to it.

- c2. id: grd-abs-a-003-c2
  - subject: content_and_ground_relation
  - predicate: is
  - object: identity_where_ground_connection_is_internalized_as_content_essential_form
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [489-503] content is identity of form and matter in which positedness returns to ground; content has ground-connection as essential form and ground has content.
    - [504-529] content of ground is informed identity with opposition-determinations posited as negated.

- c3. id: grd-abs-a-003-c3
  - subject: determinate_ground
  - predicate: has_determinateness
  - object: twofold_determinateness_of_form_and_content
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [530-536] ground makes itself determinate ground; determinateness is explicitly twofold: form-determinateness and content-determinateness.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-abs-a-002
  - note: develops section `b`'s transition from restored form/matter unity into explicit content.
  - sourceClaimIds: [`grd-abs-a-003-c1`, `grd-abs-a-003-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`grd-abs-a-002-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-det-b-001
  - note: determinate ground announced at section end provides handoff to Part B (`determinate.txt`).
  - sourceClaimIds: [`grd-abs-a-003-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `c`; Part A first-order claim coverage is now complete.
