# Relation Part A (TopicMap) Workbook (V2)

Part: `A. THE RELATION OF WHOLE AND PARTS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `whole-parts.txt` as authority for Part A.
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

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-19 (initial Part A scaffold)

Scope:

- file: `whole-parts.txt`
- active range: lines `2-263` (`A. THE RELATION OF WHOLE AND PARTS` block)

Decision:

- Part analysis begins strictly at `A. THE RELATION OF WHOLE AND PARTS`.
- Chapter/section idea material remains in `RELATION-IDEA-V2-WORKBOOK.md` and is not mixed into Part A entries.
- User-guided decomposition markers:
  - `c-001` begins with opening `First` movement.
  - `c-002` begins at explicit marker `2.` (line `52`).
  - `c-003` is unnumbered and begins at `Now...` synthesis marker (line `133`).

## Decomposition status

- completed: `rel-whp-a-001` for opening `First` movement (line `4` to `51`)
- completed: `rel-whp-a-002` for `2.` movement (line `52` to `132`)
- completed: `rel-whp-a-003` for unmarked synthesis movement beginning `Now...` (line `133` to `263`)

### Entry rel-whp-a-001 — Immediate whole/parts relation as split self-subsistence in one unity

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/whole-parts.txt`
- lineStart: 4
- lineEnd: 51

Summary:

In the opening movement, essential relation appears as immediate whole/parts relation in which reflected and immediate self-subsistence are both posited as moments and as concrete self-subsistences, each containing the other only as reflected reference.

Key points: (KeyPoint)

- k1. Essential relation begins as reflected self-subsistence that is identical with its opposite, immediate self-subsistence.
- k2. The two sides are determined as whole (reflected self-subsistence) and parts (immediate manifold concrete existence), each containing reference to the other.
- k3. In the immediate relation, momenthood and concrete self-subsistence coexist via the “also,” with substrate and positedness distributed inversely across whole and parts.

Claims: (Claim)

- c1. id: rel-whp-a-001-c1
  - subject: essential_relation
  - predicate: is_initially
  - object: reflected_self_subsistence_identical_with_immediate_opposite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-14] essential relation contains reflected self-subsistence and identity with opposite immediate self-subsistence.

- c2. id: rel-whp-a-001-c2
  - subject: whole_parts_sides
  - predicate: are
  - object: reflected_whole_and_immediate_manifold_parts_in_mutual_reference
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [15-27] other side is immediate multifarious manifold with reference to reflected unity.
    - [22-33] whole corresponds to in-and-for-itself side, parts to world-of-appearance side; each shines in the other.

- c3. id: rel-whp-a-001-c3
  - subject: immediate_whole_parts_relation
  - predicate: has_structure
  - object: dual_positedness_of_moments_and_self_subsistent_substrates
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [34-42] negative unity and positive self-subsistence are bound by the “also”; sides are both moments and self-subsistent.
    - [43-51] on whole-side reflected unity is substrate and immediate is moment; on parts-side immediate manifold is substrate and whole is external reference.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: rel-idea-ch1
  - sourceClaimIds: [rel-whp-a-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection
  - note: opening Part A presupposes the chapter-level idea that essential relation unifies reflection and immediacy.

- r2. type: transitions_to
  - targetEntryId: rel-whp-a-002
  - sourceClaimIds: [rel-whp-a-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: proceeds to explicit contradiction and reciprocal conditioning beginning at marker `2.`.

Review outcome:

- review_pending
- notes: `a-001` complete with opening `First` boundary preserved.

### Entry rel-whp-a-002 — Contradiction of reciprocal conditioning and the rise of the unconditioned

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/whole-parts.txt`
- lineStart: 52
- lineEnd: 132

Summary:

In the `2.` movement, whole and parts are each both self-subsistent and momental, yielding immediate contradiction in which each has subsistence only in the other, and this reciprocal conditioning turns back into the unconditioned.

Key points: (KeyPoint)

- k1. Whole and parts each combine self-subsistence and sublatedness, so each is only relative to the other.
- k2. The whole, though reflected totality, has subsistence only in its opposite (the parts), and the parts likewise only in the whole.
- k3. Because each side is both substrate and externally connected moment, their independent standing collapses into mediated reciprocity.
- k4. Reciprocal conditioning is realized as mutual presupposition and turns back into non-relative unity, the unconditioned.

Claims: (Claim)

- c1. id: rel-whp-a-002-c1
  - subject: whole_parts_relation
  - predicate: is
  - object: immediate_contradiction_of_self_subsistence_and_momentality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [52-61] relation contains both self-subsistence and sublatedness; each side is relative to an other.
    - [62-63] explicit statement: immediate contradiction that sublates itself.

- c2. id: rel-whp-a-002-c2
  - subject: subsistence_of_whole_and_parts
  - predicate: is
  - object: reciprocal_external_dependence_in_the_other
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [64-79] whole is self-externalized and has subsistence in manifold parts.
    - [80-106] parts are likewise whole relation yet have self-subsistence only in reflected unity of whole.

- c3. id: rel-whp-a-002-c3
  - subject: realized_reciprocal_conditioning
  - predicate: yields
  - object: turning_back_into_the_unconditioned
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [107-126] each side conditions and presupposes the other; each immediate self-subsistence is mediated through the other.
    - [127-132] the whole relation is turning back of conditioning into itself, the non-relative unconditioned.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-whp-a-001
  - sourceClaimIds: [rel-whp-a-002-c1, rel-whp-a-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [rel-whp-a-001-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `a-002` develops the immediate dual-positedness of `a-001` into explicit contradiction and mutual dependence.

- r2. type: transitions_to
  - targetEntryId: rel-whp-a-003
  - sourceClaimIds: [rel-whp-a-002-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: the unconditioned turn-back opens the `Now...` synthesis movement.

Review outcome:

- review_pending
- notes: `a-002` complete at explicit `2.` boundary.

### Entry rel-whp-a-003 — Synthesis through mediation beyond whole-and-parts immediacy

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/whole-parts.txt`
- lineStart: 133
- lineEnd: 263

Summary:

In the `Now...` synthesis movement, whole and parts are shown as a single identity that is at once split into indifferent self-subsistences, whose tautological equality and self-destruction disclose mediation as truth, sublating the relation into force and expression.

Key points: (KeyPoint)

- k1. Whole and parts are one identity in which each is only a moment, yet also appear as two indifferent self-subsistent concrete existences.
- k2. Their equality (whole=parts; parts=whole) is tautological and does not secure real distinction or stable self-subsistence.
- k3. As indifferent, each side negates itself and has subsistence only in the other, so each supposed first is mediated by its other.
- k4. Truth is mediation as negative unity of reflected and immediate moments, through which whole/parts passes over into force and expression.

Claims: (Claim)

- c1. id: rel-whp-a-003-c1
  - subject: whole_parts_relation_in_now_movement
  - predicate: is
  - object: identity_of_moments_that_equally_split_into_indifferent_self_subsistences
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [133-139] one identity of two moments, yet also two self-subsistent concrete existences indifferent to each other.

- c2. id: rel-whp-a-003-c2
  - subject: equality_of_whole_and_parts
  - predicate: is
  - object: tautological_self_equality_without_stable_differentiated_ground
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [140-165] whole equals parts as indivisible identity, but this reduces to whole equal to whole.
    - [166-184] parts equal whole only as apportioned parts, likewise yielding tautology (parts equal themselves).

- c3. id: rel-whp-a-003-c3
  - subject: truth_of_relation
  - predicate: is
  - object: mediation_as_negative_unity_that_sublates_whole_parts_into_force_expression
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [185-233] indifferent sides destroy themselves; each has subsistence only in the other; each first begins in its other.
    - [234-263] truth is mediation/negative unity; immediacy passes into positedness; relation of whole/parts passes into force and expression.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-whp-a-002
  - sourceClaimIds: [rel-whp-a-003-c1, rel-whp-a-003-c3]
  - sourceKeyPointIds: [k1, k4]
  - targetClaimIds: [rel-whp-a-002-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `a-003` develops the unconditioned turn-back of `a-002` into explicit mediation and transition beyond the whole/parts form.

- r2. type: transitions_to
  - targetEntryId: rel-frc-b-001
  - sourceClaimIds: [rel-whp-a-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: explicit final sentence transitions from whole-and-parts to force and its expressions.

Review outcome:

- review_pending
- notes: `a-003` complete at `Now...` marker; Part A (`a-001..a-003`) is complete.
