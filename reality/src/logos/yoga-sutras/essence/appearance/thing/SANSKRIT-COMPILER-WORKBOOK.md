# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `APPEARANCE/THING` (YS IV.11 - IV.14)
Status: active
Authority: TS source files (`ys_iv_11.ts` through `ys_iv_14.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry ys-iv-011 — The Facticity of Appearance (Absence Rule)

Span:

- sourceFile: `src/logos/ys/ys_iv_11.ts`
- lineStart: 4
- lineEnd: 24

Summary:
Because all such phenomena are comprehensively encompassed (_saṅgṛhītatvāt_) by a dependency complex comprising cause (_hetu_), result (_phala_), substrate (_āśraya_), and object-contact (_ālambana_), the absence or disablement of any required member guarantees the non-arising (_abhāva_) of the phenomena.

Key points: (KeyPoint)

- k1. The dependency set $D = \{hetu, phala, āśraya, ālambana\}$ acts as the absolute structural container (facticity) for appearance.
- k2. The "Absence Rule" states that if any member of $D$ is removed, the target phenomenon cannot arise (dependent origination).
- k3. _Asparśa-yoga_ (the yoga of non-contact) functions essentially by setting _ālambana_ (object-contact) to null, thus cleanly invoking the absence rule.

Claims: (Claim)

- c1. id: ys-iv-011-c1
  - subject: phenomenon_arising
  - predicate: requires
  - object: complete_dependency_complex
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [21-22] "What appears (the target phenomena) is factically encompassed by {hetu, phala, āśraya, ālambana}. Remove any required member and the phenomena do not arise."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-012
  - note: Serves as the definitive entry point into Appearance mapping out structural constraints governing time, states, and things.
  - sourceClaimIds: [`ys-iv-011-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: `pending`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-012 — Time as Mode of Appearance

Span:

- sourceFile: `src/logos/ys/ys_iv_12.ts`
- lineStart: 4
- lineEnd: 43

Summary:
Past and future (_atīta-anāgatam_) exist in/by their own form (_svarūpataḥ asti_); time manifests structurally because the _dharmas_ (determinacies of appearance) differ dynamically across temporal paths (_adhva-bhedād dharmāṇām_).

Key points: (KeyPoint)

- k1. Time marks a mode of qualitative appearance, not an external geometric container.
- k2. Existence (_asti_) here is defined firmly as form-existence (_svarūpa_), refuting naive presentist claims that past/future possess zero existential weight.
- k3. The term _svarūpataḥ_ harbors a debate: reading it impersonally ("in its own form") verses reflexively via the seer ("in my own form").

Claims: (Claim)

- c1. id: ys-iv-012-c1
  - subject: past_and_future_modalities
  - predicate: exist_as
  - object: form_existence_svarupatah
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [34-35] "Existence here = existence-as-form (svarūpa), not presence-now"

- c2. id: ys-iv-012-c2
  - subject: time_structure
  - predicate: is_derived_from
  - object: temporal_path_difference_adhva_bheda
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [36-37] "Past/future exist as forms because dharmas differ by temporal path (adhva-bheda)"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-011
  - note: Validates how facticity parameters apply selectively across differentiated temporal vectors.
  - sourceClaimIds: [`ys-iv-012-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-011-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

### Entry ys-iv-013 — The Substrate of the State-Spectrum

Span:

- sourceFile: `src/logos/ys/ys_iv_13.ts`
- lineStart: 4
- lineEnd: 30

Summary:
The _dharmas_—whether manifest (_vyakta_) or subtle (_sūkṣma_)—are fundamentally of the nature of the _guṇas_ (_guṇātmānaḥ_).

Key points: (KeyPoint)

- k1. The _dharmas_ occupy a continuous state-spectrum ranging from latency (subtle) to presentation (manifest).
- k2. This entire spectrum is grounded completely in the _guṇas_ (the constitutive factors of appearance: sattva/rajas/tamas).
- k3. Changes in temporal path (IV.12) are physically handled by shifting the ratio configurations of these substratic components.

Claims: (Claim)

- c1. id: ys-iv-013-c1
  - subject: dharmas_state_spectrum
  - predicate: is_grounded_in
  - object: guna_nature_gunatmanah
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [17-18] "Dharmas are “of the nature of guṇas” (guṇātmānaḥ)... Continuum: sūkṣma ↔ vyakta as function of guṇa-configuration"

Relations: (Relation)

- r1. type: instantiates
  - targetEntryId: ys-iv-012
  - note: Provides the material substratic mechanism (guṇas) that makes temporal path difference physically operational.
  - sourceClaimIds: [`ys-iv-013-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-012-c2`]
  - logicalOperator: mechanism_specification
  - analysisMode: first_order_claim_projection

### Entry ys-iv-014 — Thingness as Identity-in-Transformation

Span:

- sourceFile: `src/logos/ys/ys_iv_14.ts`
- lineStart: 4
- lineEnd: 42

Summary:
The valid truth of a "thing" (_vastu-tattvam_) derives directly from the singular oneness (_ekatvād_) persisting across its processual transformations (_pariṇāma_).

Key points: (KeyPoint)

- k1. _Pariṇāma_ signifies a "change-of-name" protocol within appearance, where the underlying substance presents sequentially through different states.
- k2. _Ekatvam_ signifies the mathematical unity that survives this continuous process.
- k3. _Vastu-tattvam_ (thing-reality) is not a static object posited behind appearance; it represents the stabilized appearance guaranteed by this invariant continuity.

Claims: (Claim)

- c1. id: ys-iv-014-c1
  - subject: truth_of_a_thing_vastu_tattvam
  - predicate: is_secured_by
  - object: oneness_of_transformation_parinama_ekatva
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [18-19] "Oneness of transformation: identity-across-change... Identity across change (ekatva of pariṇāma) grounds thingness/truth (vastu-tattvam) within appearance."

Relations: (Relation)

- r1. type: synthesizes
  - targetEntryId: ys-iv-013
  - note: Synthesizes the guṇic shifts (IV.13) driven by time-vectors (IV.12) into a coherent, lawful identity securing the appearance of "an object."
  - sourceClaimIds: [`ys-iv-014-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-013-c1`, `ys-iv-012-c1`]
  - logicalOperator: synthetic_conclusion
  - analysisMode: first_order_claim_projection
