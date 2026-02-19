# Thing Part A (TopicMap) Workbook (V2)

Part: `A. THE THING AND ITS PROPERTIES`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `thing.txt` as authority for Part A.
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

## Session: 2026-02-19 (initial scaffold)

Scope:

- file: `thing.txt`
- active range: lines `211-643` (`A. THE THING AND ITS PROPERTIES` block)

Decision:

- Initialize Part A workbook structure before full entry extraction.
- Binding span rule: subpart `a` must obey explicit numbered boundaries (`1`, `2`, `3`) when defining entry spans.
- Subparts `b` and `c` are non-numbered and should be chunked by complete conceptual sentence groups only.
- Do not force synthetic numbering outside subpart `a`.
- Methodology update: decomposition boundaries may be user-guided during analysis, and accepted guidance must be recorded explicitly in `Decomposition status` before entry drafting.

## Decomposition status

- `a. The thing in itself and concrete existence`
  - completed: `thg-a-a-001` for `a.1` (line `240` to pre-`303`)
  - completed: `thg-a-a-002` for `a.2` (line `303` to pre-`374`)
  - completed: `thg-a-a-003` for `a.3` (line `374` to pre-`436`)
- `b. Property`
  - completed: `thg-a-b-001` for initial-property framing + `First` paragraph (line `438` to pre-`467`)
  - completed: `thg-a-b-002` for `Second` paragraph (line `467` to pre-`490`)
  - completed: `thg-a-b-003` for final grounding paragraph (line `490` to pre-`537`)
- `c. The reciprocal action of things`
  - completed: `thg-a-c-001` for paragraph 1 (line `539` to `587`)
  - completed: `thg-a-c-002` for paragraph 2 (line `589` to `601`)
  - completed: `thg-a-c-003` for paragraph 3 (line `603` to `642`)

### Entry thg-a-a-001 — Thing-in-itself as essential concrete existence and externalized unessentiality

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 240
- lineEnd: 302

Summary:

In `a.1`, the thing-in-itself is established as essential concrete existence arising from sublated mediation, while its manifold external existence is unessential positedness determined through external reflection.

Key points: (KeyPoint)

- k1. The thing-in-itself is the essential immediate that results from sublated mediation.
- k2. Concrete existence contains both essential thing-in-itself and mediated unessential being.
- k3. The thing-in-itself is an indeterminate, unmoved unity and not the ground of unessential existence.
- k4. The manifold determinations attributed to the thing-in-itself arise only through external reflection and are not its own determinations.

Claims: (Claim)

- c1. id: thg-a-a-001-c1
  - subject: thing_in_itself
  - predicate: is
  - object: essential_immediate_resulting_from_sublated_mediation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [240-247] "The thing in itself is the concrete existent as the essential immediate that has resulted from the sublated mediation."
    - [267-272] thing-in-itself exists concretely as essential concrete existence.

- c2. id: thg-a-a-001-c2
  - subject: mediated_being_of_the_thing
  - predicate: is
  - object: unessential_posited_concrete_existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [248-259] mediated being appears as external, manifold, other-to-itself existence.
    - [271-273] mediated being is the thing's unessential concrete existence.

- c3. id: thg-a-a-001-c3
  - subject: manifoldness_ascribed_to_thing_in_itself
  - predicate: is_grounded_in
  - object: external_reflection_not_own_determination
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [275-284] the thing-in-itself is unmoved, indeterminate unity and not ground of unessential existence.
    - [285-302] it gains manifoldness only through exposure to external reflection; such aspects are not its own determinations.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: thg-a-a-002
  - sourceClaimIds: [thg-a-a-001-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: transition from external-reflection dependence in `a.1` to explicit treatment of the "other" as reflection in `a.2`.

Review outcome:

- review_pending
- notes: `a.1` boundary is complete and respects explicit numbered segmentation.

### Entry thg-a-a-002 — External reflection collapses into thing-in-itself identity

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 303
- lineEnd: 373

Summary:

In `a.2`, the "other" is defined as external reflection whose manifoldness has no self-subsistence apart from the thing-in-itself, and whose collapse yields the identity of thing-in-itself and external concrete existence.

Key points: (KeyPoint)

- k1. External reflection is both manifoldness and reference to thing-in-itself as presupposition, and these moments are one.
- k2. The manifold "other" lacks independent subsistence and exists only as reflective shine in reference to thing-in-itself.
- k3. Essenceless external reflection collapses and becomes essential identity, yielding identity of thing-in-itself with external concrete existence.
- k4. Through this collapse, plurality of things-in-themselves appears, but their determinateness relative to one another falls into external reflection.

Claims: (Claim)

- c1. id: thg-a-a-002-c1
  - subject: external_reflection
  - predicate: is
  - object: unity_of_manifoldness_and_reference_to_thing_in_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [303-312] external reflection is manifold and refers to thing-in-itself as absolute presupposition.
    - [313-319] these two moments are one and the same.

- c2. id: thg-a-a-002-c2
  - subject: manifold_otherness
  - predicate: lacks
  - object: independent_subsistence_besides_thing_in_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [315-326] manifoldness has no independent subsistence besides thing-in-itself; it is reflective shine.
    - [327-336] otherness exists only as reference and repulsion, as unsupported rebound.

- c3. id: thg-a-a-002-c3
  - subject: essenceless_external_reflection
  - predicate: collapses_into
  - object: essential_identity_of_thing_in_itself_and_external_concrete_existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [338-349] essenceless reflection founders and comes to be essential identity or thing-in-itself.
    - [350-358] thing-in-itself is thus identical with external concrete existence.
    - [359-373] plurality appears, but determinateness between things-in-themselves falls into external reflection.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-a-001
  - sourceClaimIds: [thg-a-a-002-c1, thg-a-a-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [thg-a-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `a.2` explains why the manifold attributed in `a.1` lacks independent standing and remains externally reflective.

- r2. type: transitions_to
  - targetEntryId: thg-a-a-003
  - sourceClaimIds: [thg-a-a-002-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: shift from collapse of external reflection into identity toward explicit reciprocal mediation of things-in-themselves in `a.3`.

Review outcome:

- review_pending
- notes: `a.2` boundary complete; handoff to `a.3` remains pending.

### Entry thg-a-a-003 — Reciprocal mediation collapses plurality into self-related determinateness

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 374
- lineEnd: 435

Summary:

In `a.3`, external reflection is articulated as reciprocal mediation among things-in-themselves, but the apparent plurality collapses into one self-relating thing-in-itself whose determinateness is its own reflected mediation, preparing the transition to property.

Key points: (KeyPoint)

- k1. External reflection presents things-in-themselves as extremes mediated through external concrete existence.
- k2. Their difference lies only in the connecting reference, with each determined through the other.
- k3. Because each has difference only in the other, the supposed plurality collapses into self-relation of one thing-in-itself.
- k4. Determinateness is internalized as essential self-mediation of thing-in-itself, immediately yielding property.

Claims: (Claim)

- c1. id: thg-a-a-003-c1
  - subject: external_reflection
  - predicate: structures
  - object: reciprocal_mediation_of_things_in_themselves_via_external_concrete_existence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [374-382] things-in-themselves are extremes; middle term is their external concrete existence.
    - [383-388] difference falls in their connecting reference while they remain indifferent as extremes.

- c2. id: thg-a-a-003-c2
  - subject: reciprocal_determination_of_things_in_themselves
  - predicate: entails
  - object: non_distinctness_of_the_supposed_extremes
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [389-404] each thing is determined through the other and has supposition in the other.
    - [405-412] since each has difference not in itself but in the other, they are not distinct things.

- c3. id: thg-a-a-003-c3
  - subject: thing_in_itself
  - predicate: has_determinateness_as
  - object: self_relation_to_itself_as_other_grounding_property
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [413-429] mediation is relation of thing-in-itself to itself; determinateness is essential self-mediation.
    - [430-435] the two collapse into one, and its self-reference as other constitutes determinateness.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-a-002
  - sourceClaimIds: [thg-a-a-003-c2]
  - sourceKeyPointIds: [k2, k3]
  - targetClaimIds: [thg-a-a-002-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `a.3` extends `a.2` by showing how collapse into identity determines the fate of plurality and mediation.

- r2. type: transitions_to
  - targetEntryId: thg-a-b-001
  - sourceClaimIds: [thg-a-a-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: final sentence of `a.3` states that this determinateness is the property of the thing, opening subpart `b`.

Review outcome:

- review_pending
- notes: `a` subpart (`1/2/3`) is now complete under explicit numbered-span discipline.

### Entry thg-a-b-001 — Property as reflective negativity and first-order posited reference

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 438
- lineEnd: 466

Summary:

In the initial and `First` movement of `b. Property`, property is defined as the negativity of reflection (not immediate quality), a self-referential mediation of thing-in-itself, and first as determinate reference-to-other that expresses the thing's posited side.

Key points: (KeyPoint)

- k1. Property is reflective negativity through which concrete existence is concrete existent and thing-in-itself.
- k2. Unlike immediate quality, this negativity is self-referential mediation (reference to self as other) and constitution.
- k3. As first determination, properties are determinate references to others and constitute the external reflective side of the thing.

Claims: (Claim)

- c1. id: thg-a-b-001-c1
  - subject: property_of_the_thing
  - predicate: is
  - object: negativity_of_reflection_constituting_thing_in_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [438-445] property is negativity of reflection by virtue of which concrete existence is concrete existent and thing-in-itself.

- c2. id: thg-a-b-001-c2
  - subject: reflective_negativity
  - predicate: is_determined_as
  - object: self_referential_mediation_and_constitution_excluded_from_simple_otherness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [446-457] sublated mediation is essential reference to self as other, not unreflected reference.
    - [454-459] thing-in-itself is determined in itself as constitution and excluded from passing into otherness.

- c3. id: thg-a-b-001-c3
  - subject: first_determination_of_property
  - predicate: is
  - object: determinate_reference_to_something_other_as_positedness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [460-466] "A thing has properties; these are, first, its determinate references to something other..." and this is the external reflection/posited side.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-a-003
  - sourceClaimIds: [thg-a-b-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [thg-a-a-003-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: explicit emergence of property in `b` concretizes the terminus reached at end of `a.3`.

- r2. type: transitions_to
  - targetEntryId: thg-a-b-002
  - sourceClaimIds: [thg-a-b-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: movement from `First` (property as reciprocal reference/positedness) to `Second` (property preserved in alteration).

Review outcome:

- review_pending
- notes: segmentation of `b` now follows explicit user rule: initial+`First`, then `Second`, then final paragraph.

### Entry thg-a-b-002 — Property preserved in alteration and proto-causal efficacy

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 467
- lineEnd: 489

Summary:

In the `Second` movement of `b. Property`, property is affirmed as the thing's self-identical substrate that persists through externality and alteration, thereby making the thing effective as cause while not yet fully actual cause.

Key points: (KeyPoint)

- k1. In positedness, the thing remains in itself, and property is not lost in exposure to becoming and alteration.
- k2. Property expresses efficacy in another only under corresponding constitution, while remaining the thing's own substrate.
- k3. Through properties the thing becomes cause, but at this stage it is still only immediate reflection, not yet positing reflection.

Claims: (Claim)

- c1. id: thg-a-b-002-c1
  - subject: property
  - predicate: is_preserved_in
  - object: externality_and_alteration_of_concrete_existence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [467-472] in positedness the thing is in itself; property is not lost under becoming and alteration.

- c2. id: thg-a-b-002-c2
  - subject: property
  - predicate: is
  - object: conditional_efficacy_and_self_identical_substrate_of_the_thing
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [473-480] property effects in another under corresponding constitution and is the thing's own self-identical substrate.

- c3. id: thg-a-b-002-c3
  - subject: thing_with_properties
  - predicate: becomes
  - object: cause_not_yet_actual_cause
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [481-489] through properties the thing becomes cause, but remains static and not yet actual cause/positing reflection.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-b-001
  - sourceClaimIds: [thg-a-b-002-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [thg-a-b-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `Second` deepens `First` by showing that posited reference does not dissolve in alteration.

- r2. type: transitions_to
  - targetEntryId: thg-a-b-003
  - sourceClaimIds: [thg-a-b-002-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: prepares the final grounding paragraph where property/ground identity is made explicit.

Review outcome:

- review_pending
- notes: `Second` paragraph captured as its own entry under the user-guided decomposition protocol.

### Entry thg-a-b-003 — Property as ground-connection and in-itselfness in externality

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 490
- lineEnd: 536

Summary:

In the final paragraph of `b. Property`, thing-in-itself is explicitly identified with its properties as ground-connection, where posited externality is internalized as reflected ground, yielding the unity of concrete external immediacy and in-itselfness.

Key points: (KeyPoint)

- k1. Thing-in-itself is not an indeterminate substrate behind concrete existence; in its properties it is present as ground and self-identity in positedness.
- k2. Ground here is conditioned and externally reflected, yet reflected into itself through that very externality.
- k3. Property is itself ground as implicitly existent positedness, and the thing concretely exists essentially as in-itselfness in external immediacy.

Claims: (Claim)

- c1. id: thg-a-b-003-c1
  - subject: thing_in_itself
  - predicate: is
  - object: ground_present_in_its_properties_not_substrate_behind_existence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [490-499] properties are own determinations; thing is not substrate on other side of existence.
    - [498-501] thing is present in properties as ground and self-identity in positedness.

- c2. id: thg-a-b-003-c2
  - subject: ground_connection_of_thinghood
  - predicate: is
  - object: conditioned_external_reflection_that_reflects_into_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [501-505] thing is conditioned ground; positedness is reflection external to itself.
    - [506-513] through concrete existence and alteration, thing remains reflected immediacy of the ground.

- c3. id: thg-a-b-003-c3
  - subject: property
  - predicate: is_determined_as
  - object: ground_as_implicit_positedness_unifying_external_immediacy_and_in_itselfness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [514-531] property is not separate from ground; it is ground passed into externality and reflected into itself.
    - [532-536] thing concretely exists essentially; external immediacy is at the same time in-itselfness.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-b-002
  - sourceClaimIds: [thg-a-b-003-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [thg-a-b-002-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: final grounding clarifies why `b.2` could treat causal efficacy as not yet fully positing reflection.

- r2. type: transitions_to
  - targetEntryId: thg-a-c-001
  - sourceClaimIds: [thg-a-b-003-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: completion of ground/property identity opens the c-movement into reciprocal action of things.

Review outcome:

- review_pending
- notes: `b. Property` is now complete under the user-guided three-part decomposition.

### Entry thg-a-c-001 — Reciprocal action reduces thinghood to property-determined reflection

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 539
- lineEnd: 587

Summary:

In the first paragraph of `c`, things are presented as reciprocally active through properties, but this interaction reveals that thinghood has essentiality only in property-determined reflection and that bare thinghood without determinate property collapses into external quantitative indifference.

Key points: (KeyPoint)

- k1. Thing-in-itself concretely exists as a property-bearing thing among distinct things in essential reciprocal action.
- k2. Property is the reciprocal connecting reference and middle term, so thinghood is reduced to indeterminate self-identity with essentiality only in property.
- k3. Without determinate property, thing-difference is merely quantitative/external, and what remains is unessential abstract in-itselfness.

Claims: (Claim)

- c1. id: thg-a-c-001-c1
  - subject: many_things_in_themselves
  - predicate: stand_in
  - object: essential_reciprocal_action_by_virtue_of_properties
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [539-548] thing-in-itself as thing with properties; many distinct things stand in essential reciprocal action.
    - [548-556] reciprocal determination is middle term and self-identical reflection.

- c2. id: thg-a-c-001-c2
  - subject: thinghood
  - predicate: is_reduced_to
  - object: indeterminate_self_identity_with_essentiality_only_in_property
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [556-559] thinghood reduced to indeterminate self-identity with essentiality only in property.
    - [560-575] determinateness of "this thing" lies solely in properties as differentiating reflection.

- c3. id: thg-a-c-001-c3
  - subject: thing_without_determinate_property
  - predicate: has
  - object: merely_external_quantitative_difference_and_unessential_remainder
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [560-568] without determinate property, difference is indifferent/quantitative and separations are external.
    - [576-587] without properties only unessential compass and abstract in-itselfness remain; thinghood passes over into property.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-b-003
  - sourceClaimIds: [thg-a-c-001-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [thg-a-b-003-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `c.1` operationalizes the `b.3` result by showing thinghood's essentiality only in property/ground reflection.

- r2. type: transitions_to
  - targetEntryId: thg-a-c-002
  - sourceClaimIds: [thg-a-c-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: from reduction of bare thinghood to property toward explicit critique of extremes/middle-term structure in paragraph 2.

Review outcome:

- review_pending
- notes: `c` paragraph 1 captured as a standalone entry under the natural three-paragraph decomposition.

### Entry thg-a-c-002 — Extremes vanish into property as the true continuity

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 589
- lineEnd: 601

Summary:

In `c` paragraph 2, the thing as supposed extreme and property as middle term are reconfigured by a single reflective continuity in which self-subsisting extremes vanish, leaving property as the effective connective reality.

Key points: (KeyPoint)

- k1. The initial schema posits thing as extreme and property as middle term between connected things.
- k2. The connection is self-repelling reflection in which distinction and connection are one continuity.
- k3. The supposed self-subsisting things vanish into that continuity, which is property.

Claims: (Claim)

- c1. id: thg-a-c-002-c1
  - subject: initial_thing_relation_schema
  - predicate: posits
  - object: thing_as_extreme_and_property_as_middle_term
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [589-592] thing as extreme term; property as middle term between connected things.

- c2. id: thg-a-c-002-c2
  - subject: connecting_reference_of_things
  - predicate: is
  - object: one_reflection_and_continuity_of_distinction_and_connection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [593-597] connection is where things meet as self-repelling reflection; distinction and connecting reference are one reflection/continuity.

- c3. id: thg-a-c-002-c3
  - subject: self_subsisting_extremes
  - predicate: vanish_into
  - object: continuity_that_is_property
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [598-601] things fall only within this continuity (property) and vanish as self-subsisting extremes outside it.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-c-001
  - sourceClaimIds: [thg-a-c-002-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [thg-a-c-001-c2]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: paragraph 2 sharpens paragraph 1 by formally collapsing supposed extremes into property-continuity.

- r2. type: transitions_to
  - targetEntryId: thg-a-c-003
  - sourceClaimIds: [thg-a-c-002-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: prepares paragraph 3 where property is made explicitly self-subsistent and things declared unessential.

Review outcome:

- review_pending
- notes: `c` paragraph 2 captured as a standalone entry in the user-guided decomposition sequence.

### Entry thg-a-c-003 — Property becomes self-subsistent matter and thing is reduced to unessential form

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/thing.txt`
- lineStart: 603
- lineEnd: 642

Summary:

In `c` paragraph 3, the inversion is completed: property, not the thing as such, is self-subsistent, while thinghood is reduced to an unessential unity, and the outcome is a manifold of self-subsisting matters constituting the thing.

Key points: (KeyPoint)

- k1. Property is declared self-subsistent, while things are unessential except as self-differentiating reflection.
- k2. The truth of the thing is an unessential compass, and the earlier abstraction is inverted by transition of thing-in-itself into property.
- k3. Property is freed from impotent unity and becomes self-subsisting matter, yielding a manifold of such matters composing the thing.

Claims: (Claim)

- c1. id: thg-a-c-003-c1
  - subject: property
  - predicate: is
  - object: self_subsistent_while_thing_as_such_is_unessential
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [603-610] property is self-subsistent; things are unessential except as self-referring reflection.
    - [611-616] truth of the thing is only an unessential compass/negative unity.

- c2. id: thg-a-c-003-c2
  - subject: evaluative_structure_of_thing_and_property
  - predicate: is
  - object: inverted_in_favor_of_property_through_transition_of_thing_in_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [617-623] earlier abstraction that omitted property is reversed through transition of thing-in-itself into property.
    - [624-631] abstract thing without property is no longer essential; thing is reduced to external form of property.

- c3. id: thg-a-c-003-c3
  - subject: property
  - predicate: becomes
  - object: self_subsisting_matter_manifold_constituting_the_thing
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [632-638] property freed from impotent bond and constitutes subsistence of the thing as self-subsisting matter.
    - [639-642] matter has diversity-form and a manifold of self-subsisting matters constitutes the thing.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-c-002
  - sourceClaimIds: [thg-a-c-003-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [thg-a-c-002-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: paragraph 3 consummates paragraph 2 by explicitly granting self-subsistence to property and nullifying self-subsisting extremes.

- r2. type: transitions_to
  - targetEntryId: thg-mat-b-001
  - sourceClaimIds: [thg-a-c-003-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: final line hands off to Part B (`THE CONSTITUTION OF THE THING OUT OF MATTERS`) via the manifold of self-subsisting matters.

Review outcome:

- review_pending
- notes: `c` paragraph 3 captured; Part A decomposition is now fully complete (`a + b + c`).
