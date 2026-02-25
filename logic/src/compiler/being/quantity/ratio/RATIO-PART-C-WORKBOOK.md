# Ratio Part C Workbook

Part: `C. THE RATIO OF POWERS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/ratio/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `powers.txt` as authority.
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

- file: `powers.txt`
- fixed range: lines `1-140`
- pass policy: numeric ordinary entries only

Decision:

- Use IDs: `ratio-pow-001`, `ratio-pow-002`, `ratio-pow-003`.
- Ratio has no subspecies in this workbook pass.
- Keep closure handoff explicit to Measure (`mea-a-001`) for next domain seeding.

### Entry ratio-pow-001 — Powers I: quantum self-identical in its otherness

Span:

- sourceFile: `src/compiler/being/quantity/ratio/powers.txt`
- lineStart: 4
- lineEnd: 38

Summary:

In the ratio of powers, quantum returns into itself so that its otherness is determined by itself and exponent becomes qualitative.

Key points: (KeyPoint)

- k1. Unit and amount are unified through power.
- k2. Quantum’s otherness is self-determined.
- k3. Exponent is qualitative, not immediate quantum.

Claims: (Claim)

- c1. id: ratio-pow-001-c1
  - subject: ratio_of_powers
  - predicate: is
  - object: self_identity_of_quantum_in_otherness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [14-24] unit/amount identity in power.
    - [26-33] exponent is qualitative determinateness.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ratio-pow-002
  - note: qualitative exponent is next shown as full realization of quantum’s concept.
  - sourceClaimIds: [`ratio-pow-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ratio-pow-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry ratio-pow-002 — Powers II: realized concept of quantum as self-determined difference

Span:

- sourceFile: `src/compiler/being/quantity/ratio/powers.txt`
- lineStart: 40
- lineEnd: 84

Summary:

Ratio of powers displays quantum’s concept by positing its surpassing-into-other as determined through itself.

Key points: (KeyPoint)

- k1. Ratio of powers realizes quantum’s concept.
- k2. Quantum remains self-identical through otherness.
- k3. Difference is quantum’s own quality as self-reference.

Claims: (Claim)

- c1. id: ratio-pow-002-c1
  - subject: ratio_of_powers
  - predicate: realizes
  - object: quantum_as_self_determining_relation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [47-58] ratio of powers displays what quantum is implicitly.
    - [77-83] difference is posited as quantum’s own determining quality.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ratio-pow-003
  - note: realized concept transitions into explicit passage to measure.
  - sourceClaimIds: [`ratio-pow-002-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`ratio-pow-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry ratio-pow-003 — Powers III: quantity returned into itself as measure

Span:

- sourceFile: `src/compiler/being/quantity/ratio/powers.txt`
- lineStart: 85
- lineEnd: 140

Summary:

By double transition between quantity and quality, quantum ceases to be indifferent externality and its truth is measure.

Key points: (KeyPoint)

- k1. Quantum becomes quality through mediated externality.
- k2. Scientific totality requires double transition.
- k3. Truth of quantum is measure.

Claims: (Claim)

- c1. id: ratio-pow-003-c1
  - subject: truth_of_quantum
  - predicate: is
  - object: measure
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [136-140] explicit statement that truth of quantum is measure.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mea-a-001
  - note: ratio culminates in measure domain.
  - sourceClaimIds: [`ratio-pow-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`mea-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.
