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

## Session: 2026-02-22 (deep pass)

Scope:

- files: `infinity.txt`, `alternating-infinity.txt`, `affirmative-infinity.txt`
- fixed range: full files
- pass policy: lock species (`a`/`b`/`c`) and derive triadic analytic subentries per species

Decision:

- Species-first IDs for Part C:
  - Marker: `exi-c`
  - Species groups from source labels: `c-a`, `c-b`, `c-c`
  - Analytic subentries within each species: `<species>-001..003`
- Subentries are analytic segmentations, not additional source-labeled species.

### Entry exi-c — Marker `C`: Infinity in three labeled species (`a` / `b` / `c`)

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/infinity.txt`
- lineStart: 30
- lineEnd: 44

Summary:

Part C is explicitly organized into three species: infinite in general, alternating determination, and affirmative infinity.

Key points: (KeyPoint)

- k1. `a` introduces infinite in general.
- k2. `b` develops alternating finite/infinite determination (bad infinite).
- k3. `c` resolves alternation as true affirmative infinity.

Claims: (Claim)

- c1. id: exi-c-c1
  - subject: infinity_part_c
  - predicate: has_species
  - object: c_a_c_b_c_c
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [37-44] explicit `(a)`, `(b)`, `(c)` sequence in Infinity outline.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: c-a-001
  - note: marker to species `a` triad.
  - sourceClaimIds: [`exi-c-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`c-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: c-b-001
  - note: marker to species `b` triad.
  - sourceClaimIds: [`exi-c-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`c-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: c-c-001
  - note: marker to species `c` triad.
  - sourceClaimIds: [`exi-c-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: labeled species locked; triadic subentry decomposition is analytic.

### Entry c-a-001 — `a. The infinite in general` I: absolute claim and initial risk

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/infinity.txt`
- lineStart: 2
- lineEnd: 33

Summary:

Infinity is first posited as absolute negation of finite, yet this immediate opposition still risks finitization.

Key points: (KeyPoint)

- k1. Infinity is accepted as absolute by negating finite.
- k2. This immediate negation does not yet escape finitude.
- k3. Distinguishing true from bad infinite is immediately necessary.

Claims: (Claim)

- c1. id: c-a-001-c1
  - subject: initial_infinite
  - predicate: is_presented_as
  - object: absolute_negation_of_finite_yet_prone_to_finitization
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [13-20] infinite accepted as absolute via negation of finite.
    - [21-29] true/bad distinction and finitized infinite warning.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: b-c-c-003
  - note: opens Part C from Part B's finite->infinite transition.
  - sourceClaimIds: [`c-a-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`b-c-c-003-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: c-a-002
  - note: moves from initial claim to explicit triadic determination.
  - sourceClaimIds: [`c-a-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-a`, moment 1.

### Entry c-a-002 — `a. The infinite in general` II: explicit triadic determination

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/infinity.txt`
- lineStart: 30
- lineEnd: 45

Summary:

The text explicitly declares a threefold development: simple infinite, alternation, and true infinite.

Key points: (KeyPoint)

- k1. The first moment is affirmative infinite as negation of finite.
- k2. The second moment is alternating one-sided infinite.
- k3. The third moment is one-process self-sublation of both.

Claims: (Claim)

- c1. id: c-a-002-c1
  - subject: infinity_schema
  - predicate: is_explicitly_structured_as
  - object: a_affirmative_b_alternating_c_true_infinite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [30-35] explicit three moments listed.
    - [42-44] third moment named as true infinite.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: c-a-003
  - note: schema culminates in substantive account of finite's own transcendence.
  - sourceClaimIds: [`c-a-002-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`c-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-a`, moment 2.

### Entry c-a-003 — `a. The infinite in general` III: finite's own becoming-infinite

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/infinity.txt`
- lineStart: 46
- lineEnd: 92

Summary:

Finite transcends itself by its own nature, so infinity is its affirmative vocation and truth.

Key points: (KeyPoint)

- k1. Infinite is negation of negation and true being.
- k2. Finite's transition is immanent, not externally imposed.
- k3. What is, is only the infinite as finite's truth.

Claims: (Claim)

- c1. id: c-a-003-c1
  - subject: finite
  - predicate: is_determined_as
  - object: self_transcending_into_infinite_by_its_own_nature
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [48-53] infinite as negation of negation.
    - [62-84] finite's transition into infinite is its own nature.
    - [89-92] finite vanishes; what is, is infinite.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-c
  - note: grounds marker-level claim about species `a`.
  - sourceClaimIds: [`c-a-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`exi-c-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: c-b-001
  - note: passes from infinite in general to alternating determination.
  - sourceClaimIds: [`c-a-003-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`c-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-a`, moment 3.

### Entry c-b-001 — `b. Alternating determination` I: finite infinite

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/alternating-infinity.txt`
- lineStart: 2
- lineEnd: 97

Summary:

Opposing infinite to finite as an other produces the finitized (bad) infinite.

Key points: (KeyPoint)

- k1. Immediate infinite falls back into limited something.
- k2. Finite and infinite stand as qualitative others.
- k3. The over-against infinite is itself finite.

Claims: (Claim)

- c1. id: c-b-001-c1
  - subject: bad_infinite
  - predicate: is_determined_as
  - object: infinite_fixed_over_against_finite_and_thereby_finite
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-18] infinite falls back into category of something with limit.
    - [73-83] explicit bad infinite designation.
    - [93-97] over-against infinite is finitized.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: c-b-002
  - note: contradiction deepens into explicit alternating process.
  - sourceClaimIds: [`c-b-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-b`, moment 1.

### Entry c-b-002 — `b. Alternating determination` II: external alternation

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/alternating-infinity.txt`
- lineStart: 98
- lineEnd: 227

Summary:

Finite and infinite alternate by externally transcending one another, repeatedly reinstating the same opposition.

Key points: (KeyPoint)

- k1. Each side posits its other as non-being.
- k2. Transcending appears as external doing.
- k3. Process reproduces the same finite/infinite pattern.

Claims: (Claim)

- c1. id: c-b-002-c1
  - subject: infinite_progress
  - predicate: has_form
  - object: external_alternation_of_finite_and_infinite
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [124-159] each side has limit and posits the other outside itself.
    - [178-207] alternation as repeated external transition.
    - [224-227] each is unity of itself and its other.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: c-b-003
  - note: repetition reveals unresolved monotony and concealed unity.
  - sourceClaimIds: [`c-b-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-b`, moment 2.

### Entry c-b-003 — `b. Alternating determination` III: unreconciled bad infinity

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/alternating-infinity.txt`
- lineStart: 228
- lineEnd: 288

Summary:

Bad infinite is the unresolved monotony of alternation, driven by an unreflected unity.

Key points: (KeyPoint)

- k1. Progress is contradiction repeatedly restated.
- k2. Bad infinite cannot free itself from finite.
- k3. Unity is present only as hidden impulse.

Claims: (Claim)

- c1. id: c-b-003-c1
  - subject: bad_infinite_progress
  - predicate: is
  - object: repetitive_unresolved_alternation_with_hidden_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [239-252] abstract transcending remains incomplete.
    - [253-259] repetitive monotony of finite/infinite alternation.
    - [260-267] unity present as impulse but unreflected.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-c
  - note: grounds marker-level claim about species `b`.
  - sourceClaimIds: [`c-b-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`exi-c-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: c-c-001
  - note: unresolved alternation necessitates affirmative infinity.
  - sourceClaimIds: [`c-b-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`c-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-b`, moment 3.

### Entry c-c-001 — `c. Affirmative infinity` I: inseparability and unity

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/affirmative-infinity.txt`
- lineStart: 2
- lineEnd: 118

Summary:

Affirmative infinity begins by showing finite and infinite as inseparable moments whose unity negates qualitative isolation.

Key points: (KeyPoint)

- k1. Each term contains the other in its determination.
- k2. Isolated standing turns each into finite.
- k3. Their unity removes qualitative fixation.

Claims: (Claim)

- c1. id: c-c-001-c1
  - subject: finite_infinite_unity
  - predicate: is_determined_as
  - object: inseparable_moments_not_external_combination
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [20-31] neither finite nor infinite graspable without the other.
    - [45-59] isolated infinite is finite infinite.
    - [80-108] unity negates qualitative fixation.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: c-c-002
  - note: unity is developed as self-sublation and self-return.
  - sourceClaimIds: [`c-c-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-c`, moment 1.

### Entry c-c-002 — `c. Affirmative infinity` II: negation of negation and circle

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/affirmative-infinity.txt`
- lineStart: 119
- lineEnd: 390

Summary:

True infinity is mediated self-return (negation of negation), imaged as circle rather than endless line.

Key points: (KeyPoint)

- k1. Unity is each term's self-sublation, not external synthesis.
- k2. Infinite progression contains a self-closing movement.
- k3. Circle image expresses true infinite against line image of bad infinite.

Claims: (Claim)

- c1. id: c-c-002-c1
  - subject: true_infinite
  - predicate: is
  - object: mediated_self_return_as_negation_of_negation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [157-170] each is unity only as sublating itself.
    - [236-266] self-closing movement restores through mediation.
    - [372-382] circle image versus line image.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: c-c-003
  - note: self-return is further determined as concrete reality/ideality.
  - sourceClaimIds: [`c-c-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-c`, moment 2.

### Entry c-c-003 — `c. Affirmative infinity` III: concrete reality and ideality

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/affirmative-infinity.txt`
- lineStart: 391
- lineEnd: 427

Summary:

Affirmative infinity is concrete reality, while finite existence is idealized as moment within it.

Key points: (KeyPoint)

- k1. True infinite is concrete presence, not unattainable beyond.
- k2. Finite in true infinite has the status of idealized moment.
- k3. Fixing finite as sole real repeats bad-infinite one-sidedness.

Claims: (Claim)

- c1. id: c-c-003-c1
  - subject: affirmative_infinity
  - predicate: grounds
  - object: higher_reality_with_finite_as_ideality_moment
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [391-399] true infinite as higher reality with concrete content.
    - [408-414] finite as ideality moment in true infinite.
    - [421-427] finite-realism reverts to bad-infinite one-sidedness.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-c
  - note: confirms marker-level claim that species `c` is true infinite.
  - sourceClaimIds: [`c-c-002-c1`, `c-c-003-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`exi-c-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-a
  - note: true infinite as concrete self-reference opens Being-for-self.
  - sourceClaimIds: [`c-c-002-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`bfs-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: species `c-c`, moment 3 and handoff to Being-for-self.
