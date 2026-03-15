# Judgment Part C Workbook

Part: `C. JUDGMENT OF NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
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
- summary: one sentence focusing on the active cognitive movement of the judging Subject.
- keyPoints: (KeyPoint) 3-8 non-redundant points explicitly naming the cognitive activity.
- claims: (Claim) 1-3 minimum, formalizing subjective operations with evidence.
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)
  - logicalOperator: the specific self-acting cognitive operation.
  - cognitiveOperation: precise protocol describing the living logic move of the Subject.

## Session: 2026-03-10 (Cognitive Protocol Upgrade)

Scope:

- file: `necessity.txt`
- fixed range: lines `4-449`
- pass policy: Upgrading "Logic" to active protocols implementing the living logic via precise Logical Operations.

Decision:

- Transform relations to include `logicalOperator` and `cognitiveOperation`.
- Refocus key points and claims to reflect the "self-acting" cognitive operations of the Subject.
- Preserve the existing numbered ID pattern (`judgm-nec-<letter>-<nnn>`).
- Preserve transition readiness to Part D (`judgment of concept`).

### Entry judgm-nec-i-001 — Necessity setup: positing objective universality as genus/species ground

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 4
- lineEnd: 24

Summary:

The Subject actively posits objective universality not as a static substance, but as the concept-immanent necessity that dialectically compels its own differentiation into the genus/species relation.

Key points: (KeyPoint)

- k1. The Subject conceptualizes objective universality as possessing its distinctions immanently, unlike external substantiality.
- k2. The Subject actively posits this internal distinction as logical necessity.
- k3. The Subject organizes this necessary differentiation into the active structural relation of genus and species.

Claims: (Claim)

- c1. id: judgm-nec-i-001-c1
  - subject: cognitive_subject
  - predicate: determines_objective_universality_as
  - object: concept_immanent_posited_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-13] universality exists in-and-for-itself and is posited necessity of determinations.
    - [14-15] contrast: substance does not hold distinction as internal principle.

- c2. id: judgm-nec-i-001-c2
  - subject: cognitive_subject
  - predicate: organizes_universality_into
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
  - targetEntryId: judgm-ref-c-003
  - note: necessity setup presupposes Reflection's copular identity result.
  - sourceClaimIds: [`judgm-nec-i-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`judgm-ref-c-003-c3`]
  - logicalOperator: conceptual_presupposition
  - cognitiveOperation: The Subject takes the hard-won copular identity from the judgment of reflection and establishes it as the baseline objective necessity for all subsequent categorical operations.
  - analysisMode: active_cognitive_protocol

- r2. type: transitions_to
  - targetEntryId: judgm-nec-a-001
  - note: setup proceeds directly into the first categorical subtopic.
  - sourceClaimIds: [`judgm-nec-i-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-nec-a-001-c1`]
  - logicalOperator: sequential_transition
  - cognitiveOperation: The Subject initiates the exploration of this new genus/species matrix by examining its most immediate form.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: base setup entry fixed for Part C.

### Entry judgm-nec-a-001 — Formulating the immediate categorical: genus and external singularity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 28
- lineEnd: 52

Summary:

The Subject executes a reciprocal but immediate categorical subsumption, placing a singular subject under a genus that initially acts only as a higher universality, not yet completely owning the subject's specific determinateness.

Key points: (KeyPoint)

- k1. The Subject constructs the categorical judgment as the first, immediate expression of necessity.
- k2. The Subject recognizes a reciprocal dependency: the genus divides into species, while the species achieves higher universality through the genus.
- k3. The Subject critically observes that objective universality remains at an immediate stage, failing to act directly as the proximate generating principle of the subject's specific singularity.

Claims: (Claim)

- c1. id: judgm-nec-a-001-c1
  - subject: cognitive_subject
  - predicate: structures_genus_species_relation_as
  - object: reciprocal_necessity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [28-35] reciprocal genus/species determination.

- c2. id: judgm-nec-a-001-c2
  - subject: cognitive_subject
  - predicate: executes_categorical_judgment_as
  - object: immediate_judgment_of_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [36-40] predicate is universality in which subject has immanent nature; first/immediate necessity.

- c3. id: judgm-nec-a-001-c3
  - subject: cognitive_subject
  - predicate: evaluates_objective_universality_as
  - object: immediate_particularization_lacking_proximate_principle
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [46-52] determinate genus relation still immediate and not directly specific principle.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k1
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-nec-a-002
  - note: The immediate formulation forces the Subject to seek a deeper substantial identity.
  - sourceClaimIds: [`judgm-nec-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-nec-a-002-c1`]
  - logicalOperator: immediate_categorical_subsumption
  - cognitiveOperation: The Subject structures an initial subsumption where the genus hovers over the singular subject as a valid classification, but registers that the specific "why" of the subject's determinateness remains externally given.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: first categorical cognitive action.

### Entry judgm-nec-a-002 — Purging accidentality: establishing categorical copula-necessity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 54
- lineEnd: 85

Summary:

Reflecting on the inadequacy of mere external properties, the Subject aggressively purges accidental predicates from its logic, firmly fusing subject and predicate through the copula as an expression of substantial, immanent identity.

Key points: (KeyPoint)

- k1. The Subject redefines the categorical predicate to exclusively signify the subject's immanent nature, forcefully distinguishing it from external, contingent properties (e.g., distinguishing "is a plant" from "is red").
- k2. The Subject treats any remaining distinguishing marks between subject and predicate as unessential "positedness" or mere naming.
- k3. The Subject actively loads the copula with the strict meaning of necessity, superseding the abstract "is" of previous judgment forms.

Claims: (Claim)

- c1. id: judgm-nec-a-002-c1
  - subject: cognitive_subject
  - predicate: establishes_necessity_as
  - object: substantial_identity_of_subject_and_predicate
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [54-58] substantial identity; distinctions as unessential positedness.

- c2. id: judgm-nec-a-002-c2
  - subject: cognitive_subject
  - predicate: purges_and_bans
  - object: accidental_external_properties_from_predicate
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [59-73] rose-red vs rose-plant marks accidental vs immanent nature.

- c3. id: judgm-nec-a-002-c3
  - subject: cognitive_subject
  - predicate: defines_categorical_copula_as
  - object: strict_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [74-85] categorical distinguished from positive/negative; copula means necessity.

Claim ↔ key point map:

- c1 -> k2
- c2 -> k1
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-nec-a-003
  - note: Copula necessity still leaves a remainder of unexplained contingency in the subject.
  - sourceClaimIds: [`judgm-nec-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-nec-a-003-c1`]
  - logicalOperator: purging_accidentality_into_substantial_identity
  - cognitiveOperation: The Subject executes a conceptual filtering, deliberately stripping away accidental attributes to isolate the sheer substantial identity holding the categorical judgment together.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: second categorical cognitive action.

### Entry judgm-nec-a-003 — Confronting contingent determinateness and demanding essentiality

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 87
- lineEnd: 102

Summary:

Discovering that the subject’s particularizing determinateness still operates contingently against the predicate, the Subject actively insists that this determinateness must be conceptually bound as an essential self-determination of the objective universal.

Key points: (KeyPoint)

- k1. The Subject detects that the specific trait making the subject a "particular" is still loosely/contingently attached to the predicate.
- k2. The Subject realizes the necessity achieved so far remains merely "inner" because the outer form is not strictly logically compelled.
- k3. The Subject enforces a prescriptive logical demand: the objective universal must not treat its own determinateness as accidental, but must actively posit it as its own essential identity.

Claims: (Claim)

- c1. id: judgm-nec-a-003-c1
  - subject: cognitive_subject
  - predicate: diagnoses_subject_determinateness_as
  - object: contingently_connected_to_predicate
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [87-93] determinateness contingent; necessity still inner.

- c2. id: judgm-nec-a-003-c2
  - subject: cognitive_subject
  - predicate: demands_that_objective_universal
  - object: posit_determinateness_as_essential_identity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [98-102] determinateness is not to be posited as merely accidental.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-nec-a-004
  - note: Binding the determinateness as essential forces the logical form out of the categorical frame entirely.
  - sourceClaimIds: [`judgm-nec-a-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-nec-a-004-c1`]
  - logicalOperator: demand_for_essential_determinateness
  - cognitiveOperation: The Subject issues a logical injunction against contingency, demanding that the universal take full conceptual responsibility for the specific differences that individualize its subjects.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: third categorical cognitive action.

### Entry judgm-nec-a-004 — Transition paragraph: exploding into the hypothetical judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 104
- lineEnd: 107

Summary:

The Subject realizes that the categorical judgment relies entirely on the necessity of immediate being, and systematically dismantles the categorical frame to explicitly articulate this conditional dependency as the hypothetical judgment.

Key points: (KeyPoint)

- k1. The Subject acknowledges that categorical adequacy is conditioned heavily on the necessity of immediate being.
- k2. The Subject actively transitions to a new form to express the conditionality that inherently underpins that objective universality.

Claims: (Claim)

- c1. id: judgm-nec-a-004-c1
  - subject: cognitive_subject
  - predicate: determines_categorical_adequacy_as_conditioned_on
  - object: necessity_of_immediate_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [104-106] conformity through necessity of immediate being.

- c2. id: judgm-nec-a-004-c2
  - subject: cognitive_subject
  - predicate: executes_transition_to
  - object: hypothetical_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [107-107] explicit transition sentence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-nec-b-001
  - note: Transition into examining the conditional mechanics of necessity.
  - sourceClaimIds: [`judgm-nec-a-004-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`judgm-nec-b-001-c1`]
  - logicalOperator: transition_to_conditional_necessity
  - cognitiveOperation: The Subject actively ruptures the simple categorical "is", unfolding the hidden dependency within it into an explicit "if-then" linkage to expose the outer scaffolding of necessity.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: final phase of categorical sequence.

### Entry judgm-nec-b-001 — Deploying conditional form: positing finite being as being-of-an-other

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 111
- lineEnd: 163

Summary:

The Subject suspends the immediate assertion of the extremes ("A" and "B") and constructs a conditional "if-then" bond, conceptualizing finite being unequivocally as the being-of-an-other.

Key points: (KeyPoint)

- k1. The Subject deploys the conditional "if-then" expression to trace the outer architecture of necessity.
- k2. The Subject deliberately refrains from asserting the actual existence of the extremes, solely positing their necessary connective link.
- k3. The Subject articulates the conceptual truth of finite things: a finite entity is simultaneously its own being and intrinsically the being of something else.

Claims: (Claim)

- c1. id: judgm-nec-b-001-c1
  - subject: cognitive_subject
  - predicate: structures_necessity_as
  - object: conditional_if_then_link
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [111-117] explicit formula and necessary connectedness.

- c2. id: judgm-nec-b-001-c2
  - subject: cognitive_subject
  - predicate: suspends_positing_of
  - object: actual_existence_of_extremes
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [134-138] connectedness exists; extremes not posited as existing.

- c3. id: judgm-nec-b-001-c3
  - subject: cognitive_subject
  - predicate: determines_finite_being_as
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
  - targetEntryId: judgm-nec-b-002
  - note: Examining the hypothetical link reveals it as an expression of underlying unified universality.
  - sourceClaimIds: [`judgm-nec-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-nec-b-002-c1`]
  - logicalOperator: conditional_positing_of_being_as_other
  - cognitiveOperation: The Subject brackets the absolute existence of independent entities, rewiring its logic to exclusively recognize "things" only as nodes defined entirely by their necessary connective transmissions to "other things".
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: first hypothetical cognitive action.

### Entry judgm-nec-b-002 — Sublating the proposition-form into concrete universality

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 165
- lineEnd: 204

Summary:

Recognizing that the "if-then" form remains an indeterminate proposition masking a deeper conceptual unity, the Subject sublates this conditionality, collapsing the relational dependencies back into the single concrete identity of the concept itself.

Key points: (KeyPoint)

- k1. The Subject observes that ground/consequence and causal reflections are merely recurring moments of a single unified identity.
- k2. The Subject critically dismisses the hypothetical judgment's shape as overly proposition-like and structurally indeterminate regarding subject and predicate.
- k3. The Subject actively translates the dependent connection of "being of an other" into the explicit truth of the situation: a concrete, self-identical universality holding determinations as its own dependent particularities.

Claims: (Claim)

- c1. id: judgm-nec-b-002-c1
  - subject: cognitive_subject
  - predicate: evaluates_hypothetical_relations_as
  - object: moments_of_single_identical_universality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [165-175] ground/consequence, causality, etc. recur as moments of one identity.

- c2. id: judgm-nec-b-002-c2
  - subject: cognitive_subject
  - predicate: critiques_hypothetical_form_as
  - object: proposition_like_and_structurally_indeterminate
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [180-185] shape more like proposition; indeterminate form/content conformity.

- c3. id: judgm-nec-b-002-c3
  - subject: cognitive_subject
  - predicate: sublates_conditionality_into
  - object: concrete_universality_of_the_concept
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
  - targetEntryId: judgm-nec-c-001
  - note: Concrete universality inherently demands the disjunctive articulation of its implicit particularities.
  - sourceClaimIds: [`judgm-nec-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-nec-c-001-c1`]
  - logicalOperator: sublation_of_conditionality_into_concrete_universality
  - cognitiveOperation: The Subject short-circuits the linear "if A then B" causal chain, folding it inward so that A and B are revealed merely as two internal organs functioning within the singular biological body of the Concept.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: final hypothetical cognitive action.

### Entry judgm-nec-c-001 — Executing conceptual disjunctive fracturing: the either/or

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 208
- lineEnd: 316

Summary:

The Subject fractures the universal into the disjunctive judgment ("A is either B or C"), strictly organizing the species not as an empirical grab-bag, but as a necessary totality where species simultaneously unite in the genus and definitively exclude each other.

Key points: (KeyPoint)

- k1. The Subject deploys the "either-or" mechanism not as mere choice, but as the strict necessity of a conceptual totality.
- k2. The Subject manages a dual logic: species maintain absolute positive identity within their genus, while exerting absolute negative mutual exclusion against parallel species.
- k3. The Subject consciously wholly rejects empirical or enumerative disjunction as a "barbarism" lacking an immanent, conceptually necessary principle.

Claims: (Claim)

- c1. id: judgm-nec-c-001-c1
  - subject: cognitive_subject
  - predicate: organizes_disjunction_as
  - object: necessary_conceptual_totality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [223-238] either-or as necessity of concept with identity, difference, and totality.

- c2. id: judgm-nec-c-001-c2
  - subject: cognitive_subject
  - predicate: establishes_species_relations_as
  - object: simultaneously_identical_in_genus_and_mutually_excluding
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [249-260] positive and negative connection in one genus-unity.
    - [311-316] unity is truth of contrary/contradictory determinations.

- c3. id: judgm-nec-c-001-c3
  - subject: cognitive_subject
  - predicate: rejects_and_bans
  - object: empirical_unprincipled_disjunction
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
  - targetEntryId: judgm-nec-c-002
  - note: Proper disjunctive fracturing allows the Subject to identify the proximate genus leading to the Concept itself.
  - sourceClaimIds: [`judgm-nec-c-001-c1`, `judgm-nec-c-001-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`judgm-nec-c-002-c1`]
  - logicalOperator: disjunctive_conceptual_fracturing
  - cognitiveOperation: The Subject actively splits the conceptual genus with absolute surgical precision, ensuring the resulting "pieces" (species) completely exhaust the logical space of the universal totality with zero overlap and zero remainder.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: first disjunctive cognitive action.

### Entry judgm-nec-c-002 — Finalizing the proximate genus and resolving into the posited concept

Span:

- sourceFile: `src/compiler/concept/subject/judgment/necessity.txt`
- lineStart: 318
- lineEnd: 449

Summary:

By proving that the fully articulated proximate genus drives its own disjunction, the Subject resolves the absolute identity of subject and predicate into the copula, finalizing the transition of "necessity" into the pure "Judgment of the Concept."

Key points: (KeyPoint)

- k1. The Subject identifies the proximate genus as the concrete universality that literally contains its species-differences as its own essential determinateness.
- k2. The Subject verifies that valid disjunction proceeds exclusively from the concept's own progressive self-determination, not from external forces.
- k3. The Subject executes the final logical sublation: fusing the subject (genus) and predicate (totality of species) completely within the copula, which is now revealed as the posited concept itself.

Claims: (Claim)

- c1. id: judgm-nec-c-002-c1
  - subject: cognitive_subject
  - predicate: defines_proximate_genus_as
  - object: concrete_universality_possessing_immanent_species_difference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [343-353] concrete genus as unity of concept-moments with real difference in species.

- c2. id: judgm-nec-c-002-c2
  - subject: cognitive_subject
  - predicate: drives_disjunction_through
  - object: progressive_self_determination_of_the_concept
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [381-394] concept's progressive determination posits disjunction and totality of particulars.
    - [395-399] invalid disjunction indicates failure to proceed from concept.

- c3. id: judgm-nec-c-002-c3
  - subject: cognitive_subject
  - predicate: resolves_judgment_into
  - object: copular_unity_as_the_posited_concept_itself
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

- r1. type: transitions_to
  - targetEntryId: judgm-con-i-001
  - note: Direct Part D boundary handoff.
  - sourceClaimIds: [`judgm-nec-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending`]
  - logicalOperator: resolution_into_posited_concept
  - cognitiveOperation: The Subject forces the extreme ends of the judgment to collapse perfectly upon one another, recognizing that the "is" (the copula) is no longer a mere link, but the fully manifest, living Concept that now actively dictates its own internal logic.
  - analysisMode: active_cognitive_protocol

Review outcome:

- review_pending
- notes: final disjunctive cognitive action and Part C bridge to Part D.
