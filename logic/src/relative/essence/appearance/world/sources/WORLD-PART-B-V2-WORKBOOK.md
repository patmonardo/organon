# World Part B (TopicMap) Workbook (V2)

Part: `B. THE WORLD OF APPEARANCE AND THE WORLD-IN-ITSELF`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `world.txt` as authority for Part B.
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

## Session: 2026-02-19 (initial Part B scaffold)

Scope:

- file: `world.txt`
- active range: lines `2-259` (`B. THE WORLD OF APPEARANCE AND THE WORLD-IN-ITSELF` block)

Decision:

- Initialize Part B workbook before entry extraction.
- Decompose into three conceptual entries aligned to the source flow.
- User-guided boundary rule: keep the final paragraph as a dedicated `b-003` span.

## Decomposition status

- completed: `wld-wld-b-001` for section `1` opening movement (line `4` to `156`)
- completed: `wld-wld-b-002` for section `2` main opposition/ground movement (line `157` to `241`)
- completed: `wld-wld-b-003` for final paragraph (line `242` to `259`) [includes: "The world that exists in and for itself is ... inversion of the world of appearance."]

### Entry wld-wld-b-001 — From law as substrate to suprasensible world as essential totality

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/world.txt`
- lineStart: 4
- lineEnd: 156

Summary:

Section `1` develops the movement in which law shifts from simple substrate of appearance to negative unity and posited reality, so that appearance reflected into itself becomes an in-and-for-itself (suprasensible) world of essential, reflected concrete existence.

Key points: (KeyPoint)

- k1. The concretely existing world raises itself to a kingdom of laws where appearance, though changing, endures in lawful self-identity.
- k2. Law is not merely substrate but also negative unity: each side's subsistence is its own non-subsistence in the other.
- k3. Through this, law becomes posited and real totality, integrating essential negativity and yielding a world in-and-for-itself.
- k4. This in-and-for-itself world is suprasensible reflected concrete existence, overcoming both naive sensible immediacy and unconscious reflection.

Claims: (Claim)

- c1. id: wld-wld-b-001-c1
  - subject: kingdom_of_laws
  - predicate: is
  - object: enduring_self_identity_of_appearance_in_flux
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-11] world raises to kingdom of laws; appearance in changing is also enduring and its positedness is law.
    - [12-23] law as simple identity/substrate of appearance, with grounds and conditions among appearing things.

- c2. id: wld-wld-b-001-c2
  - subject: law
  - predicate: develops_into
  - object: negative_unity_of_reciprocal_positedness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [24-41] law is other of appearance and reflection into itself; appearance has in law its opposite.
    - [42-74] each side's positedness is the other's; subsistence is non-subsistence; negative unity becomes explicit.

- c3. id: wld-wld-b-001-c3
  - subject: appearance_reflected_into_itself
  - predicate: becomes
  - object: suprasensible_world_of_reflected_essential_concrete_existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [75-108] law obtains negative form, becomes essential totality, and discloses world in-and-for-itself.
    - [109-156] this world is suprasensible reflected concrete existence and marks overcoming of merely sensible immediacy.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: wld-law-a-003
  - sourceClaimIds: [wld-wld-b-001-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [wld-law-a-003-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: Part B.1 realizes the necessity deficit from Part A.3 by making negative unity explicit in law itself.

- r2. type: transitions_to
  - targetEntryId: wld-wld-b-002
  - sourceClaimIds: [wld-wld-b-001-c3]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: once the in-and-for-itself world is established, section 2 develops its internal split and ground-connection to the world of appearance.

Review outcome:

- review_pending
- notes: `b-001` complete with section-1 boundary preserved.

### Entry wld-wld-b-002 — World opposition as restored ground-connection of negative identity

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/world.txt`
- lineStart: 157
- lineEnd: 241

Summary:

Section `2` determines the in-and-for-itself world as totality that internally splits into opposed worlds, where restored ground-connection is no longer simple law-like identity but the essential negative identity and becoming-transition of opposed determinations.

Key points: (KeyPoint)

- k1. The in-and-for-itself world is totality and absolute negativity, so it splits into sensible world and world of appearance.
- k2. The essential world is both negative unity and positing/determinate ground of the world of appearance.
- k3. Ground-connection is restored as essential connection of opposed sides, not as mere diversified or identical content.
- k4. Because opposition is intrinsic, ground returns as sublated ground in appearance, whose identity is becoming/transition.

Claims: (Claim)

- c1. id: wld-wld-b-002-c1
  - subject: world_in_and_for_itself
  - predicate: is
  - object: totality_of_concrete_existence_that_splits_by_absolute_negativity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [157-168] totality of concrete existence, absolute negativity, and split into world of senses/otherness/appearance.
    - [169-179] as totality it is one side over against world of appearance, which returns to it as ground.

- c2. id: wld-wld-b-002-c2
  - subject: essential_world
  - predicate: functions_as
  - object: positing_and_determinate_ground_of_world_of_appearance
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [176-183] essential world is positing ground that makes itself posited immediacy as world of appearance.
    - [184-199] as kingdom of laws with content, it is determinate ground and total content of appearance's manifoldness.

- c3. id: wld-wld-b-002-c3
  - subject: restored_ground_connection
  - predicate: is
  - object: essential_negative_identity_of_opposed_sides_as_becoming_transition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [200-225] connection restored not as simple identity but as total/negative essential connection of opposed contents.
    - [226-241] opposition as foundered ground yields sublated ground-connection whose identity is becoming and transition.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-wld-b-001
  - sourceClaimIds: [wld-wld-b-002-c2, wld-wld-b-002-c3]
  - sourceKeyPointIds: [k2, k3]
  - targetClaimIds: [wld-wld-b-001-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `b-002` makes explicit that the in-and-for-itself world from `b-001` is internally oppositional and ground-connected through negative identity.

- r2. type: transitions_to
  - targetEntryId: wld-wld-b-003
  - sourceClaimIds: [wld-wld-b-002-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: transition to the final paragraph where this relation is stated as inversion of the world of appearance.

Review outcome:

- review_pending
- notes: `b-002` complete; ready for final paragraph entry `b-003`.

### Entry wld-wld-b-003 — Identity as opposition: the in-itself world as inversion of appearance

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/world.txt`
- lineStart: 242
- lineEnd: 259

Summary:

In the concluding paragraph, the world existing in-and-for-itself is shown as identical with the world of appearance as its ground, yet this very identity is determinately oppositional, so their specific relation is inversion.

Key points: (KeyPoint)

- k1. The in-and-for-itself world is internally differentiated and contains manifold content.
- k2. It is identical with the world of appearance (the posited world) and is thereby its ground.
- k3. This identity is simultaneously determined as opposition through reflection-into-otherness.
- k4. Therefore, the in-and-for-itself world stands as inversion of the world of appearance.

Claims: (Claim)

- c1. id: wld-wld-b-003-c1
  - subject: world_in_and_for_itself
  - predicate: is
  - object: internally_differentiated_totality_of_manifold_content
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [242-244] in-and-for-itself world is itself distinguished within itself in manifold content.

- c2. id: wld-wld-b-003-c2
  - subject: world_in_and_for_itself
  - predicate: is_identical_with
  - object: world_of_appearance_as_its_posited_ground_relation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [245-247] it is identical with the world of appearance/posited world and to this extent its ground.

- c3. id: wld-wld-b-003-c3
  - subject: specific_connection_of_the_two_worlds
  - predicate: is
  - object: oppositional_identity_as_inversion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [248-256] identity connection is at the same time opposition because appearance is reflection into otherness and returns into itself in the other world as opposite.
    - [257-259] explicit statement: in-and-for-itself world is inversion of world of appearance.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-wld-b-002
  - sourceClaimIds: [wld-wld-b-003-c3]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [wld-wld-b-002-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `b-003` gives the explicit formula (inversion) for the oppositional ground-connection developed in `b-002`.

- r2. type: transitions_to
  - targetEntryId: wld-dis-c-001
  - sourceClaimIds: [wld-wld-b-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: inversion relation opens directly into `C. THE DISSOLUTION OF APPEARANCE`.

Review outcome:

- review_pending
- notes: `b-003` complete; Part B is now fully complete (`b-001..b-003`).
