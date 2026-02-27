# HEGEL COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/FOUNDATION` (YS IV.5 - IV.6)
Status: active
Authority: TS source files (`ys_iv_05.ts`, `ys_iv_06.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Hegel baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry heg-iv-005 — The Objective-to-Subjective Logic Bridge

Span:

- sourceFile: `src/logos/ys/ys_iv_05.ts`
- lineStart: 20
- lineEnd: 24

Summary:
The dialectic between the single determining Cogito and the plurality of operations prefigures Hegel's bridge from Objective to Subjective Logic, locating an identity-in-difference engine prior to explicit conceptual syllogistics.

Key points: (KeyPoint)

- k1. _Cittam ekam_ determining _anekeṣām_ maps to Hegel's bridge into Subjective Logic (the Concept).
- k2. This represents a true dialectical identity-in-difference prior to the formalized syllogism.

Claims: (Claim)

- c1. id: heg-iv-005-c1
  - subject: ys_iv_5_dialectic
  - predicate: maps_to
  - object: hegel_objective_to_subjective_logic_bridge
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [23-23] "Hegel (Objective → Subjective Logic bridge): Identity-in-difference prior to explicit conceptual syllogism."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: heg-iv-006
  - note: The transition establishes subjective conceptual dialectic, leading to perfectly resolved logical cognition.
  - sourceClaimIds: [`heg-iv-005-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: `pending_cross_workbook`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry heg-iv-006 — Proactive Resolution and Logical Closure

Span:

- sourceFile: `src/logos/ys/ys_iv_06.ts`
- lineStart: 24
- lineEnd: 27

Summary:
The _anāśayam_ (residue-free) state translates to a formal meta-logical closure where proactive resolution (constructive identity mediation) actively prevents any unassimilated remainder or latent contradictory queue.

Key points: (KeyPoint)

- k1. Authentic dialectical _dhyāna_ is a proactive resolution, not a paraconsistent tolerance of unresolved contradictions.
- k2. Logical closure is achieved by zero latent contradiction queue (_anāśayam_).

Claims: (Claim)

- c1. id: heg-iv-006-c1
  - subject: dialectic_proactive_resolution
  - predicate: enforces
  - object: zero_latent_contradiction_queue
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [26-26] "Logic (meta): proactive resolution (constructive identity mediation) vs paraconsistent tolerance; here closure = zero latent contradiction queue."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: heg-iv-005
  - note: Clarifies that the Subjective Logic "concept" maintains its identity-in-difference rigorously; it resolves opposition entirely rather than suppressing it.
  - sourceClaimIds: [`heg-iv-006-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`heg-iv-005-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
