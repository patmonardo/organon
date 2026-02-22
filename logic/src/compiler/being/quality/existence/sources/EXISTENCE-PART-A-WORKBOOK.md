# Existence Part A Workbook

Part: `A. EXISTENCE AS SUCH`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/existence/sources/WORKBOOK-CONTRACT-V1.md`
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

## Session: 2026-02-22 (seed pass)

Scope:

- file: `existence.txt`
- fixed range: full file
- pass policy: lock 1 marker + up to 3 numbered entries only

Decision:

- Two-level IDs:
  - Level 1 marker: `exi-a`
  - Level 2 numbered entries: `exi-a-<nnn>`
- Keep first pass at human-comprehension scale; no micro-fragmentation.
- Preserve explicit transition target toward Part B marker `exi-b`.

### Entry exi-a — Marker `A`: Existence as such (`a`/`b`/`c` segmentation)

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 4
- lineEnd: 220

Summary:

Existence as such unfolds in three textual subtopics: existence in general, quality, and something.

Key points: (KeyPoint)

- k1. Subtopic `a` develops existence as immediate unity from becoming.
- k2. Subtopic `b` develops quality as determinate unity of being and non-being.
- k3. Subtopic `c` develops something as mediated self-equality through sublation of distinction.

Claims: (Claim)

- c1. id: exi-a-c1
  - subject: existence_as_such
  - predicate: has_subtopic
  - object: existence_in_general
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [15-17] explicit subtopic heading and opening statement.

- c2. id: exi-a-c2
  - subject: existence_as_such
  - predicate: has_subtopic
  - object: quality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [102-104] explicit subtopic heading and opening movement.

- c3. id: exi-a-c3
  - subject: existence_as_such
  - predicate: has_subtopic
  - object: something
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [157-160] explicit subtopic heading and opening distinction.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-a-001
  - note: marker to subtopic `a` entry.
  - sourceClaimIds: [`exi-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-a-002
  - note: marker to subtopic `b` entry.
  - sourceClaimIds: [`exi-a-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`exi-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: exi-a-003
  - note: marker to subtopic `c` entry.
  - sourceClaimIds: [`exi-a-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: segmentation now explicitly obeys text's own `a/b/c` structure.

### Entry exi-a-001 — `a. Existence in general`: immediate from becoming and methodological restraint

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 15
- lineEnd: 100

Summary:

Existence in general is immediate unity from becoming while the exposition explicitly distinguishes concept-posited content from external reflection.

Key points: (KeyPoint)

- k1. Existence proceeds from becoming as oneness of being and nothing.
- k2. Its mediation has sublated itself, so existence appears as immediate first.
- k3. Conceptual content is restricted to what is posited in the concept, not anticipatory reflection.

Claims: (Claim)

- c1. id: exi-a-001-c1
  - subject: existence
  - predicate: proceeds_from
  - object: becoming_as_unity_of_being_and_nothing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [17-24] existence proceeds from becoming and appears as immediate first.
    - [35-43] being-with-non-being in simple unity.

- c2. id: exi-a-001-c2
  - subject: existence
  - predicate: has_character
  - object: immediate_form_with_sublated_mediation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [19-24] immediacy form with mediation lying behind as sublated.

- c3. id: exi-a-001-c3
  - subject: conceptual_elaboration_rule
  - predicate: distinguishes
  - object: posited_determinacy_from_external_reflection
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [52-64] only posited determinacy belongs to conceptual content.
    - [65-92] anticipatory reflection must be distinguished from the fact's own advance.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bei-c-b-002
  - note: explicates the becoming->existence transition announced in Being Part C.
  - sourceClaimIds: [`exi-a-001-c1`, `exi-a-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bei-c-b-002-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-a-002
  - note: moves from existence in general into the explicit quality subtopic.
  - sourceClaimIds: [`exi-a-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: bounded to one coherent subtopic span.

### Entry exi-a-002 — `b. Quality`: reality/negation and immediate unity without posited differentiation

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 102
- lineEnd: 155

Summary:

Quality is existent determinateness within immediate unity of being and non-being, differentiated as reality and negation while still lacking fully posited separation.

Key points: (KeyPoint)

- k1. Existence is determined as unity of being and non-being.
- k2. Quality is isolated existent determinateness and names immediate simplicity.
- k3. Quality differentiates as reality and negation (limit/restriction) without becoming abstract nothing.

Claims: (Claim)

- c1. id: exi-a-002-c1
  - subject: existence
  - predicate: is_determined_as
  - object: immediate_unity_of_being_and_non_being
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [104-119] immediacy of unity and non-posited differentiation between being/determinateness.

- c2. id: exi-a-002-c2
  - subject: quality
  - predicate: is
  - object: existent_determinateness_as_immediate_simplicity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [121-129] determinateness isolated as existent is quality, simple and immediate.

- c3. id: exi-a-002-c3
  - subject: quality_differentiation
  - predicate: unfolds_as
  - object: reality_and_negation_as_limit_restriction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [135-143] quality posited in determination of nothing, yielding reality and negation.
    - [145-155] negation is existence-quality determined with non-being.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-a-003
  - note: differentiated quality is sublated into the determination of something.
  - sourceClaimIds: [`exi-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: keeps dense quality movement intact without over-splitting.

### Entry exi-a-003 — `c. Something`: sublated distinction and first negation-of-negation

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 157
- lineEnd: 220

Summary:

Something arises as existence made self-equal through sublation of reality/negation distinction, determined as first negation of negation and as beginning of concrete self-reference.

Key points: (KeyPoint)

- k1. Reality and negation are both in existence and are sublated as separate distinctions.
- k2. The result is mediated self-equal existence: being-in-itself as existent something.
- k3. Something is first negation of negation and initiates concrete self-reference beyond abstract determinations.

Claims: (Claim)

- c1. id: exi-a-003-c1
  - subject: reality_and_negation
  - predicate: are
  - object: moments_of_existence_sublated_in_their_distinction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [159-171] both reality and negation are existence and unseparated from it.
    - [173-180] distinction is not omitted but sublated.

- c2. id: exi-a-003-c2
  - subject: existence_after_sublation
  - predicate: is_determined_as
  - object: mediated_self_equal_being_in_itself_or_something
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [181-187] self-equal mediated simplicity; existence is being-in-itself, existent something.

- c3. id: exi-a-003-c3
  - subject: something
  - predicate: is_determined_as
  - object: first_negation_of_negation_and_beginning_of_subjective_intensity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [189-194] something as first negation of negation and simple self-reference.
    - [209-217] beginning character: not yet fully concrete but bearing negative unity with itself.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-b
  - note: something opens the Part B development toward something/other, constitution, and finitude.
  - sourceClaimIds: [`exi-a-003-c2`, `exi-a-003-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes requested three-subtopic extraction at bounded first-pass density.
