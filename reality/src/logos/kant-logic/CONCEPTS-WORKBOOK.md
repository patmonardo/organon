# Concepts Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes how the Analytic of Concepts derives the categories from the judging function of understanding.
- In the current presentation, it should be read as the first full articulation of pure understanding after the Intro Logic has fixed the analytic field.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `CONCEPTS-DISTILLATION.md`
- Upstream source authority: `concepts.md`
- This workbook covers only the Analytic of Concepts module.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not collapse the categories into a later dialectical logic.
- Do not treat logical criteria of cognition as extra ontological categories.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (upgrade pass)

Scope:

- files:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook architectonic but fuller than the earlier seed pass.
- Preserve the movement from systematic discovery, through judgment, to the table of categories and its internal structure.
- Keep the final anti-inflation boundary explicit so the table is not confused with predicables or scholastic transcendentals.

### Entry kant-trans-concepts — Marker `Analytic of Concepts`

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:7-23`
  - `concepts.md:34-44`
  - `concepts.md:64-69`
  - `concepts.md:79-116`
  - `concepts.md:246-315`
  - `concepts.md:381-398`
  - `concepts.md:442-485`
- summary: The Analytic of Concepts researches the pure concepts of understanding by analyzing the understanding as a faculty of judging and deriving from it the complete table of categories.

Key points: (KeyPoint)

- k1. Pure understanding must be discovered systematically rather than mechanically.
- k2. Understanding is discursive and operative through judgment.
- k3. Categories arise from the same functions that structure judgments.
- k4. The table has internal order and fixed limits.

Claims: (Claim)

- c1. id: kant-trans-concepts-c1
  - subject: analytic_of_concepts
  - predicate: researches
  - object: possibility_of_pure_concepts_by_analyzing_understanding_itself_as_a_system
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `concepts.md:15-23`
    - `concepts.md:34-44`
    - `concepts.md:64-69`

- c2. id: kant-trans-concepts-c2
  - subject: categories
  - predicate: arise_from
  - object: same_functions_of_unity_that_are_exhibited_in_judgments_and_required_for_synthesis_of_intuition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:289-301`
    - `concepts.md:313-315`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: the introduction's analytic side first becomes concrete in the search for pure concepts of understanding.
  - sourceClaimIds: [`kant-trans-concepts-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-trans-intro-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-deduction-trans-a
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: once the categories are identified, their lawful relation to objects must be justified by deduction.
  - sourceClaimIds: [`kant-trans-concepts-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the full route from systematic discovery to the category table before the deduction begins.

### Entry kant-trans-concepts-system — `Analytic of Concepts`: system, birthplace, and principle

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:7-23`
  - `concepts.md:34-44`
  - `concepts.md:54-69`
- summary: Kant insists that pure concepts must be discovered from the understanding itself and ordered systematically under one idea rather than collected mechanically.

Key points: (KeyPoint)

- k1. Completeness requires an idea of the whole.
- k2. The Analytic of Concepts studies the understanding itself as the birthplace of a priori concepts.
- k3. Discovery by principle replaces rhapsodic aggregation.

Claims: (Claim)

- c1. id: kant-trans-concepts-system-c1
  - subject: pure_understanding
  - predicate: must_be_grasped_as
  - object: self_sufficient_system_of_a_priori_cognition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:15-23`

- c2. id: kant-trans-concepts-system-c2
  - subject: transcendental_philosophy
  - predicate: seeks
  - object: pure_concepts_in_accordance_with_a_principle_that_determines_their_place_and_completeness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:34-44`
    - `concepts.md:54-69`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: this entry fixes the method of discovery before the judging function is unfolded.
  - sourceClaimIds: [`kant-trans-concepts-system-c1`, `kant-trans-concepts-system-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-concepts-c1`]

Review outcome:

- review_pending
- notes: the anti-rhapsodic demand is kept explicit because it governs the claim to completeness.

### Entry kant-trans-concepts-judgment — `Analytic of Concepts`: discursive understanding and judgment

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:79-116`
- summary: Understanding is discursive, concepts are functions of unity, and judgment reveals the operative form through which understanding cognizes objects mediately.

Key points: (KeyPoint)

- k1. Human understanding cognizes through concepts rather than intuition.
- k2. Concepts rest on functions that order representations under a common one.
- k3. Judgment is the mediate cognition of an object.
- k4. Understanding can be represented as a faculty for judging.

Claims: (Claim)

- c1. id: kant-trans-concepts-judgment-c1
  - subject: understanding
  - predicate: is
  - object: discursive_faculty_of_cognition_through_concepts
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `concepts.md:79-88`

- c2. id: kant-trans-concepts-judgment-c2
  - subject: judgment
  - predicate: discloses
  - object: functions_of_unity_by_which_understanding_mediately_cognizes_objects
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:90-116`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: the route to categories depends on the prior thesis that all actions of understanding can be traced to judgments.
  - sourceClaimIds: [`kant-trans-concepts-judgment-c1`, `kant-trans-concepts-judgment-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-concepts-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the judgment route explicit so the transition to categories remains principled.

### Entry kant-trans-concepts-table — `Analytic of Concepts`: the table of judgments

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:126-236`
- summary: The table of judgments exhaustively displays the functions of thinking under quantity, quality, relation, and modality, including transcendental clarifications not preserved by ordinary textbook logic.

Key points: (KeyPoint)

- k1. The logical functions of judgment fall under four titles with three moments each.
- k2. Singular and infinite judgments require transcendental attention.
- k3. Hypothetical and disjunctive judgments already disclose rule, implication, and community.
- k4. Modality concerns the value of the copula in thinking rather than the content judged.

Claims: (Claim)

- c1. id: kant-trans-concepts-table-c1
  - subject: table_of_judgments
  - predicate: exhaustively_displays
  - object: functions_of_thinking_under_quantity_quality_relation_and_modality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `concepts.md:126-133`
    - `concepts.md:233-236`

- c2. id: kant-trans-concepts-table-c2
  - subject: transcendental_logic
  - predicate: preserves
  - object: distinctions_in_singular_infinite_and_disjunctive_forms_because_they_matter_for_pure_cognition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `concepts.md:135-174`
    - `concepts.md:188-201`
    - `concepts.md:202-231`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-concepts-categories
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: the table of judgments supplies the clue from which the pure concepts of understanding are generated.
  - sourceClaimIds: [`kant-trans-concepts-table-c1`, `kant-trans-concepts-table-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-concepts-categories-c1`, `kant-trans-concepts-categories-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the judgment table distinct from the category table while preserving their functional continuity.

### Entry kant-trans-concepts-categories — `Categories`: synthesis and table

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:246-321`
- summary: Transcendental logic starts from a manifold of pure intuition, shows that synthesis and its unity are necessary for cognition, and derives the categories as the pure concepts corresponding to the functions of judgment.

Key points: (KeyPoint)

- k1. Pure intuition provides a manifold that must be synthesized.
- k2. Imagination effects synthesis, but understanding gives it necessary conceptual unity.
- k3. The same function that unifies judgments also unifies synthesis in intuition.
- k4. Exactly as many categories arise as there are logical functions of judgment.

Claims: (Claim)

- c1. id: kant-trans-concepts-categories-c1
  - subject: pure_concepts_of_understanding
  - predicate: give
  - object: necessary_synthetic_unity_to_the_manifold_of_intuition_for_object_cognition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:249-287`

- c2. id: kant-trans-concepts-categories-c2
  - subject: categories
  - predicate: correspond_in_number_and_origin_to
  - object: logical_functions_of_all_possible_judgments
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:289-315`
    - `concepts.md:317-321`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: this entry states the central transcendental claim of the chapter: categories arise from the same unity-functions first displayed in judgment.
  - sourceClaimIds: [`kant-trans-concepts-categories-c1`, `kant-trans-concepts-categories-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-concepts-c2`]

Review outcome:

- review_pending
- notes: synthesis is kept explicit here so the categories do not appear as merely re-labeled logical forms.

### Entry kant-trans-concepts-structure — `Categories`: mathematical, dynamical, and mediated thirds

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:381-431`
- summary: Kant adds internal architectonic remarks to the category table, dividing it into mathematical and dynamical classes and arguing that the third category in each class arises through a special act combining the first two.

Key points: (KeyPoint)

- k1. The first two classes are mathematical and the latter two dynamical.
- k2. Each class has three categories because the third mediates the first two.
- k3. Community clarifies the reciprocal whole implied in disjunctive division.

Claims: (Claim)

- c1. id: kant-trans-concepts-structure-c1
  - subject: category_table
  - predicate: divides_into
  - object: mathematical_and_dynamical_classes_with_distinct_orientations
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `concepts.md:381-387`

- c2. id: kant-trans-concepts-structure-c2
  - subject: third_category_in_each_class
  - predicate: arises_from
  - object: special_act_combining_first_two_terms_without_reducing_to_them
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `concepts.md:389-406`

- c3. id: kant-trans-concepts-structure-c3
  - subject: category_of_community
  - predicate: corresponds_to
  - object: reciprocal_coordination_exhibited_in_disjunctive_judgment
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - `concepts.md:408-431`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-concepts-categories
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: the structural remarks show that the category table has internal architectonic articulation rather than mere completeness.
  - sourceClaimIds: [`kant-trans-concepts-structure-c1`, `kant-trans-concepts-structure-c2`, `kant-trans-concepts-structure-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-concepts-categories-c2`]

Review outcome:

- review_pending
- notes: the mediated-third structure is kept explicit because it is one of the chapter's most important architectonic remarks.

### Entry kant-trans-concepts-boundary — `Categories`: predicables and scholastic transcendentals

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `concepts.md:333-350`
  - `concepts.md:442-485`
- summary: Kant distinguishes derivative predicables from original categories and rejects unity, truth, and perfection as extra transcendental predicates of objects.

Key points: (KeyPoint)

- k1. Predicables are derivative pure concepts subordinate to the categories.
- k2. Unity, truth, and perfection are logical criteria of cognition.
- k3. The category table must not be inflated by mistaking formal criteria for object-concepts.

Claims: (Claim)

- c1. id: kant-trans-concepts-boundary-c1
  - subject: predicables
  - predicate: are
  - object: pure_derivative_concepts_subordinate_to_original_categories
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `concepts.md:333-350`

- c2. id: kant-trans-concepts-boundary-c2
  - subject: unity_truth_and_perfection
  - predicate: are
  - object: logical_criteria_of_cognition_not_additional_categories_of_objects
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `concepts.md:442-485`

Relations: (Relation)

- r1. type: bounds
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: this entry fixes both the legitimate extension and the strict limit of the category table.
  - sourceClaimIds: [`kant-trans-concepts-boundary-c1`, `kant-trans-concepts-boundary-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-concepts-c2`]

Review outcome:

- review_pending
- notes: the closing boundary is held separate so derivative concepts and logical criteria do not obscure the completed table.
