# Life Part B Workbook

Part: `B. THE LIFE-PROCESS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `life-process.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries follow coherent analysis windows of complete sentence groups.

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Working template

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-23 (first Life Part B pass)

Scope:

- file: `life-process.txt`
- fixed range: lines `4-180`

Decision:

- Part B is sectioned as three determinations (immediate, mediation, concrete return), not by numeric/letter subentries.
- Keep first pass conservative with minimal, line-anchored claims.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

Extraction map:

- First determination (immediate): lines `4-68`
- Second determination (mediation): lines `69-130`
- Third determination (concrete return): lines `131-180`

### Entry con-idea-life-b-001 — First determination (immediate): need, contradiction, and pain

Span:

- sourceFile: `src/compiler/concept/idea/life/life-process.txt`
- lineStart: 4
- lineEnd: 68

Summary:

The life-process begins from self-certainty against externality and unfolds as contradiction whose lived actuality is pain.

Key points: (KeyPoint)

- k1. The subject confronts external world as null and unessential relative to its immanent purpose.
- k2. Need is twofold self-determination: self-loss and preserved identity in one movement.
- k3. Living contradiction becomes concrete existence as pain.

Claims: (Claim)

- c1. id: con-idea-life-b-001-c1
  - subject: living_subject
  - predicate: is_determined_as
  - object: self_purpose_against_presupposed_external_nullity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-20] subject as purpose unto itself with certainty of other's nullity.
    - [21-29] inner process mediated by indifferent objective totality.

- c2. id: con-idea-life-b-001-c2
  - subject: living_contradiction
  - predicate: has_existent_form
  - object: pain
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [30-58] need and self-determination produce rupture as absolute inequality.
    - [59-68] pain is the concrete actuality of contradiction in living natures.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-life-b-002
  - note: contradiction in pain moves into purposive mediation with indifferent objectivity.
  - sourceClaimIds: [`con-idea-life-b-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first determination isolates the contradiction/pain core.

### Entry con-idea-life-b-002 — Second determination (mediation): subjective certainty and sublation of external purposiveness

Span:

- sourceFile: `src/compiler/concept/idea/life/life-process.txt`
- lineStart: 69
- lineEnd: 130

Summary:

Feeling-driven impulse treats objective externality as appearance and transforms external purposiveness into concept-immanent determination.

Key points: (KeyPoint)

- k1. Feeling translates rupture into impulse toward identity through negation of negation.
- k2. Objective world is externally apt yet conceptually impotent against life.
- k3. Violence over object interrupts mechanism/chemism and internalizes purposiveness.

Claims: (Claim)

- c1. id: con-idea-life-b-002-c1
  - subject: impulse_identity
  - predicate: is_determined_as
  - object: subjective_certainty_mediating_objective_appearance
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [69-84] pain transitions to impulse and identity through negation.
    - [85-107] objectivity is indifferent appearance becoming apt to subject.

- c2. id: con-idea-life-b-002-c2
  - subject: external_purposiveness
  - predicate: is_sublated_into
  - object: immanent_conceptual_determination
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [109-120] subject's mediated violence interrupts external process.
    - [121-130] concept posits itself as object's immanent essence.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-life-b-001
  - note: specifies how pain-driven contradiction is processed into purposive mediation.
  - sourceClaimIds: [`con-idea-life-b-002-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`con-idea-life-b-001-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-life-b-003
  - note: internalized purposiveness moves into assimilation/reproduction and genus transition.
  - sourceClaimIds: [`con-idea-life-b-002-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second determination captures the mediation pivot.

### Entry con-idea-life-b-003 — Third determination (concrete return): assimilation, reproduction, and rise to genus

Span:

- sourceFile: `src/compiler/concept/idea/life/life-process.txt`
- lineStart: 131
- lineEnd: 180

Summary:

Assimilation transforms external process into reproduction, through which individuality regains itself and raises itself to real universal life as genus.

Key points: (KeyPoint)

- k1. Appropriation makes object a means by conferring subjectivity as its substance.
- k2. Mechanical/chemical process is subordinated by life as their truth and universality.
- k3. Rejoined objectivity yields singularity and elevation of particularity to genus.

Claims: (Claim)

- c1. id: con-idea-life-b-003-c1
  - subject: assimilation
  - predicate: culminates_in
  - object: reproduction_as_self_identical_return
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [132-158] appropriation and transformation produce reproduction.
    - [160-169] objective process yields feeling of self as negative unity.

- c2. id: con-idea-life-b-003-c2
  - subject: external_life_process
  - predicate: results_in
  - object: real_universal_life_as_genus
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [170-177] individual rejoins objectivity and sublates particularity.
    - [178-180] explicit transition to genus.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-life-b-002
  - note: confirms that sublated external purposiveness becomes internal reproduction.
  - sourceClaimIds: [`con-idea-life-b-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-idea-life-b-002-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third determination terminates with explicit genus handoff.
