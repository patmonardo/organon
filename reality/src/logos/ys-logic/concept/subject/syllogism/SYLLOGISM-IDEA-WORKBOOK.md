# Syllogism Idea Workbook

Part: `IDEA. SYLLOGISM`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Syllogism source files in this folder.
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

## Session: 2026-02-21 (initial scaffold)

Scope:

- files:
  - `existence.txt`
  - `reflection.txt`
  - `necessity.txt`
- focus: initialize Syllogism as final qualitative-logic transition toward Reason/Idea and seed precise SubTopic/Entry extraction.

Decision:

- Keep this workbook as the sphere-level Syllogism-IDEA coordinator.
- Keep Part A/B/C workbooks as first-order extraction artifacts.
- Represent all three syllogisms as both genera and species via marker SubTopics with numbered Entries.
- Track the U-U-U mathematical syllogism as an explicit fourth subspecies inside Part A (`d`).
- Keep Kant/Hegel comparative placement as a method note until all Part A claims are stabilized.

### Entry syl-idea-001 — Syllogism as closure of qualitative logic and transition to Reason

Span:

- sourceFile: `src/logos/ys-logic/concept/subject/syllogism/syllogism-idea.txt`
- lineStart: 1
- lineEnd: 152

Summary:

Syllogism completes the qualitative-formal sequence while internally transitioning concept-determinations toward reflection, necessity, and Idea-level mediation.

Key points: (KeyPoint)

- k1. Syllogism is treated as end of qualitative/formal logic while preserving contentful determinations.
- k2. The three primary syllogisms are organized for extraction as both genera and species.
- k3. The U-U-U mathematical syllogism is retained as an immanent figure in the transition.

Claims: (Claim)

- c1. id: syl-idea-001-c1
  - subject: syllogism_of_existence
  - predicate: initiates
  - object: transition_from_immediate_formal_mediation_to_reflective_mediation
  - modality: asserted
  - confidence: 0.86
  - evidence:
    - [2-37] opening determination of immediate/formal syllogism and dialectical movement of mediation.
    - [874-895] transition from formal figures to syllogism of reflection.

- c2. id: syl-idea-001-c2
  - subject: fourth_figure_mathematical_syllogism
  - predicate: is_posited_as
  - object: integral_moment_of_syllogism_of_existence_development
  - modality: asserted
  - confidence: 0.86
  - evidence:
    - [785-873] explicit fourth figure U-U-U and its role.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-idea-002
  - note: Transition from general framing to the essential elements of the syllogistic machine.
  - sourceClaimIds: [`syl-idea-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry syl-idea-002 — Syllogism as Triadic Connection Machine (Rationality/Term Logic)

- span: `93-113`
- summary: The syllogism is the essential unity of extremes via a middle term, acting as the triadic engine for rationality and structural term networks.
- keyPoints: (KeyPoint)
  - k1. The essential element is the unity of extremes, the middle term, and the supporting ground.
  - k2. Abstraction distorts this unity by treating the middle term as an external, spatial "connection."
  - k3. Rationality resides in the internal unity where the middle term is posited as the ground.
  - k4. Syllogistic rationality is "Term Logic": a structural network of termini specified as connections.
- claims: (Claim)
  - c1. id: syl-idea-002-c1
    - subject: middle_term
    - predicate: is_posited_as
    - object: absolute_relation_causality
    - modality: asserted
    - confidence: 0.95
    - evidence: [93-96] the middle term as the ground that supports them.
  - c2. id: syl-idea-002-c2
    - subject: syllogism
    - predicate: unites
    - object: being_and_essence
    - modality: asserted
    - confidence: 0.90
    - evidence: [8-17] restoration of the concept as the unity of the extremes (mapped to the lifecycle of being and essence).
  - c3. id: syl-idea-002-c3
    - subject: syllogism
    - predicate: operates_as
    - object: triadic_connection_machine
    - modality: asserted
    - confidence: 0.92
    - evidence: [93-113] the specification of the triadic unity (extremes + middle/ground) as the essential element.
  - c4. id: syl-idea-002-c4
    - subject: rationality
    - predicate: transition_from
    - object: external_connection_to_internal_term_logic_network
    - modality: asserted
    - confidence: 0.88
    - evidence: [102-113] critique of the "external connection" vs. the "reason of the syllogism."
- relations: (Relation)
  - r1. type: refines
    - targetEntryId: syl-idea-001
    - note: Moves from the general definition of Syllogism to its internal mechanics as a Connection Machine.
  - r2. type: transitions_to
    - targetEntryId: syl-exi-a
    - note: The structural term logic of the Idea section provides the protocol for the Syllogism of Existence.

### Entry syl-idea-003 — USP as Restoration and Content-Rich Proposition

- span: `65-84`
- summary: The syllogism is the USP protocol that restores the concept's unity, transitioning from formal inference to content-rich rationality where the Object is the concept's matter.
- keyPoints: (KeyPoint)
  - k1. The concept's form is the Universal (U), whereas its matter is the Object.
  - k2. Rationality is not a content-free formal cognition but is essentially replete with content.
  - k3. The USP protocol (Universal-Singular-Particular) restores the concept lost in the diremption of judgment.
  - k4. Rationality ("The Rational") is the unity of determinate extremes (e.g., the cookie ready to be eaten).
- claims: (Claim)
  - c1. id: syl-idea-003-c1
    - subject: concept_form
    - predicate: is
    - object: universal
    - modality: asserted
    - confidence: 0.90
    - evidence: [21-26] understanding as faculty of abstract universality vs reason as totality and unity.
  - c2. id: syl-idea-003-c2
    - subject: concept_matter
    - predicate: is
    - object: object
    - modality: inferred
    - confidence: 0.85
    - evidence: [65-71] rational cognition characterized through its objects.
  - c3. id: syl-idea-003-c3
    - subject: syllogism
    - predicate: implements
    - object: USP_protocol
    - modality: asserted
    - confidence: 0.92
    - evidence: [72-78] the fulfilled universality as unity of determined differences.
  - c4. id: syl-idea-003-c4
    - subject: rational_proposition
    - predicate: is
    - object: content_rich_qualitative_judgment
    - modality: asserted
    - confidence: 0.95
    - evidence: [55-56] no content can be rational except by virtue of rational form; [79-84] the rational as negativity replete with content.
- relations: (Relation)
  - r1. type: refines
    - targetEntryId: syl-idea-002
    - note: Specifies the protocol (USP) and the material nature (Object/Content) of the Connection Machine.

Review outcome:

- review_pending
- notes: initialized with one special IDEA entry; refine confidence and comparative claims after Part A deep pass.
