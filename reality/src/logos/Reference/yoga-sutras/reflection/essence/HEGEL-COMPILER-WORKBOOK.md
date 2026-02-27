# HEGEL COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/ESSENCE` (YS IV.2 - IV.4)
Status: active
Authority: TS source files (`ys_iv_02.ts`, `ys_iv_03.ts`, `ys_iv_04.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Hegel baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry heg-iv-002 — Essence to Actuality Form Transition

Span:

- sourceFile: `src/logos/ys/ys_iv_02.ts`
- lineStart: 18
- lineEnd: 20

Summary:
The _jāti-antara-pariṇāma_ (threshold transition through saturation) corresponds to Hegel's logic of Essence moving toward Actuality, where the completion of determinate background conditions yields a leap into a new concrete form.

Key points: (KeyPoint)

- k1. Saturation of the given matrix forces a dialectical shift.
- k2. The fulfillment of conditions is not just a collection, but constitutes a concrete formal change.

Claims: (Claim)

- c1. id: heg-iv-002-c1
  - subject: completion_of_determinate_conditions
  - predicate: yields_transition_to
  - object: new_concrete_form
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [20-20] "Hegel (Essence → Actuality): completion of determinate conditions yields transition to a new concrete form."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: heg-iv-003
  - note: This leap into form sets up how mediation operates in sutra 3.
  - sourceClaimIds: [`heg-iv-002-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: `pending_cross_workbook`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry heg-iv-003 — Pure Mediation and the Removal of Opacity

Span:

- sourceFile: `src/logos/ys/ys_iv_03.ts`
- lineStart: 21
- lineEnd: 50

Summary:
The non-impelling, negative catalysis of _nimitta_ aligns with Hegelian Appearance, where essence emerges strictly through removing the opacity of immediacy—a process of pure mediation as removal.

Key points: (KeyPoint)

- k1. Essence does not create Appearance ex-nihilo; it emerges by dissolving the contradictory barricade of immediate Being.
- k2. The farmer opening the sluice acts as "pure mediation."

Claims: (Claim)

- c1. id: heg-iv-003-c1
  - subject: hegel_mediation_as_removal
  - predicate: aligns_with
  - object: obstruction_clearing_varana_bheda
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [22-22] "Hegel (Essence → Appearance): essence emerges through removing immediacy’s opacity; the 'farmer' figure = pure mediation-as-removal (relieving contradiction-barricade)."
    - [50-50] "crosswalkHegelMediation: Mapping: pure mediation as removal letting essence appear"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: heg-iv-002
  - note: The mechanism allowing transition to concrete form (sutra 2) is exactly this pure mediation (sutra 3).
  - sourceClaimIds: [`heg-iv-003-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`heg-iv-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

### Entry heg-iv-004 — Pre-logical Identity Field Modulation

Span:

- sourceFile: `src/logos/ys/ys_iv_04.ts`
- lineStart: 23
- lineEnd: 95

Summary:
While the Fichte crosswalk is emphasized in the source TS, the Hegelian correlate is the derivation of the logical stratum (_cittas_) from a single modulated pre-logical parameter (_asmitā_), where the secondary formalization splits internal differentiation into an Objective Logic.

Key points: (KeyPoint)

- k1. Logic is a secondary formalization (a tamasic shadow) of primary identity modulation.
- k2. Plural forms emerge from one substantive base identity.

Claims: (Claim)

- c1. id: heg-iv-004-c1
  - subject: logical_derivative_stratum
  - predicate: operates_after
  - object: pre_logical_identity_field_modulation
  - modality: inferred
  - confidence: 0.85
  - evidence:
    - [24-24] "logicalDerivativeStratum: Logic as secondary formalization of already-modulated identity patterns"
    - [93-94] "logicalDerivativeStratum := after(reflectiveShadowLogic(nirmanaSetPatterns))"

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: heg-iv-003
  - note: The appearance of multiple logical modes presupposes the mediating removal of the absolute barrier.
  - sourceClaimIds: [`heg-iv-004-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`heg-iv-003-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: second_order_inference
