# Judgment Part A Workbook

Part: `A. JUDGMENT OF EXISTENCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `existence.txt` as authority.
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

## Session: 2026-02-21 (first Existence pass)

Scope:

- file: `existence.txt`
- fixed range: lines `4-798`
- pass policy: first pass on Judgment parts first; then IDEA pass; then second pass over Parts A-D

Decision:

- Remove marker-only entries for this pass and keep only substantive, evidence-bearing entries.
- Keep the existing numbered ID pattern (`judgm-exi-<letter>-<nnn>`) so inbound/outbound cross-part links remain stable.
- Keep claim decomposition minimal and evidence-anchored for first pass.
- Preserve explicit transition into Reflection (`judgm-ref-i-001`).

### Entry judgm-exi-i-001 — Existence-judgment setup: immediacy, quality, inherence

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 4
- lineEnd: 37

Summary:

Judgment of existence starts as immediate qualitative judgment and, as judgment of inherence, privileges the subject as immediate basis and predicate as non-self-subsistent.

Key points: (KeyPoint)

- k1. Subjective judgment intends concept-reality agreement but begins as immediate.
- k2. First judgment is qualitative/immediate existence.
- k3. As inherence judgment, subject is essential basis and predicate depends on it.

Claims: (Claim)

- c1. id: judgm-exi-i-001-c1
  - subject: judgment_of_existence
  - predicate: begins_as
  - object: immediate_qualitative_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [14-20] first judgment is immediate and therefore judgment of immediate existence / qualitative.

- c2. id: judgm-exi-i-001-c2
  - subject: judgment_of_existence
  - predicate: is_determined_as
  - object: judgment_of_inherence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [27-36] judgment of inherence: subject immediate/essential, predicate grounded in subject.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-a-001
  - note: setup proceeds directly into the first positive subtopic.
  - sourceClaimIds: [`judgm-exi-i-001-c1`, `judgm-exi-i-001-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`judgm-exi-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: base setup entry preceding the substantive positive sequence.

### Entry judgm-exi-a-001 — Immediate positive form: abstract extremes and positive copula

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 40
- lineEnd: 88

Summary:

In the first positive form, subject and predicate are immediate abstract concept-moments and the copula is immediate abstract being.

Key points: (KeyPoint)

- k1. Predicate is abstract universal with mediation only presupposed.
- k2. Subject is abstract singular/externalized concept side.
- k3. Copula is immediate and unmediated, hence positive.

Claims: (Claim)

- c1. id: judgm-exi-a-001-c1
  - subject: predicate
  - predicate: is_determined_as
  - object: abstract_universal
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [53-57] predicate as abstract universal with mediation presupposed.

- c2. id: judgm-exi-a-001-c2
  - subject: subject
  - predicate: is_determined_as
  - object: abstract_singular
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [74-80] subject as abstract singular/side of externality.

- c3. id: judgm-exi-a-001-c3
  - subject: copula_is
  - predicate: is
  - object: immediate_abstract_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [83-88] immediate unmediated connection defines positive judgment.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-a-002
  - note: moves from immediate structure to explicit propositional articulation.
  - sourceClaimIds: [`judgm-exi-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-exi-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first positive subtopic.

### Entry judgm-exi-a-002 — Propositional unfolding: form/content reciprocity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 90
- lineEnd: 274

Summary:

Positive judgment unfolds as "the singular is universal" while content reciprocity yields "the universal is singular," with both poles held in one immediate judgment.

Key points: (KeyPoint)

- k1. Explicit form-proposition: singular is universal.
- k2. Objective meaning includes perishability of singulars and persistence of concept.
- k3. Reciprocal determination yields universal-in-subject and singularized predicate.
- k4. Immediate positive judgment still holds form/content unity.

Claims: (Claim)

- c1. id: judgm-exi-a-002-c1
  - subject: first_pure_positive_expression
  - predicate: is
  - object: singular_is_universal
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [90-93] first pure expression of positive judgment.
    - [102-107] every judgment in principle says singular is universal.

- c2. id: judgm-exi-a-002-c2
  - subject: positive_judgment_objective_meaning
  - predicate: contains
  - object: perishability_of_singulars_and_positive_subsistence_in_concept
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [130-138] perishableness of singular things and concept's persistence.
    - [145-148] judgment as universal resolving into singular through negativity.

- c3. id: judgm-exi-a-002-c3
  - subject: reciprocal_determination
  - predicate: yields
  - object: universal_is_singular_and_singular_is_universal_in_one_judgment
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [190-210] reciprocal determination of subject and predicate.
    - [241-250] twofold connection constitutes one positive judgment.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-a-003
  - note: unity remains immediate and therefore leads to contradiction and negation.
  - sourceClaimIds: [`judgm-exi-a-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`judgm-exi-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second positive subtopic.

### Entry judgm-exi-a-003 — Contradiction of immediacy and transition to negation

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 275
- lineEnd: 335

Summary:

When form and content propositions are explicitly combined, the immediate positive configuration becomes contradictory and must be posited as negative judgment.

Key points: (KeyPoint)

- k1. External combination would produce empty identity (particular is particular).
- k2. Immediate extremes prevent completed union in positive form.
- k3. Both sides jointly compel transition to negative judgment.

Claims: (Claim)

- c1. id: judgm-exi-a-003-c1
  - subject: combination_of_form_and_content_propositions
  - predicate: yields
  - object: empty_identity_if_taken_as_completed_result
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [282-295] combination would become empty identical proposition.

- c2. id: judgm-exi-a-003-c2
  - subject: positive_judgment_immediacy
  - predicate: blocks
  - object: union_of_singularity_and_universality_into_particularity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [299-307] singularity/universality cannot yet be united due to immediacy.

- c3. id: judgm-exi-a-003-c3
  - subject: positive_judgment
  - predicate: must_be_posited_as
  - object: negative_judgment
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [314-322] immediate singular not universal / predicate of wider extension.
    - [334-335] positive judgment must be posited as negative.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-b-001
  - note: contradiction of positive form enters the negative judgment's logical content.
  - sourceClaimIds: [`judgm-exi-a-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-exi-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes the positive sequence.

### Entry judgm-exi-b-001 — Logical content and first negation to particularity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 339
- lineEnd: 559

Summary:

Negative judgment clarifies that logical content lies in singular-universal form itself and yields particularity as the immediate mediated determination.

Key points: (KeyPoint)

- k1. Judgment truth concerns concept determinations, not empirical examples.
- k2. Positive judgment contradicts itself in its logical content.
- k3. Negation of abstract universality yields particularity.

Claims: (Claim)

- c1. id: judgm-exi-b-001-c1
  - subject: logical_content_of_judgment
  - predicate: consists_in
  - object: relation_of_singular_and_universal
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [347-353] concepts relate as singular/universal; this is logical content.
    - [358-360] judgment says singular is universal and vice versa.

- c2. id: judgm-exi-b-001-c2
  - subject: positive_judgment
  - predicate: contradicts
  - object: requirement_of_noncontradiction
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [362-367] positive judgment not true and contradicts itself.

- c3. id: judgm-exi-b-001-c3
  - subject: non_universal
  - predicate: is_determined_as
  - object: particular
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [553-559] in concept's continuity, not immediately positive; non-universal directly particular.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-b-002
  - note: moves into explicit account of residual universal sphere and positive side of negation.
  - sourceClaimIds: [`judgm-exi-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-exi-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first negative subtopic.

### Entry judgm-exi-b-002 — Residual positivity of negation and stabilized particularity

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 560
- lineEnd: 611

Summary:

Because negative judgment still preserves a universal sphere, negation is partial and retains determinateness as transformed universal determinateness, i.e., particularity.

Key points: (KeyPoint)

- k1. Negation targets predicate abstraction, not universality as such.
- k2. Subject-predicate connection remains partly positive through retained universal sphere.
- k3. Determinateness remains as particularity.

Claims: (Claim)

- c1. id: judgm-exi-b-002-c1
  - subject: negative_judgment
  - predicate: is
  - object: not_total_negation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [571-579] universality not negated as such; universal sphere remains.

- c2. id: judgm-exi-b-002-c2
  - subject: rose_not_red_example
  - predicate: shows
  - object: retained_universal_sphere_color
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [582-590] color sphere retained despite negated determinateness.

- c3. id: judgm-exi-b-002-c3
  - subject: transformed_determinateness
  - predicate: is
  - object: particularity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [594-600] singular is particular; predicate still determinate.
    - [607-611] determinateness retained as universal determinateness, i.e., particularity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-b-003
  - note: mediated particularity now drives second negation toward infinite judgment.
  - sourceClaimIds: [`judgm-exi-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-exi-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second negative subtopic.

### Entry judgm-exi-b-003 — Negation of negation and emergence of infinite judgment

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 612
- lineEnd: 699

Summary:

Particularity mediates to second negation, restoring mediated singularity and purified universality while negating residual connection, thereby producing the infinite judgment.

Key points: (KeyPoint)

- k1. Negative judgment mediates toward reflection-into-self.
- k2. Negation of negation yields singular-is-singular / universal-is-universal.
- k3. Full predicate extent is negated, producing infinite judgment.

Claims: (Claim)

- c1. id: judgm-exi-b-003-c1
  - subject: particularity
  - predicate: functions_as
  - object: mediating_term_for_third_step
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [612-617] particularity mediates toward reflection of judgment into itself.

- c2. id: judgm-exi-b-003-c2
  - subject: negation_of_negative_judgment
  - predicate: yields
  - object: singular_is_singular_and_universal_is_universal
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [642-648] negation is posited as negation of negation.
    - [667-679] resulting double proposition.

- c3. id: judgm-exi-b-003-c3
  - subject: second_negation_of_residual_connection
  - predicate: culminates_in
  - object: infinite_judgment
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [694-699] whole predicate extent negated; this is infinite judgment.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-c-001
  - note: the negated connection now appears as explicitly infinite judgment form.
  - sourceClaimIds: [`judgm-exi-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-exi-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes the negative sequence.

### Entry judgm-exi-c-001 — Negative infinite judgment as sublated judgment-form

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 702
- lineEnd: 746

Summary:

The negative infinite judgment annuls ordinary judgment form: it grammatically links subject and predicate while abolishing any valid common sphere.

Key points: (KeyPoint)

- k1. Infinite judgment is introduced as supposed truth of negative judgment.
- k2. Negative expression sublates judgment form and becomes nonsensical.
- k3. Examples negate both determinate predicate and universal sphere.

Claims: (Claim)

- c1. id: judgm-exi-c-001-c1
  - subject: infinite_judgment_negative_expression
  - predicate: sublates
  - object: form_of_judgment
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [702-706] infinite judgment as truth of negative judgment yet sublated form.

- c2. id: judgm-exi-c-001-c2
  - subject: negatively_infinite_examples
  - predicate: are
  - object: correct_but_nonsensical_non_judgments
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [719-727] spirit-not-red / rose-not-elephant examples and verdict.

- c3. id: judgm-exi-c-001-c3
  - subject: crime_as_infinite_judgment
  - predicate: negates
  - object: universal_sphere_of_right_as_right
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [739-742] crime negates not merely particular right but right as right.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k2, k3
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-c-002
  - note: from negative infinite side to positive reflection side.
  - sourceClaimIds: [`judgm-exi-c-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`judgm-exi-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first infinite subtopic.

### Entry judgm-exi-c-002 — Positive infinite side: reflected singularity and universality

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 747
- lineEnd: 770

Summary:

The positive infinite moment posits singularity and universality as self-returning through negation-of-negation.

Key points: (KeyPoint)

- k1. Positive infinite element is reflective self-identity.
- k2. Singularity is first posited as mediated self-identity.
- k3. Universality is equally posited as turning back into itself.

Claims: (Claim)

- c1. id: judgm-exi-c-002-c1
  - subject: positive_infinite_judgment
  - predicate: states
  - object: singular_is_singular_by_reflection_into_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [747-754] positive element and "singular is singular" in reflection.
    - [758-760] singular posited for first time through mediation.

- c2. id: judgm-exi-c-002-c2
  - subject: universal
  - predicate: is_posited_as
  - object: turning_back_into_itself
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [767-770] universal is universal and turned back into itself.

Claim ↔ key point map:

- c1 -> k1
- c1 -> k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
- targetEntryId: judgm-exi-c-003
- note: after positive infinite self-return, judgment-form sublation is made explicit.
- sourceClaimIds: [`judgm-exi-c-002-c1`, `judgm-exi-c-002-c2`]
- sourceKeyPointIds: [`k1`, `k2`, `k3`]
- targetClaimIds: [`judgm-exi-c-003-c1`]
- logicalOperator: sequential_transition
- analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second infinite subtopic.

### Entry judgm-exi-c-003 — Judgment-form collapse after infinite extremes

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 772
- lineEnd: 783

Summary:

Through reflection of judgment determinations into themselves, both negative and positive infinite forms exceed judgment-form, yielding no remaining judgment as such.

Key points: (KeyPoint)

- k1. Reflective return sublates judgment as judgment.
- k2. Negative infinite has too much difference for positive connection.
- k3. Positive infinite has only identity and thus no judgment difference.

Claims: (Claim)

- c1. id: judgm-exi-c-003-c1
  - subject: reflection_of_judgment_determinations
  - predicate: sublates
  - object: judgment_form_as_such
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [772-775] reflection into themselves sublates judgment.

- c2. id: judgm-exi-c-003-c2
  - subject: negatively_infinite_judgment
  - predicate: lacks
  - object: positive_subject_predicate_connection
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [775-779] difference too great for judgmental connection.

- c3. id: judgm-exi-c-003-c3
  - subject: positively_infinite_judgment
  - predicate: lacks
  - object: judgment_due_to_total_identity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [780-783] only identity remains, hence no judgment.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-exi-t-001
  - note: from collapse of judgment-form to explicit transition into reflection judgment.
  - sourceClaimIds: [`judgm-exi-c-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`judgm-exi-t-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third infinite subtopic.

### Entry judgm-exi-t-001 — Transition paragraph: existence judgment passes to reflection

Span:

- sourceFile: `src/compiler/concept/subject/judgment/existence.txt`
- lineStart: 785
- lineEnd: 798

Summary:

The final paragraph explicitly states that judgment of existence has sublated itself and passed over into judgment of reflection via reflected terms of the copula's conceptual unity.

Key points: (KeyPoint)

- k1. Existence judgment is explicitly sublated.
- k2. Copular identity now posits reflected, non-immediate terms.
- k3. Direct passage into judgment of reflection is declared.

Claims: (Claim)

- c1. id: judgm-exi-t-001-c1
  - subject: judgment_of_existence
  - predicate: has_sublated_itself
  - object: qualitative_extremes_in_copular_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [785-791] existence judgment sublated; qualitative extremes sublated in identity.

- c2. id: judgm-exi-t-001-c2
  - subject: resulting_judgment_terms
  - predicate: are_determined_as
  - object: reflected_into_themselves_not_immediate
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [792-796] unity as concept tears into judgment with reflected terms.

- c3. id: judgm-exi-t-001-c3
  - subject: judgment_of_existence
  - predicate: passes_over_into
  - object: judgment_of_reflection
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [797-798] explicit passage sentence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: judgm-ref-i-001
  - note: concrete handoff into Part B Reflection setup entry.
  - sourceClaimIds: [`judgm-exi-t-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`judgm-ref-i-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition subtopic; completes Part A first-pass boundary.
