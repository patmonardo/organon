# Existence Idea Workbook

Part: `IDEA. EXISTENCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/existence/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Existence source files in this folder.
- Claims must be line-anchored.
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

## Session: 2026-02-22 (initial scaffold)

Scope:

- files:
  - `existence.txt`
  - `something-and-other.txt`
  - `constitution.txt`
  - `finitude.txt`
  - `infinity.txt`
  - `alternating-infinity.txt`
  - `affirmative-infinity.txt`
- focus: sphere-level Existence map across Part A/B/C and handoff toward Being-for-self

Decision:

- Keep this workbook as the Existence-IDEA coordinator.
- Keep `EXISTENCE-PART-A/B/C-WORKBOOK.md` as first-order extraction artifacts.
- Follow labeled subtopic priority (`a/b/c`) before numbered micro-structure.
- Defer higher-order synthesis claims until part-level spans and transitions are stabilized.

Review outcome:

- review_pending
- notes: initialized; ready for sphere-level synthesis after Part A/B/C seed stabilization.

### Entry exi-idea-001 — Existence sphere coordinator: A/B/C articulation and progression

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 2
- lineEnd: 13

Summary:

The Existence sphere is coordinated as a three-part progression: Existence-as-such, Finitude, and Infinity, with each part carrying an internal `a/b/c` articulation and an explicit onward movement.

Key points: (KeyPoint)

- k1. Part A explicitly unfolds Existence as such through labeled moments `a/b/c`.
- k2. Part B explicitly develops Finitude through `something/other -> constitution/limit -> finite`.
- k3. Part C explicitly develops Infinity through `infinite in general -> alternating infinite -> affirmative true infinite`.

Claims: (Claim)

- c1. id: exi-idea-001-c1
  - subject: existence_sphere
  - predicate: begins_with
  - object: existence_as_such_with_labeled_a_b_c_structure
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [2-13] explicit `A. EXISTENCE AS SUCH` heading and `a/b/c` schema in `existence.txt`.

- c2. id: exi-idea-001-c2
  - subject: existence_sphere
  - predicate: develops_into
  - object: finitude_via_three_bounded_moments
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [2-19] explicit `B. FINITUDE` and `(a)/(b)/(c)` movement in `something-and-other.txt`.

- c3. id: exi-idea-001-c3
  - subject: existence_sphere
  - predicate: culminates_in
  - object: true_infinite_as_affirmative_resolution
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [2-44] `C. INFINITY` schema and `(c)` as true infinite in `infinity.txt`.
    - [335-343] true infinite as becoming, in `affirmative-infinity.txt`.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-a
  - note: coordinator claim grounds the Part A marker as first movement of the sphere.
  - sourceClaimIds: [`exi-idea-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-a-c1`, `exi-a-c2`, `exi-a-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: exi-b
  - note: coordinator claim grounds the Part B marker as second movement of the sphere.
  - sourceClaimIds: [`exi-idea-001-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`exi-b-c1`, `exi-b-c2`, `exi-b-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: supports
  - targetEntryId: exi-c
  - note: coordinator claim grounds the Part C marker as third movement of the sphere.
  - sourceClaimIds: [`exi-idea-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-c-c1`, `exi-c-c2`, `exi-c-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first sphere-level coordinator entry added; keeps synthesis bounded to stabilized part markers.
