# Existence Part B Workbook

Part: `B. FINITUDE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/existence/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `something-and-other.txt`, `constitution.txt`, and `finitude.txt` as authority.
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

- files: `something-and-other.txt`, `constitution.txt`, `finitude.txt`
- fixed range: full files
- pass policy: lock 1 marker + up to 3 numbered entries only

Decision:

- Seven-source artifact is normalized here into one logical Part B workbook.
- Two-level IDs:
  - Level 1 marker: `exi-b`
  - Level 2 numbered entries: `exi-b-<nnn>`
- Keep transitions explicit (`something/other -> constitution/limit -> finitude`).

### Entry exi-b — Marker `B`: Finitude across three bounded source segments

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/something-and-other.txt`
- lineStart: 2
- lineEnd: 329

Summary:

Part B is developed through three bounded movements: something/other, determination-constitution-limit, and finitude proper.

Key points: (KeyPoint)

- k1. The first movement establishes the dialectic of something and other.
- k2. The second movement internalizes otherness as determination/constitution/limit.
- k3. The third movement develops finite contradiction into transition toward the infinite.

Claims: (Claim)

- c1. id: exi-b-c1
  - subject: finitude_part_b
  - predicate: begins_with
  - object: something_and_other
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-9] explicit `(a) Something and other` opening movement.

- c2. id: exi-b-c2
  - subject: finitude_part_b
  - predicate: develops_through
  - object: determination_constitution_limit
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [11-16] explicit `(b)` movement toward constitution and limit.

- c3. id: exi-b-c3
  - subject: finitude_part_b
  - predicate: culminates_in
  - object: finite_as_immanent_determination
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [18-19] explicit `(c)` closure: something thus is finite.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-b-001
  - note: marker to first bounded text segment (`something-and-other.txt`).
  - sourceClaimIds: [`exi-b-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-b-002
  - note: marker to second bounded text segment (`constitution.txt`).
  - sourceClaimIds: [`exi-b-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`exi-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: exi-b-003
  - note: marker to third bounded text segment (`finitude.txt`).
  - sourceClaimIds: [`exi-b-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: aligns exactly with requested three-text analysis structure.

### Entry exi-b-001 — `something-and-other.txt`: from indifferent otherness to being-in-itself/being-for-other identity

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/something-and-other.txt`
- lineStart: 32
- lineEnd: 329

Summary:

The first segment develops something and other from external indifference into internally connected moments of being-in-itself and being-for-other, with limit emerging as immanent determinateness.

Key points: (KeyPoint)

- k1. Something and other are mutually reversible and initially indifferent.
- k2. Being-in-itself and being-for-other are moments of one unity, not isolated terms.
- k3. Limit emerges as immanent negation that both joins and separates somethings.

Claims: (Claim)

- c1. id: exi-b-001-c1
  - subject: something_and_other
  - predicate: are_determined_as
  - object: reciprocal_and_indifferent_others
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [34-46] each is equally other; ordering is indifferent.
    - [81-89] sameness/difference initially falls within external reflection.

- c2. id: exi-b-001-c2
  - subject: something
  - predicate: contains
  - object: unity_of_being_in_itself_and_being_for_other
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [147-160] being-for-other and being-in-itself posited as moments of one unity.
    - [199-216] identity of both moments in one and the same something.

- c3. id: exi-b-001-c3
  - subject: limit
  - predicate: is_determined_as
  - object: immanent_negation_joining_and_separating_somethings
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [219-227] determinateness as limit.
    - [258-285] limit as mediation in which each something both is and is not.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: exi-a-003
  - note: specifies the `something` reached at Part A closure by unfolding its relation to otherness.
  - sourceClaimIds: [`exi-b-001-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`exi-a-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-b-002
  - note: limit/otherness dialectic is further developed as determination-constitution-limit.
  - sourceClaimIds: [`exi-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: dense segment compressed into three anchor claims without micro-fragmentation.

### Entry exi-b-002 — `constitution.txt`: determination/constitution reciprocity and immanent limit

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/constitution.txt`
- lineStart: 2
- lineEnd: 410

Summary:

The second segment shows determination and constitution as distinct yet reciprocal moments whose transition into one another yields immanent limit as the negative unity of something and other.

Key points: (KeyPoint)

- k1. Determination is affirmative in-itselfness present in the something.
- k2. Constitution is determinateness as externally conditioned being-for-other.
- k3. Their reciprocal passage produces immanent negation and limit.

Claims: (Claim)

- c1. id: exi-b-002-c1
  - subject: determination
  - predicate: is_determined_as
  - object: affirmative_in_itselfness_present_in_existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [27-53] determination as affirmative determinateness and in-itself present in the something.

- c2. id: exi-b-002-c2
  - subject: constitution
  - predicate: is_determined_as
  - object: externally_conditioned_determinateness_of_something
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [80-101] constitution as determinateness distinguished from in-itself and given over to externality.
    - [103-108] alteration belongs to constitution-side while something preserves itself.

- c3. id: exi-b-002-c3
  - subject: determination_and_constitution
  - predicate: pass_over_into
  - object: immanent_limit_as_negative_unity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [121-123] reciprocal passage of determination and constitution.
    - [163-179] transition yields two somethings with immanent negation.
    - [219-227] determinateness as limit.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-b-001
  - note: provides structural grounding for why something/other relation is internally mediated.
  - sourceClaimIds: [`exi-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-b-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-b-003
  - note: immanent limit develops into explicit finitude contradiction and transition.
  - sourceClaimIds: [`exi-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: keeps long segment readable by retaining only core movement claims.

### Entry exi-b-003 — `finitude.txt`: finite contradiction, ought/restriction, and transition to infinite

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/finitude.txt`
- lineStart: 2
- lineEnd: 310

Summary:

The third segment determines finitude as immanent non-being of the finite, develops restriction and ought as inseparable moments, and culminates in the finite's self-sublation into the infinite.

Key points: (KeyPoint)

- k1. Finitude means non-being belongs to the finite's own nature.
- k2. Restriction and ought are mutually implicated moments of finite determination.
- k3. Finite contradiction drives self-sublation and transition into the infinite.

Claims: (Claim)

- c1. id: exi-b-003-c1
  - subject: finite
  - predicate: is_determined_as
  - object: being_whose_nature_includes_non_being_and_perishing
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [15-22] non-being constitutes finite nature.
    - [27-37] finite being has end/perishing as immanent determination.

- c2. id: exi-b-003-c2
  - subject: finite_structure
  - predicate: includes
  - object: restriction_and_ought_as_inseparable_moments
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [179-189] restriction yields ought.
    - [211-217] ought and restriction both moments of finite.
    - [260-266] both are indivisible in finite determination.

- c3. id: exi-b-003-c3
  - subject: finite_contradiction
  - predicate: transitions_to
  - object: infinite_as_negation_of_negation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [277-283] finite as contradiction sublates itself and ceases.
    - [306-310] identity with itself as negation of negation is the infinite.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-b
  - note: third movement confirms Part B culmination in explicit finitude determination.
  - sourceClaimIds: [`exi-b-003-c1`, `exi-b-003-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`exi-b-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-c
  - note: finite's self-sublation opens Part C (`Infinity`).
  - sourceClaimIds: [`exi-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: completes the requested three-text Part B extraction at bounded density.
