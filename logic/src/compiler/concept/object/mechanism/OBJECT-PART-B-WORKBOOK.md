# Object Part B Workbook

Part: `B. THE MECHANICAL PROCESS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `process.txt` as authority.
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

## Session: 2026-02-22 (first Process pass)

Scope:

- file: `process.txt`
- fixed range: lines `4-455`
- segmentation basis:
  - preface: `4-94`
  - `a. The formal mechanical process`: numeric subentries `1|2|3`
  - `b. The real mechanical process`: three large paragraphs as subspecies
  - `c. The product of the mechanical process`: three paragraphs as subspecies + final transition paragraph

Decision:

- Use strict in-text boundaries for `a.1-3`.
- For `b` and `c`, use paragraph-level subspecies exactly as requested.
- Keep relation typing conservative and primarily sequential.

### Entry obj-mech-b-000 — Preface: monad critique and causal non-originariness

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 4
- lineEnd: 94

Summary:

The preface argues that monadic closure and substantial causality fail as objective grounds, so mechanism is defined as posited causality with externally related objects.

Key points: (KeyPoint)

- k1. Self-enclosed totalities cannot truly interact.
- k2. Monad self-reference is abstract and leaves determinateness externally grounded.
- k3. Causal originariness collapses into mediated positedness within mechanism.
- k4. Objects are indifferent both to causal unity and to their aggregate recombination.

Claims: (Claim)

- c1. id: obj-mech-b-000-c1
  - subject: monad_model
  - predicate: fails_to_ground
  - object: genuine_self_determined_interaction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-18] monad-like closure blocks interaction and makes determinateness other-posited.
    - [26-42] mirroring self-reference is passivity; development degree has ground in an other.

- c2. id: obj-mech-b-000-c2
  - subject: mechanism
  - predicate: is_determined_as
  - object: causality_as_mere_positedness_of_indifferent_objects
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [59-71] efficient causality is mediated and accidental to the object.
    - [75-94] causality as representation-product; indifferent self-subsistence itself posited; aggregation possible.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-a-001
  - note: preface grounds the formal process as explicit positing of mechanism's contradiction.
  - sourceClaimIds: [`obj-mech-b-000-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`obj-mech-b-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: preface retained because it establishes the method-necessity of the `a/b/c` process forms.

### Entry obj-mech-b-a-001 — a.1 Communication as universalized determinateness

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 102
- lineEnd: 147

Summary:

Formal process step 1 defines interaction as communication: generated determinateness is universalized and propagated across objects without immediate opposition.

Key points: (KeyPoint)

- k1. Interaction first posits identical connection via universality.
- k2. Communication generalizes determinateness across media.
- k3. Universality as objective content is resistant to singular opposition.

Claims: (Claim)

- c1. id: obj-mech-b-a-001-c1
  - subject: interaction_of_objects
  - predicate: initially_posits
  - object: identical_connection_as_communication
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [102-108] interaction is positing of identical connection via universality.
    - [120-125] formal object-totality enables unimpeded continuation of determinateness.

- c2. id: obj-mech-b-a-001-c2
  - subject: objective_universal_content
  - predicate: pervades
  - object: spirit_and_body_beyond_singular_resistance
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [131-140] objective universals in spirit (laws/morals/conceptions) impose themselves.
    - [141-147] bodily communicables (motion/heat/magnetism/electricity) are imponderable agents.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-a-002
  - note: pure communication requires the counter-moment of particularity and reaction.
  - sourceClaimIds: [`obj-mech-b-a-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`obj-mech-b-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numeric subsection boundary preserved.

### Entry obj-mech-b-a-002 — a.2 Reaction as particularization of the communicated universal

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 149
- lineEnd: 207

Summary:

Step 2 introduces reaction: communicated universality is distributed and particularized, producing species and reciprocal repulsion before returning toward rest.

Key points: (KeyPoint)

- k1. Particularity must be co-posited with universality.
- k2. Reaction is positive distribution/particularization, not mere cancellation.
- k3. Reaction equals action as reciprocal repulsion with specific shares.
- k4. Negative reaction restores singularity and externality, moving to rest.

Claims: (Claim)

- c1. id: obj-mech-b-a-002-c1
  - subject: reaction
  - predicate: is_determined_as
  - object: coequal_moment_particularizing_communicated_universal
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [149-157] universality requires particularity; objects produce singularity as reaction.
    - [161-179] communicated universal remains itself while distributed and particularized into species.

- c2. id: obj-mech-b-a-002-c2
  - subject: reciprocal_reaction
  - predicate: culminates_in
  - object: restoration_of_external_singularity_and_passage_to_rest
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [181-193] reaction equals action through reciprocal repulsion and specific shares.
    - [194-207] repulsion of other-positedness restores self-reference; action passes into rest.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-a-003
  - note: return to rest necessitates explicit determination of process-product.
  - sourceClaimIds: [`obj-mech-b-a-002-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`obj-mech-b-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numeric subsection boundary preserved.

### Entry obj-mech-b-a-003 — a.3 Product: posited concept-totality and accidental outcome

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 209
- lineEnd: 250

Summary:

Step 3 defines product as posited concept-totality that reproduces object-form as externally mediated composition, yielding an outcome accidental to the object's first existence.

Key points: (KeyPoint)

- k1. Product unites communicated universal with singularity through particularity.
- k2. Product is the object only as mediated by another.
- k3. Product determinateness is externally posited and accidental.

Claims: (Claim)

- c1. id: obj-mech-b-a-003-c1
  - subject: product_of_formal_mechanical_process
  - predicate: is_determined_as
  - object: posited_totality_of_concept
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [209-223] product as concept-totality and mediated unity of universal/singular via particularity.

- c2. id: obj-mech-b-a-003-c2
  - subject: mechanical_object_as_product
  - predicate: has
  - object: externally_posited_accidental_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [224-236] object is object only as product through mediation of another.
    - [238-250] result is not precontained as purpose; accidental relative to initial existence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-b-001
  - note: external product/rest drives transition from formal to real mechanism.
  - sourceClaimIds: [`obj-mech-b-a-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`obj-mech-b-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numeric subsection boundary preserved.

### Entry obj-mech-b-b-001 — b.1 Real process paragraph 1: differentiated opposition after formal rest

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 254
- lineEnd: 297

Summary:

Real mechanism begins by re-inscribing external determinateness as reflected within the object, producing determined opposition between singular self-subsistence and universal non-self-subsistence.

Key points: (KeyPoint)

- k1. Rest and activity remain externally related to the object.
- k2. Through mediation, determinateness is reflected back into the object.
- k3. Objects become determinedly opposed, not merely diverse.

Claims: (Claim)

- c1. id: obj-mech-b-b-001-c1
  - subject: real_mechanical_process
  - predicate: begins_with
  - object: reflected_external_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [254-269] process passes into rest, but determinateness as posited is reflected into object.

- c2. id: obj-mech-b-b-001-c2
  - subject: opposed_objects
  - predicate: are_determined_as
  - object: singular_self_subsistence_versus_universal_non_self_subsistence
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [270-289] objects/process gain more determined relation and opposition.
    - [290-297] quantitative differences do not exhaust the opposition; both remain positively self-subsistent.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-b-002
  - note: determined opposition opens the communication-overpowering dynamic.
  - sourceClaimIds: [`obj-mech-b-b-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`obj-mech-b-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first large paragraph in `b.` treated as requested subspecies.

### Entry obj-mech-b-b-002 — b.2 Real process paragraph 2: communication, resistance, overpowering, violence

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 299
- lineEnd: 356

Summary:

The second `b` paragraph analyzes communication under asymmetry, where resistance mediates overpowering and objective power becomes violence when not reflected as the object's own negativity.

Key points: (KeyPoint)

- k1. Communication requires shared sphere/contact.
- k2. Resistance is the first moment of distribution and singularization.
- k3. Overpowering occurs when singular capacity is incommensurate with communicated universal.
- k4. Violence is objective power unassimilated as immanent self-reference.

Claims: (Claim)

- c1. id: obj-mech-b-b-002-c1
  - subject: communication_under_real_mechanism
  - predicate: requires
  - object: shared_sphere_and_contact
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [299-324] stronger seizes weaker only where one sphere/contact is constituted.

- c2. id: obj-mech-b-b-002-c2
  - subject: resistance_and_power_relation
  - predicate: develops_into
  - object: overpowering_and_violence_when_negativity_remains_abstract
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [325-341] resistance mediates overpowering and failure to subjectivize communicated universal.
    - [342-356] violence defined where power is objective universality but not object's own reflected negativity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-b-003
  - note: violence/power determination transitions to fate and ethical-political mechanism.
  - sourceClaimIds: [`obj-mech-b-b-002-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`obj-mech-b-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second large paragraph in `b.` treated as requested subspecies.

### Entry obj-mech-b-b-003 — b.3 Real process paragraph 3: fate and self-consciousness entering mechanism

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 357
- lineEnd: 409

Summary:

The third `b` paragraph identifies fate as objective universality operating mechanically when unrecognized by the subject, and shows self-consciousness enters mechanism through deed-based particularization.

Key points: (KeyPoint)

- k1. Fate is objective universality appearing as blind mechanism.
- k2. Strict fate pertains to self-conscious freedom and alienation.
- k3. Deed-based particularization exposes subjectivity to mechanical externality.

Claims: (Claim)

- c1. id: obj-mech-b-b-003-c1
  - subject: fate_within_mechanism
  - predicate: is_determined_as
  - object: objective_universality_unrecognized_by_subject
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [357-378] fate as objective universality/blindness; contingency for merely animate natures.

- c2. id: obj-mech-b-b-003-c2
  - subject: self_consciousness
  - predicate: enters
  - object: mechanical_relation_through_deed_and_particularization
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [379-396] self-consciousness has fate strictly by freedom and alienation from objective universality.
    - [397-409] deed/individualization externalizes subject and brings it into mechanism.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-c-001
  - note: fate-mediated mechanism transitions to explicit account of product in section `c`.
  - sourceClaimIds: [`obj-mech-b-b-003-c1`, `obj-mech-b-b-003-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`obj-mech-b-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third large paragraph in `b.` treated as requested subspecies.

### Entry obj-mech-b-c-001 — c.1 Product paragraph 1: objective singularity as sublated semblance

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 413
- lineEnd: 437

Summary:

The first `c` paragraph defines the product as posited determinateness reflected into itself, yielding objective singularity through sublation of merely oppositional self-subsistence.

Key points: (KeyPoint)

- k1. Product preserves formal rest while positively reflecting determinateness.
- k2. True singularity emerges as posited concept-totality.
- k3. Oppositional semblance of singular self-subsistence is sublated.

Claims: (Claim)

- c1. id: obj-mech-b-c-001-c1
  - subject: product_of_mechanism
  - predicate: is_determined_as
  - object: posited_totality_with_true_objective_singularity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [413-428] product includes rest plus positive reflection into posited concept-totality.
    - [429-437] object now objective singular; merely oppositional semblance is sublated.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-c-002
  - note: objective singularity is specified as centered individual self-subsistence.
  - sourceClaimIds: [`obj-mech-b-c-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`obj-mech-b-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first paragraph in `c.` treated as requested subspecies.

### Entry obj-mech-b-c-002 — c.2 Product paragraph 2: center as individual self-subsistence

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 438
- lineEnd: 441

Summary:

The second `c` paragraph specifies the reflected objective unity as a centered individual self-subsistence.

Key points: (KeyPoint)

- k1. Objective oneness is no longer merely aggregate identity.
- k2. This oneness is individualized as center.

Claims: (Claim)

- c1. id: obj-mech-b-c-002-c1
  - subject: objective_oneness
  - predicate: is_determined_as
  - object: individual_self_subsistence_center
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [438-441] explicit identification of objective oneness as center.

Claim ↔ key point map:

- c1 -> k1, k2

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-c-003
  - note: centered oneness is complemented by universal law as rational fate.
  - sourceClaimIds: [`obj-mech-b-c-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`obj-mech-b-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second paragraph in `c.` treated as requested subspecies.

### Entry obj-mech-b-c-003 — c.3 Product paragraph 3: law as immanent rational fate

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 443
- lineEnd: 451

Summary:

The third `c` paragraph determines universality as law: an immanently particularizing rational fate that remains fixed through processual instability.

Key points: (KeyPoint)

- k1. Universality is recast from blind fate to rational immanence.
- k2. Law particularizes from within while remaining at rest through instability.

Claims: (Claim)

- c1. id: obj-mech-b-c-003-c1
  - subject: law
  - predicate: is_determined_as
  - object: immanent_rational_fate_particularizing_from_within
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [443-451] law defined as rational fate and internally particularizing universality.

Claim ↔ key point map:

- c1 -> k1, k2

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-mech-b-tr-001
  - note: law+center synthesis leads to truth/foundation transition statement.
  - sourceClaimIds: [`obj-mech-b-c-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`obj-mech-b-tr-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third paragraph in `c.` treated as requested subspecies.

### Entry obj-mech-b-tr-001 — Transition: result as truth/foundation of mechanism

Span:

- sourceFile: `src/compiler/concept/object/mechanism/process.txt`
- lineStart: 453
- lineEnd: 455

Summary:

The closing sentence states the product-structure as the truth and foundation of the mechanical process, functioning as transition to the next determination.

Key points: (KeyPoint)

- k1. Result retroactively grounds process.
- k2. Closure functions as transition marker.

Claims: (Claim)

- c1. id: obj-mech-b-tr-001-c1
  - subject: resulting_structure_center_plus_law
  - predicate: is
  - object: truth_and_foundation_of_mechanical_process
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [453-455] explicit truth/foundation closure.

Claim ↔ key point map:

- c1 -> k1, k2

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: pending_part_c_entry
  - note: explicit bridge sentence to next part-level determination.
  - sourceClaimIds: [`obj-mech-b-tr-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`pending`]
  - logicalOperator: boundary_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition captured as standalone entry per request.
