# Syllogism Part A Workbook

Part: `A. THE SYLLOGISM OF EXISTENCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
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

## Session: 2026-02-21 (seed pass)

Scope:

- file: `existence.txt`
- fixed range: lines `2-895`
- pass policy: initialize marker SubTopics and first numbered Entries for fine-grained extraction.

Decision:

- Adopt a two-level ID system for precision:
  - Level 1 SubTopic markers: `syl-exi-a`, `syl-exi-b`, `syl-exi-c`, `syl-exi-d`
  - Level 2 numbered Entries under each marker: `syl-exi-<letter>-<nnn>`
- Treat numerically labeled subsubtopics as Entries (`001`, `002`, ...).
- Integrate the moved numeral-1 exposition into `a. First figure` entry sequence.
- Retain `d. U-U-U` as explicit fourth subspecies and transition gate toward Reflection.

### Entry syl-exi-idea-001 — Part A framing: end of qualitative logic and content-bearing form

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 2
- lineEnd: 37

Summary:

Part A frames the immediate formal syllogism while insisting that mediation must be explicitly posited through dialectical development.

Key points: (KeyPoint)

- k1. The first syllogism appears as immediate/formal.
- k2. The middle initially unites singular and universal only in immediate double-sidedness.
- k3. The movement of Part A is to posit mediation moments explicitly.

Claims: (Claim)

- c1. id: syl-exi-idea-001-c1
  - subject: first_syllogism
  - predicate: is_determined_as
  - object: immediate_formal_syllogism
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [2-17] immediate moments and first strictly formal syllogism.

- c2. id: syl-exi-idea-001-c2
  - subject: syllogism_of_existence
  - predicate: has_task
  - object: positing_mediation_moments
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [27-37] dialectical movement consists in positing moments of mediation.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-a
  - note: framing transitions to first-figure SubTopic marker.
  - sourceClaimIds: [`syl-exi-idea-001-c1`, `syl-exi-idea-001-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`syl-exi-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: special seed entry inserted before marker chain.

### Entry syl-exi-a — Marker `a`: first figure (S-P-U)

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 38
- lineEnd: 483

Summary:

The first figure articulates formal S-P-U mediation, demonstrates content-contingency in qualitative syllogism, and forces reconfiguration of mediation form.

Key points: (KeyPoint)

- k1. First figure schema is S-P-U.
- k2. Formal syllogism is contingent in content because middle selection is external/arbitrary.
- k3. Progression of proving premises reveals deficiency of immediate-premise form.

Claims: (Claim)

- c1. id: syl-exi-a-c1
  - subject: first_figure
  - predicate: has_schema
  - object: S_P_U
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [38-41] explicit schema statement.

- c2. id: syl-exi-a-c2
  - subject: formal_syllogism
  - predicate: is_contingent_with_respect_to
  - object: content
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [212-347] contingency/arbitrariness of middle terms and contradictory deductions.

- c3. id: syl-exi-a-c3
  - subject: immediate_premise_form
  - predicate: transitions_to
  - object: alternate_mediation_forms_P_S_U_and_S_U_P
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [348-483] infinite progression and transition to other mediation shapes.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-a-001
  - note: marker to numbered entry sequence for fine-grained extraction.
  - sourceClaimIds: [`syl-exi-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syl-exi-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-exi-b
  - note: first figure passes into second figure.
  - sourceClaimIds: [`syl-exi-a-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-exi-b-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for first-figure SubTopic.

### Entry syl-exi-a-001 — First figure opening: formal schema and mediated connection claim

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 38
- lineEnd: 211

Summary:

The first-figure opening develops S-P-U as non-immediate connection and distinguishes objective syllogistic mediation from merely subjective proposition-chaining.

Key points: (KeyPoint)

- k1. Extremes are connected through a distinct middle.
- k2. The “therefore” is grounded in term-nature, not only subjective reflection.
- k3. Syllogism cannot be reduced to a mere aggregate of three judgments.

Claims: (Claim)

- c1. id: syl-exi-a-001-c1
  - subject: therefore_relation
  - predicate: is_grounded_in
  - object: nature_of_extremes_via_middle
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [122-159] therefore not merely external to proposition; grounded in middle-term connection.

- c2. id: syl-exi-a-001-c2
  - subject: syllogism
  - predicate: is_not_reducible_to
  - object: three_isolated_judgments
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [160-210] critique of formalistic three-judgment view.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-a-002
  - note: continue numbered decomposition of first-figure internal phases.
  - sourceClaimIds: [`syl-exi-a-001-c1`, `syl-exi-a-001-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first-figure subsection `1` stabilized.

### Entry syl-exi-a-002 — First figure subsection 2: qualitative content and contingency of middle selection

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 212
- lineEnd: 347

Summary:

The second subsection shows that in qualitative immediacy the chosen middle term is contingent, enabling equally correct yet contradictory inferences.

Key points: (KeyPoint)

- k1. Immediate content offers indeterminate multiplicity of possible middle terms.
- k2. Formal inferences can yield contradictory conclusions from the same subject.
- k3. Contingency is rooted in form-abstractness, not merely accidental examples.

Claims: (Claim)

- c1. id: syl-exi-a-002-c1
  - subject: immediate_singular_content
  - predicate: provides
  - object: indeterminate_plurality_of_middle_terms
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [226-251] infinite manifold of determinacies; any may function as middle.

- c2. id: syl-exi-a-002-c2
  - subject: formal_syllogistic_inference
  - predicate: permits
  - object: contradictory_conclusions_with_equal_formal_correctness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [271-330] examples of opposing conclusions through alternate middle terms.

- c3. id: syl-exi-a-002-c3
  - subject: contingency_of_formal_syllogism
  - predicate: is_grounded_in
  - object: abstract_form_of_single_immediate_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [335-347] form itself fixes one-sided quality and therefore contingency.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-a-003
  - note: contingency analysis transitions to explicit critique of immediate premises and infinite proof regress.
  - sourceClaimIds: [`syl-exi-a-002-c2`, `syl-exi-a-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first-figure subsection `2` stabilized.

### Entry syl-exi-a-003 — First figure subsection 3: contradiction of immediate premises and re-formation of mediation

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 348
- lineEnd: 483

Summary:

The third subsection shows immediate-premise form contradicts syllogistic truth, generating infinite regress and forcing mediation into new figure-shapes.

Key points: (KeyPoint)

- k1. Immediate premise-form contradicts the syllogistic requirement of posited mediation.
- k2. Proof-of-premises strategy generates bad infinity.
- k3. Mediation must be reformulated into P-S-U and S-U-P.

Claims: (Claim)

- c1. id: syl-exi-a-003-c1
  - subject: premise_form_as_immediate_judgment
  - predicate: contradicts
  - object: nature_of_syllogism_as_posited_mediation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [364-373] immediate premises conflict with syllogistic truth.

- c2. id: syl-exi-a-003-c2
  - subject: proving_premises_by_further_syllogisms
  - predicate: yields
  - object: infinite_regress
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [381-389] geometric progression of premises/conclusions.
    - [402-425] bad infinity as repetition of original deficiency.

- c3. id: syl-exi-a-003-c3
  - subject: mediation_reformulation
  - predicate: takes_shape
  - object: P_S_U_and_S_U_P
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [438-442] explicit reshaping of mediation.
    - [443-483] concept-side grounding of the transition.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-exi-a
  - note: numbered decomposition grounds marker-level transition claim for first figure.
  - sourceClaimIds: [`syl-exi-a-003-c1`, `syl-exi-a-003-c2`, `syl-exi-a-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-exi-a-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-exi-b-001
  - note: explicit handoff from first-figure completion to second-figure subsection `1`.
  - sourceClaimIds: [`syl-exi-a-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-exi-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first-figure subsection `3` stabilized.

### Entry syl-exi-b — Marker `b`: second figure (P-S-U)

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 484
- lineEnd: 671

Summary:

The second figure posits contingency of immediate singular mediation and points beyond itself to universality as mediating ground.

Key points: (KeyPoint)

- k1. Second figure restructures mediation as P-S-U.
- k2. Figure incompletion reveals mismatch with first formal schema.
- k3. Contingent singular mediation self-sublates toward universal mediation.

Claims: (Claim)

- c1. id: syl-exi-b-c1
  - subject: second_figure
  - predicate: has_schema
  - object: P_S_U
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [484-485] heading and schema statement.

- c2. id: syl-exi-b-c2
  - subject: singular_mediation
  - predicate: points_beyond_itself_to
  - object: universal_mediation
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [640-671] immediacy points to opposite, reflected universality.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-b-001
  - note: marker to numbered entry sequence.
  - sourceClaimIds: [`syl-exi-b-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-exi-c
  - note: second figure passes into third figure.
  - sourceClaimIds: [`syl-exi-b-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-exi-c-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry initialized.

### Entry syl-exi-b-001 — Second figure subsection 1: contingency-mediated universality and negative unity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 486
- lineEnd: 541

Summary:

The first subsection of the second figure presents singularity as contingent middle while introducing mediation as negative unity.

Key points: (KeyPoint)

- k1. First figure truth: connection through contingency/singularity, not in-itself qualitative unity.
- k2. Second figure posits mediation with a negative moment.
- k3. Universal and particular exchange formal positions through mediated relation.

Claims: (Claim)

- c1. id: syl-exi-b-001-c1
  - subject: truth_of_first_qualitative_syllogism
  - predicate: is
  - object: contingent_union_through_singularity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [486-497] contingency/singularity as real middle.

- c2. id: syl-exi-b-001-c2
  - subject: second_figure_mediation
  - predicate: is_determined_as
  - object: negative_unity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [501-510] mediation contains negative moment.

- c3. id: syl-exi-b-001-c3
  - subject: second_figure_extremes
  - predicate: are_repositioned_as
  - object: particular_and_universal_with_exchanged_formal_roles
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [511-541] role exchange and external positedness.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-b-002
  - note: first subsection moves to explicit form-variance critique.
  - sourceClaimIds: [`syl-exi-b-001-c2`, `syl-exi-b-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second-figure subsection `1` stabilized.

### Entry syl-exi-b-002 — Second figure subsection 2: mismatch with first schema and formal indeterminacy

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 542
- lineEnd: 626

Summary:

The second subsection argues the figure's truth is transition from formal schema, not commensurate species status within that schema.

Key points: (KeyPoint)

- k1. Transition realizes concept but also alters pure form.
- k2. Second figure does not conform to S-P-U as fixed species pattern.
- k3. Particular conclusion value is indeterminate under formal treatment.

Claims: (Claim)

- c1. id: syl-exi-b-002-c1
  - subject: transition_to_second_figure
  - predicate: is
  - object: realization_of_concept_and_alteration_of_pure_form
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [552-560] realization + alteration duality.

- c2. id: syl-exi-b-002-c2
  - subject: second_figure
  - predicate: lacks
  - object: strict_conformity_to_general_schema_S_P_U
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [561-589] non-conformity analysis.

- c3. id: syl-exi-b-002-c3
  - subject: particular_conclusion_in_second_figure
  - predicate: is
  - object: formally_indeterminate_in_value
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [609-626] particular judgment's weak determinacy and indifferent major/minor handling.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-b-003
  - note: indeterminate formality transitions to self-sublation of contingent mediation.
  - sourceClaimIds: [`syl-exi-b-002-c2`, `syl-exi-b-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second-figure subsection `2` stabilized.

### Entry syl-exi-b-003 — Second figure subsection 3: self-sublation of contingency into reflected universality

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 627
- lineEnd: 671

Summary:

The third subsection concludes that contingent mediation by singular immediacy self-sublates and points to reflected universality.

Key points: (KeyPoint)

- k1. Contingency of first mediation is now explicitly posited.
- k2. Immediate singular mediation points beyond itself.
- k3. Resulting truth is reflected universality as new middle determination.

Claims: (Claim)

- c1. id: syl-exi-b-003-c1
  - subject: second_figure_mediation
  - predicate: posits
  - object: contingency_of_first_mediation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [631-635] contingency now posited.

- c2. id: syl-exi-b-003-c2
  - subject: mediation_by_immediate_singularity
  - predicate: self_sublates_into
  - object: need_for_immediate_identity_of_extremes
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [636-651] self-external mediation and immediate-identity requirement.

- c3. id: syl-exi-b-003-c3
  - subject: resulting_immediacy
  - predicate: is
  - object: reflected_universality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [652-671] opposite immediacy as reflected universal and transition to next form.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-exi-b
  - note: subsection completion grounds marker-level transition toward third figure.
  - sourceClaimIds: [`syl-exi-b-003-c1`, `syl-exi-b-003-c2`, `syl-exi-b-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-exi-b-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-exi-c-001
  - note: direct handoff to third-figure subsection `1`.
  - sourceClaimIds: [`syl-exi-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-exi-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second-figure subsection `3` stabilized.

### Entry syl-exi-c — Marker `c`: third figure (S-U-P)

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 672
- lineEnd: 784

Summary:

The third figure completes reciprocal presupposition among figures and exposes formalism where abstract universality mediates without containing extreme determinateness.

Key points: (KeyPoint)

- k1. Third figure is S-U-P and presupposes the other figures reciprocally.
- k2. Abstract universal as middle reveals formal deficiency.
- k3. Subject/predicate and major/minor assignments become indifferent under this form.

Claims: (Claim)

- c1. id: syl-exi-c-c1
  - subject: third_figure
  - predicate: has_schema
  - object: S_U_P
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [672-684] heading and reciprocal presupposition setup.

- c2. id: syl-exi-c-c2
  - subject: abstract_universal_middle
  - predicate: discloses
  - object: formalism_of_syllogism
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [688-713] truth of formal syllogism as abstract-universal mediation.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-c-001
  - note: marker to numbered decomposition entry.
  - sourceClaimIds: [`syl-exi-c-c1`, `syl-exi-c-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-exi-d
  - note: transition to fourth figure U-U-U.
  - sourceClaimIds: [`syl-exi-c-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-d-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry initialized.

### Entry syl-exi-c-001 — Third figure subsection 1: reciprocal presupposition and formal truth

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 674
- lineEnd: 705

Summary:

The first subsection of the third figure completes reciprocal presupposition and identifies abstract universality as the truth of formal syllogism.

Key points: (KeyPoint)

- k1. Third figure presupposes and is presupposed by first and second figures.
- k2. Formal truth appears as universal middle lacking extreme determinateness.
- k3. This exposes formalism as immediate-content indifference to form.

Claims: (Claim)

- c1. id: syl-exi-c-001-c1
  - subject: third_figure
  - predicate: completes
  - object: reciprocal_presupposition_of_figures
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [674-687] reciprocal presupposition and incomplete mediation in each figure.

- c2. id: syl-exi-c-001-c2
  - subject: syllogism_S_U_P
  - predicate: reveals
  - object: abstract_universal_middle_as_formal_truth
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [688-695] universal middle and non-contained essential determinateness.

- c3. id: syl-exi-c-001-c3
  - subject: formalism_of_syllogism
  - predicate: consists_in
  - object: immediate_content_indifferent_to_form
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [696-705] explicit formalism statement.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-c-002
  - note: from formal truth statement to legitimacy/indifference analysis.
  - sourceClaimIds: [`syl-exi-c-001-c2`, `syl-exi-c-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third-figure subsection `1` stabilized.

### Entry syl-exi-c-002 — Third figure subsection 2: formal legitimacy and indifference of term-placement

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 706
- lineEnd: 760

Summary:

The second subsection shows that third-figure legitimacy requires negative-judgment indifference, rendering subject/predicate and major/minor placements formally idle.

Key points: (KeyPoint)

- k1. Universal middle is both unity and determinate abstraction.
- k2. Figure legitimacy passes through indifferent subject/predicate relation in negative judgment.
- k3. Major/minor assignment becomes indifferent and supports emergence of fourth figure.

Claims: (Claim)

- c1. id: syl-exi-c-002-c1
  - subject: universal_middle_in_third_figure
  - predicate: is
  - object: abstract_unity_distinct_from_extremes
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [706-714] universal middle as abstract from determinate extremes.

- c2. id: syl-exi-c-002-c2
  - subject: third_figure_legitimation
  - predicate: depends_on
  - object: indifferent_subject_predicate_relation_of_negative_judgment
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [723-730] legitimating move and necessary negativity of conclusion.

- c3. id: syl-exi-c-002-c3
  - subject: formal_term_assignment
  - predicate: becomes
  - object: indifferent_and_idle
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [731-760] indifference of subject/predicate and major/minor; fourth-figure ground.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-c-003
  - note: formal indifference transitions to objective-significance and U-U-U emergence.
  - sourceClaimIds: [`syl-exi-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-exi-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third-figure subsection `2` stabilized.

### Entry syl-exi-c-003 — Third figure subsection 3: abstract universal middle and emergence of relationless U-U-U

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 761
- lineEnd: 784

Summary:

The third subsection states that abstract universality as middle leaves extremes externally conjoined, yielding the relationless U-U-U figure.

Key points: (KeyPoint)

- k1. Universal middle is essential unity only abstractly.
- k2. Extreme conjunction requires external mediation under this form.
- k3. Bare abstraction yields relationless U-U-U.

Claims: (Claim)

- c1. id: syl-exi-c-003-c1
  - subject: objective_significance_of_universal_middle
  - predicate: is_limited_by
  - object: abstract_universality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [761-768] universal middle essential yet only abstract.

- c2. id: syl-exi-c-003-c2
  - subject: conjunction_of_extremes
  - predicate: requires
  - object: external_contingent_mediation
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [769-776] conjunction depends on mediation outside this syllogism.

- c3. id: syl-exi-c-003-c3
  - subject: abstraction_from_qualitative_difference
  - predicate: yields
  - object: relationless_syllogism_U_U_U
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [777-784] explicit emergence of relationless U-U-U.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-exi-c
  - note: subsection completion grounds marker-level transition claim to fourth figure.
  - sourceClaimIds: [`syl-exi-c-003-c1`, `syl-exi-c-003-c2`, `syl-exi-c-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-exi-c-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-exi-d-001
  - note: direct handoff into fourth-figure subsection `1`.
  - sourceClaimIds: [`syl-exi-c-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-exi-d-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third-figure subsection `3` stabilized.

### Entry syl-exi-d — Marker `d`: fourth figure (U-U-U / mathematical syllogism)

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 785
- lineEnd: 895

Summary:

The fourth figure abstracts to equality-form mediation yet yields the positive transition from formal figure-circulation to reflection-based mediation.

Key points: (KeyPoint)

- k1. Fourth figure is U-U-U and removes inherence/subsumption form.
- k2. Mathematical self-evidence rests on abstraction from qualitative determinateness.
- k3. The result is positive transition toward syllogism of reflection.

Claims: (Claim)

- c1. id: syl-exi-d-c1
  - subject: fourth_figure
  - predicate: has_schema
  - object: U_U_U
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [785-801] heading and equal-to-a-third formulation.

- c2. id: syl-exi-d-c2
  - subject: mathematical_syllogism
  - predicate: abstracts_from
  - object: qualitative_differentiation
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [817-847] abstraction account and formal self-evidence critique.

- c3. id: syl-exi-d-c3
  - subject: syllogism_of_existence
  - predicate: transitions_to
  - object: syllogism_of_reflection
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [874-895] explicit transition to syllogism of reflection.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-ref-001
  - note: handoff to Part B first entry placeholder.
  - sourceClaimIds: [`syl-exi-d-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: includes explicit U-U-U subspecies as requested.

### Entry syl-exi-d-001 — Fourth figure subsection 1: mathematical equal-to-a-third schema

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 787
- lineEnd: 803

Summary:

The first subsection defines the mathematical syllogism as equality-through-third, eliminating inherence/subsumption and fixing mediation choice as external.

Key points: (KeyPoint)

- k1. U-U-U form is equality of two terms via a third.
- k2. Inherence/subsumption relation is removed.
- k3. Choice of mediating arrangement is externally given.

Claims: (Claim)

- c1. id: syl-exi-d-001-c1
  - subject: mathematical_syllogism
  - predicate: has_form
  - object: equal_to_a_third_implies_equal_to_each_other
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [787-790] explicit mathematical schema.

- c2. id: syl-exi-d-001-c2
  - subject: U_U_U_relation
  - predicate: eliminates
  - object: inherence_and_subsumption_of_terms
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [791-792] inherence/subsumption removed.

- c3. id: syl-exi-d-001-c3
  - subject: mediation_selection
  - predicate: depends_on
  - object: external_circumstances
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [797-803] selection of immediate/mediated connections is external.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-d-002
  - note: schema exposition transitions to critique of axiomatic self-evidence.
  - sourceClaimIds: [`syl-exi-d-001-c2`, `syl-exi-d-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-d-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: fourth-figure subsection `1` stabilized.

### Entry syl-exi-d-002 — Fourth figure subsection 2: axiomatic self-evidence as abstraction-effect

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 804
- lineEnd: 832

Summary:

The second subsection critiques axiomatic status as arising from formal abstraction from qualitative and conceptual determination.

Key points: (KeyPoint)

- k1. Mathematical syllogism is treated as axiom by immediacy claim.
- k2. Its self-evidence stems from abstraction from qualitative difference.
- k3. Consequently, it contains no conceptual comprehension.

Claims: (Claim)

- c1. id: syl-exi-d-002-c1
  - subject: mathematical_syllogism_in_mathematics
  - predicate: is_treated_as
  - object: immediate_axiom
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [804-808] axiom characterization.

- c2. id: syl-exi-d-002-c2
  - subject: axiomatic_self_evidence
  - predicate: rests_on
  - object: abstraction_from_qualitative_difference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [812-819] formalism and abstraction account.

- c3. id: syl-exi-d-002-c3
  - subject: mathematical_syllogism
  - predicate: lacks
  - object: conceptual_comprehension
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [826-832] no concept determinations/comprehension present.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-d-003
  - note: abstraction critique transitions to positive result and reflection handoff.
  - sourceClaimIds: [`syl-exi-d-002-c2`, `syl-exi-d-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syl-exi-d-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: fourth-figure subsection `2` stabilized.

### Entry syl-exi-d-003 — Fourth figure subsection 3: positive result and transition to reflection

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 833
- lineEnd: 895

Summary:

The third subsection yields the positive result: mediation through concrete identity of determinacies, closing reciprocal presupposition and transitioning to syllogism of reflection.

Key points: (KeyPoint)

- k1. Result is not mere abstract dissolution into quantity.
- k2. Figures close as circle of mediation-on-mediation.
- k3. Mediation through concrete identity transitions to reflection syllogism.

Claims: (Claim)

- c1. id: syl-exi-d-003-c1
  - subject: result_of_syllogism_of_existence
  - predicate: includes
  - object: positive_concretion_of_determinateness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [833-840] positive side beyond abstract dissolution.

- c2. id: syl-exi-d-003-c2
  - subject: reciprocal_presupposition_of_figures
  - predicate: forms
  - object: closed_totality_of_self_referring_mediation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [842-865] mediation based on mediation and closed circle.

- c3. id: syl-exi-d-003-c3
  - subject: immediate_syllogism_of_existence
  - predicate: passes_over_into
  - object: syllogism_of_reflection
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [890-895] explicit transition sentence.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-exi-d
  - note: subsection completion grounds marker-level transition claim.
  - sourceClaimIds: [`syl-exi-d-003-c1`, `syl-exi-d-003-c2`, `syl-exi-d-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-exi-d-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-ref-001
  - note: concrete handoff to Part B reflection opening.
  - sourceClaimIds: [`syl-exi-d-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: fourth-figure subsection `3` stabilized; Part A first-pass numeric decomposition complete.
