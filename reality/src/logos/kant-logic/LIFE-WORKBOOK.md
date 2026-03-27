# Life Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file treats Life as the practical operator-manual inside the Idea chapter.
- It covers definition, division, method, and meditation.

## Quick orientation

- First question: what does Life do?
  Answer: it furthers logical perfection by clarifying concepts, partitioning spheres, and ordering scientific treatment.
- Second question: what are the main operator families?
  Answer: definition, logical division, divisions of method, and meditation.
- Third question: why does this matter for the earlier modules?
  Answer: Life supplies the working procedures that sharpen concept and prepare scientific ordering.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `life.md` and `LIFE-DISTILLATION.md`
- Upstream source authority: `life.md`
- This workbook covers only the life module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Treat Life as an internal division of Idea, not as a later metaphysical doctrine.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (operator pass)

Scope:

- files:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- pass policy: 1 marker entry + expanded analytic entries by practical operator family

Decision:

- Rebuild Life as the practical method side of the new Idea pass.
- Keep definition, division, method, and meditation distinct so their procedures are not compressed.
- Preserve the four-moment structure of definition explicitly.
- Keep the file readable enough for study while still fixing the KG operator inventory.

### Entry kant-life - Marker `Life`

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:7-200`
- summary: Life furthers the logical perfection of cognition through operators of definition, division, scientific method, and reflective meditation.

Key points: (KeyPoint)

- k1. Life advances logical perfection through practical operators.
- k2. Definition and division clarify conceptual content and sphere.
- k3. Method and meditation order cognition into scientific form.

Claims: (Claim)

- c1. id: kant-life-c1
  - subject: life
  - predicate: furthers
  - object: logical_perfection_of_cognition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `LIFE-DISTILLATION.md:7-17`

- c2. id: kant-life-c2
  - subject: life
  - predicate: organizes_by
  - object: definition_division_method_and_meditation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:21-200`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: life is one internal division of the Idea chapter.
  - sourceClaimIds: [`kant-life-c1`, `kant-life-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: marker entry fixes Life as a practical operator-manual rather than a loose appendix on pedagogy.

### Entry kant-life-definition-origin - `Life`: definition as adequacy and origin-split

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:21-45`
- summary: Definition begins as a distinct, precise, adequate concept and divides by origin into analytic and synthetic procedures for given and made concepts.

Key points: (KeyPoint)

- k1. Definition is sufficiently distinct, precise, and completely determined.
- k2. Analytic definition concerns given concepts and synthetic definition concerns made concepts.
- k3. The origin-split is crossed by a priori and a posteriori status.

Claims: (Claim)

- c1. id: kant-life-definition-origin-c1
  - subject: definition
  - predicate: is
  - object: distinct_precise_and_completely_determined_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `LIFE-DISTILLATION.md:21-31`

- c2. id: kant-life-definition-origin-c2
  - subject: definition
  - predicate: divides_into
  - object: analytic_given_and_synthetic_made_forms
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:35-45`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: this entry fixes the first operator split within definition.
  - sourceClaimIds: [`kant-life-definition-origin-c1`, `kant-life-definition-origin-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-c2`]

Review outcome:

- review_pending
- notes: the opening of Life is kept procedural rather than merely terminological.

### Entry kant-life-definition-procedures - `Life`: synthetic limits, analytic clarification, exposition, and description

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:49-87`
- summary: Definition proceeds either through limited synthetic routes or through analytic mark-clarification, with exposition and description serving as approximation operators beneath full definition.

Key points: (KeyPoint)

- k1. Synthetic definition splits into exposition and construction, but empirical synthesis cannot define completely.
- k2. Given concepts are defined analytically by successive clarification of marks.
- k3. Exposition and description provide lower-order clarification where full definition is unavailable.

Claims: (Claim)

- c1. id: kant-life-definition-procedures-c1
  - subject: synthetic_definition
  - predicate: is_limited_by
  - object: exposition_construction_and_the_impossibility_of_complete_empirical_definition
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:49-59`

- c2. id: kant-life-definition-procedures-c2
  - subject: analytic_definition_and_exposition_description
  - predicate: clarify
  - object: given_concepts_by_mark_analysis_or_approximation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `LIFE-DISTILLATION.md:63-87`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: this entry fixes the actual clarification procedures used inside Life.
  - sourceClaimIds: [`kant-life-definition-procedures-c1`, `kant-life-definition-procedures-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-c1`, `kant-life-c2`]

Review outcome:

- review_pending
- notes: approximation is preserved as part of the methodical ladder rather than treated as a failure case outside logic.

### Entry kant-life-definition-completeness - `Life`: nominal and real definition under the four moments

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:91-115`
- summary: Definition separates nominal from real cognition and judges completeness through quantity, quality, relation, and modality.

Key points: (KeyPoint)

- k1. Nominal definition fixes logical essence for distinction.
- k2. Real definition presents inner determinations and possibility.
- k3. The completeness of definition is governed by the four moments.

Claims: (Claim)

- c1. id: kant-life-definition-completeness-c1
  - subject: definition
  - predicate: divides_into
  - object: nominal_and_real_forms
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:91-101`

- c2. id: kant-life-definition-completeness-c2
  - subject: completeness_of_definition
  - predicate: is_governed_by
  - object: quantity_quality_relation_and_modality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `LIFE-DISTILLATION.md:105-115`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-life-definition-origin
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: nominal/real distinction and four-moment completeness specify what successful definition must achieve.
  - sourceClaimIds: [`kant-life-definition-completeness-c1`, `kant-life-definition-completeness-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-definition-origin-c1`, `kant-life-definition-origin-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the judgment-style four-moment protocol explicit inside Life.

### Entry kant-life-definition-discipline - `Life`: testing and preparing definitions

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:119-129`
- summary: Definition is made into a repeatable discipline through the same four acts of testing and preparation.

Key points: (KeyPoint)

- k1. Definitions are tested by truth, distinctness, detail, and determinacy.
- k2. Definitions are prepared by the same four-act discipline.
- k3. Marks must be checked so they do not collapse into hidden presupposition or subordination.

Claims: (Claim)

- c1. id: kant-life-definition-discipline-c1
  - subject: testing_of_definition
  - predicate: follows
  - object: four_act_protocol_of_truth_distinctness_detail_and_determinacy
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:119-129`

- c2. id: kant-life-definition-discipline-c2
  - subject: preparation_of_definition
  - predicate: repeats
  - object: the_same_four_act_protocol_constructively
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:119-129`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-life-definition-completeness
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: testing and preparation operationalize the completeness requirements of definition.
  - sourceClaimIds: [`kant-life-definition-discipline-c1`, `kant-life-definition-discipline-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-definition-completeness-c2`]

Review outcome:

- review_pending
- notes: this entry preserves Life as a protocol of work, not only a taxonomy of definition kinds.

### Entry kant-life-division - `Life`: lawful partition of conceptual sphere

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:133-171`
- summary: Logical division partitions the sphere of a concept through opposed members under a common higher concept, with codivision and subdivision tracking formal variants.

Key points: (KeyPoint)

- k1. Logical division determines what is possible under a concept through opposed lower members.
- k2. Valid division requires exclusion, common genus, and equality to the divided sphere.
- k3. Codivision, subdivision, dichotomy, and polytomy are formal variants of partition.

Claims: (Claim)

- c1. id: kant-life-division-c1
  - subject: logical_division
  - predicate: articulates
  - object: conceptual_sphere_into_opposed_members_under_a_higher_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `LIFE-DISTILLATION.md:133-157`

- c2. id: kant-life-division-c2
  - subject: codivision_subdivision_dichotomy_and_polytomy
  - predicate: are
  - object: formal_variants_of_logical_partition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `LIFE-DISTILLATION.md:161-171`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: this entry fixes the sphere-partition operator family inside Life.
  - sourceClaimIds: [`kant-life-division-c1`, `kant-life-division-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-c2`]

Review outcome:

- review_pending
- notes: division is kept explicitly tied to sphere and opposition, which matches the earlier concept and judgment passes.

### Entry kant-life-method-families - `Life`: families of scientific presentation

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:175-186`
- summary: Method divides into multiple presentation operators that distinguish thoroughness, exposition, inferential chaining, and questioning.

Key points: (KeyPoint)

- k1. Scientific, systematic, and analytic methods order cognition by different aims and directions.
- k2. Syllogistic and tabular methods contrast inferential chaining with completed display.
- k3. Acroamatic and erotematic methods contrast exposition with question-led teaching.

Claims: (Claim)

- c1. id: kant-life-method-families-c1
  - subject: method
  - predicate: divides_into
  - object: scientific_popular_systematic_fragmentary_analytic_synthetic_syllogistic_tabular_and_acroamatic_erotematic_families
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `LIFE-DISTILLATION.md:175-186`

- c2. id: kant-life-method-families-c2
  - subject: method_families
  - predicate: differentiate
  - object: aims_directions_displays_and_teaching_modes_of_science
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - `LIFE-DISTILLATION.md:175-186`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: this entry gathers the chapter's presentation operators without collapsing them into one generic method concept.
  - sourceClaimIds: [`kant-life-method-families-c1`, `kant-life-method-families-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-life-method
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry expands the life-side method family summarized inside the Idea workbook.
  - sourceClaimIds: [`kant-life-method-families-c1`, `kant-life-method-families-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-life-method-c2`]

Review outcome:

- review_pending
- notes: the method families are preserved as a structured operator inventory rather than a glossary.

### Entry kant-life-meditation - `Life`: meditation as reflective accompaniment of method

- sourceFiles:
  - `life.md`
  - `LIFE-DISTILLATION.md`
- lineSpans:
  - `LIFE-DISTILLATION.md:190-200`
- summary: Meditation is the reflective operator that must accompany reading and learning by ordering thoughts according to method.

Key points: (KeyPoint)

- k1. Meditation is reflection or methodical thought.
- k2. It must accompany reading and learning.
- k3. It orders and connects thought according to method.

Claims: (Claim)

- c1. id: kant-life-meditation-c1
  - subject: meditation
  - predicate: is
  - object: reflection_and_methodical_thought
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `LIFE-DISTILLATION.md:190-200`

- c2. id: kant-life-meditation-c2
  - subject: meditation
  - predicate: accompanies
  - object: reading_learning_and_ordering_of_thought
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `LIFE-DISTILLATION.md:190-200`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: meditation closes Life by returning every operator to reflective practice.
  - sourceClaimIds: [`kant-life-meditation-c1`, `kant-life-meditation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-life-c1`, `kant-life-c2`]

Review outcome:

- review_pending
- notes: the chapter ends in active reflection rather than static taxonomy, and the workbook keeps that closure visible.
