# Judgment Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file only fixes the four moments of judgment and its immediate extension toward science.
- Use the distillation as the fast commentary path out of this workbook.
- Return to the source text when checking whether the workbook remains complete and correct.

## Quick orientation

- First question: what is being operated on here?
  Answer: the unity of consciousness as judgment-form.
- Second question: what are the core operator families?
  Answer: quantity and quality as sphere and placement protocols, relation as matter-form operators, modality as consciousness-status operator.
- Third question: why is the workbook denser than the distillation?
  Answer: the distillation gives the narrative pass, while the workbook fixes the operator inventory for KG use.
- Fourth question: how should this file be read against the distillation?
  Answer: read the workbook for logical commitments, the distillation for quick commentary, and the source text for verification.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `judgment.md` and `JUDGMENT-DISTILLATION.md`
- Distillation role: commentary on workbooked logic and structural moves.
- Upstream source authority: `judgment.md`
- This workbook covers only the judgment module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Do not confuse logical form with empirical acts of asserting.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (third pass)

Scope:

- files:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- pass policy: 1 marker entry + expanded analytic entries by logical moment and operator

Decision:

- Keep this workbook readable.
- Separate quantity and quality so their operative protocols are not collapsed into overview language.
- Separate the relation operators so categorical, hypothetical, and disjunctive each keep their own matter/form protocol.
- Make the disjunctive operator precise enough to show how relation and modality cross inside it.
- Keep the late scientific extension grouped after the formal machinery is fixed.

### Entry kant-judgment — Marker `Judgment`

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:7-170`
- summary: Judgment is the explicit unity of consciousness organized by four logical moments and extended toward science.

Key points: (KeyPoint)

- k1. Judgment is ordered by quantity, quality, relation, and modality.
- k2. Judgment binds representations into explicit unity.
- k3. Logical form already prepares scientific use.

Claims: (Claim)

- c1. id: kant-judgment-c1
  - subject: judgment
  - predicate: is_ordered_by
  - object: quantity_quality_relation_modality
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `JUDGMENT-DISTILLATION.md:7-17`

- c2. id: kant-judgment-c2
  - subject: judgment
  - predicate: makes_explicit
  - object: unity_of_consciousness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `JUDGMENT-DISTILLATION.md:49-59`
    - `JUDGMENT-DISTILLATION.md:147-157`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-subj-triad
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: judgment is the second explicit moment of subjectivity.
  - sourceClaimIds: [`kant-judgment-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-subj-triad-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the formal role of judgment before downstream scientific classifications are expanded.

### Entry kant-judgment-quantity — `Judgment`: quantity and spherical enclosure

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:21-31`
- summary: Quantity determines judgment through the degree to which one conceptual sphere is enclosed under another.

Key points: (KeyPoint)

- k1. Universal judgment wholly encloses one conceptual sphere within another.
- k2. Particular and singular judgments differentiate partial enclosure from the case of a concept with no sphere.
- k3. Quantity is a protocol of extent through spherical inclusion.

Claims: (Claim)

- c1. id: kant-judgment-quantity-c1
  - subject: universal_particular_and_singular_judgment
  - predicate: are_differentiated_by
  - object: degree_of_spherical_enclosure
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `JUDGMENT-DISTILLATION.md:21-31`

- c2. id: kant-judgment-quantity-c2
  - subject: quantity_of_judgment
  - predicate: governs
  - object: conceptual_extent_of_subsumption
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:21-31`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: quantity makes the first protocol of judgment explicit through spherical inclusion.
  - sourceClaimIds: [`kant-judgment-quantity-c1`, `kant-judgment-quantity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`]

Review outcome:

- review_pending
- notes: this entry restores the protocol of spheres that the compressed pass omitted.

### Entry kant-judgment-quality — `Judgment`: quality and positional determination

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:35-45`
- summary: Quality determines judgment through the placement of the subject within, outside, or in a determinate outside of the predicate sphere.

Key points: (KeyPoint)

- k1. Affirmative judgment places the subject within the predicate sphere.
- k2. Negative judgment places the subject outside the predicate sphere.
- k3. Infinite judgment places the subject in a determinate outside rather than merely negating.

Claims: (Claim)

- c1. id: kant-judgment-quality-c1
  - subject: quality_of_judgment
  - predicate: determines
  - object: subject_position_relative_to_predicate_sphere
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `JUDGMENT-DISTILLATION.md:35-45`

- c2. id: kant-judgment-quality-c2
  - subject: infinite_judgment
  - predicate: places
  - object: subject_in_determinate_outside
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:37-45`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: quality makes judgment explicit as a protocol of positional determination.
  - sourceClaimIds: [`kant-judgment-quality-c1`, `kant-judgment-quality-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`]

Review outcome:

- review_pending
- notes: this entry keeps infinite judgment as a distinct logical operation rather than collapsing it into bare negation.

### Entry kant-judgment-relation — `Judgment`: relation as subordination protocol

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:49-59`
- summary: Relation determines judgment by three protocols of subordination for the unity of consciousness.

Key points: (KeyPoint)

- k1. Relation subordinates representations for the unity of consciousness.
- k2. Categorical, hypothetical, and disjunctive are distinct operators of that subordination.
- k3. Relation is the core operator family within judgment.

Claims: (Claim)

- c1. id: kant-judgment-relation-c1
  - subject: relation_of_judgment
  - predicate: subordinates
  - object: representations_for_unity_of_consciousness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `JUDGMENT-DISTILLATION.md:49-59`

- c2. id: kant-judgment-relation-c2
  - subject: categorical_hypothetical_and_disjunctive_judgment
  - predicate: are
  - object: three_protocols_of_subordination
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:49-59`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: relation states the operator-family that governs subordination within judgment.
  - sourceClaimIds: [`kant-judgment-relation-c1`, `kant-judgment-relation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`, `kant-judgment-c2`]

Review outcome:

- review_pending
- notes: this entry fixes relation as the operator-family rather than leaving it as a simple list of judgment kinds.

### Entry kant-judgment-categorical — `Judgment`: categorical matter and form

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:63-73`
- summary: Categorical judgment operates by binding subject and predicate through the copula.

Key points: (KeyPoint)

- k1. Subject and predicate are the matter of categorical judgment.
- k2. The copula is the form that determines and expresses agreement or opposition.
- k3. Categorical judgment is the operator of predicative binding.

Claims: (Claim)

- c1. id: kant-judgment-categorical-c1
  - subject: categorical_judgment
  - predicate: has_matter
  - object: subject_and_predicate
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `JUDGMENT-DISTILLATION.md:63-73`

- c2. id: kant-judgment-categorical-c2
  - subject: categorical_judgment
  - predicate: has_form
  - object: copula
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `JUDGMENT-DISTILLATION.md:63-73`

Relations: (Relation)

- r1. type: specializes
  - targetEntryId: kant-judgment-relation
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: categorical judgment specifies the first relation operator.
  - sourceClaimIds: [`kant-judgment-categorical-c1`, `kant-judgment-categorical-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-relation-c2`]

Review outcome:

- review_pending
- notes: the copula now appears as the formal operator rather than disappearing into generic predication language.

### Entry kant-judgment-hypothetical — `Judgment`: hypothetical matter and form

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:77-104`
- summary: Hypothetical judgment operates by connecting antecedent and consequent through consequentia and its modes of lawful transition.

Key points: (KeyPoint)

- k1. Antecedent and consequent are the matter of hypothetical judgment.
- k2. Consequentia is the form of the ground-consequence connection.
- k3. Modus ponens and modus tollens express the lawful transitions internal to this form.

Claims: (Claim)

- c1. id: kant-judgment-hypothetical-c1
  - subject: hypothetical_judgment
  - predicate: has_matter
  - object: antecedent_and_consequent
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `JUDGMENT-DISTILLATION.md:77-87`

- c2. id: kant-judgment-hypothetical-c2
  - subject: hypothetical_judgment
  - predicate: has_form
  - object: consequentia_with_modus_ponens_and_modus_tollens
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `JUDGMENT-DISTILLATION.md:77-104`

Relations: (Relation)

- r1. type: specializes
  - targetEntryId: kant-judgment-relation
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: hypothetical judgment specifies the second relation operator.
  - sourceClaimIds: [`kant-judgment-hypothetical-c1`, `kant-judgment-hypothetical-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-relation-c2`]

Review outcome:

- review_pending
- notes: consequentia and its two modes are now preserved as formal machinery instead of being reduced to a generic if-then schema.

### Entry kant-judgment-disjunctive — `Judgment`: disjunctive matter, form, and truth protocol

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:108-136`
- summary: Disjunctive judgment operates by distributing a conceptual whole into complementary members and forcing truth through exhaustive division.

Key points: (KeyPoint)

- k1. The members of the disjunction are the matter of disjunctive judgment.
- k2. Disjunction itself is the form relating members as complementary parts of a whole sphere.
- k3. Disjunctive form carries an exhaustive truth protocol because problematic members are forced to assertoric resolution within the completed division.

Claims: (Claim)

- c1. id: kant-judgment-disjunctive-c1
  - subject: disjunctive_judgment
  - predicate: has_matter_and_form
  - object: members_of_disjunction_and_distributed_whole_relation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `JUDGMENT-DISTILLATION.md:108-122`

- c2. id: kant-judgment-disjunctive-c2
  - subject: disjunctive_judgment
  - predicate: carries
  - object: exhaustive_truth_protocol
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:126-136`

Relations: (Relation)

- r1. type: specializes
  - targetEntryId: kant-judgment-relation
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: disjunctive judgment specifies the third relation operator.
  - sourceClaimIds: [`kant-judgment-disjunctive-c1`, `kant-judgment-disjunctive-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-relation-c2`]

Review outcome:

- review_pending
- notes: disjunctive judgment now preserves both the whole-member complementarity and the internal shift from problematic members to assertoric truth.

### Entry kant-judgment-modality — `Judgment`: modality as consciousness operator

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:133-143`
- summary: Modality determines how the whole judgment is held by consciousness as possible, actual, or necessary.

Key points: (KeyPoint)

- k1. Modality marks possibility, actuality, and necessity.
- k2. Judgment is not only content but also cognitive stance.
- k3. Modality determines how the judgment is held by consciousness.

Claims: (Claim)

- c1. id: kant-judgment-modality-c1
  - subject: modality_of_judgment
  - predicate: determines
  - object: cognitive_stance_toward_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `JUDGMENT-DISTILLATION.md:135-143`

- c2. id: kant-judgment-modality-c2
  - subject: modality_of_judgment
  - predicate: is
  - object: consciousness_operator_of_holding
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:135-143`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: modality fixes the stance of the whole judgment after relation fixes its inner structure.
  - sourceClaimIds: [`kant-judgment-modality-c1`, `kant-judgment-modality-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`, `kant-judgment-c2`]

Review outcome:

- review_pending
- notes: modality is now kept distinct from the scientific extension and stated as the operator of holding.

### Entry kant-judgment-disjunctive-modality — `Judgment`: disjunctive crossing of relation and modality

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:119-157`
- summary: Disjunctive judgment is the place where relation and modality visibly cross, because problematic members are resolved within an exhaustive whole.

Key points: (KeyPoint)

- k1. Disjunctive members are problematic in isolation.
- k2. Exhaustive division forces one member to hold assertorically.
- k3. Disjunction coordinates operator structure and truth-status together.

Claims: (Claim)

- c1. id: kant-judgment-disjunctive-modality-c1
  - subject: disjunctive_judgment
  - predicate: distributes
  - object: modality_across_problematic_members
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:126-136`
    - `JUDGMENT-DISTILLATION.md:153-157`

- c2. id: kant-judgment-disjunctive-modality-c2
  - subject: disjunctive_judgment
  - predicate: forces
  - object: assertoric_resolution_within_exhaustive_division
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:121-129`
    - `JUDGMENT-DISTILLATION.md:149-157`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: this entry shows the strongest crossing of relation and modality inside the judgment machinery itself.
  - sourceClaimIds: [`kant-judgment-disjunctive-modality-c1`, `kant-judgment-disjunctive-modality-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c1`, `kant-judgment-c2`]

Review outcome:

- review_pending
- notes: this entry isolates the crossing point where status and operator are internally coordinated inside disjunctive form.

### Entry kant-judgment-science — `Judgment`: scientific extension

- sourceFiles:
  - `judgment.md`
  - `JUDGMENT-DISTILLATION.md`
- lineSpans:
  - `JUDGMENT-DISTILLATION.md:161-170`
- summary: The later sections extend judgment from formal machinery into proof, principle, and scientific practice.

Key points: (KeyPoint)

- k1. Judgment opens into proof and demonstration.
- k2. Principles are classified by their mode of grounding.
- k3. Logical form becomes the infrastructure of science.

Claims: (Claim)

- c1. id: kant-judgment-science-c1
  - subject: judgment
  - predicate: opens_into
  - object: proof_and_demonstration
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:161-170`

- c2. id: kant-judgment-science-c2
  - subject: logical_form
  - predicate: becomes
  - object: infrastructure_of_science
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `JUDGMENT-DISTILLATION.md:161-170`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: this terminal entry preserves the late move from logical form into scientific practice.
  - sourceClaimIds: [`kant-judgment-science-c1`, `kant-judgment-science-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-judgment-c2`]

Review outcome:

- review_pending
- notes: the science bridge is now preserved as its own terminal extension rather than being mixed into the disjunctive-modality operator.
