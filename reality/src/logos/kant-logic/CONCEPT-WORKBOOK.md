# Concept Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file only fixes what concept is and how conceptual order is built.
- In one sentence: a concept has an inside and an outside, and concepts form an ordered ladder.
- Use the distillation as the fast commentary path out of this workbook.
- Return to the source text when checking whether the workbook remains complete and correct.

## Quick orientation

- First question: what is a concept here?
  Answer: a universal form of thought with content and extension.
- Second question: how are concepts ordered?
  Answer: by higher and lower position within logical subordination.
- Third question: how does the ladder move?
  Answer: by abstraction upward and determination downward.
- Fourth question: how should this file be read against the distillation?
  Answer: read the workbook for logical commitments, the distillation for quick commentary, and the source text for verification.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `concept.md` and `CONCEPT-DISTILLATION.md`
- Distillation role: commentary on workbooked logic and structural moves.
- Upstream source authority: `concept.md`
- This workbook covers only the concept module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Do not treat conceptual hierarchy as ontology.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (operator pass)

Scope:

- files:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- pass policy: 1 marker entry + expanded analytic entries by conceptual operator

Decision:

- Keep this workbook on the General Logic side.
- Separate the concept operators so content/extension, subordination, rule transfer, and ladder-generation are not collapsed.
- Preserve grounding language where lower concepts are under higher ones rather than inside them.

### Entry kant-concept — Marker `Concept`

- sourceFiles:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- lineSpans:
  - `CONCEPT-DISTILLATION.md:7-137`
- summary: Concept is a two-sided form of thought with content and extension, ordered by subordination and generated through logical ascent and descent.

Key points: (KeyPoint)

- k1. Every concept has content and extension.
- k2. Higher and lower concepts express logical subordination.
- k3. Concept-logic works through grounding, rule transfer, and reversible generation.

Claims: (Claim)

- c1. id: kant-concept-c1
  - subject: concept
  - predicate: is
  - object: universal_form_of_thought
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `CONCEPT-DISTILLATION.md:7-17`

- c2. id: kant-concept-c2
  - subject: concept
  - predicate: is_ordered_by
  - object: content_extension_subordination_and_generation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `CONCEPT-DISTILLATION.md:7-124`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-subj-triad
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: concept is the first explicit moment of subjectivity.
  - sourceClaimIds: [`kant-concept-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`kant-subj-triad-c1`]

Review outcome:

- review_pending
- notes: if reviewing quickly, this is the entry to read first.

### Entry kant-concept-content-extension — `Concept`: content, extension, and sphere

- sourceFiles:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- lineSpans:
  - `CONCEPT-DISTILLATION.md:7-43`
- summary: Concept begins as a two-sided structure of inner marks and outer sphere, governed by an inverse law between content and extension.

Key points: (KeyPoint)

- k1. Content is what the concept contains in itself as marks.
- k2. Extension is what stands under the concept as its sphere.
- k3. Content and extension stand in an inverse law.

Claims: (Claim)

- c1. id: kant-concept-content-extension-c1
  - subject: concept
  - predicate: has_two_sides
  - object: content_and_extension
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `CONCEPT-DISTILLATION.md:7-17`

- c2. id: kant-concept-content-extension-c2
  - subject: content_and_extension_of_concept
  - predicate: stand_in
  - object: inverse_relation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `CONCEPT-DISTILLATION.md:21-31`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-concept
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: content and extension provide the first operator structure of concept.
  - sourceClaimIds: [`kant-concept-content-extension-c1`, `kant-concept-content-extension-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-concept-c2`]

Review outcome:

- review_pending
- notes: this entry restores the two-sided machinery and spherical protocol of concept.

### Entry kant-concept-subordination — `Concept`: higher, lower, genus, and species

- sourceFiles:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- lineSpans:
  - `CONCEPT-DISTILLATION.md:46-97`
- summary: Conceptual hierarchy is a relation of remote marks, broader and narrower spheres, and positional genus-species ordering.

Key points: (KeyPoint)

- k1. Higher concepts are remote marks with lower concepts under them.
- k2. Genus and species are positional differences within subordination.
- k3. Lower concepts are under higher ones because the higher contains their ground of cognition.

Claims: (Claim)

- c1. id: kant-concept-subordination-c1
  - subject: higher_and_lower_concepts
  - predicate: express
  - object: remote_mark_subordination
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `CONCEPT-DISTILLATION.md:46-83`

- c2. id: kant-concept-subordination-c2
  - subject: lower_concept
  - predicate: is_contained_under
  - object: higher_concept_as_ground_of_cognition
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPT-DISTILLATION.md:105-115`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-concept
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: this entry fixes conceptual hierarchy as grounding relation rather than flat inclusion.
  - sourceClaimIds: [`kant-concept-subordination-c1`, `kant-concept-subordination-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-concept-c2`]

Review outcome:

- review_pending
- notes: remote mark and ground-of-cognition language are now preserved explicitly.

### Entry kant-concept-rules — `Concept`: universal transmission rules

- sourceFiles:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- lineSpans:
  - `CONCEPT-DISTILLATION.md:100-110`
- summary: Conceptual subordination carries universal downward and upward rule transfer.

Key points: (KeyPoint)

- k1. What belongs to a higher concept also belongs to its lower concepts.
- k2. What belongs to all lower concepts also belongs to the higher concept.
- k3. Conceptual order is a rule-transfer machine.

Claims: (Claim)

- c1. id: kant-concept-rules-c1
  - subject: conceptual_subordination
  - predicate: transmits
  - object: predicates_and_contradictions_downward
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPT-DISTILLATION.md:100-110`

- c2. id: kant-concept-rules-c2
  - subject: conceptual_subordination
  - predicate: transmits
  - object: common_predicates_upward
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPT-DISTILLATION.md:100-110`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-concept-subordination
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: the universal rules operationalize the hierarchy fixed by subordination.
  - sourceClaimIds: [`kant-concept-rules-c1`, `kant-concept-rules-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-concept-subordination-c1`, `kant-concept-subordination-c2`]

Review outcome:

- review_pending
- notes: this entry isolates the transmissibility that makes concept into stable machinery.

### Entry kant-concept-generation — `Concept`: abstraction and determination

- sourceFiles:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- lineSpans:
  - `CONCEPT-DISTILLATION.md:113-124`
- summary: Abstraction and determination generate the conceptual ladder through reversible ascent and descent.

Key points: (KeyPoint)

- k1. Abstraction produces higher concepts by stripping determinations away.
- k2. Determination produces lower concepts by adding determinations.
- k3. The ladder reaches toward the most abstract and the thoroughly determinate.

Claims: (Claim)

- c1. id: kant-concept-generation-c1
  - subject: abstraction_and_determination
  - predicate: generate
  - object: conceptual_ladder
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `CONCEPT-DISTILLATION.md:113-124`

- c2. id: kant-concept-generation-c2
  - subject: conceptual_ladder
  - predicate: is_structured_by
  - object: ascent_and_descent_of_determination
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPT-DISTILLATION.md:113-124`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-concept
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: abstraction and determination generate the hierarchy instead of leaving it static.
  - sourceClaimIds: [`kant-concept-generation-c1`, `kant-concept-generation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-concept-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the ladder dynamic and reversible.

### Entry kant-concept-use — `Concept`: abstract and concrete use

- sourceFiles:
  - `concept.md`
  - `CONCEPT-DISTILLATION.md`
- lineSpans:
  - `CONCEPT-DISTILLATION.md:127-137`
- summary: Abstract and concrete are relational modes of using concepts within the ladder.

Key points: (KeyPoint)

- k1. Every concept can be used in abstracto or in concreto.
- k2. Lower concepts are abstract relative to higher ones.
- k3. Higher concepts are concrete relative to lower ones.

Claims: (Claim)

- c1. id: kant-concept-use-c1
  - subject: abstract_and_concrete
  - predicate: are
  - object: relational_use_modes_of_concept
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `CONCEPT-DISTILLATION.md:127-137`

- c2. id: kant-concept-use-c2
  - subject: concept_use
  - predicate: shifts_with
  - object: position_in_subordination
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `CONCEPT-DISTILLATION.md:127-137`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-concept-generation
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: abstract and concrete name how concepts are used at different points in the generated ladder.
  - sourceClaimIds: [`kant-concept-use-c1`, `kant-concept-use-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-concept-generation-c2`]

Review outcome:

- review_pending
- notes: concept-use is now preserved as relational function rather than a closing aside.
