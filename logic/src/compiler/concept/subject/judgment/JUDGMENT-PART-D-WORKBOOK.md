# Judgment Part D Workbook

Part: `D. JUDGMENT OF THE CONCEPT`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `concept.txt` as authority.
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

## Session: 2026-02-21 (first Concept-judgment seed pass)

Scope:

- file: `concept.txt`
- fixed range: lines `4-374`
- pass policy: seed pass now; span-analysis and claim refinement deferred to dedicated review phase

Decision:

- Remove marker-only entries for this pass and keep only substantive, evidence-bearing entries.
- Keep the existing numbered ID pattern (`judgm-con-<letter>-<nnn>`) so inbound/outbound cross-part links remain stable.
- Keep this pass minimal but preserve the key special role of Part D: explicit copula repletion and transition to syllogism.
- Keep syllogism handoff explicit, anchored to extracted entry `syllo-exi-idea-001`.

### Entry judgm-con-001 — Concept-judgment setup: true adjudication against ought

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 4
- lineEnd: 91

Summary:

Judgment of the concept is the first true adjudicative form: actuality is measured against the concept as ought, and the disjunctive genus/species articulation is driven into explicit subject/predicate dyads toward syllogistic conclusion.

Key points: (KeyPoint)

- k1. Concept-judgment measures fact against presupposed ought.
- k2. It is objective truth of judgment, not external subjectivism.
- k3. Disjunctive genus/species identity must unfold into subject/predicate dyadic diremption.

Claims: (Claim)

- c1. id: judgm-con-001-c1
  - subject: judgment_of_the_concept
  - predicate: is_determined_as
  - object: first_true_adjudication_against_ought
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [12-19] concept as basis/ought; predicates like good/bad/true/right as measure.

- c2. id: judgm-con-001-c2
  - subject: judgment_of_the_concept
  - predicate: is
  - object: objective_truth_of_judgment
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [49-56] judgment of concept is objective and true, resting on concept as concept.

- c3. id: judgm-con-001-c3
  - subject: disjunctive_result
  - predicate: must_develop_into
  - object: subject_predicate_dyads_out_of_genus_species_structure
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [68-76] shortcoming: singularity not yet explicitly determined.
    - [86-91] unity is dirempted as subject (singular) and predicate (determinate connection of moments).

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: judgm-nec-c-002
  - note: elaborates Necessity's transition into concept-judgment by specifying ought/adjudication structure.
  - sourceClaimIds: [`judgm-con-001-c1`, `judgm-con-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`judgm-nec-c-002-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: judgm-con-a-001
  - note: setup proceeds directly to the first assertoric subtopic.
  - sourceClaimIds: [`judgm-con-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: base setup entry fixed for Part D special role.

### Entry judgm-con-a-001 — Assertoric immediacy: ought, constitution, and split extremes

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 95
- lineEnd: 125

Summary:

The first assertoric paragraph presents the subject as ought plus constituted existence and states that conceptive unity is not yet posited between these extremes.

Key points: (KeyPoint)

- k1. Subject includes ought-side universality.
- k2. Subject includes external constituted singularity.
- k3. Conceptive unity between extremes is not yet posited.

Claims: (Claim)

- c1. id: judgm-con-a-001-c1
  - subject: assertoric_subject
  - predicate: contains
  - object: universal_ought_and_constituted_particularity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [103-115] two moments: oughted universal and constituted actuality.

- c2. id: judgm-con-a-001-c2
  - subject: assertoric_form
  - predicate: lacks
  - object: posited_conceptive_unity_of_extremes
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [122-125] concept as posited unity connecting extremes is still lacking.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-a-002
  - note: second paragraph develops assertoric status as subjective assurance with external connectedness.
  - sourceClaimIds: [`judgm-con-a-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first assertoric subtopic (first paragraph).

### Entry judgm-con-a-002 — Assertoric assurance and external copula

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 127
- lineEnd: 147

Summary:

The second assertoric paragraph makes the credential purely subjective: connectedness remains external/in-itself and the copula is still immediate abstract being.

Key points: (KeyPoint)

- k1. Assertoric credential is subjective assurance.
- k2. Copula remains immediate abstract being.
- k3. Connectedness is only external and not yet posited in the fact.

Claims: (Claim)

- c1. id: judgm-con-a-002-c1
  - subject: assertoric_credential
  - predicate: is
  - object: subjective_assurance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [127-133] external connectedness means only internal/in-itself status.

- c2. id: judgm-con-a-002-c2
  - subject: copula_in_assertoric
  - predicate: remains
  - object: immediate_abstract_being
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [140-147] connectedness not yet posited; copula still immediate abstract being.

- c3. id: judgm-con-a-002-c3
  - subject: assertoric_connectedness
  - predicate: remains
  - object: external_and_not_posited_as_immanent_connection
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [131-147] externally posited connection; copula immediate abstract being.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-a-003
  - note: third paragraph articulates equal-opposed assurances and the explicit problematic conclusion.
  - sourceClaimIds: [`judgm-con-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second assertoric subtopic (second paragraph).

### Entry judgm-con-a-003 — Opposed assurances and explicit problematic status

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 149
- lineEnd: 164

Summary:

The third assertoric paragraph establishes that opposed assurances are equally justified and concludes explicitly that the judgment is essentially problematic.

Key points: (KeyPoint)

- k1. Opposed assurances are equally justified at the assertoric level.
- k2. Immediate singular subject lacks posited determinateness for conceptive conformity.
- k3. Assertoric judgment is therefore explicitly problematic.

Claims: (Claim)

- c1. id: judgm-con-a-003-c1
  - subject: assertoric_assurance
  - predicate: has
  - object: equally_justified_opposition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [149-155] "this action is good" / "this action is bad" equally justified.

- c2. id: judgm-con-a-003-c2
  - subject: assertoric_judgment
  - predicate: is_essentially
  - object: problematic
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [156-164] contingent conformity due to immediate singular abstraction; explicit problematic conclusion.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-b-001
  - note: assertoric problematicity passes to explicit problematic judgment development.
  - sourceClaimIds: [`judgm-con-a-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third assertoric subtopic and transition paragraph.

### Entry judgm-con-b-001 — Problematic as immanent positive/negative valence

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 168
- lineEnd: 184

Summary:

The first problematic paragraph defines problematic judgment as assertoric taken positively and negatively and states that contingency is now immanently present in the content.

Key points: (KeyPoint)

- k1. Problematicity is more immanent here than in earlier forms.
- k2. It carries both positive and negative valence.
- k3. Determination of the immediate as contingent is explicitly present.

Claims: (Claim)

- c1. id: judgm-con-b-001-c1
  - subject: problematic_form
  - predicate: is
  - object: immanent_posited_contingency_of_immediate
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [179-184] in problematic judgment, contingency of immediate is itself present.

- c2. id: judgm-con-b-001-c2
  - subject: problematic_judgment
  - predicate: is_determined_as
  - object: assertoric_taken_positively_and_negatively
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [168-174] problematic as assertoric with equal positive and negative valence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c1 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-b-002
  - note: second paragraph locates problematicity on subject-immediacy while preserving predicate objectivity.
  - sourceClaimIds: [`judgm-con-b-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first problematic subtopic (first paragraph).

### Entry judgm-con-b-002 — Predicate objectivity and subject-side contingency

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 186
- lineEnd: 210

Summary:

The second problematic paragraph places indeterminacy on subject immediacy while preserving predicate objectivity and singular reference.

Key points: (KeyPoint)

- k1. Predicate remains objective concrete universality.
- k2. Problematic element falls on contingent subject immediacy.
- k3. Singularity cannot be abstracted away in this judgment.

Claims: (Claim)

- c1. id: judgm-con-b-002-c1
  - subject: predicate
  - predicate: remains
  - object: objective_concrete_universality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [191-194] predicate has no determination to gain; already objective concrete universality.

- c2. id: judgm-con-b-002-c2
  - subject: problematic_element
  - predicate: falls_on
  - object: immediacy_of_subject_as_contingency
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [194-210] contingency and singular constitution are explicit in subject-side problematicity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-b-003
  - note: third paragraph shows subject partition into universal/particular as judgment's own structure.
  - sourceClaimIds: [`judgm-con-b-002-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second problematic subtopic (second paragraph).

### Entry judgm-con-b-003 — Subject partition as ground of conformity judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 212
- lineEnd: 224

Summary:

The third problematic paragraph differentiates the subject into objective ought and constituted existence and thereby grounds whether it is or is not what it ought to be.

Key points: (KeyPoint)

- k1. Subject is differentiated into universality and constituted existence.
- k2. This partition grounds being or not being what it ought to be.
- k3. Problematic negativity is this partition of a unity already containing both moments.

Claims: (Claim)

- c1. id: judgm-con-b-003-c1
  - subject: subject
  - predicate: contains
  - object: ground_for_being_or_not_being_what_it_ought
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [212-217] subject differentiated into ought and constituted existence; contains ground.

- c2. id: judgm-con-b-003-c2
  - subject: problematic_negativity
  - predicate: is
  - object: original_partition_of_subject_unity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [218-224] negativity amounts to original partition into universal and particular.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-b-004
  - note: next paragraph explicates the duplicity of subjectivity and its one-sided misreadings.
  - sourceClaimIds: [`judgm-con-b-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-b-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third problematic subtopic (third paragraph).

### Entry judgm-con-b-004 — Duplicity of subjectivity and unity-in-opposition

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 226
- lineEnd: 252

Summary:

The fourth problematic paragraph unfolds the dual meaning of subjectivity (conceptive and constituted), showing each one-sided reading is untrue outside their unity.

Key points: (KeyPoint)

- k1. Subjectivity has conceptive and constituted senses.
- k2. Truth of subjectivity is their unity-in-opposition.
- k3. One-sided fixation on either side is untrue.

Claims: (Claim)

- c1. id: judgm-con-b-004-c1
  - subject: subjectivity
  - predicate: has
  - object: dual_meaning_conceptive_and_constituted
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [226-236] both sides of subject called subjectivity.

- c2. id: judgm-con-b-004-c2
  - subject: truth_of_subjectivity
  - predicate: is
  - object: unity_of_opposed_meanings
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [240-252] truth is that opposite meanings are in one; one-sidedness is exposed.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-b-005
  - note: final paragraph explicitly converts problematic to apodictic judgment.
  - sourceClaimIds: [`judgm-con-b-004-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`judgm-con-b-005-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: fourth problematic subtopic (fourth paragraph).

### Entry judgm-con-b-005 — Transition paragraph: problematic to apodictic

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 254
- lineEnd: 258

Summary:

The final problematic paragraph states that once problematic character is posited as character of the fact, the judgment is no longer problematic but apodictic.

Key points: (KeyPoint)

- k1. Problematic character is posited as character of the fact itself.
- k2. This positing removes merely problematic status.
- k3. Judgment explicitly becomes apodictic.

Claims: (Claim)

- c1. id: judgm-con-b-005-c1
  - subject: problematic_judgment
  - predicate: becomes
  - object: apodictic_when_character_of_fact_is_posited
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [254-258] explicit transition sentence.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-c-001
  - note: transition paragraph hands off into apodictic development.
  - sourceClaimIds: [`judgm-con-b-005-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition subtopic; completes the problematic sequence.

### Entry judgm-con-c-001 — Apodictic subject and predicate-correspondence universal

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 262
- lineEnd: 288

Summary:

The first apodictic paragraph presents objective correspondence: subject and predicate share conceptive content, and predicate universality is correspondence of existence to ought.

Key points: (KeyPoint)

- k1. Subject includes universal ought and constituted singularity.
- k2. Predicate universality is correspondence, not abstract genus alone.
- k3. Subject and predicate correspond in the same concrete concept.

Claims: (Claim)

- c1. id: judgm-con-c-001-c1
  - subject: apodictic_subject
  - predicate: includes
  - object: universal_ought_and_constitutional_ground
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [262-270] subject includes ought and constitution grounding predication.

- c2. id: judgm-con-c-001-c2
  - subject: apodictic_predicate_universal
  - predicate: is
  - object: correspondence_of_existence_to_ought
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [282-288] universality is correspondence, not ought/genus by itself.

- c3. id: judgm-con-c-001-c3
  - subject: apodictic_judgment
  - predicate: is
  - object: objective_correspondence_of_shared_conceptive_content
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [271-279] judgment is objective truth; shared conceptive content of universality/particularization.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-c-002
  - note: second paragraph develops fact-truth as fracture and reconnection of ought and being.
  - sourceClaimIds: [`judgm-con-c-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first apodictic subtopic (first paragraph).

### Entry judgm-con-c-002 — Fact-truth as fracture and reconnection

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 290
- lineEnd: 303

Summary:

The second apodictic paragraph states the fact's truth as internal partition into ought and being that turns back into concrete conceptive unity.

Key points: (KeyPoint)

- k1. Fact contains ought and being in immediate unity.
- k2. Truth is internal fracture into those moments.
- k3. Their reconnection is concrete identity (the soul of the fact).

Claims: (Claim)

- c1. id: judgm-con-c-002-c1
  - subject: fact_truth
  - predicate: is
  - object: fractured_and_reunified_connection_of_ought_and_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [293-303] fracture into ought/being and reconnection into concrete identity.

Claim ↔ key point map:

- c1 -> k1
- c1 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-c-003
  - note: third paragraph develops ground/correspondence through determinate copula.
  - sourceClaimIds: [`judgm-con-c-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second apodictic subtopic (second paragraph).

### Entry judgm-con-c-003 — Grounded copula as determinate correspondence

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 304
- lineEnd: 325

Summary:

The third apodictic paragraph shows the transition to a determinate accomplished copula: the judgment has its ground in subject constitution and the copula becomes developed correspondence.

Key points: (KeyPoint)

- k1. Transition to correspondence lies in particular determinateness of fact.
- k2. Judgment has its ground in constitution of the subject.
- k3. Copula develops from abstract is into determinate connection.

Claims: (Claim)

- c1. id: judgm-con-c-003-c1
  - subject: apodictic_copula
  - predicate: is_determined_as
  - object: developed_ground_of_subject_predicate_correspondence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [304-320] transition from abstract is to developed ground.
    - [321-325] copula as subject-universality correspondence.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-c-004
  - note: fourth paragraph shows form's passing-away as content identity and connecting activity coincide.
  - sourceClaimIds: [`judgm-con-c-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-c-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third apodictic subtopic (third paragraph).

### Entry judgm-con-c-004 — Form passing-away and recovered conceptive identity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 327
- lineEnd: 344

Summary:

The fourth apodictic paragraph states that judgment-form passes away because content identity and connecting activity coincide, thereby recovering concrete conceptive identity in the whole.

Key points: (KeyPoint)

- k1. Subject and predicate are in themselves same content.
- k2. Connecting activity passes into predicate-content.
- k3. Concrete identity of the concept is recovered in the whole.

Claims: (Claim)

- c1. id: judgm-con-c-004-c1
  - subject: judgment_form
  - predicate: passes_away_in
  - object: identity_of_content_and_posited_connecting
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [327-337] form passes away as connecting becomes predicate-content.
    - [338-344] concrete identity recovered in the whole.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-con-c-005
  - note: final paragraph presents replete copula and explicit syllogism transition.
  - sourceClaimIds: [`judgm-con-c-004-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-con-c-005-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: fourth apodictic subtopic (fourth paragraph).

### Entry judgm-con-c-005 — Transition paragraph: replete copula to syllogism

Span:

- sourceFile: `src/compiler/concept/subject/judgment/concept.txt`
- lineStart: 346
- lineEnd: 374

Summary:

The final apodictic paragraph identifies the copula as replete unity of concept and explicitly concludes that by this repletion the judgment has become syllogism.

Key points: (KeyPoint)

- k1. Subject and predicate are each the whole concept in apodictic form.
- k2. Determinate copular connection is universal-particular mediation.
- k3. Replete copula yields explicit transition to syllogism.

Claims: (Claim)

- c1. id: judgm-con-c-005-c1
  - subject: copula_replete_of_content
  - predicate: grounds
  - object: transition_to_syllogism
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [366-372] copula as unity of concept re-emerging.
    - [373-374] explicit statement: judgment has become syllogism.

Claim ↔ key point map:

- c1 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-exi-idea-001
  - note: concrete boundary handoff into syllogism extraction.
  - sourceClaimIds: [`judgm-con-c-005-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-exi-idea-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes the apodictic sequence and Part D seed-pass boundary.
