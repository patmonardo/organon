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

## Session: 2026-02-22 (deep pass)

Scope:

- file: `being-for-self.txt`
- fixed range: full file
- pass policy: lock species (`a`/`b`/`c`) and derive triadic analytic subentries

Decision:

- Species-first IDs for Part A:
  - Marker: `bfs-a`
  - Species groups from source labels: `a-a`, `a-b`, `a-c`
  - Numbered subentries within each species: `<species>-001..003`
- Numbered subentries are analytic segmentations, not additional source-labeled species.
- Single-claim entries are allowed where the source span is conceptually tight.
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
  - targetEntryId: c-c-002
  - note: Part A opening specifies the sphere that follows Existence's true infinite closure.
  - sourceClaimIds: [`bfs-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`c-c-002-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: a-a-001
  - note: marker hands off to species `a` triad.
  - sourceClaimIds: [`bfs-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`a-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker stabilized for deep pass.

### Entry a-a-001 — `a. Existence and being-for-itself` I: immediate infinity

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 104
- lineEnd: 113

Summary:

Being-for-itself is first determined as infinity sunk into simple being.

Key points: (KeyPoint)

- k1. Infinity appears in immediate form.
- k2. Determinateness appears as negation in general.
- k3. Being-for-itself is stated in concise ontological form.

Claims: (Claim)

- c1. id: a-a-001-c1
  - subject: being_for_itself
  - predicate: is_determined_as
  - object: infinity_sunk_into_simple_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [106-113] being-for-itself as infinity in immediate form of negation.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-a-002
  - note: immediate determination proceeds to the status of existence as moment.
  - sourceClaimIds: [`a-a-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`a-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-a`, moment 1.

### Entry a-a-002 — `a. Existence and being-for-itself` II: retained existence

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 114
- lineEnd: 123

Summary:

Existence is distinguished from being-for-itself yet retained as one of its moments.

Key points: (KeyPoint)

- k1. Being is distinguished from infinite determinateness.
- k2. Existence remains present in being-for-itself.
- k3. Moment-status avoids external dualism.

Claims: (Claim)

- c1. id: a-a-002-c1
  - subject: existence
  - predicate: remains_as
  - object: moment_within_being_for_itself
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [119-123] existence remains a moment because being-for-itself contains being affected by negation.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-a-003
  - note: retained moment is bent back into unity as being-for-one.
  - sourceClaimIds: [`a-a-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`a-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-a`, moment 2.

### Entry a-a-003 — `a. Existence and being-for-itself` III: bent-back otherness

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 123
- lineEnd: 128

Summary:

Determinateness as being-for-other is bent back into the unity of being-for-itself as being-for-one.

Key points: (KeyPoint)

- k1. Otherness is not left external.
- k2. Unity is internalized as being-for-one.
- k3. Species `a` closes into transition to species `b`.

Claims: (Claim)

- c1. id: a-a-003-c1
  - subject: determinateness_as_being_for_other
  - predicate: is_bent_back_into
  - object: being_for_one_within_infinite_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [123-128] being-for-other bent back into infinite unity as being-for-one.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-b-001
  - note: species `a` opens into species `b` (being-for-one).
  - sourceClaimIds: [`a-a-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-a`, moment 3.

### Entry a-b-001 — `b. Being-for-one` I: ideality statement

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 129
- lineEnd: 133

Summary:

Being-for-one first names finite ideality in unity with the infinite.

Key points: (KeyPoint)

- k1. Being-for-one is the expression of ideality.
- k2. Finite/infinite unity is explicit.
- k3. The section begins from conceptual statement before distinctions.

Claims: (Claim)

- c1. id: a-b-001-c1
  - subject: being_for_one
  - predicate: expresses
  - object: finite_in_unity_with_infinite_ideality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [131-133] being-for-one as expression of finite in unity with infinite.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-b-002
  - note: conceptual statement advances to exclusion of an external one.
  - sourceClaimIds: [`a-b-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`a-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-b`, moment 1.

### Entry a-b-002 — `b. Being-for-one` II: no external one

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 139
- lineEnd: 147

Summary:

Being-for-one does not yet posit an external one distinct from itself.

Key points: (KeyPoint)

- k1. There is no fixed external one.
- k2. Both sides remain being-for-one.
- k3. Distinction remains undifferentiated at this stage.

Claims: (Claim)

- c1. id: a-b-002-c1
  - subject: being_for_one
  - predicate: lacks
  - object: external_one_as_distinct_term
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [139-147] no distinct one yet; both sides remain only being-for-one.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-b-003
  - note: undifferentiated status resolves in explicit inseparability thesis.
  - sourceClaimIds: [`a-b-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`a-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-b`, moment 2.

### Entry a-b-003 — `b. Being-for-one` III: inseparable moments

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 158
- lineEnd: 193

Summary:

Being-for-one and being-for-itself are shown as essential, inseparable moments.

Key points: (KeyPoint)

- k1. Self-reference structure unifies both moments.
- k2. The relation is essential, not accidental.
- k3. Species `b` closes by preparing emergence of the one.

Claims: (Claim)

- c1. id: a-b-003-c1
  - subject: being_for_itself_and_being_for_one
  - predicate: are
  - object: essential_inseparable_moments_of_ideality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [158-170] self-reference of being-for-one in being-for-itself.
    - [190-193] explicit inseparability statement.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bfs-a
  - note: species `b` grounds marker-level ideality claim.
  - sourceClaimIds: [`a-b-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bfs-a-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: a-c-001
  - note: inseparable moments resolve into the one.
  - sourceClaimIds: [`a-b-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-b`, moment 3.

### Entry a-c-001 — `c. The one` I: immediate one

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 194
- lineEnd: 205

Summary:

The one appears as being-for-itself in immediate existent form.

Key points: (KeyPoint)

- k1. Unity of moments collapses into immediacy.
- k2. Being-for-itself is posited as existent-for-itself.
- k3. The one is the threshold to plurality.

Claims: (Claim)

- c1. id: a-c-001-c1
  - subject: the_one
  - predicate: is_determined_as
  - object: immediate_existent_for_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [196-205] being-for-itself as existent-for-itself in immediate unity.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-c-002
  - note: immediacy sharpens into abstract self-limit.
  - sourceClaimIds: [`a-c-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`a-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-c`, moment 1.

### Entry a-c-002 — `c. The one` II: abstract limit

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 205
- lineEnd: 214

Summary:

The one is explicitly posited as the totally abstract limit of itself.

Key points: (KeyPoint)

- k1. Inner meaning vanishes in immediacy.
- k2. The one is a self-limit.
- k3. Abstraction sets up contradiction-logic.

Claims: (Claim)

- c1. id: a-c-002-c1
  - subject: the_one
  - predicate: is
  - object: totally_abstract_limit_of_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [205-207] explicit formulation of one as abstract self-limit.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: a-c-003
  - note: abstract limit opens the contradiction of moments.
  - sourceClaimIds: [`a-c-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`a-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-c`, moment 2.

### Entry a-c-003 — `c. The one` III: contradictory moments

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 215
- lineEnd: 230

Summary:

The one's moments appear externally separated by immediacy yet remain inseparable, generating contradiction.

Key points: (KeyPoint)

- k1. Moments occur one outside another.
- k2. Each moment is posited as self-standing.
- k3. Moments are nevertheless inseparable, producing contradiction.

Claims: (Claim)

- c1. id: a-c-003-c1
  - subject: moments_of_the_one
  - predicate: are_posited_as
  - object: inseparable_yet_contradictory_external_determinacies
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [222-230] moments occur apart in immediacy yet remain inseparable, yielding contradiction.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bfs-a
  - note: species `c` grounds marker-level culmination in the one.
  - sourceClaimIds: [`a-c-001-c1`, `a-c-002-c1`, `a-c-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`bfs-a-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-b
  - note: abstract one opens Part B development of one-many plurality.
  - sourceClaimIds: [`a-c-002-c1`, `a-c-003-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`bfs-b-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `a-c`, moment 3 and handoff to Part B.
