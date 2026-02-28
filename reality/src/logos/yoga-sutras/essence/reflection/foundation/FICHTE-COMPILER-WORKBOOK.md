# FICHTE COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/FOUNDATION` (YS IV.5 - IV.6)
Status: active
Authority: TS source files (`ys_iv_05.ts`, `ys_iv_06.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Fichte baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry fic-iv-005 — Absolute Insight and Genesis-of-Genesis

Span:

- sourceFile: `src/logos/ys/ys_iv_05.ts`
- lineStart: 161
- lineEnd: 175

Summary:
The identity-difference engine corresponds to Fichte's "Absolute Insight" where ideal construction is organically posited within essence without a real hiatus; consciousness proper operates as a second-order regeneration (genesis-of-genesis).

Key points: (KeyPoint)

- k1. Absolute insight posits ideal construction entirely inside essence, denying any real internal gap.
- k2. The "how" of self-construction remains opaque, which paradoxically secures its absoluteness.
- k3. A gap or hiatus exists only relative to the observer ("We").
- k4. Plurality of operations is still governed by the single absolute insight.

Claims: (Claim)

- c1. id: fic-iv-005-c1
  - subject: absolute_insight
  - predicate: is_posited_as
  - object: ideal_construction_organically_within_essence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [163-163] "absoluteInsight: Insight positing ideal construction organically within essence (no hiatus)"
    - [165-165] "noHiatusPrinciple: Denial of any real gap inside essence during self-construction"

- c2. id: fic-iv-005-c2
  - subject: gap_or_hiatus
  - predicate: subsists_only_for
  - object: observer_standpoint_We
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [169-169] "gapForWeOnly: Apparent gap subsists only relative to the We (observer standpoint)"

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: fic-iv-006
  - note: The apparent "gap" sets up the mechanism for the projection act and conditional normativity discussed next.
  - sourceClaimIds: [`fic-iv-005-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: `pending_cross_workbook`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry fic-iv-006 — The Projection Act and the Hypothetical Should

Span:

- sourceFile: `src/logos/ys/ys_iv_06.ts`
- lineStart: 168
- lineEnd: 302

Summary:
Residue-free cognition correlates with knowing the "projection act" itself, bridging the irrational gap; furthermore, the absolutely categorical normative requirement ("should") necessarily appears in the conditioned form of a hypothesis ("if").

Key points: (KeyPoint)

- k1. The irrational gap is an unprincipled aperture that turns intrinsic genesis into a factical semblance.
- k2. After annulment of factical being, only the pure projection act remains.
- k3. Dialectic cognition prevents contradiction (residue) by being aware of this projection as an act.
- k4. An absolutely categorical "should" (normative ground) appears only as a hypothetical ("if... should").

Claims: (Claim)

- c1. id: fic-iv-006-c1
  - subject: awareness_of_projection_act
  - predicate: prevents
  - object: latent_contradiction_storage_asaya
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [182-182] "contradictionPreclusionMechanism: Mechanism: awareness-of-projection prevents latent contradiction storage"

- c2. id: fic-iv-006-c2
  - subject: absolute_categorical_should
  - predicate: appears_as
  - object: hypothetical_antecedent
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [293-293] "categoricalAsHypothetical: Phenomenon: categoricalShould appears only as hypothetical antecedent (“if…should…”)"
    - [294-294] "appearanceProcessChiefPrinciple: Law: process of appearance = categoricalShould presenting as hypothetical"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: fic-iv-005
  - note: Clarifies how the single absolute insight prevents error/residue—by recognizing the hypothetical/categorical dynamic inside the "gap".
  - sourceClaimIds: [`fic-iv-006-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`fic-iv-005-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
