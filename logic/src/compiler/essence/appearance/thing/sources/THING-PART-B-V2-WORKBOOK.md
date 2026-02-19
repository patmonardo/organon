# Thing Part B (TopicMap) Workbook (V2)

Part: `B. THE CONSTITUTION OF THE THING OUT OF MATTERS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `matter.txt` as authority for Part B.
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

## Session: 2026-02-19 (initial scaffold)

Scope:

- file: `matter.txt`
- active range: full Part B source in this file

Decision:

- Initialize Part B workbook structure before full entry extraction.
- Part B is non-numbered in this source; do not impose synthetic `1/2/3` decomposition.
- Use conceptual sentence-group spans only.
- Methodology update: adopt user-guided marker decomposition for this chapter's non-numbered text where explicit discourse markers appear.
- For `matter.txt`, use `First/Second/Third` markers as primary boundaries:
  - prelude + `First` paragraph -> entry `thg-mat-b-001` (b-a)
  - `Second` paragraph -> entry `thg-mat-b-002` (b-b)
  - `Third` paragraph + remainder -> entry `thg-mat-b-003` (b-c)

## Decomposition status

- completed: `thg-mat-b-001` (b-a) for prelude + `First` paragraph (line `3` to pre-`57`)
- completed: `thg-mat-b-002` (b-b) for `Second` paragraph (line `57` to pre-`73`)
- completed: `thg-mat-b-003` (b-c) for `Third` paragraph + remainder (line `73` to `153`)

### Entry thg-mat-b-001 — Transition from property to matter and preservation of negative thinghood

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/matter.txt`
- lineStart: 3
- lineEnd: 56

Summary:

In Part B's opening movement (`prelude + First`), property is recast as self-subsistent matter, thinghood is reduced to an unessential moment of reflection, yet a negative moment of thinghood is preserved within matter's self-subsistence.

Key points: (KeyPoint)

- k1. The transition from property to matter is exemplified by chemical reification of qualities into stuffs.
- k2. The transition is necessary because properties function as the essential, self-subsistent truth of things, reducing thinghood's independent status.
- k3. In the `First` movement, the negative moment persists as restored thinghood within matter's self-continuity.

Claims: (Claim)

- c1. id: thg-mat-b-001-c1
  - subject: transition_property_to_matter
  - predicate: is_exhibited_as
  - object: reification_of_properties_into_self_subsistent_stuffs
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [3-13] chemistry treats color/smell/etc. as luminous/coloring/odorific and other matters.
    - [14-24] common discourse treats things as composed of matters/stuffs with unclear thing-status.

- c2. id: thg-mat-b-001-c2
  - subject: necessity_of_transition_to_matters
  - predicate: is_grounded_in
  - object: essential_self_subsistence_of_property_and_reduction_of_thinghood
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [25-31] properties are what is essential in things and thus truly self-subsistent.
    - [32-43] reflection of property sublates distinction into continuity; thinghood reduced to unessential moment.

- c3. id: thg-mat-b-001-c3
  - subject: first_negative_moment
  - predicate: is_preserved_as
  - object: restored_thinghood_within_negative_self_subsistence_of_matter
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [45-51] property becomes self-subsistent matter through sublation of thing-difference; continuity includes negativity.
    - [52-56] negative unity yields restored thinghood as negative self-subsistence against positive self-subsistence of stuff.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-a-c-003
  - sourceClaimIds: [thg-mat-b-001-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: Part B opening operationalizes Part A's endpoint where property became self-subsistent matter.

- r2. type: transitions_to
  - targetEntryId: thg-mat-b-002
  - sourceClaimIds: [thg-mat-b-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: move from preserved negative moment (`First`) to determination of the thing as fully determined (`Second`).

Review outcome:

- review_pending
- notes: `b-a` entry complete under user-guided First/Second/Third decomposition.

### Entry thg-mat-b-002 — Thing returns from property-continuity into determinate thisness

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/matter.txt`
- lineStart: 57
- lineEnd: 72

Summary:

In the `Second` movement, the thing advances from indeterminate abstract identity to full determinateness by way of properties; the distinction from others is sublated through continuity, and the thing returns into itself as this determinate thing.

Key points: (KeyPoint)

- k1. The thing progresses from indeterminacy toward full determinateness.
- k2. As thing-in-itself it begins as abstract, indeterminate negative concrete existence and is then determined through properties.
- k3. Since property makes the thing continuous with others, imperfect distinction is sublated and the thing returns into itself as this thing.

Claims: (Claim)

- c1. id: thg-mat-b-002-c1
  - subject: thing
  - predicate: progresses_to
  - object: full_determinateness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [57-58] explicit progression from indeterminacy to full determinateness.

- c2. id: thg-mat-b-002-c2
  - subject: thing_in_itself
  - predicate: is_determined_from
  - object: abstract_indeterminate_identity_through_properties
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [59-63] thing-in-itself as abstract identity/indeterminate negative concrete existence.
    - [63-66] thing is determined through its properties.

- c3. id: thg-mat-b-002-c3
  - subject: distinction_of_thing_from_others
  - predicate: is_sublated_into
  - object: return_into_itself_as_this_thing
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [66-69] through property the thing is continuous with others, so imperfect distinction is sublated.
    - [70-72] thing returns into itself and is determined in itself as this thing.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-mat-b-001
  - sourceClaimIds: [thg-mat-b-002-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [thg-mat-b-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `Second` specifies how preserved negative thinghood from `First` acquires determinate self-return.

- r2. type: transitions_to
  - targetEntryId: thg-mat-b-003
  - sourceClaimIds: [thg-mat-b-002-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: handoff to `Third` where this self-return is recast as unessential determinateness within self-subsistent matter.

Review outcome:

- review_pending
- notes: `b-b` entry complete; only `b-c` remains in Part B decomposition.

### Entry thg-mat-b-003 — Determinate thisness collapses into external material collectivity

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/matter.txt`
- lineStart: 73
- lineEnd: 153

Summary:

In the `Third + remainder` movement, the thing's self-returned determinateness is shown as inessential within self-subsistent matter; property's reflected unity splits into matters and "this" thing, and the thing resolves into a merely quantitative collection of indifferent self-subsistent matters.

Key points: (KeyPoint)

- k1. The third movement recasts self-referring determinateness as inessential within the element of self-subsistent matter.
- k2. Property's external-essential unity differentiates into matters and this thing, with thinghood reduced to a sublated negative connection.
- k3. The thing as "this" is only an external, quantitative linkage (an "also") of indifferent, impenetrable matters.

Claims: (Claim)

- c1. id: thg-mat-b-003-c1
  - subject: third_turning_back_of_the_thing
  - predicate: is_determined_as
  - object: unessential_determinateness_within_self_subsistent_matter
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [73-82] turning-back determinateness is unessential; difference/determinateness is sublated into externality of self-subsistent matter.
    - [83-85] complete determinateness exists in the element of inessentiality.

- c2. id: thg-mat-b-003-c2
  - subject: movement_of_property
  - predicate: yields
  - object: split_between_matters_and_this_thing_with_sublated_thing_unity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [86-99] property's unity of externality/essentiality repels into reflected self-subsistent moments: matters and this thing.
    - [100-123] thing freed from inherence appears as other of itself; negative connection in the one is now sublated.

- c3. id: thg-mat-b-003-c3
  - subject: thing_as_this
  - predicate: is
  - object: merely_quantitative_collection_of_indifferent_self_subsistent_matters
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [124-144] thing consists of self-subsistent matters; their link is unessential, matters are mutually indifferent/impenetrable.
    - [145-153] thing is their quantitative connection/"also"; combination itself constitutes the thing.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-mat-b-002
  - sourceClaimIds: [thg-mat-b-003-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [thg-mat-b-002-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: `Third` resolves the `Second` return-into-self by showing thisness as inessential in material externality.

- r2. type: transitions_to
  - targetEntryId: thg-dis-c-001
  - sourceClaimIds: [thg-mat-b-003-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: quantitative, externally linked collectivity of matters opens directly into Part C dissolution dynamics.

Review outcome:

- review_pending
- notes: `b-c` entry complete; Part B decomposition is now fully complete (`b-a + b-b + b-c`).
