# Ground Part B (TopicMap) Workbook (V2)

Part: `B. DETERMINATE GROUND`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `determinate.txt` as authority.
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

## Session: 2026-02-19 (first Part B pass)

Scope:

- file: `determinate.txt`
- active section range: lines `4-397` (`a. Formal ground`, `b. Real ground`, `c. Complete ground`)
- excluded for this pass: none

Decision:

- Initialize workbook with strict contract lock.
- Execute narrow/deep pass section-by-section (`a`, then `b`, then `c`) for this pass.
- Keep triadic claim structure (`c1..c3`) as stable first-order representation.
- For section `c`, align triadic claims directly to numbered subsections `1/2/3`.
- Defer relation expansion until all first-order section entries are stable.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay consistently across Ground Part A/B/C in this session.

### Entry grd-det-b-001 — Formal ground as sufficient but merely formal mediation

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/determinate.txt`
- lineStart: 4
- lineEnd: 104

Summary:

Section `a` determines formal ground as a determinate-content mediation in which ground and grounded are identical in form and content, yielding sufficiency that remains only formal.

Key points: (KeyPoint)

- k1. Ground has determinate content, with substrate/content as the positive mediating factor against form's mediation.
- k2. Formal mediation is negative unity in which ground and grounded mutually pass over and presuppose one identical content.
- k3. Ground and grounded are each whole form; mediation can begin from either side with equal legitimacy.
- k4. By identity of content and form, the ground is sufficient relative to the grounded.
- k5. This sufficiency is still merely formal because content is simple determinateness indifferent to form, and form remains external.

Claims: (Claim)

- c1. id: grd-det-b-001-c1
  - subject: formal_ground
  - predicate: has
  - object: determinate_content_as_positive_mediating_substrate
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [6-15] ground has determinate content; substrate/content is indifferent positive unity and mediating factor.

- c2. id: grd-det-b-001-c2
  - subject: ground_grounded_mediation
  - predicate: is
  - object: symmetric_formal_mediation_with_identical_content_and_form
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [38-52] each side passes into the other and presupposes identical determinate content.
    - [53-75] each side is both ground and posited, each the whole mediation/form; form and content are one identity.

- c3. id: grd-det-b-001-c3
  - subject: formal_ground_sufficiency
  - predicate: is_limited_as
  - object: sufficient_yet_only_formal_due_to_content_form_externality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [77-88] identity of ground and grounded in content/form yields sufficiency.
    - [89-104] relation remains formal: no real distinct content yet; content indifferent to form and form external.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-abs-a-003
  - note: unfolds determinate ground announced at the end of Part A into its first formal articulation.
  - sourceClaimIds: [`grd-det-b-001-c1`, `grd-det-b-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-det-b-002
  - note: limitation of formal sufficiency opens section `b. Real ground`.
  - sourceClaimIds: [`grd-det-b-001-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`grd-det-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `a` of Part B; section chain now completed through `b` and `c`.

### Entry grd-det-b-002 — Real ground as realized but externally split connection

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/determinate.txt`
- lineStart: 105
- lineEnd: 230

Summary:

Section `b` transforms formal ground into real ground by introducing diverse content, where grounding is realized yet breaks into external connections and external ground.

Key points: (KeyPoint)

- k1. Real ground arises because content bears internal form-difference, so ground and grounded become different in content.
- k2. With diverse content, ground-connection ceases to be tautological formal mediation and becomes realized grounding.
- k3. The grounded contains essential identity with ground but also an unessential manifold that remains indifferent and ungrounded by the essential content.
- k4. Real grounding splits into two substrates: essential compact identity and an external tie over indifferent determinations.
- k5. Ground-connection thus externalizes itself, requiring an external ground that determines what is ground and what is posited.

Claims: (Claim)

- c1. id: grd-det-b-002-c1
  - subject: real_ground_transition
  - predicate: occurs_when
  - object: ground_and_grounded_have_diverse_content_and_form_difference_is_internal_to_content
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [105-133] content and form are not external; each side is identity of whole and thereby has diverse content.
    - [134-143] once content differs, grounding ceases to be formal tautology and becomes realized.

- c2. id: grd-det-b-002-c2
  - subject: real_grounding_connection
  - predicate: contains
  - object: essential_identity_continuity_plus_unessential_indifferent_manifold
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [165-182] grounded contains simple essential compactness of ground; additional manifold is unessential and free from that ground.
    - [183-201] connection of substrate and unessential manifold is only external tie, not true ground.

- c3. id: grd-det-b-002-c3
  - subject: real_ground
  - predicate: is_determined_as
  - object: self_externalized_ground_connection_requiring_external_ground_reference
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [202-214] real ground breaks into external determinations; formal self-identical ground-form vanishes.
    - [215-230] ground-connection becomes external to itself; real ground is reference to another and to an immediate not posited by it.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-det-b-001
  - note: realizes the limitation noted in formal ground by introducing genuinely diverse content.
  - sourceClaimIds: [`grd-det-b-002-c1`, `grd-det-b-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`grd-det-b-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-det-b-003
  - note: externalization of real ground necessitates section `c. Complete ground`.
  - sourceClaimIds: [`grd-det-b-002-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`grd-det-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `b` of Part B; completed sequence now continues through section `c`.

### Entry grd-det-b-003 — Complete ground as conditioning mediation

Span:

- sourceFile: `src/relative/essence/reflection/ground/sources/determinate.txt`
- lineStart: 231
- lineEnd: 397

Summary:

Section `c` presents complete ground as the restoration of identity through the sublation of real ground's externality, culminating in conditioning mediation.

Key points: (KeyPoint)

- k1. Real ground-connection is sublated as merely posited linkage and is regrounded in an immanent connection identical in content.
- k2. Complete ground unifies formal and real ground by mediating the content determinations that were immediate in real ground.
- k3. The two-something structure yields a mediated inference-form where shared determination grounds the posited determination.
- k4. Ground-connection is both formal and real: form moments become contentful and one content determination functions as essential substrate.
- k5. In totality, ground is presupposing reflection that repels itself into immediacy and returns as conditioning mediation.

Claims: (Claim)

- c1. id: grd-det-b-003-c1
  - subject: complete_ground_initial_moment
  - predicate: is_determined_as
  - object: sublation_of_real_ground_into_immanent_link_restoring_formality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [233-255] real ground as content/connection is only substrate and posited linkage, thus sublated and regrounded.
    - [256-264] complete ground contains formal and real ground together and mediates formerly immediate content determinations.

- c2. id: grd-det-b-003-c2
  - subject: complete_ground_mediation_structure
  - predicate: is
  - object: two_connection_inference_where_shared_determination_mediates_posited_determination
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [265-307] second connection shares content but differs by immediate vs posited linkage; ground/grounded distinction is formal.
    - [308-360] formal and real moments integrate; one content determination is essential substrate grounding the other in the second something.

- c3. id: grd-det-b-003-c3
  - subject: complete_ground_totality
  - predicate: culminates_in
  - object: presupposing_reflection_as_conditioning_mediation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [361-383] real ground's externality is restored into self-mediating identity where formal and real ground interpenetrate.
    - [384-397] total ground-connection is presupposing reflection and takes on the determination of conditioning mediation.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: grd-det-b-002
  - note: resolves real ground's externalization by reintegrating formal and real mediation.
  - sourceClaimIds: [`grd-det-b-003-c1`, `grd-det-b-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`grd-det-b-002-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: grd-cnd-c-001
  - note: emergence of conditioning mediation hands off to Ground Part C (`condition.txt`).
  - sourceClaimIds: [`grd-det-b-003-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first deep pass complete for section `c`; claims are aligned to subsection blocks `1/2/3`.
