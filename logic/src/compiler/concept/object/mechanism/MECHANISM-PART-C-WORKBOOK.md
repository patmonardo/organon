# Object Part C Workbook

Part: `C. ABSOLUTE MECHANISM`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `mechanism.txt` as authority.
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

## Session: 2026-02-22 (first Absolute Mechanism pass)

Scope:

- file: `mechanism.txt`
- fixed range: lines `6-279`
- segmentation basis:
  - `a. The center`: block-level best effort (`a-001 ... a-006`)
  - `b. The law`: paragraph division (`b-001`, `b-002`, `b-003`)
  - `c. Transition of mechanism`: block-level best effort (`c-001 ... c-003`)

Decision:

- Treat entries as coarse conceptual blocks for now.
- Prioritize execution-graph-relevant relations over fine moment subdivision.
- Keep section `b` strictly paragraph-segmented as requested.

### Entry mecha-mec-a-001 — a.1 Objective singularity and universality as middle term

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 6
- lineEnd: 23

Summary:

The opening block gathers object-manifoldness into objective singularity and defines essential determinateness as the real middle term and immanent universality of interacting objects.

Key points: (KeyPoint)

- k1. Empty manifoldness is recollected into a self-determining middle point.
- k2. Immediate totality remains externally multiple and indifferent.
- k3. Essential determinateness functions as objective universality immanent to objects.

Claims: (Claim)

- c1. id: mecha-mec-a-001-c1
  - subject: objective_singularity
  - predicate: emerges_as
  - object: self_determining_middle_point
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [6-8] manifoldness gathered into objective singularity/middle point.

- c2. id: mecha-mec-a-001-c2
  - subject: essential_determinateness
  - predicate: functions_as
  - object: real_middle_term_and_immanent_objective_universality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [13-17] determinateness unites many interacting objects as real middle term.
    - [21-23] objective universality is pervading immanent essence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-a-002
  - note: conceptual middle term is concretized as central body and concrete rest.
  - sourceClaimIds: [`mecha-mec-a-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`mecha-mec-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: coarse block retained for later moment-splitting.

### Entry mecha-mec-a-002 — a.2 Central body, true rest, and return from instability

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 25
- lineEnd: 64

Summary:

This block determines the center as individualized universality: bodies strive back toward concrete rest at center, and resistance/friction are interpreted as centrality-effects.

Key points: (KeyPoint)

- k1. Central body is genus-like individualized universality of mechanical process.
- k2. Identity with center is true rest, though only as an ought under persisting externality.
- k3. Friction/resistance are phenomena of centrality returning motion to self.

Claims: (Claim)

- c1. id: mecha-mec-a-002-c1
  - subject: central_body
  - predicate: is_determined_as
  - object: individualized_universality_of_mechanical_objects
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [25-27] central body named genus/individualized universality.

- c2. id: mecha-mec-a-002-c2
  - subject: striving_toward_center
  - predicate: aims_at
  - object: concrete_rest_not_externally_posited
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [34-45] identity with center as rest/concept; striving toward true concrete rest.

- c3. id: mecha-mec-a-002-c3
  - subject: friction_and_resistance
  - predicate: are_interpreted_as
  - object: phenomena_of_centrality
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [52-58] friction/resistance derive from union with center.
    - [59-64] center-unity principle extends analogically to spiritual domain.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-a-003
  - note: centrality shifts from abstract body-role to individual immanent form.
  - sourceClaimIds: [`mecha-mec-a-002-c1`, `mecha-mec-a-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`mecha-mec-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: block kept broad to preserve execution-level semantics.

### Entry mecha-mec-a-003 — a.3 Center as individual immanent form and true One

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 66
- lineEnd: 79

Summary:

The central body ceases to be a mere object and is determined as individual immanent form, a self-determining principle binding inhering objects into a true One.

Key points: (KeyPoint)

- k1. Central body acquires being-for-itself of objective totality.
- k2. Its determinateness is immanent form, not external arrangement.
- k3. This immanent form binds objects in true unity.

Claims: (Claim)

- c1. id: mecha-mec-a-003-c1
  - subject: central_body
  - predicate: is_determined_as
  - object: individual_immanent_self_determining_principle
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [66-72] center no longer mere object; can be regarded as individual.
    - [73-79] determinateness as immanent self-determining form binding objects into true One.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-a-004
  - note: individual center now dirempts into extremes and distributed centralities.
  - sourceClaimIds: [`mecha-mec-a-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`mecha-mec-a-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: compact block to keep center-individual thesis explicit.

### Entry mecha-mec-a-004 — a.4 Diremption: relative centers and mediated unity

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 81
- lineEnd: 97

Summary:

The central individual dirempts into extremes, producing relative centers and non-self-subsistent objects that are again united through the absolute middle term.

Key points: (KeyPoint)

- k1. Negative unity splits into true extremes.
- k2. Previously self-external objects become individuals via conceptual retreat.
- k3. Relative centers and dependent objects are reunified by absolute middle term.

Claims: (Claim)

- c1. id: mecha-mec-a-004-c1
  - subject: central_individual
  - predicate: dirempts_into
  - object: relative_centers_and_non_self_subsistent_objects
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [81-87] central individual as negative unity dirempts and external objects become individuals.
    - [93-97] secondary centers/objects are brought into unity by absolute middle term.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-a-005
  - note: diremption is formalized as nested syllogistic mediation.
  - sourceClaimIds: [`mecha-mec-a-004-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`mecha-mec-a-005-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: block treated as structural pivot.

### Entry mecha-mec-a-005 — a.5 Three syllogisms and civic analogy of mediated totality

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 99
- lineEnd: 145

Summary:

This block develops tri-syllogistic mediation linking absolute/relative centers and formal objects, then mirrors the structure in a government-citizen-needs analogy.

Key points: (KeyPoint)

- k1. Relative centers mediate between absolute center and dependent objects.
- k2. Formal objects mediate absolute and relative individuality as third syllogism.
- k3. Government-citizen-needs triad analogizes the same mediating structure.

Claims: (Claim)

- c1. id: mecha-mec-a-005-c1
  - subject: syllogistic_structure_of_centered_mechanism
  - predicate: consists_of
  - object: nested_middle_terms_linking_absolute_relative_and_formal_levels
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [99-127] second/third syllogism articulation and formal middle-term roles.

- c2. id: mecha-mec-a-005-c2
  - subject: civic_analogy
  - predicate: instantiates
  - object: same_tripartite_mediation_logic
  - modality: asserted
  - confidence: 0.91
  - evidence:
    - [128-145] government, citizens, and needs mapped as reciprocal middle-term structure.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-a-006
  - note: multi-syllogistic mediation closes as free mechanism and objective law.
  - sourceClaimIds: [`mecha-mec-a-005-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`mecha-mec-a-006-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: preserved as execution-graph-relevant architecture block.

### Entry mecha-mec-a-006 — a.6 Free mechanism and law emergence

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 146
- lineEnd: 165

Summary:

The completed syllogistic totality constitutes free mechanism, where external connections are subordinated and order passes into objective immanent determination: law.

Key points: (KeyPoint)

- k1. Free mechanism is totalized concept-relation across the tri-syllogistic structure.
- k2. Pressure/impact/attraction remain at externality level.
- k3. Mere order sublates into objective determination as law.

Claims: (Claim)

- c1. id: mecha-mec-a-006-c1
  - subject: free_mechanism
  - predicate: is_determined_as
  - object: completed_totality_of_conceptual_syllogistic_relations
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [146-156] free mechanism as totality with objective universality.

- c2. id: mecha-mec-a-006-c2
  - subject: order
  - predicate: passes_over_into
  - object: immanent_objective_determination_law
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [157-165] external connections contextualized; order becomes objective law.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-b-001
  - note: law announced at end of `a` becomes thematic object of section `b`.
  - sourceClaimIds: [`mecha-mec-a-006-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`mecha-mec-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: closes section `a` as block-level foundation for law.

### Entry mecha-mec-b-001 — b.1 Law paragraph 1: idealized reality versus external striving

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 169
- lineEnd: 205

Summary:

First law paragraph distinguishes idealized reality from merely external reality, defining law as system-soul where individuality and universality are concretely reconciled.

Key points: (KeyPoint)

- k1. Law arises from opposition between centrality and determined externality.
- k2. Mere identity with external reference remains an ought/striving.
- k3. Real ideality is systemic identity of objective totality.

Claims: (Claim)

- c1. id: mecha-mec-b-001-c1
  - subject: law_domain
  - predicate: articulates
  - object: difference_between_idealized_and_external_reality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [169-187] contrast of idealized objectivity vs external reality and striving.

- c2. id: mecha-mec-b-001-c2
  - subject: real_ideality
  - predicate: is_determined_as
  - object: soul_and_identity_of_objective_system
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [188-205] individuality as concrete negative unity; real ideality as system identity.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-b-002
  - note: system-identity is specified as self-moving law-principle.
  - sourceClaimIds: [`mecha-mec-b-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`mecha-mec-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph segmentation kept exact per request (`001`).

### Entry mecha-mec-b-002 — b.2 Law paragraph 2: law as self-movement principle

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 207
- lineEnd: 218

Summary:

Second law paragraph presents objective totality as negative unity that determines externality ideally, yielding law as determinateness of self-movement.

Key points: (KeyPoint)

- k1. Objective totality manifests as center’s negative unity.
- k2. Unity maintains subjective individuality within external objectivity.
- k3. Law is concept-difference as animating self-movement determination.

Claims: (Claim)

- c1. id: mecha-mec-b-002-c1
  - subject: self_determining_unity
  - predicate: reduces
  - object: external_objectivity_to_ideality_as_self_movement_principle
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [207-218] explicit sequence from negative unity to self-movement principle and law.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-b-003
  - note: self-movement law is contrasted with dead mechanism’s rule-like uniformity.
  - sourceClaimIds: [`mecha-mec-b-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`mecha-mec-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph segmentation kept exact per request (`002`).

### Entry mecha-mec-b-003 — b.3 Law paragraph 3: free necessity beyond dead mechanism

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 220
- lineEnd: 235

Summary:

Third law paragraph differentiates dead mechanism from free mechanism: rule-like uniformity is not law; true law belongs to concept-existing individuality and is free necessity.

Key points: (KeyPoint)

- k1. Dead mechanism yields contingency or mere formal uniformity.
- k2. Uniform rule is not law.
- k3. Free mechanism alone has law as free necessity.

Claims: (Claim)

- c1. id: mecha-mec-b-003-c1
  - subject: dead_mechanism
  - predicate: lacks
  - object: genuine_law
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [220-227] dead mechanism yields contingency/formal uniformity; rule not law.

- c2. id: mecha-mec-b-003-c2
  - subject: free_mechanism_law
  - predicate: is_determined_as
  - object: free_necessity_of_concept_existing_for_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [228-235] law proper to pure individuality/concept and characterized as free necessity.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-c-001
  - note: free necessity in law opens transition problem of mechanism.
  - sourceClaimIds: [`mecha-mec-b-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`mecha-mec-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph segmentation kept exact per request (`003`).

### Entry mecha-mec-c-001 — c.1 Transition block: law still immersed in objectivity

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 239
- lineEnd: 254

Summary:

Transition begins by stating that the law-soul remains immersed in objectivity: ideality is immanent but objects are not yet differentiated in law’s ideal non-indifference.

Key points: (KeyPoint)

- k1. Law has not yet fully stepped opposite its object.
- k2. Ideality remains diffused in objectivity.
- k3. Objects are not yet law-differentiated in ideal non-indifference.

Claims: (Claim)

- c1. id: mecha-mec-c-001-c1
  - subject: transition_state_of_mechanism
  - predicate: is_determined_as
  - object: law_immersed_in_body_like_objectivity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [239-244] soul/law still immersed; centrality diffused in objectivity.
    - [250-254] law immanent but object-difference not yet law-immanent.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-c-002
  - note: immersion gives way to explicit non-self-subsistence under concept-judgment.
  - sourceClaimIds: [`mecha-mec-c-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`mecha-mec-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: coarse block retained for later moment split.

### Entry mecha-mec-c-002 — c.2 Transition block: collapse of abstract self-subsistence

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 255
- lineEnd: 270

Summary:

The object’s self-subsistence is reduced to concept-posited determinateness, so prior striving toward center no longer preserves appearance of independent external objecthood.

Key points: (KeyPoint)

- k1. Essential self-subsistence belongs only to idealized centrality/law.
- k2. Object cannot resist concept-judgment.
- k3. Non-self-subsistence is now explicit concept-posited determinateness.

Claims: (Claim)

- c1. id: mecha-mec-c-002-c1
  - subject: object_self_subsistence
  - predicate: is_subordinated_to
  - object: idealized_centrality_and_law
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [255-263] object lacks resistance; existence is determinateness posited by concept.

- c2. id: mecha-mec-c-002-c2
  - subject: prior_striving_relation
  - predicate: is_transformed_from
  - object: apparent_external_independence_to_explicit_non_self_subsistence
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [264-270] no longer mere striving preserving external-object appearance.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-mec-c-003
  - note: collapse of abstract independence culminates in tense reciprocal opposition.
  - sourceClaimIds: [`mecha-mec-c-002-c1`, `mecha-mec-c-002-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`mecha-mec-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: coarse block retained for later moment split.

### Entry mecha-mec-c-003 — c.3 Transition block: objectified opposition and chemism threshold

Span:

- sourceFile: `src/compiler/concept/object/mechanism/mechanism.txt`
- lineStart: 271
- lineEnd: 279

Summary:

Final transition block states that center and object split into reciprocally negative tension, and free mechanism determines itself onward as chemism.

Key points: (KeyPoint)

- k1. Center itself falls apart into objectified opposition.
- k2. Centrality becomes reciprocal negative tension.
- k3. This tension is the threshold transition to chemism.

Claims: (Claim)

- c1. id: mecha-mec-c-003-c1
  - subject: centrality
  - predicate: transforms_into
  - object: reciprocally_negative_tense_connection_of_objectivities
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [271-278] center falls apart; centrality becomes reciprocal negative tension.

- c2. id: mecha-mec-c-003-c2
  - subject: free_mechanism
  - predicate: transitions_to
  - object: chemism
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [279-279] explicit terminal statement: free mechanism determines itself to chemism.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: pending_chemism_part_a_entry
  - note: explicit boundary transition into Chemism sequence.
  - sourceClaimIds: [`mecha-mec-c-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending`]
  - logicalOperator: boundary_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: final block intentionally kept compact as execution-boundary marker.
