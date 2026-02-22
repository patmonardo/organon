# Being-for-self Part A Workbook

Part: `A. BEING-FOR-ITSELF`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being-for-self/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `being-for-self.txt` as authority.
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

- file: `being-for-self.txt`
- fixed range: full file
- pass policy: lock 1 marker + up to 3 numbered entries only

Decision:

- Two-level IDs:
  - Level 1 marker: `bfs-a`
  - Level 2 numbered entries: `bfs-a-<nnn>`
- Keep first pass at conceptual readability level; no micro-fragmentation.
- Preserve transition target toward Part B marker `bfs-b`.

### Entry bfs-a — Marker `A`: Being-for-itself as such

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 49
- lineEnd: 230

Summary:

Part A introduces being-for-itself as infinite self-reference in which otherness is sublated, culminating in the one as existent-for-itself.

Key points: (KeyPoint)

- k1. Being-for-itself completes qualitative being by positing negation as self-reference.
- k2. Being-for-one and being-for-itself are inseparable moments of ideality.
- k3. The one emerges as abstract existent-for-itself and transition node to plurality.

Claims: (Claim)

- c1. id: bfs-a-c1
  - subject: being_for_itself
  - predicate: is_determined_as
  - object: infinite_self_reference_of_qualitative_negation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [22-33] distinction of being/negation equalized in posited negation of negation.
    - [49-64] being-for-itself as infinite turning back into itself.

- c2. id: bfs-a-c2
  - subject: ideality_structure
  - predicate: includes
  - object: inseparable_being_for_itself_and_being_for_one
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [158-170] being-for-one as self-reference of being-for-itself.
    - [190-192] explicit inseparability of being-for-itself and being-for-one.

- c3. id: bfs-a-c3
  - subject: being_for_itself
  - predicate: culminates_in
  - object: the_one_as_abstract_limit_of_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [194-207] transition to the one as existent-for-itself and abstract self-limit.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: exi-c-003
  - note: Part A opening specifies the sphere that follows Existence's true infinite closure.
  - sourceClaimIds: [`bfs-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-c-003-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-a-001
  - note: marker hands off to labeled subtopic `a. Existence and being-for-itself`.
  - sourceClaimIds: [`bfs-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`bfs-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first bounded seed entry for Part A.

### Entry bfs-a-001 — `a. Existence and being-for-itself`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 104
- lineEnd: 128

Summary:

Being-for-itself is defined as infinity sunk into simple being, where existence is retained as a moment bent back into the self-unity of being-for-itself.

Key points: (KeyPoint)

- k1. Being-for-itself is infinity in immediate form, as existence affected by negation.
- k2. Existence remains a moment within being-for-itself.
- k3. Determinateness as otherness is bent back into unity as being-for-one.

Claims: (Claim)

- c1. id: bfs-a-001-c1
  - subject: being_for_itself
  - predicate: is_determined_as
  - object: infinity_sunk_into_simple_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [106-113] being-for-itself as infinity in immediate form of negation.

- c2. id: bfs-a-001-c2
  - subject: existence
  - predicate: remains_as
  - object: moment_within_being_for_itself
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [119-123] existence remains moment because being-for-itself contains being affected by negation.

- c3. id: bfs-a-001-c3
  - subject: determinateness_as_being_for_other
  - predicate: is_bent_back_into
  - object: being_for_one_within_infinite_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [123-128] being-for-other is bent back into infinite unity as being-for-one.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: bfs-a-002
  - note: subtopic `a` opens directly into `b. Being-for-one`.
  - sourceClaimIds: [`bfs-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: labeled subtopic `a` generated at bounded density.

### Entry bfs-a-002 — `b. Being-for-one`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 129
- lineEnd: 193

Summary:

Being-for-one expresses the ideality of the finite in unity with the infinite, and is shown to be inseparable from being-for-itself rather than a relation to an external one.

Key points: (KeyPoint)

- k1. Being-for-one articulates finite ideality in unity with the infinite.
- k2. There is no external one yet; the one for which something is, is only itself.
- k3. Being-for-one and being-for-itself are not separate determinacies but inseparable moments.

Claims: (Claim)

- c1. id: bfs-a-002-c1
  - subject: being_for_one
  - predicate: expresses
  - object: finite_in_unity_with_infinite_ideality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [131-133] being-for-one as expression of finite in unity with infinite.

- c2. id: bfs-a-002-c2
  - subject: being_for_one
  - predicate: lacks
  - object: external_one_as_distinct_term
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [139-147] no distinct one yet; both sides remain only being-for-one.

- c3. id: bfs-a-002-c3
  - subject: being_for_itself_and_being_for_one
  - predicate: are
  - object: essential_inseparable_moments_of_ideality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [158-170] self-reference structure of being-for-one in being-for-itself.
    - [190-193] explicit inseparability statement.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bfs-a
  - note: subtopic `b` grounds marker-level ideality claim.
  - sourceClaimIds: [`bfs-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-a-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-a-003
  - note: inseparable moments resolve into the immediate abstraction of the one.
  - sourceClaimIds: [`bfs-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: labeled subtopic `b` generated at bounded density.

### Entry bfs-a-003 — `c. The one`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 194
- lineEnd: 230

Summary:

The one is the immediate existent-for-itself in which moments collapse into abstraction, while its determinations appear externally separated and mutually contradictory.

Key points: (KeyPoint)

- k1. The one is being-for-itself as immediate existent self-reference.
- k2. It is the abstract limit of itself grounded in posited negation.
- k3. The one's moments appear both inseparable and externally opposed, generating the transition problem.

Claims: (Claim)

- c1. id: bfs-a-003-c1
  - subject: the_one
  - predicate: is_determined_as
  - object: immediate_existent_for_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [196-205] being-for-itself as existent-for-itself in immediate unity.

- c2. id: bfs-a-003-c2
  - subject: the_one
  - predicate: is
  - object: totally_abstract_limit_of_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [205-207] explicit formulation of the one as abstract self-limit.

- c3. id: bfs-a-003-c3
  - subject: moments_of_the_one
  - predicate: are_posited_as
  - object: inseparable_yet_contradictory_external_determinacies
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [222-230] moments occur apart in immediacy yet remain inseparable, yielding contradiction.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bfs-a
  - note: subtopic `c` grounds marker-level culmination in the one.
  - sourceClaimIds: [`bfs-a-003-c1`, `bfs-a-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bfs-a-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-b
  - note: abstract one opens Part B development of one-many plurality.
  - sourceClaimIds: [`bfs-a-003-c2`, `bfs-a-003-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`bfs-b-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: labeled subtopic `c` generated; Part A now follows lettered subtopic rule.
