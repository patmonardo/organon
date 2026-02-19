# Relation Part B (TopicMap) Workbook (V2)

Part: `B. THE RELATION OF FORCE AND ITS EXPRESSION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `force-expression.txt` as authority for Part B.
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

## Session: 2026-02-19 (initial Part B scaffold)

Scope:

- file: `force-expression.txt`
- active range: lines `2-370` (`B. THE RELATION OF FORCE AND ITS EXPRESSION` block)

Decision:

- User boundary rule: divide Part B by explicit `a./b./c.` markers only.
- Text prior to marker `a.` (lines `4-56`) is treated as chapter/section idea lead and is stored in `RELATION-IDEA-V2-WORKBOOK.md`.
- Part B entry extraction starts at `a.`.

## Decomposition status

- completed: `rel-frc-b-001` for `a. The conditionedness of force` (line `57` to `188`)
- completed: `rel-frc-b-002` for `b. The solicitation of force` (line `189` to `341`)
- completed: `rel-frc-b-003` for `c. The infinity of force` (line `342` to `370`)

### Entry rel-frc-b-001 — Conditioned force as self-mediated relation to force

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/force-expression.txt`
- lineStart: 57
- lineEnd: 188

Summary:

In `a. The conditionedness of force`, force is determined first as posited against immediate thinghood, then as active self-repelling contradiction, and finally as essentially conditioned by and through another force that is in truth its own relational determination.

Key points: (KeyPoint)

- k1. Force initially appears as reflected positedness presupposing an immediate existing thing or matter.
- k2. This immediacy is sublated: force is negative unity that posits externality as its own moment rather than inhering in a self-subsistent thing.
- k3. Force is active self-repelling contradiction, unity of reflected and immediate subsistence, passing into external manifoldness.
- k4. The condition of force is itself force, so conditionedness is force's own self-relation through another.

Claims: (Claim)

- c1. id: rel-frc-b-001-c1
  - subject: force_in_initial_determination
  - predicate: appears_as
  - object: reflected_positedness_conditioned_by_immediate_thinghood
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [59-81] force appears as reflected/posited relative to an immediate thing or matter, seemingly externally impressed.

- c2. id: rel-frc-b-001-c2
  - subject: force
  - predicate: is
  - object: negative_unity_that_sublates_thinghood_into_posited_externality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [82-115] matter-like interpretation is critiqued; immediate concrete existence is only transient moment.
    - [116-123] force itself is positing of externality; determinate matter-self-subsistence has passed into appearance.

- c3. id: rel-frc-b-001-c3
  - subject: conditionedness_of_force
  - predicate: is
  - object: self_relation_of_force_through_another_force
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [124-149] force as active self-repelling contradiction, passing over into externality while preserving unity.
    - [150-172] condition is not a thing; the self-subsistent other is itself force; activity conditioned through another force.
    - [173-188] relation of forces: conditionedness through another is force's own positing act in itself.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rel-frc-b-002
  - sourceClaimIds: [rel-frc-b-001-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: conditionedness of force opens directly into explicit solicitation in subpart `b`.

- r2. type: supports
  - targetEntryId: rel-whp-a-003
  - sourceClaimIds: [rel-frc-b-001-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [rel-whp-a-003-c3]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection
  - note: force as posited externality confirms the Part A transition beyond whole/parts immediacy.

Review outcome:

- review_pending
- notes: `b-001` complete at explicit `a.` boundary.

### Entry rel-frc-b-002 — Solicitation as reciprocal mediation of force with itself through the other

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/force-expression.txt`
- lineStart: 189
- lineEnd: 341

Summary:

In `b. The solicitation of force`, external stimulus is shown as force's own presupposing activity posited as other force, so soliciting/solicited pass over into reciprocal mediation where passivity and activity are one reactive self-return of force.

Key points: (KeyPoint)

- k1. Force's conditioned externality is its own presupposing activity initially posited as another force.
- k2. The stimulus is not external determination of an inert subject but solicitation in which force posits and sublates externality as its own expression.
- k3. The distinction between soliciting and solicited forces is only momental, since each determination belongs equally to both.
- k4. Force's reaction unifies passive and active moments as mediated self-reference: positing the external is sublating it and conversely.

Claims: (Claim)

- c1. id: rel-frc-b-002-c1
  - subject: solicitation_structure_of_force
  - predicate: is
  - object: reciprocal_presupposing_of_force_as_other_force
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [189-205] conditioned immediacy is posited/presupposed; externality is own presupposing activity first as another force.
    - [206-214] reciprocal presupposing: each force posits its externality as other force.

- c2. id: rel-frc-b-002-c2
  - subject: stimulus
  - predicate: functions_as
  - object: soliciting_moment_of_force_own_expression
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [215-236] presupposing activity sublates external negation and posits externality as its own.
    - [237-249] stimulus only solicits; force's act is reducing externality to stimulus and positing it as own self-repelling expression.

- c3. id: rel-frc-b-002-c3
  - subject: relation_between_soliciting_and_solicited_forces
  - predicate: is
  - object: mediated_identity_of_active_and_passive_reactivity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [250-312] soliciting/solicited roles are equally distributed and essentially mediated in both forces.
    - [313-341] passivity-to-activity is turning-back into self; expression/reaction and sublation/positing of externality are one.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-frc-b-001
  - sourceClaimIds: [rel-frc-b-002-c1, rel-frc-b-002-c3]
  - sourceKeyPointIds: [k1, k4]
  - targetClaimIds: [rel-frc-b-001-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `b-002` explicates how conditionedness in `b-001` operates concretely as reciprocal solicitation.

- r2. type: transitions_to
  - targetEntryId: rel-frc-b-003
  - sourceClaimIds: [rel-frc-b-002-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: reactive unity of externality/inwardness leads directly into `c. The infinity of force`.

Review outcome:

- review_pending
- notes: `b-002` complete at explicit `b.` boundary.

### Entry rel-frc-b-003 — Infinity of force as identity of externality and inwardness

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/force-expression.txt`
- lineStart: 342
- lineEnd: 370

Summary:

In `c. The infinity of force`, force overcomes its finite immediacy by expressing that reference-to-other is self-reference, so stimulus and passivity are mediated moments of its own activity and externality is identical with inwardness.

Key points: (KeyPoint)

- k1. Force is finite so long as presupposing and self-referring reflection stand apart in immediacy.
- k2. The activity of force sublates externality and determines it as the medium of force's self-identity.
- k3. Solicitation and passivity are force's own mediated self-soliciting activity.
- k4. The infinite determination of force is the identity of externality and inwardness.

Claims: (Claim)

- c1. id: rel-frc-b-003-c1
  - subject: finitude_of_force
  - predicate: consists_in
  - object: immediacy_of_separated_moments_of_presupposing_and_reflection
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [342-350] force is finite while moments have immediate form; presupposing and self-referring reflection appear as separated.
    - [351-353] conditionedness in form and restriction in content persist.

- c2. id: rel-frc-b-003-c2
  - subject: activity_of_force
  - predicate: is
  - object: sublation_of_externality_into_self_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [354-358] activity consists in expressing itself by sublating externality and determining it as identical with itself.

- c3. id: rel-frc-b-003-c3
  - subject: infinite_truth_of_force
  - predicate: is
  - object: identity_of_reference_to_other_with_self_reference_externality_with_inwardness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [359-367] reference to other is reference to self; passivity is activity; stimulus is own soliciting; externality is mediated.
    - [368-370] explicit conclusion: force expresses identity of externality with inwardness.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-frc-b-002
  - sourceClaimIds: [rel-frc-b-003-c3]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [rel-frc-b-002-c3]
  - logicalOperator: consummative_refinement
  - analysisMode: first_order_claim_projection
  - note: `b-003` consummates `b-002` by explicitly naming the mediated unity of active/passive and internal/external as the infinity of force.

- r2. type: transitions_to
  - targetEntryId: rel-oin-c-001
  - sourceClaimIds: [rel-frc-b-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: Part B closes by preparing the transition into `C. RELATION OF OUTER AND INNER`.

Review outcome:

- review_pending
- notes: `b-003` complete; Part B (`b-001..b-003`) is complete.
