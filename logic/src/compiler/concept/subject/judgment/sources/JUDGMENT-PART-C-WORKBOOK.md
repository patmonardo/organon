# Judgment Part C Workbook

Part: `C. JUDGMENT OF NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `necessity.txt` as authority.
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

### Entry (Topic) `id` — `title`

- span: `lineStart-lineEnd`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-21 (first Necessity seed pass)

Scope:

- file: `necessity.txt`
- fixed range: lines `4-449`
- pass policy: seed pass now; span-analysis optimization deferred to post A-D review

Decision:

- Use the same two-layer Entry system as Parts A-B:
  - Level 1 marker entries: `jud-nes-a`, `jud-nes-b`, `jud-nes-c`
  - Level 2 numbered entries under each marker: `jud-nes-<letter>-<nnn>`
- Keep this pass minimally decomposed while preserving explicit transitions.
- Keep Part D handoff explicit but claim target pending until Part D extraction.

### Entry jud-nes-001 — Necessity setup: objective universality as genus/species ground

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 4
- lineEnd: 24

Summary:

Necessity-judgment begins from objective universality as concept-immanent necessity, where universality is posited as substantial basis of differentiated particularity (genus/species).

Key points: (KeyPoint)

- k1. Objective universality is in-and-for-itself universality corresponding to substantiality.
- k2. In concept, distinction is immanent and posited as necessity.
- k3. Universality is determined as genus/species relation.

Claims: (Claim)

- c1. id: jud-nes-001-c1
  - subject: objective_universality
  - predicate: is_determined_as
  - object: concept_immanent_posited_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-13] universality exists in-and-for-itself and is posited necessity of determinations.
    - [14-15] contrast: substance does not hold distinction as internal principle.

- c2. id: jud-nes-001-c2
  - subject: universality_in_judgment_of_necessity
  - predicate: grounds
  - object: genus_species_determination
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [17-24] objective universality posited as substantial basis of diverse particularity; genus/species.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: jud-ref-c-003
  - note: necessity setup presupposes Reflection's copular identity result.
  - sourceClaimIds: [`jud-nes-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`jud-ref-c-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: jud-nes-a
  - note: setup proceeds into categorical judgment as first necessity form.
  - sourceClaimIds: [`jud-nes-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: base setup entry fixed for Part C.

### Entry jud-nes-a — Marker `a`: categorical judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 26
- lineEnd: 107

Summary:

Categorial necessity presents subject-predicate substantial identity in genus/species form, while immediate subject determinateness remains contingent and drives transition to the hypothetical.

Key points: (KeyPoint)

- k1. Genus/species relation structures categorical judgment.
- k2. Its necessary core is substantial identity of subject and predicate.
- k3. Immediate determinateness remains only inwardly necessary, forcing transition.

Claims: (Claim)

- c1. id: jud-nes-a-c1
  - subject: categorical_judgment
  - predicate: is_determined_as
  - object: immediate_judgment_of_necessity_in_genus_species_form
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [28-40] genus divides into species; categorical is first/immediate judgment of necessity.

- c2. id: jud-nes-a-c2
  - subject: necessity_in_categorical_judgment
  - predicate: is
  - object: substantial_identity_of_subject_and_predicate
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [53-58] substantial identity; distinctions only unessential positedness.
    - [81-84] copula has meaning of necessity.

- c3. id: jud-nes-a-c3
  - subject: categorical_judgment
  - predicate: transitions_to
  - object: hypothetical_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [86-93] determinateness still contingent; necessity still inner.
    - [103-107] by necessity of immediate being it passes over into hypothetical.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-a-001
  - note: marker to numbered articulation of categorical necessity.
  - sourceClaimIds: [`jud-nes-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`jud-nes-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: jud-nes-b
  - note: explicit handoff from categorical to hypothetical marker.
  - sourceClaimIds: [`jud-nes-a-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-b-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for layer-1 node `a`.

### Entry jud-nes-a-001 — Genus/species immediate structure of the categorical

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 28
- lineEnd: 52

Summary:

The first paragraph presents categorical judgment as immediate necessity in genus/species form, but with subject determinateness still tied to external concrete immediacy.

Key points: (KeyPoint)

- k1. Genus exists only through species and species through genus/singulars.
- k2. Categorical judgment is first/immediate necessity.
- k3. Objective universality is still first immediate particularization and not yet proximate genus-principle.

Claims: (Claim)

- c1. id: jud-nes-a-001-c1
  - subject: genus_species_relation
  - predicate: is
  - object: reciprocal_structure_of_necessity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [28-35] reciprocal genus/species determination.

- c2. id: jud-nes-a-001-c2
  - subject: categorical_judgment
  - predicate: is_determined_as
  - object: first_or_immediate_judgment_of_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [36-40] predicate is universality in which subject has immanent nature; first/immediate necessity.

- c3. id: jud-nes-a-001-c3
  - subject: objective_universality_in_categorical_form
  - predicate: remains
  - object: immediate_particularization_not_yet_proximate_genus_principle
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [46-52] determinate genus relation still immediate and not directly specific principle.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-a-002
  - note: first paragraph gives way to substantial identity and non-accidental predicate determination.
  - sourceClaimIds: [`jud-nes-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first numbered child under marker `a` (first paragraph).

### Entry jud-nes-a-002 — Substantial identity and categorical copula-necessity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 54
- lineEnd: 85

Summary:

The second paragraph states substantial identity of subject and predicate, distinguishes immanent predicate-content from accidental predicates, and fixes the categorical copula as necessity.

Key points: (KeyPoint)

- k1. Necessity here is substantial identity of subject and predicate.
- k2. Categorical predicate is immanent nature, not accidental property.
- k3. Copula signifies necessity, unlike abstract immediate being.

Claims: (Claim)

- c1. id: jud-nes-a-002-c1
  - subject: necessity_in_categorical_judgment
  - predicate: is
  - object: substantial_identity_of_subject_and_predicate
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [54-58] substantial identity; distinctions as unessential positedness.

- c2. id: jud-nes-a-002-c2
  - subject: categorical_predicate
  - predicate: is_determined_as
  - object: immanent_nature_not_external_property
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [59-73] rose-red vs rose-plant marks accidental vs immanent nature.

- c3. id: jud-nes-a-002-c3
  - subject: categorical_copula
  - predicate: means
  - object: necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [74-85] categorical distinguished from positive/negative; copula means necessity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-a-003
  - note: from substantial identity account to paragraph on remaining contingency/inner necessity.
  - sourceClaimIds: [`jud-nes-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second numbered child under marker `a` (lines 54-85).

### Entry jud-nes-a-003 — Contingent determinateness and inner necessity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 87
- lineEnd: 102

Summary:

The third paragraph shows that subject-particularity remains initially contingent, so necessity is still inner and objective universality must bind determinateness as non-accidental.

Key points: (KeyPoint)

- k1. Subject-particularity is at first contingent with respect to predicate.
- k2. Necessity is first inner, not yet fully formalized by determinateness.
- k3. Determinateness must be posited as essential, not merely accidental.

Claims: (Claim)

- c1. id: jud-nes-a-003-c1
  - subject: subject_determinateness
  - predicate: is_initially
  - object: contingent_with_respect_to_predicate
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [87-93] determinateness contingent; necessity still inner.

- c2. id: jud-nes-a-003-c2
  - subject: objective_universal_self_determination
  - predicate: requires
  - object: non_accidental_identity_with_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [98-102] determinateness is not to be posited as merely accidental.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-a-004
  - note: final paragraph gives explicit passage from categorical to hypothetical judgment.
  - sourceClaimIds: [`jud-nes-a-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-a-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third numbered child under marker `a` (prior paragraph).

### Entry jud-nes-a-004 — Transition paragraph: passage to hypothetical judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 104
- lineEnd: 107

Summary:

The final paragraph states that only through necessity of immediate being does the categorical conform to objective universality and pass over into the hypothetical judgment.

Key points: (KeyPoint)

- k1. Immediate being must be necessary for categorical adequacy.
- k2. Categorical judgment conforms thereby to objective universality.
- k3. This conformity explicitly transitions to the hypothetical judgment.

Claims: (Claim)

- c1. id: jud-nes-a-004-c1
  - subject: categorical_judgment
  - predicate: conforms_to
  - object: objective_universality_through_necessity_of_immediate_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [104-106] conformity through necessity of immediate being.

- c2. id: jud-nes-a-004-c2
  - subject: categorical_judgment
  - predicate: passes_over_into
  - object: hypothetical_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [107-107] explicit transition sentence.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-b-001
  - note: transition paragraph hands off into the first hypothetical subtopic.
  - sourceClaimIds: [`jud-nes-a-004-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition subtopic for marker `a`; completes marker `a` numbered layer.

### Entry jud-nes-b — Marker `b`: hypothetical judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 109
- lineEnd: 204

Summary:

Hypothetical judgment explicitly posits necessary connectedness of immediate determinacies, where each extreme is the being of the other, and develops toward concrete universality as disjunctive form.

Key points: (KeyPoint)

- k1. Hypothetical form posits connectedness, not independent extremes.
- k2. Finite being is both its own and the being of an other.
- k3. Connected identity of differentiated moments yields disjunctive judgment.

Claims: (Claim)

- c1. id: jud-nes-b-c1
  - subject: hypothetical_judgment
  - predicate: posits
  - object: necessary_connectedness_of_immediate_determinacies
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [111-117] if A then B; connectedness now posited.
    - [134-138] connectedness posited as existing, not extremes.

- c2. id: jud-nes-b-c2
  - subject: finite_being_in_hypothetical_form
  - predicate: is
  - object: own_being_and_being_of_other
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [145-149] finite is its own being and being of an other.
    - [158-163] concept posits concrete self-identity as being-of-other.

- c3. id: jud-nes-b-c3
  - subject: hypothetical_judgment
  - predicate: transitions_to
  - object: disjunctive_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [200-204] concrete identity of concept with dependent particularities yields disjunctive judgment.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-b-001
  - note: marker to first numbered articulation of conditional necessity.
  - sourceClaimIds: [`jud-nes-b-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`jud-nes-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: jud-nes-c
  - note: explicit handoff from hypothetical to disjunctive marker.
  - sourceClaimIds: [`jud-nes-b-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-c-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for layer-1 node `b`.

### Entry jud-nes-b-001 — Conditional form and being-of-another

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 111
- lineEnd: 163

Summary:

Hypothetical judgment expresses conditional necessity where immediacy is possible-being and each term is posited as the being of an other.

Key points: (KeyPoint)

- k1. Conditional expression externalizes necessity not yet explicit in categorical form.
- k2. Extremes are not asserted as such, only their conditional link.
- k3. Conceptual truth of finite being is being-of-other.

Claims: (Claim)

- c1. id: jud-nes-b-001-c1
  - subject: hypothetical_form
  - predicate: is
  - object: if_a_then_b_conditional_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [111-117] explicit formula and necessary connectedness.

- c2. id: jud-nes-b-001-c2
  - subject: hypothetical_link
  - predicate: posits
  - object: connectedness_without_asserting_extremes
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [134-138] connectedness exists; extremes not posited as existing.

- c3. id: jud-nes-b-001-c3
  - subject: finite_being
  - predicate: is_formally_true_as
  - object: being_of_an_other
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [145-149] finite both own being and being of an other.
    - [158-163] concept posits identity as being-of-other.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-b-002
  - note: from conditional structure to conceptual-universal determination.
  - sourceClaimIds: [`jud-nes-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first numbered child under marker `b`.

### Entry jud-nes-b-002 — Indeterminate proposition-form to concrete universality

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 165
- lineEnd: 204

Summary:

Although hypotheticals still appear proposition-like and indeterminate in form/content matching, their true result is concrete universality where determinations are dependent particularities.

Key points: (KeyPoint)

- k1. Reflection relations recur as moments of one identity.
- k2. Hypothetical retains proposition-like indeterminacy.
- k3. Its truth is concrete conceptual universality.

Claims: (Claim)

- c1. id: jud-nes-b-002-c1
  - subject: hypothetical_judgment_relations
  - predicate: are
  - object: moments_of_single_identity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [165-175] ground/consequence, causality, etc. recur as moments of one identity.

- c2. id: jud-nes-b-002-c2
  - subject: hypothetical_form
  - predicate: remains
  - object: proposition_like_and_indeterminate
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [180-185] shape more like proposition; indeterminate form/content conformity.

- c3. id: jud-nes-b-002-c3
  - subject: posited_truth_of_hypothetical
  - predicate: is
  - object: universality_as_concrete_identity_of_concept
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [200-204] universality as concrete identity; determinations as dependent particularities; hence disjunctive.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-c-001
  - note: concrete universality now appears as explicit disjunctive form.
  - sourceClaimIds: [`jud-nes-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes marker `b` numbered layer.

### Entry jud-nes-c — Marker `c`: disjunctive judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 206
- lineEnd: 449

Summary:

Disjunctive judgment posits objective universality in union with form: species are positively grounded in genus and negatively exclusive, and this identity culminates in the copula as posited concept, transitioning to judgment of concept.

Key points: (KeyPoint)

- k1. Disjunction unites concrete genus-universality with differentiated totality.
- k2. Species are both positively identical in genus and negatively exclusive.
- k3. True disjunction is concept-immanent, not empirical listing.
- k4. Copular unity of subject/predicate rises to judgment of concept.

Claims: (Claim)

- c1. id: jud-nes-c-c1
  - subject: disjunctive_judgment
  - predicate: is_determined_as
  - object: objective_universality_united_with_form
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [216-223] objective universality in union with form; genus plus differentiated totality.

- c2. id: jud-nes-c-c2
  - subject: species_relation_in_disjunction
  - predicate: is
  - object: positive_identity_and_negative_exclusion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [245-260] B as well as C (positive identity) and either-or (negative connection).
    - [311-316] unity of exclusion and concrete universality as truth.

- c3. id: jud-nes-c-c3
  - subject: disjunctive_copula
  - predicate: raises
  - object: judgment_of_necessity_to_judgment_of_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [443-447] unity/coupling is concept itself as posited.
    - [448-449] explicit rise to judgment of concept.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-c-001
  - note: marker to numbered articulation of disjunctive necessity structure.
  - sourceClaimIds: [`jud-nes-c-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`jud-nes-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: jud-conc-001
  - note: boundary handoff into Part D (judgment of concept) to be concretized in Part D extraction.
  - sourceClaimIds: [`jud-nes-c-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`pending`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for layer-1 node `c`.

### Entry jud-nes-c-001 — Disjunctive form as concrete universal totality

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 208
- lineEnd: 316

Summary:

The disjunctive form presents genus-universality as totalized particularity in which species are simultaneously united and excluding, grounding necessary rather than empirical disjunction.

Key points: (KeyPoint)

- k1. "Either-or" is conceptual necessity, not mere external enumeration.
- k2. Species hold positive identity in genus and negative exclusion in difference.
- k3. Empirical disjunction lacks this immanent principle.

Claims: (Claim)

- c1. id: jud-nes-c-001-c1
  - subject: disjunctive_form
  - predicate: expresses
  - object: necessity_of_conceptual_totality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [223-238] either-or as necessity of concept with identity, difference, and totality.

- c2. id: jud-nes-c-001-c2
  - subject: species_in_disjunction
  - predicate: are
  - object: positively_identical_and_negatively_connected
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [249-260] positive and negative connection in one genus-unity.
    - [311-316] unity is truth of contrary/contradictory determinations.

- c3. id: jud-nes-c-001-c3
  - subject: empirical_disjunction
  - predicate: lacks
  - object: immanent_necessity_of_totality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [270-279] empirical listing gives subjective completeness only.
    - [286-293] differences rest on external accidental principle.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: jud-nes-c-002
  - note: from disjunctive necessity to proximate-genus and copular culmination.
  - sourceClaimIds: [`jud-nes-c-001-c1`, `jud-nes-c-001-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`jud-nes-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first numbered child under marker `c`.

### Entry jud-nes-c-002 — Proximate genus, conceptive disjunction, and rise to concept-judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/sources/necessity.txt`
- lineStart: 318
- lineEnd: 449

Summary:

Disjunctive judgment determines genus as proximate through concept-immanent differentiation, then shows subject/predicate identity in the copula as the posited concept that lifts necessity-judgment into judgment of concept.

Key points: (KeyPoint)

- k1. Proximate genus is concrete universality with species-difference in its own determinateness.
- k2. Valid disjunction proceeds from concept, not arbitrary empirical partitions.
- k3. Subject-predicate/categorical split resolves in copula as concept itself.

Claims: (Claim)

- c1. id: jud-nes-c-002-c1
  - subject: proximate_genus
  - predicate: is_determined_as
  - object: concrete_universality_with_immanent_species_difference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [343-353] concrete genus as unity of concept-moments with real difference in species.

- c2. id: jud-nes-c-002-c2
  - subject: concept
  - predicate: posits
  - object: its_own_disjunction
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [381-394] concept's progressive determination posits disjunction and totality of particulars.
    - [395-399] invalid disjunction indicates failure to proceed from concept.

- c3. id: jud-nes-c-002-c3
  - subject: copula_unity_of_disjunctive_judgment
  - predicate: is
  - object: concept_as_posited_and_transition_to_judgment_of_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [443-447] unity is concept itself as posited.
    - [448-449] necessity judgment rises to judgment of concept.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: jud-nes-c
  - note: grounds marker-level transition claim from disjunctive necessity to concept judgment.
  - sourceClaimIds: [`jud-nes-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`jud-nes-c-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: jud-conc-001
  - note: direct Part D boundary handoff; claim target to be set during Part D extraction.
  - sourceClaimIds: [`jud-nes-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes marker `c` numbered layer and Part C seed-pass boundary.
