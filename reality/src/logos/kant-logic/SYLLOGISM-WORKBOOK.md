# Syllogism Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file treats Syllogism as the derivational region of Subjectivity.
- It keeps the faculty split explicit across understanding, reason, and reflective judgment.
- It also preserves the stronger derivative reading that this is the clearest Kantian site for the truth of ground on the General Logic side.
- The strongest internal complexity of the chapter lies in the categorical syllogism, especially in the pure first figure.

## Quick orientation

- First question: what is the main split in the chapter?
  Answer: understanding transforms judgment-form immediately, reason mediates necessity through rules, and reflective judgment ascends from particulars by common ground.
- Second question: where is the densest center of inferential complexity?
  Answer: in the categorical syllogism, and especially in the pure first figure to which the other figures are reduced.
- Third question: where does the chapter become especially important for our larger architecture?
  Answer: where categorical mediation, hypothetical ground, reflective common ground, and chains of grounds and consequences can be differentiated without being flattened.
- Fourth question: how should this workbook be used?
  Answer: read the workbook for commitments, the distillation for fast commentary, and the source text for verification.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `syllogism.md` and `SYLLOGISM-DISTILLATION.md`
- Upstream source authority: `syllogism.md`
- This workbook covers only the syllogism module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Do not flatten understanding, reason, and reflective judgment into one undifferentiated faculty.
- Do not force later Hegelian Ground doctrine into Kant's source text.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-28 (grounded mediation pass)

Scope:

- files:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- pass policy: 1 marker entry + compact analytic entries by inferential region and grounded operator

Decision:

- Regenerate the derivative layers directly from `syllogism.md` without altering the Kant source.
- Keep the faculty split explicit.
- Make the chapter's movement from formal alteration to grounded mediation to reflective and chained extension easier to see.
- Preserve the insight that this is the strongest Kantian site for reading syllogism as the truth of ground, but hold that reading at the derivative level.
- Treat the pure first-figure categorical syllogism as the chapter's privileged center of complexity, while keeping any stricter hierarchy over hypothetical and disjunctive inference clearly derivative.

### Entry kant-syllogism - Marker `Syllogism`

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:1-596`
- summary: Syllogism is the derivational region of Subjectivity, unfolding inference as a field that moves from immediate formal transformation through grounded mediation to reflective and chained extension, with the pure categorical syllogism as its privileged center.

Key points: (KeyPoint)

- k1. Immediate inference belongs to the understanding and alters only form.
- k2. Inference of reason mediates necessity through rule, subsumption, and conclusion.
- k3. Reflective judgment and chained reasoning extend inferential movement beyond a single deduction.
- k4. The chapter's densest inferential doctrine lies in the categorical syllogism and its first figure.

Claims: (Claim)

- c1. id: kant-syllogism-c1
  - subject: syllogism
  - predicate: organizes
  - object: inferential_transition_across_understanding_reason_and_reflective_judgment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism.md:1-133`
    - `syllogism.md:138-468`
    - `syllogism.md:470-596`

- c2. id: kant-syllogism-c2
  - subject: syllogism
  - predicate: culminates_in
  - object: grounded_reflective_and_chained_forms_of_mediation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `syllogism.md:397-596`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-subj-triad
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: syllogism completes the subjectivity triad as the region of derivational transition.
  - sourceClaimIds: [`kant-syllogism-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-subj-triad-c1`]

Review outcome:

- review_pending
- notes: marker entry now fixes syllogism as derivationally structured and no longer as a bare list of inferential species.

### Entry kant-syllogism-understanding - `Syllogism`: immediate inference as formal transformation

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:1-133`
- summary: The understanding-side of syllogism transforms judgment-form without changing matter, distributing immediate inference across quantity, quality, relation, and modality.

Key points: (KeyPoint)

- k1. Immediate inference alters form while keeping subject and predicate fixed.
- k2. Subalternation, opposition, conversion, and contraposition rework the moments of judgment into inferential operators.
- k3. The understanding prepares derivation without yet introducing mediated necessity.

Claims: (Claim)

- c1. id: kant-syllogism-understanding-c1
  - subject: immediate_inference
  - predicate: alters
  - object: judgment_form_while_preserving_matter
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:3-9`

- c2. id: kant-syllogism-understanding-c2
  - subject: understanding_inference
  - predicate: distributes_into
  - object: subalternation_opposition_conversion_and_contraposition
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `syllogism.md:12-133`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry fixes the understanding-side operator family of the chapter.
  - sourceClaimIds: [`kant-syllogism-understanding-c1`, `kant-syllogism-understanding-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`]

Review outcome:

- review_pending
- notes: immediate inference is preserved as preparatory formal mobility, not confused with reason proper.

### Entry kant-syllogism-reason-ground - `Syllogism`: reason as mediated necessity

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:138-200`
- summary: Inference of reason introduces necessity through subsumption under a universal rule and divides mediated inference into categorical, hypothetical, and disjunctive families according to the form of the major premise, but the chapter does not load these three families evenly.

Key points: (KeyPoint)

- k1. Inference of reason cognizes necessity through a universal rule.
- k2. Major, minor, and conclusion are the essential three moments of mediated inference.
- k3. Reason divides by major premise into categorical, hypothetical, and disjunctive families.
- k4. Jasche names all three as inferences of reason, but the categorical receives the full doctrine of pure mediation while hypothetical and disjunctive are treated more briefly.

Claims: (Claim)

- c1. id: kant-syllogism-reason-ground-c1
  - subject: inference_of_reason
  - predicate: cognizes
  - object: necessity_through_subsumption_under_a_universal_rule
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism.md:138-175`

- c2. id: kant-syllogism-reason-ground-c2
  - subject: inference_of_reason
  - predicate: divides_into
  - object: categorical_hypothetical_and_disjunctive_families_by_major_premise
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:177-200`

- c3. id: kant-syllogism-reason-ground-c3
  - subject: syllogism_chapter
  - predicate: gives_privileged_weight_to
  - object: categorical_mediation_within_the_reason_section
  - modality: interpreted
  - confidence: 0.9
  - evidence:
    - `syllogism.md:177-200`
    - `syllogism.md:203-394`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry fixes reason as the proper site of mediated necessity.
  - sourceClaimIds: [`kant-syllogism-reason-ground-c1`, `kant-syllogism-reason-ground-c2`, `kant-syllogism-reason-ground-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-syllogism-c1`]

Review outcome:

- review_pending
- notes: Jasche still names all three as inferences of reason here, so any stricter redistribution of hypothetical and disjunctive functions must stay explicitly derivative.

### Entry kant-syllogism-categorical - `Syllogism`: the middle term and the legitimacy of figure

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:203-394`
- summary: The categorical syllogism mediates through the middle term as a mark, develops the chapter's fullest doctrine of reasoned mediation, and then tests the legitimacy of its mediation through the doctrine of figures, where only the first figure is pure and the latter three are mixed through silent immediate inference.

Key points: (KeyPoint)

- k1. Major, minor, and middle term structure categorical mediation.
- k2. The principle of categorical inference is mark-transmission.
- k3. The first figure is legitimate, while the latter figures require reduction and are mixed.
- k4. This makes the pure first-figure categorical syllogism the chapter's privileged scientific center on a derivative reading.

Claims: (Claim)

- c1. id: kant-syllogism-categorical-c1
  - subject: categorical_inference
  - predicate: mediates_through
  - object: middle_term_as_nota_intermedia
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism.md:203-229`

- c2. id: kant-syllogism-categorical-c2
  - subject: categorical_inference
  - predicate: rests_on
  - object: mark_transmission_from_the_middle_term_to_the_thing
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:220-277`

- c3. id: kant-syllogism-categorical-c3
  - subject: doctrine_of_figures
  - predicate: distinguishes
  - object: pure_first_figure_from_mixed_latter_figures_requiring_reduction
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `syllogism.md:278-394`

- c4. id: kant-syllogism-categorical-c4
  - subject: pure_first_figure_categorical_inference
  - predicate: functions_as
  - object: privileged_center_of_the_chapter's_reasoned_complexity
  - modality: interpreted
  - confidence: 0.9
  - evidence:
    - `syllogism.md:220-394`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-syllogism-reason-ground
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: categorical inference supplies the most elaborate internal doctrine of rule-mediated necessity.
  - sourceClaimIds: [`kant-syllogism-categorical-c1`, `kant-syllogism-categorical-c2`, `kant-syllogism-categorical-c3`, `kant-syllogism-categorical-c4`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-syllogism-reason-ground-c1`, `kant-syllogism-reason-ground-c2`, `kant-syllogism-reason-ground-c3`]

Review outcome:

- review_pending
- notes: this entry keeps figure doctrine subordinate to the question of legitimate mediation and makes explicit why the first figure bears the chapter's greatest logical weight.

### Entry kant-syllogism-ground-division - `Syllogism`: hypothetical ground, disjunctive division, and contracted expression

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:397-468`
- summary: Hypothetical inference states the principle of ground and opens the line from ground-grounded to condition-conditioned sequence, disjunctive inference states the principle of excluded middle over members of division, and dilemma plus covert inference show combined and contracted forms of mediation; on a derivative reading, the first two serve as objective preparation for categorical science.

Key points: (KeyPoint)

- k1. Hypothetical inference is governed by the principle of ground.
- k2. Disjunctive inference is governed by excluded middle over a complete division.
- k3. Dilemma and covert inference show compound and contracted expression of mediation.
- k4. Compared with the categorical syllogism, hypothetical and disjunctive inference here function more as exposed necessity- and division-forms than as equally elaborated doctrines of pure mediation.
- k5. Hypothetical and disjunctive inference can be read as objective servants of science: the one discovers grounds, the other orders conceptual wholes and exhaustive alternatives.

Claims: (Claim)

- c1. id: kant-syllogism-ground-division-c1
  - subject: hypothetical_inference
  - predicate: rests_on
  - object: principle_of_ground
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism.md:397-412`

- c1b. id: kant-syllogism-ground-division-c1b
  - subject: hypothetical_inference
  - predicate: exposes
  - object: relation_of_condition_and_conditioned
  - modality: interpreted
  - confidence: 0.9
  - evidence:
    - `syllogism.md:138-149`
    - `syllogism.md:397-412`

- c2. id: kant-syllogism-ground-division-c2
  - subject: disjunctive_inference
  - predicate: rests_on
  - object: excluded_middle_over_members_of_division
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:414-438`

- c2b. id: kant-syllogism-ground-division-c2b
  - subject: disjunctive_inference
  - predicate: orders
  - object: conceptual_whole_through_exhaustive_members_and_exclusion
  - modality: interpreted
  - confidence: 0.9
  - evidence:
    - `syllogism.md:414-438`

- c2c. id: kant-syllogism-ground-division-c2c
  - subject: hypothetical_and_disjunctive_inference
  - predicate: prepare
  - object: objective_material_for_categorical_science
  - modality: interpreted
  - confidence: 0.86
  - evidence:
    - `syllogism.md:177-200`
    - `syllogism.md:397-438`
    - `syllogism.md:220-277`

- c3. id: kant-syllogism-ground-division-c3
  - subject: dilemma_and_covert_inference
  - predicate: exhibit
  - object: combined_or_contracted_forms_of_mediation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `syllogism.md:440-468`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-syllogism-reason-ground
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: hypothetical and disjunctive reasoning make explicit the grounded and divisive forms already announced in the general division of reason.
  - sourceClaimIds: [`kant-syllogism-ground-division-c1`, `kant-syllogism-ground-division-c1b`, `kant-syllogism-ground-division-c2`, `kant-syllogism-ground-division-c2b`, `kant-syllogism-ground-division-c2c`, `kant-syllogism-ground-division-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`kant-syllogism-reason-ground-c2`]

Review outcome:

- review_pending
- notes: this is the clearest source location where ground becomes explicit inside the chapter, while the derivative reading treats these forms as preparatory material for the stricter scientific center in categorical mediation.

### Entry kant-syllogism-reflective - `Syllogism`: reflective ascent by common ground

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:470-510`
- summary: The power of judgment proceeds reflectively from particulars to empirical universals under the principle that the many agree in one only through a common ground, yielding induction and analogy.

Key points: (KeyPoint)

- k1. Reflective judgment moves from particular to universal.
- k2. Its universality is empirical and only analogous to the logical.
- k3. Induction and analogy are grounded modes of reflective ascent.

Claims: (Claim)

- c1. id: kant-syllogism-reflective-c1
  - subject: reflective_power_of_judgment
  - predicate: proceeds
  - object: from_particular_to_empirical_universal
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:470-489`

- c2. id: kant-syllogism-reflective-c2
  - subject: reflective_inference
  - predicate: rests_on
  - object: common_ground_and_operates_through_induction_and_analogy
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:491-510`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry preserves reflective judgment as a distinct inferential direction inside the chapter.
  - sourceClaimIds: [`kant-syllogism-reflective-c1`, `kant-syllogism-reflective-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`, `kant-syllogism-c2`]

Review outcome:

- review_pending
- notes: reflective judgment is retained as inferential ascent, not collapsed into weak deduction.

### Entry kant-syllogism-chains-defects - `Syllogism`: serial mediation and its failures

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:513-596`
- summary: Composite inference chains grounds and consequences through polysyllogism and sorites, while fallacy, leap, circular proof, and disproportionate proof mark failures in the proportion of mediation.

Key points: (KeyPoint)

- k1. Composite inference subordinates several inferences as grounds and consequences.
- k2. Sorites compresses a chained series into a shortened path.
- k3. Inferential defect appears where mediation is omitted, presupposed, circular, excessive, or insufficient.

Claims: (Claim)

- c1. id: kant-syllogism-chains-defects-c1
  - subject: composite_inference
  - predicate: extends_into
  - object: polysyllogism_prosyllogism_episyllogism_and_sorites
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `syllogism.md:513-560`

- c2. id: kant-syllogism-chains-defects-c2
  - subject: inferential_defect
  - predicate: consists_in
  - object: distortion_or_omission_of_mediation_in_fallacy_leap_and_bad_proof
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `syllogism.md:562-596`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry preserves the chapter's architectonic extension into chains and anti-mediation.
  - sourceClaimIds: [`kant-syllogism-chains-defects-c1`, `kant-syllogism-chains-defects-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c2`]

Review outcome:

- review_pending
- notes: the close of the chapter is treated as essential machinery, not leftover miscellany.

### Entry kant-syllogism-truth-of-ground - `Syllogism`: derivative reading of grounded mediation

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:406-438`
  - `syllogism.md:491-553`
- summary: On a source-guided derivative reading, this chapter is the strongest Kantian site for treating syllogism as the truth of ground, with the pure categorical syllogism as the scientific center, hypothetical inference as the clearest display of necessity through ground and condition, and disjunctive inference as the division-and-excluded-middle side of the inferential field that orders conceptual wholes for science.

Key points: (KeyPoint)

- k1. Ground is explicit in hypothetical inference.
- k2. Common ground is explicit in reflective judgment.
- k3. Grounds and consequences are explicit in chained reasoning.
- k4. The pure first-figure categorical syllogism is the privileged center from which the chapter's scientific weight is best understood.
- k5. Hypothetical and disjunctive inference can be read as objective servants furnishing the material of categorical science.

Claims: (Claim)

- c1. id: kant-syllogism-truth-of-ground-c1
  - subject: syllogism_chapter
  - predicate: makes_explicit
  - object: ground_common_ground_and_grounds_consequences_as_inferential_operators
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `syllogism.md:406-438`
    - `syllogism.md:491-553`

- c2. id: kant-syllogism-truth-of-ground-c2
  - subject: syllogism
  - predicate: is_the_strongest_kantian_site_for
  - object: reading_grounded_mediation_within_general_logic
  - modality: interpreted
  - confidence: 0.88
  - evidence:
    - `syllogism.md:406-438`
    - `syllogism.md:491-553`

- c3. id: kant-syllogism-truth-of-ground-c3
  - subject: pure_first_figure_categorical_syllogism
  - predicate: can_be_read_as
  - object: chapter's_nearest_kantian_analogue_to_a_syllogism_of_science
  - modality: interpreted
  - confidence: 0.84
  - evidence:
    - `syllogism.md:220-394`
    - `syllogism.md:397-438`

- c4. id: kant-syllogism-truth-of-ground-c4
  - subject: hypothetical_and_disjunctive_inference
  - predicate: can_be_read_as
  - object: objective_servants_preparing_material_for_categorical_science
  - modality: interpreted
  - confidence: 0.85
  - evidence:
    - `syllogism.md:177-200`
    - `syllogism.md:397-438`

Relations: (Relation)

- r1. type: interprets
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this derivative entry names the Ground-reading without altering the source or importing a full later doctrine of Ground.
  - sourceClaimIds: [`kant-syllogism-truth-of-ground-c1`, `kant-syllogism-truth-of-ground-c2`, `kant-syllogism-truth-of-ground-c3`, `kant-syllogism-truth-of-ground-c4`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`kant-syllogism-c2`]

Review outcome:

- review_pending
- notes: this is the correct derivative place to connect Syllogism to Ground and to a later science-reading, while still keeping the phraseology explicitly interpretive rather than source-attributed.
