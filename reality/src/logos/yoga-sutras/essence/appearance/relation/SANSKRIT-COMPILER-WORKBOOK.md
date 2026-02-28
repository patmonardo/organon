# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `APPEARANCE/RELATION` (YS IV.19 - IV.21)
Status: active
Authority: TS source files (`ys_iv_19.ts` through `ys_iv_21.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry ys-iv-019 — The Mind is an Object, Not the Light

Span:

- sourceFile: `src/logos/ys/ys_iv_19.ts`
- lineStart: 4
- lineEnd: 24

Summary:
The mind (_citta_) is fundamentally not self-luminous (_na tat sva-ābhāsam_) precisely because it is an object that is seen (_dṛśyatvāt_).

Key points: (KeyPoint)

- k1. The _citta_ is not the ultimate seer; it is the seen. It appears by borrowed light (_sāttvika_ reflection) originating from the invariant Witness (_Puruṣa_).
- k2. Claiming the _citta_ is self-luminous entails an ontological category error, confusing the instrument of appearance with the illuminating ground.
- k3. This introduces the strict seer/seen divide within the Essential Relation, preventing reflexive regress.

Claims: (Claim)

- c1. id: ys-iv-019-c1
  - subject: mind_citta
  - predicate: is_qualified_as
  - object: not_self_luminous_and_seen
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [18-19] "Citta is not self-luminous (na tat sva-ābhāsam)... Because of being seen (object-status)"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-018
  - note: Clarifies the relationship established in IV.18 by defining the _citta_ structurally as an illuminated object, ensuring the Puruṣa remains the sole non-agentive Light.
  - sourceClaimIds: [`ys-iv-019-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: `pending`
  - logicalOperator: structural_differentiation
  - analysisMode: first_order_claim_projection

### Entry ys-iv-020 — The Prohibition of Double-Determination

Span:

- sourceFile: `src/logos/ys/ys_iv_20.ts`
- lineStart: 4
- lineEnd: 29

Summary:
Within a single cognitive instant (_eka-samaye_), it is impossible to determine/fixate upon both the seer and the seen simultaneously (_ca-ubhaya-anavadhāraṇam_).

Key points: (KeyPoint)

- k1. A single act of knowing cannot simultaneously hold itself as both the subjective agent and the objective target without collapsing its own structure.
- k2. This functions as a rigorous regress guard, proving physically why the mind (IV.19) cannot be self-illuminating.
- k3. It biologically prepares the necessity of the _Puruṣa_ as a "Seer without Objecthood."

Claims: (Claim)

- c1. id: ys-iv-020-c1
  - subject: single_cognitive_act_eka_samaya
  - predicate: cannot_simultaneously_determine
  - object: both_seer_and_seen_ubhaya
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [20-21] "A single act cannot simultaneously fix both knower and known"

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ys-iv-019
  - note: Provides the mechanical/logical proof for why IV.19's assertion (that the mind is not self-luminous) holds true.
  - sourceClaimIds: [`ys-iv-020-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-019-c1`]
  - logicalOperator: deductive_proof
  - analysisMode: first_order_claim_projection

### Entry ys-iv-021 — The Refutation of Mind-on-Mind Transparency

Span:

- sourceFile: `src/logos/ys/ys_iv_21.ts`
- lineStart: 4
- lineEnd: 31

Summary:
If one mind were seen directly by another mind (_citta-antara-dṛśye_), there would be an absurd infinite regress of intellects observing intellects (_buddhi-buddher atiprasaṅgaḥ_) and a chaotic commingling of memory streams (_smṛti-saṅkaraś ca_).

Key points: (KeyPoint)

- k1. Minds are not transparent to one another directly. The "Worldness" mediation established earlier (IV.15-16) requires intersubjective overlap via objects (_vastu_), not brute telepathy.
- k2. A hypothetical stack of "minds seeing minds" generates an infinite regress (_atiprasaṅga_) destroying causality.
- k3. It would also erase distinct personal identities, causing memory streams to catastrophically fuse (_smṛti-saṅkara_).
- k4. This finalizes the logic locking the _Puruṣa_ as the sole non-agentive Witness of all _cittas_, while the _cittas_ themselves remain mutually opaque observers of the Appearance layer.

Claims: (Claim)

- c1. id: ys-iv-021-c1
  - subject: direct_mind_on_mind_observation
  - predicate: generates
  - object: absurd_regress_and_memory_fusion
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [23-25] "Unwanted overextension/absurd regress (infinite layering of minds seeing minds)... Intermixing/confusion of memory streams across minds"

Relations: (Relation)

- r1. type: synthesizes
  - targetEntryId: ys-iv-020
  - note: Finalizes the boundary conditions of the Essential Relation (Appearance). Knowing must be mediated; there is no unmediated absolute knowing within the horizontal plane of _cittas_.
  - sourceClaimIds: [`ys-iv-021-c1`]
  - sourceKeyPointIds: [`k1`, `k4`]
  - targetClaimIds: [`ys-iv-020-c1`, `ys-iv-019-c1`]
  - logicalOperator: boundary_condition_lock
  - analysisMode: first_order_claim_projection
