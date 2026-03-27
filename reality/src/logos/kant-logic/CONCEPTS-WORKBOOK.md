# Concepts Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes how the Analytic of Concepts derives the categories from the judging function of understanding.

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

## Session: 2026-03-26 (first pass)

Scope:

- files:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- pass policy: 1 marker entry + 1 analytic entry

Decision:

- Keep this workbook architectonic rather than encyclopedic.
- Fix the judging-function route from understanding to categories.
- Keep later remarks on class structure and transcendental predicates inside one analytic entry.

### Entry kant-trans-concepts — Marker `Analytic of Concepts`

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `CONCEPTS-DISTILLATION.md:7-42`
- summary: The Analytic of Concepts researches the pure concepts of understanding by analyzing understanding as a faculty of judging.

Key points: (KeyPoint)

- k1. Pure understanding must be discovered systematically.
- k2. Understanding is discursive and operative through judgment.
- k3. The categories arise from the same functions that structure judgments.

Claims: (Claim)

- c1. id: kant-trans-concepts-c1
  - subject: analytic_of_concepts
  - predicate: researches
  - object: possibility_of_pure_concepts_by_analyzing_understanding_itself
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `CONCEPTS-DISTILLATION.md:7-16`

- c2. id: kant-trans-concepts-c2
  - subject: categories
  - predicate: arise_from
  - object: functions_of_unity_exhibited_in_judgments
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `CONCEPTS-DISTILLATION.md:20-42`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: the introduction's analytic side first becomes concrete in the search for pure concepts of understanding.
  - sourceClaimIds: [`kant-trans-concepts-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`kant-trans-intro-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-deduction-meta
  - targetWorkbook: `DEDUCTION-META-WORKBOOK.md`
  - note: once the categories are identified, their lawful relation to objects must be justified by deduction.
  - sourceClaimIds: [`kant-trans-concepts-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-trans-deduction-meta-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the conceptual route from judging to categories before the deduction arguments are split out.

### Entry kant-trans-categories — `Categories`: table, structure, and limit

- sourceFiles:
  - `concepts.md`
  - `CONCEPTS-DISTILLATION.md`
- lineSpans:
  - `CONCEPTS-DISTILLATION.md:46-68`
- summary: The table of categories is complete because it is generated from the faculty of judging, and it must not be inflated by mistaking logical criteria for additional categories.

Key points: (KeyPoint)

- k1. The category table is systematic rather than inductive.
- k2. Mathematical and dynamical classes disclose internal structure.
- k3. Unity, truth, and perfection are logical criteria, not extra categories.

Claims: (Claim)

- c1. id: kant-trans-categories-c1
  - subject: category_table
  - predicate: is_structured_as
  - object: complete_system_of_mathematical_and_dynamical_classes
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPTS-DISTILLATION.md:46-57`

- c2. id: kant-trans-categories-c2
  - subject: unity_truth_and_perfection
  - predicate: function_as
  - object: logical_criteria_of_cognition_not_additional_categories
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPTS-DISTILLATION.md:59-68`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: the analytic entry clarifies both the internal structure and the boundary of the category table.
  - sourceClaimIds: [`kant-trans-categories-c1`, `kant-trans-categories-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-concepts-c2`]

Review outcome:

- review_pending
- notes: the synthetic role of the third category and the scholastic predicate issue remain grouped here at seed scale.
