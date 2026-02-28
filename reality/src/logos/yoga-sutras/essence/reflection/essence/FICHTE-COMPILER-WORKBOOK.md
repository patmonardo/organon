# FICHTE COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/ESSENCE` (YS IV.2 - IV.4)
Status: active
Authority: TS source files (`ys_iv_02.ts`, `ys_iv_03.ts`, `ys_iv_04.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Fichte baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry fic-iv-002 — Ladder Escalation and Absolute Product Thesis

Span:

- sourceFile: `src/logos/ys/ys_iv_02.ts`
- lineStart: 18
- lineEnd: 195

Summary:
The _jāti-antara_ saturation event parallels Fichte's ladder escalation where principles saturate and elevate; furthermore, the identity (I=I) is not a primitive axiom but a digested absolute product of prior genesis.

Key points: (KeyPoint)

- k1. Saturation triggers a Fichtean ladder elevation to a higher unity.
- k2. The Absolute Product Thesis insists that the identity (I=I) emerges as a result, not a starting premise.
- k3. The conditioned middle operates between pure and impure unconditioned poles.

Claims: (Claim)

- c1. id: fic-iv-002-c1
  - subject: fichtean_ladder_elevation
  - predicate: is_triggered_by
  - object: internal_differentiation_saturation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [18-19] "Fichte (Doctrine of Essence): ladder escalation when a principle’s internal differentiation saturates, forcing elevation to a higher unity."

- c2. id: fic-iv-002-c2
  - subject: fichte_absolute_product_thesis
  - predicate: asserts
  - object: identity_I_equals_I_emerges_as_result
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [157-157] "Fichte 1804: identity (I=I) emerges as result, not starting principle"

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: fichte-primitive-axiom-dogma
  - note: Positing I=I as a primitive axiom is sublated by acknowledging its genesis (from saturation).
  - sourceClaimIds: [`fic-iv-002-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: `pending_cross_workbook`
  - logicalOperator: contrastive_negation
  - analysisMode: first_order_claim_projection

### Entry fic-iv-003 — Internal Self-Construction and Limit Removal

Span:

- sourceFile: `src/logos/ys/ys_iv_03.ts`
- lineStart: 21
- lineEnd: 271

Summary:
Mirroring Patañjali's non-impelling _citi_ that operates by removing obstructions, Fichte establishes that reflection cancels limits to reveal ground, and that Being constructs itself internally without an external productive agency (Lecture 16).

Key points: (KeyPoint)

- k1. Reflection does not create content; it cancels a limit to let inner articulation appear.
- k2. Being constructs itself; there is no external idealistic constructor-agent.
- k3. Misprojecting an outside cause violates the self-enclosed constraint of Being.

Claims: (Claim)

- c1. id: fic-iv-003-c1
  - subject: fichte_reflection
  - predicate: functions_as
  - object: limit_removal_revealing_ground
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [21-21] "Fichte (Doctrine of Essence / Reflection): reflection does not create content but cancels a limit to let inner self-articulation appear"

- c2. id: fic-iv-003-c2
  - subject: internal_self_construction
  - predicate: rejects
  - object: external_idealist_inference_of_construction
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [171-174] "Being constructs itself; no external constructing agency... We act only insofar as we are identical with being"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: fic-iv-002
  - note: Connects the Fichtean "ladder elevation" internally; it is not pushed by an ego, but released by limit removal.
  - sourceClaimIds: [`fic-iv-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`fic-iv-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

### Entry fic-iv-004 — Constructed Being and the Asmitā Parameter

Span:

- sourceFile: `src/logos/ys/ys_iv_04.ts`
- lineStart: 23
- lineEnd: 220

Summary:
_Asmitā-mātra_ maps to Fichte's concept of immediate self-construction, establishing a boundary between idealistic imaginal positing (hypothetical) and real intrinsic active being (esse = self-construction).

Key points: (KeyPoint)

- k1. Patañjali's constructed cittas based on pure _asmitā_ align with Fichte's self-modulating Being.
- k2. Fichte cautions against confusing "idealistic imaginal construction" (reflective assumption) with "real immediate self-construction."
- k3. The analytic/synthetic principle requires strict genetic deduction to separate mere hypothesis from intrinsic reality.

Claims: (Claim)

- c1. id: fic-iv-004-c1
  - subject: asmita_matra_self_construction
  - predicate: aligns_with
  - object: fichte_pre_logical_identity_field
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [23-23] "Crosswalk: self-construction = asmitā-driven modulation"

- c2. id: fic-iv-004-c2
  - subject: real_immediate_self_construction
  - predicate: is_defined_as
  - object: esse_is_self_construction
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [116-117] "Idealistic Imaginal Construction: Merely ideal / imaginal... Real Immediate Self-Construction: Intrinsic esse = self-construction: being is only in constructing itself"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: fic-iv-003
  - note: Clarifies that the internal self-construction of lecture 16 must not be mistaken for a mere imaginal hypothesis; the "I-ness" parameter is real.
  - sourceClaimIds: [`fic-iv-004-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`fic-iv-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
