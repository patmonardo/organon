# Substance Part B (TopicMap) Workbook (V1)

Part: `B. THE RELATION OF CAUSALITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `../../absolute/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `relation-causality.txt` as authority for Part B.
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
- claims: (Claim) minimum 3, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-20 (initial full Part B pass)

Scope:

- file: `relation-causality.txt`
- active range: lines `1-end` (`B. THE RELATION OF CAUSALITY`)

Decision:

- Complete Part B in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `sub-part-b-001` for lines `1-147` (`a. Formal causality`)
- completed: `sub-part-b-002` for lines `148-564` (`b. The determinate relation of causality`)
- completed: `sub-part-b-003` for lines `565-780` (`c. Action and reaction`)

### Entry sub-part-b-001 — Formal causality: cause/effect identity and extinction of form-difference

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/relation-causality.txt`
- lineStart: 1
- lineEnd: 147

Summary:

Formal causality presents cause and effect as one substantial content, where cause is actual only in effect and the distinction extinguishes itself into an indifferent immediacy.

Key points: (KeyPoint)

- k1. Substance as self-referring power posits effect; cause is substance-for-itself.
- k2. In formal causality, absolute actuosity posits accident as effect while returning to itself.
- k3. Cause has actuality only in its effect; effect is necessary manifestation of cause.
- k4. Cause/effect are mutually entailing determinations with no extra content on either side.
- k5. Their form-distinction sublates itself; causality extinguishes into indifferent immediacy.

Claims: (Claim)

- c1. id: sub-part-b-001-c1
  - subject: substance_as_power
  - predicate: is
  - object: self_referring_determining_that_posits_effect_and_is_cause
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [3-11] "Substance is power... positedness... effect... substance... is... cause."

- c2. id: sub-part-b-001-c2
  - subject: formal_causal_actuosity
  - predicate: posits
  - object: effect_as_positedness_in_which_cause_manifests_whole_substance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [52-65] "Absolute actuosity is thus cause... positedness... effect... in the effect the cause is manifested as the whole substance..."

- c3. id: sub-part-b-001-c3
  - subject: cause
  - predicate: is_truly_actual_only_in
  - object: effect_as_its_necessary_manifestation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [101-106] "cause is truly actual... only in its effect... effect is therefore necessary..."

- c4. id: sub-part-b-001-c4
  - subject: cause_effect_pair
  - predicate: contains
  - object: no_extra_content_on_either_side
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [115-124] "effect contains nothing... cause does not contain... and conversely..."

- c5. id: sub-part-b-001-c5
  - subject: formal_causality
  - predicate: extinguishes_into
  - object: immediacy_indifferent_to_cause_effect_relation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [135-147] "identity of cause and effect... distinction... sublated... immediacy... indifferent..."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: sub-part-b-002
  - note: extinction of formal distinction leads to determinate causality with explicit content/form separation.
  - sourceClaimIds: [sub-part-b-001-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-part-b-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: sub-part-a-003
  - note: develops Part A's transition from substantiality into causality.
  - sourceClaimIds: [sub-part-b-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [sub-part-a-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry sub-part-b-002 — Determinate causality: finite content, external form, and bad infinity

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/relation-causality.txt`
- lineStart: 148
- lineEnd: 564

Summary:

In determinate causality, content/form are externally related, yielding tautological explanations, finite substrate determination, and the infinite regress/progression of causes and effects.

Key points: (KeyPoint)

- k1. Cause/effect identity now appears as content indifferent to form, making causality contingent and finite.
- k2. Determinate causal explanation is analytic/tautological when same content reappears as cause and effect.
- k3. Causal language is frequently misapplied in organic/spiritual domains where the recipient transfigures the cause.
- k4. Finite substrate externalizes causality, producing regress from cause to cause and progression effect to effect.
- k5. Determinate causality yields a presupposed substrate where cause/effect are both one and other, preparing reciprocal conditioning.

Claims: (Claim)

- c1. id: sub-part-b-002-c1
  - subject: determinate_causality
  - predicate: is
  - object: finite_relation_with_content_form_externality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [150-171] "self-identity... unity indifferent to differences of form... content... contingent causality... finite substance."

- c2. id: sub-part-b-002-c2
  - subject: causal_explanation_in_this_mode
  - predicate: is
  - object: analytic_tautological_repetition_of_same_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [182-198] "identity of content... causality is an analytic proposition... same fact... nothing else in cause not in effect."

- c3. id: sub-part-b-002-c3
  - subject: application_of_cause_effect_to_life_and_spirit
  - predicate: is
  - object: inadmissible_when_taken_as_undisturbed_external_causation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [284-291] "inadmissible application... living thing... sublates [the cause] as cause."
    - [339-342] "external is not the cause within spirit..."

- c4. id: sub-part-b-002-c4
  - subject: finite_causality
  - predicate: generates
  - object: infinite_regress_from_cause_to_cause_and_progression_effect_to_effect
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [443-451] "infinite regress from cause to cause..."
    - [480-486] "endless progression from effect to effect... one and the same..."

- c5. id: sub-part-b-002-c5
  - subject: movement_of_determinate_causality
  - predicate: results_in
  - object: causality_that_presupposes_or_conditions_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [553-564] "becoming-other is... own positing... Causality thus pre-supposes itself or conditions itself."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: sub-part-b-001
  - note: unfolds formal identity into finite determinate causal structure with explicit externality.
  - sourceClaimIds: [sub-part-b-002-c1, sub-part-b-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [sub-part-b-001-c4, sub-part-b-001-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: sub-part-b-003
  - note: self-conditioning causality transitions to action/reaction with active/passive substances.
  - sourceClaimIds: [sub-part-b-002-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-part-b-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: long middle section consolidated by structural theses rather than anecdotal examples.

### Entry sub-part-b-003 — Action and reaction: reciprocal causality as self-returning activity

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/relation-causality.txt`
- lineStart: 565
- lineEnd: 780

Summary:

Action/reaction posits active and passive substances as reciprocally conditioning; violence reveals passive truth, reaction sublates one-sided causality, and activity bends into infinite reciprocal self-return.

Key points: (KeyPoint)

- k1. Causality now presupposes active/passive substances as moments of one substantial activity.
- k2. Efficient cause acts on itself as other by acting on passive substance.
- k3. Passive substance both suffers determination and reacts by rejoining itself as cause.
- k4. Violence is appearance of power that manifests truth of passive positedness.
- k5. Reaction on cause bends bad infinity into infinite reciprocal action.

Claims: (Claim)

- c1. id: sub-part-b-003-c1
  - subject: causality_in_action_reaction
  - predicate: is
  - object: presupposing_activity_of_active_and_passive_substances
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [565-573] "Causality is a presupposing activity... passive substance..."
    - [591-595] "causality alone is at the origin... substrate is passive substance..."

- c2. id: sub-part-b-003-c2
  - subject: efficient_cause
  - predicate: acts_as
  - object: activity_upon_itself_as_other_in_passive_substance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [597-606] "acts upon itself as upon an other, upon the passive substance..."

- c3. id: sub-part-b-003-c3
  - subject: passive_substance
  - predicate: is
  - object: both_preserved_and_reactive_cause_through_self_rejoining
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [655-669] "passive substance... only rejoins itself... makes itself... a cause..."
    - [671-688] "reaction... displays itself as a cause..."

- c4. id: sub-part-b-003-c4
  - subject: violence
  - predicate: is
  - object: appearance_of_power_that_posits_passive_substance_in_its_truth
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [630-638] "Violence is the appearance of power..."
    - [649-651] "through violence... passive substance... posited as what it is in truth..."

- c5. id: sub-part-b-003-c5
  - subject: conditioned_causality
  - predicate: culminates_in
  - object: infinite_reciprocal_action_returning_to_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [747-756] "in conditioned causality... refers back to itself in the effect..."
    - [774-780] "activity... is bent around and becomes an action that returns to itself, an infinite reciprocal action."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: sub-part-b-002
  - note: overcomes external regress/progression by reciprocal self-returning activity.
  - sourceClaimIds: [sub-part-b-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-part-b-002-c4, sub-part-b-002-c5]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: sub-part-c-001
  - note: infinite reciprocal action hands off to explicit reciprocity chapter determination.
  - sourceClaimIds: [sub-part-b-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-part-c-001-c1, sub-part-c-001-c5]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: refines
  - targetEntryId: sub-part-b-001
  - note: realizes formal cause/effect identity as reciprocal conditioning of active/passive substantiality.
  - sourceClaimIds: [sub-part-b-003-c1, sub-part-b-003-c3]
  - sourceKeyPointIds: [k1, k3]
  - targetClaimIds: [sub-part-b-001-c3, sub-part-b-001-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part C handoff claim targets are now resolved.
