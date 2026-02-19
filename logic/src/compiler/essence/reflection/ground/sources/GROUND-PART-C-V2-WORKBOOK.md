# Ground Part C (TopicMap) Workbook (V2)

Part: `C. CONDITION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `condition.txt` as authority.
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

## Session: 2026-02-19 (first Part C pass)

Scope:

- file: `condition.txt`
- active section range: lines `4-528` (`a. The relatively unconditioned`, `b. The absolutely unconditioned`, `c. Procession of the fact into concrete existence`)
- excluded for this pass: none

Decision:

- Initialize workbook with strict contract lock.
- Execute narrow/deep pass section-by-section (`a`, `b`, `c`).
- Keep triadic claim structure (`c1..c3`) as stable first-order representation.
- In section `a`, align each claim directly with numbered subsections `1/2/3`.
- In section `b`, use conceptual (non-numbered) span chunking for triadic claims.
- Defer higher-order relation expansion until first-order entries are stabilized.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay consistently across Ground Part A/B/C in this session.

### Entry grd-cnd-c-001 — Relative unconditioned as contradiction of immediacy and mediation

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/condition.txt`
- lineStart: 4
- lineEnd: 147

Summary:

Section `a` determines condition and ground as relatively unconditioned sides whose apparent independence is internally contradictory and mediated.

Key points: (KeyPoint)

- k1. Condition is immediate existence presupposed by ground, but this immediacy is itself posited and indifferent to being condition.
- k2. Ground-connection is self-subsistent mediation with its own informed content, distinct from the immediate material side of condition.
- k3. Condition and ground are each both unconditioned immediacy and mediated moment.
- k4. Their relation is a contradiction of independent subsistence and momentariness.
- k5. The relatively unconditioned therefore drives toward a higher unity.

Claims: (Claim)

- c1. id: grd-cnd-c-001-c1
  - subject: condition
  - predicate: is_determined_as
  - object: immediate_manifold_presupposition_of_ground_that_is_itself_posited
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [6-72] subsection `1`: condition as immediate/presupposed content of ground, yet also posited and indifferent immediacy.

- c2. id: grd-cnd-c-001-c2
  - subject: ground_connection
  - predicate: is
  - object: self_sufficient_mediation_with_peculiar_informed_content_beside_condition_material
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [73-114] subsection `2`: something has not only condition but also ground; mediation is whole form with content distinct from immediate condition-material.

- c3. id: grd-cnd-c-001-c3
  - subject: relation_of_condition_and_ground
  - predicate: culminates_as
  - object: contradiction_of_indifferent_unconditionedness_and_essential_mediation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [115-147] subsection `3`: both sides are indifferent/unconditioned yet mediated; each is contradiction of self-subsistence and being-moment.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-det-b-003
  - note: unfolds conditioning mediation announced at the close of complete ground.
  - sourceClaimIds: [`grd-cnd-c-001-c1`, `grd-cnd-c-001-c3`]
  - sourceKeyPointIds: [`k1`, `k4`, `k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-cnd-c-002
  - note: contradiction of the relative unconditioned necessitates the absolutely unconditioned.
  - sourceClaimIds: [`grd-cnd-c-001-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`grd-cnd-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `a`; claims explicitly align to subsection blocks `1/2/3`.

### Entry grd-cnd-c-002 — Absolutely unconditioned as one whole of form and content

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/condition.txt`
- lineStart: 148
- lineEnd: 308

Summary:

Section `b` establishes the absolutely unconditioned as the one substrate of condition and ground, in which their reciprocal presupposition is reduced to self-relating reflective shine.

Key points: (KeyPoint)

- k1. Condition and ground first shine in each other while each appears self-standing with its own content.
- k2. Existence as condition is intrinsically reflection, and conditioned ground is likewise whole, so both moments are one whole of form and content.
- k3. Their reciprocal presupposition resolves into one substrate: the truly unconditioned fact in itself.
- k4. Infinite regress of conditions belongs to finite determinate-existence assumptions, not to condition as such.
- k5. In the absolute fact, condition/ground relation is reduced to reflective shine and the fact's self-rejoining.

Claims: (Claim)

- c1. id: grd-cnd-c-002-c1
  - subject: condition_and_ground_initial_relation
  - predicate: appears_as
  - object: reciprocal_reflective_shine_between_relatively_unconditioned_sides
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [150-225] condition and ground reflectively shine in each other; existence as condition and conditioned ground each prove to be the whole form-process.

- c2. id: grd-cnd-c-002-c2
  - subject: truly_unconditioned_fact
  - predicate: is
  - object: one_substrate_of_condition_and_ground_as_unity_of_form_and_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [226-274] one whole of form/content; reciprocal presupposition resolves to one identity/substrate, the truly unconditioned fact.

- c3. id: grd-cnd-c-002-c3
  - subject: absolute_fact
  - predicate: self_determines_as
  - object: unity_posited_as_condition_and_ground_whose_relation_sublates_into_reflective_shine
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [275-308] absolute fact posits two moments and their presupposition; relation of condition and ground disappears into the fact's rejoining movement.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-cnd-c-001
  - note: resolves the contradiction of the relatively unconditioned into one absolute substrate.
  - sourceClaimIds: [`grd-cnd-c-002-c2`, `grd-cnd-c-002-c3`]
  - sourceKeyPointIds: [`k3`, `k5`]
  - targetClaimIds: [`grd-cnd-c-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-cnd-c-003
  - note: absolute fact now proceeds into concrete existence.
  - sourceClaimIds: [`grd-cnd-c-002-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`grd-cnd-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `b`; claim chunking follows non-numbered conceptual subdivisions.

### Entry grd-cnd-c-003 — Procession of the fact into concrete existence

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/condition.txt`
- lineStart: 309
- lineEnd: 528

Summary:

Section `c` develops the fact's self-positing procession into concrete existence through the disappearance of mediation in the unity of ground and condition.

Key points: (KeyPoint)

- k1. The absolute unconditioned, as absolute ground, posits conditions as its own presupposed immediacy.
- k2. Ground-side and condition-side are one movement in which presupposing and positing mutually sublate into absolute becoming.
- k3. When all conditions are present, the fact comes forth immediately through the disappearance of mediation.
- k4. The ground does not remain beneath the grounded; grounding is the ground's self-disappearance into self-unity.
- k5. The result is concrete existence as mediated-immediacy identical with itself.

Claims: (Claim)

- c1. id: grd-cnd-c-003-c1
  - subject: absolute_unconditioned_as_ground
  - predicate: posits
  - object: conditions_as_presupposed_immediacy_of_its_own_reflective_form
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [311-381] absolute ground posits condition-side as presupposed immediacy; condition-sphere is the fact's own reflected externality.

- c2. id: grd-cnd-c-003-c2
  - subject: ground_condition_mediation
  - predicate: is
  - object: one_self_mediation_of_unconditioned_fact_ending_in_absolute_becoming
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [382-439] other side of reflective shine and unification of internal movement; mediation becomes groundless absolute becoming and pure self-staging.

- c3. id: grd-cnd-c-003-c3
  - subject: fact_coming_forth
  - predicate: culminates_in
  - object: concrete_existence_as_immediacy_through_sublated_mediation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [440-490] when all conditions are at hand, the fact steps into existence through the disappearing of mediation.
    - [491-527] the fact proceeds from ground only as ground founders; grounding is self-unifying disappearance of ground.
    - [525-528] mediated immediacy is concrete existence.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-cnd-c-002
  - note: executes the absolute fact's self-conditioning and self-grounding as concrete procession.
  - sourceClaimIds: [`grd-cnd-c-003-c1`, `grd-cnd-c-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`grd-cnd-c-002-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-nxt-001
  - note: concrete existence as result marks handoff to the next doctrinal movement (target id provisional).
  - sourceClaimIds: [`grd-cnd-c-003-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `c`; full Part C first-order coverage is established.
