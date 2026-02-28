# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `APPEARANCE/WORLD` (YS IV.15 - IV.18)
Status: active
Authority: TS source files (`ys_iv_15.ts` through `ys_iv_18.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry ys-iv-015 — Invariants and Variants of Worldness

Span:

- sourceFile: `src/logos/ys/ys_iv_15.ts`
- lineStart: 4
- lineEnd: 31

Summary:
Even when the object remains the same (_vastu-sāmye_), the pathways of appearance are divided (_vibhaktaḥ panthāḥ_) due to the structural differences across minds (_citta-bhedāt_).

Key points: (KeyPoint)

- k1. Worldness requires an object (_vastu_) to hold mathematically constant across discrete subjective frames.
- k2. Although the object is constant, the appearance matrix splits into variant perspectival paths (_panthāḥ_) parametrized by differing mind-structures (_citta-bheda_).
- k3. The Law of Appearance mandates extracting structural invariants (world-regularities) distinct from the variants introduced by specific _citta_ conditioning.

Claims: (Claim)

- c1. id: ys-iv-015-c1
  - subject: pathways_of_appearance_panthah
  - predicate: diverge_based_on
  - object: mental_difference_citta_bheda
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [20-20] "Rule: appear(vastu | citta) yields divergent paths with identifiable invariants and variants"

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-016
  - note: Having established that multiple minds view the same object differently, IV.16 must address whether the object is merely a fabrication of a single mind.
  - sourceClaimIds: [`ys-iv-015-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: `pending`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-016 — Refutation of Single-Mind Dependence

Span:

- sourceFile: `src/logos/ys/ys_iv_16.ts`
- lineStart: 4
- lineEnd: 32

Summary:
The object is absolutely not dependent on a single mind (_na ca eka-citta-tantraṁ vastu_); if it were, and that mind's attention lapsed (leaving it unvalidated, _apramāṇakam_), what would happen to it (_tadā kim syāt_)?

Key points: (KeyPoint)

- k1. The sutra explicitly refutes solipsistic idealism (the claim that a single mind generates the object).
- k2. The rhetorical question mathematically isolates the contradiction: if an object depended wholly on _Citta A_, it would vanish when _Citta A_ looks away, violating the shared invariants established in IV.15.
- k3. This establishes the "Essential Relation"—the objective worldness validated by invariants across multiple _cittas_, rendering science possible.

Claims: (Claim)

- c1. id: ys-iv-016-c1
  - subject: object_vastu
  - predicate: is_not_dependent_on
  - object: single_mind_eka_citta_tantram
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [36-36] "Science requires intersubjective invariants; denies eka-citta-tantram"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-015
  - note: Defends the ontological stability of the _vastu_ required by IV.15 against constructivist reduction.
  - sourceClaimIds: [`ys-iv-016-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`ys-iv-015-c1`]
  - logicalOperator: falsification_guard
  - analysisMode: first_order_claim_projection

### Entry ys-iv-017 — The Epistemic Gating of Knownness

Span:

- sourceFile: `src/logos/ys/ys_iv_17.ts`
- lineStart: 4
- lineEnd: 42

Summary:
For any specific mind, the object is either known or unknown (_cittasya vastu jñāta-ajñātam_) depending entirely upon the specific coloring/overlap (_tad-upārāga-apekṣitvāt_) between the mind and the object.

Key points: (KeyPoint)

- k1. _Upārāga_ functions as an epistemic conditional gate (contact/attunement); it controls knownness, not existence.
- k2. This resolves the tension between IV.15 and IV.16: the object exists independently of the single mind (IV.16), but its _status as known_ specifically requires that mind's structural alignment (IV.17).
- k3. Confusing "unknown-for-this-citta" with "non-existent" is a severe ontological error.

Claims: (Claim)

- c1. id: ys-iv-017-c1
  - subject: knownness_jnata
  - predicate: is_conditioned_by
  - object: mind_object_overlap_uparaga
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [24-25] "Known/unknown status of vastu is a function of citta’s upārāga (not an ontic toggle on vastu)"

Relations: (Relation)

- r1. type: synthesizes
  - targetEntryId: ys-iv-018
  - note: Evaluates how the conditional knownness in the _citta_ implies a higher unconditioned necessity of knowing to anchor the entire field.
  - sourceClaimIds: [`ys-iv-017-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: `pending`
  - logicalOperator: synthetic_conclusion
  - analysisMode: first_order_claim_projection

### Entry ys-iv-018 — The Invariance of the Witness (Puruṣa)

Span:

- sourceFile: `src/logos/ys/ys_iv_18.ts`
- lineStart: 4
- lineEnd: 55

Summary:
The fluctuating modifications of the mind are always known (_sadā jñātā citta-vṛttayaḥ_) because their Lord/Witness—the Puruṣa—is absolutely unchanging (_tat-prabhoḥ puruṣasya apariṇāmitvāt_).

Key points: (KeyPoint)

- k1. Unlike the _citta_, whose knowing toggles on and off based on _upārāga_ (IV.17), the Puruṣa (the deep witness) maintains an "always-known" (_sadā jñātā_) status relative to its _citta_.
- k2. This permanence guarantees the coherence of the Essential Relation: appearance requires an invariant pole (_apariṇāmitvāt_) to register its own changes.
- k3. The Puruṣa functions structurally as the non-agentive illuminator (_akartatva_), not as a dominating empirical master.

Claims: (Claim)

- c1. id: ys-iv-018-c1
  - subject: mental_modifications_citta_vrittis
  - predicate: are_always_manifest_to
  - object: unchanging_witness_purusha
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [30-31] "All citta-vṛttis are manifest to Purusha without alteration of Purusha"

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: ys-iv-017
  - note: Provides the absolute invariant stance (always-knowing) that grounds the fluid epistemic states (known/unknown) of the conditional mind.
  - sourceClaimIds: [`ys-iv-018-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-017-c1`]
  - logicalOperator: transcendental_ground
  - analysisMode: first_order_claim_projection
