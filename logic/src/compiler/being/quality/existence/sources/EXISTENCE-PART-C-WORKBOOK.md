# Existence Part C Workbook

Part: `C. INFINITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/existence/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `infinity.txt`, `alternating-infinity.txt`, and `affirmative-infinity.txt` as authority.
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

- files: `infinity.txt`, `alternating-infinity.txt`, `affirmative-infinity.txt`
- fixed range: full files
- pass policy: lock 1 marker + up to 3 numbered entries only

Decision:

- Seven-source artifact is normalized here into one logical Part C workbook.
- Two-level IDs:
  - Level 1 marker: `exi-c`
  - Level 2 numbered entries: `exi-c-<nnn>`
- Keep first-pass extraction at conceptual readability level; postpone fine-grained inferential overlays.

### Entry exi-c — Marker `C`: Infinity in three labeled movements (`a` / `b` / `c`)

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/infinity.txt`
- lineStart: 2
- lineEnd: 92

Summary:

Part C presents infinity through three labeled movements: infinite in general, alternating (bad) infinite, and affirmative (true) infinite.

Key points: (KeyPoint)

- k1. `a` introduces infinite as negation of finite and as affirmative truth of finite self-transcendence.
- k2. `b` exhibits the alternating determination that yields bad infinite progress.
- k3. `c` resolves this alternation into true, affirmative infinity.

Claims: (Claim)

- c1. id: exi-c-c1
  - subject: infinity_part_c
  - predicate: has_subtopic
  - object: infinite_in_general
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [37-38] explicit `(a)` designation in Infinity schema.

- c2. id: exi-c-c2
  - subject: infinity_part_c
  - predicate: has_subtopic
  - object: alternating_determination_of_finite_and_infinite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [39-41] explicit `(b)` designation in Infinity schema.

- c3. id: exi-c-c3
  - subject: infinity_part_c
  - predicate: has_subtopic
  - object: true_affirmative_infinite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [42-44] explicit `(c)` designation as true infinite.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-c-001
  - note: marker to `a. The infinite in general`.
  - sourceClaimIds: [`exi-c-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-c-002
  - note: marker to `b. Alternating determination of finite and infinite`.
  - sourceClaimIds: [`exi-c-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`exi-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: exi-c-003
  - note: marker to `c. Affirmative infinity`.
  - sourceClaimIds: [`exi-c-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subtopic segmentation strictly follows labeled Infinity structure.

### Entry exi-c-001 — `a. The infinite in general`: finite self-transcendence as affirmative infinity

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/infinity.txt`
- lineStart: 46
- lineEnd: 92

Summary:

The infinite in general is introduced as negation of negation and true being, where finite by its own nature transcends restriction and becomes infinite.

Key points: (KeyPoint)

- k1. Infinite is negation of negation and affirmative being.
- k2. Finite transcends itself by its own nature, not by external force.
- k3. The finite vanishes into the infinite as its true vocation.

Claims: (Claim)

- c1. id: exi-c-001-c1
  - subject: infinite_in_general
  - predicate: is_determined_as
  - object: negation_of_negation_and_true_being
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [48-53] infinite as negation of negation and true being.

- c2. id: exi-c-001-c2
  - subject: finite
  - predicate: by_nature
  - object: transcends_restriction_into_infinite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [62-65] finite negates its negation and becomes infinite.
    - [77-84] transition is finite's own nature, not alien imposition.

- c3. id: exi-c-001-c3
  - subject: infinite
  - predicate: is
  - object: affirmative_determination_and_vocation_of_finite
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [89-92] finite vanishes into infinite; what is, is only infinite.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: exi-b-003
  - note: explicates the finite->infinite transition concluded at Part B.
  - sourceClaimIds: [`exi-c-001-c2`, `exi-c-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`exi-b-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-c-002
  - note: immediate concept of infinite passes into alternating determination.
  - sourceClaimIds: [`exi-c-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: bounded to labeled subtopic `a`.

### Entry exi-c-002 — `b. Alternating determination`: bad infinite and endless oscillation

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/alternating-infinity.txt`
- lineStart: 2
- lineEnd: 288

Summary:

The alternating determination shows infinite and finite as externally opposed terms that regenerate each other, yielding the bad infinite as endless progress without reconciliation.

Key points: (KeyPoint)

- k1. Infinite opposed to finite falls back into a limited something.
- k2. This yields bad infinite as external beyond opposed to finite this-side.
- k3. Infinite progress is repetitive alternation whose hidden unity is not yet reflected.

Claims: (Claim)

- c1. id: exi-c-002-c1
  - subject: infinite_opposed_to_finite
  - predicate: becomes
  - object: finite_infinite_with_limit
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-18] infinite as negation of finite falls into category of limited something.
    - [93-97] infinite over against finite appears itself finitized.

- c2. id: exi-c-002-c2
  - subject: bad_infinite
  - predicate: is_determined_as
  - object: unresolved_external_opposition_to_finite
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [73-83] explicit designation of bad infinite and unresolved contradiction.
    - [124-137] each side finite by immanent limit.

- c3. id: exi-c-002-c3
  - subject: infinite_progress
  - predicate: has_form
  - object: repetitive_alternation_with_unreflected_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [208-227] progress as alternating determination and endless `and so on`.
    - [260-267] unity is present as impulse but not reflected.
    - [269-288] empty unrest between unattainable beyond and regenerating finite.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: exi-c-003
  - note: bad infinite's contradiction requires affirmative resolution.
  - sourceClaimIds: [`exi-c-002-c2`, `exi-c-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`exi-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subtopic `b` retained as one bounded high-density entry.

### Entry exi-c-003 — `c. Affirmative infinity`: true infinite as mediated self-return

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/affirmative-infinity.txt`
- lineStart: 2
- lineEnd: 427

Summary:

Affirmative infinity resolves finite/infinite opposition by showing each contains the other, so true infinite is mediated self-return (negation of negation), figurable as circle rather than endless line.

Key points: (KeyPoint)

- k1. Finite and infinite are inseparable moments each containing the other.
- k2. True infinite is negation of negation, not a beyond separated from finite.
- k3. The true infinite is concrete presence/self-return and grounds ideality over finite realism.

Claims: (Claim)

- c1. id: exi-c-003-c1
  - subject: unity_of_finite_and_infinite
  - predicate: is_determined_as
  - object: true_infinite_beyond_qualitative_separation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [20-31] neither finite nor infinite graspable without the other.
    - [80-90] scandalous unity where each is unity of both moments.
    - [147-156] unity is immanent, not external combination.

- c2. id: exi-c-003-c2
  - subject: true_infinite
  - predicate: is
  - object: mediated_self_reference_as_negation_of_negation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [185-189] shared negation of negation as affirmative self-reference.
    - [327-333] true infinite as process of self-lowering and self-elevation through mediation.
    - [349-357] true infinite as being-there, not remote beyond.

- c3. id: exi-c-003-c3
  - subject: image_of_true_infinite
  - predicate: contrasts
  - object: circle_self_return_vs_linear_bad_infinite
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [372-382] bad infinite as line; true infinite as circle reaching itself.
    - [384-390] true infinite as higher reality with concrete content.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-c
  - note: affirmative infinity grounds marker-level claim that subtopic `c` is true infinite.
  - sourceClaimIds: [`exi-c-003-c1`, `exi-c-003-c2`, `exi-c-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`exi-c-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-a
  - note: true infinite closure prepares transition toward Being-for-self sequence.
  - sourceClaimIds: [`exi-c-003-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`bfs-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subtopic `c` seeded as bounded but complete first-pass resolution entry.
