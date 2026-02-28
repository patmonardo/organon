# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `ACTUALITY/ABSOLUTE` (YS IV.22 - IV.26)
Status: active
Authority: TS source files (`ys_iv_22.ts` through `ys_iv_26.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry ys-iv-022 — Actuality as Unity-in-Act

Span:

- sourceFile: `src/logos/ys/ys_iv_22.ts`
- lineStart: 6
- lineEnd: 53

Summary:
When the _buddhi_ takes on the form of the object (_tad-ākārāpattau_), and because consciousness (_citi_) does not transfer into the object (_apratisaṅkramāyās_), there arises the self-cognition of one's own _buddhi_ (_sva-buddhi-saṁvedanam_).

Key points: (KeyPoint)

- k1. The Witness (_Puruṣa_ / _citi_) never migrates or transfers (_apratisaṅkrama_) into the objects it illuminates. It remains Seer-only.
- k2. The mind (_buddhi_) physically assumes the form (_ākāra_) of the object it contemplates via _sattva-reflection_.
- k3. The self-awareness of the mind is not an innate property (it is not self-luminous, per IV.19), but an _event_—a "Unity-in-Act" comprising the invariant light of the Witness and the formal shape taken by the mind.

Claims: (Claim)

- c1. id: ys-iv-022-c1
  - subject: self_cognition_of_mind_sva_buddhi_samvedanam
  - predicate: arises_from
  - object: assumption_of_form_under_non_transference
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [29-30] "Actuality = unity-in-act of essence (witness invariance) and appearance (form-taking) without identity of roles"

Relations: (Relation)

- r1. type: synthesizes
  - targetEntryId: ys-iv-024
  - note: Having established how the mind physically registers form, we now analyze the metaphysical purpose (teleology) of that form-taking mechanism in IV.24.
  - sourceClaimIds: [`ys-iv-022-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: `pending`
  - logicalOperator: structural_progression
  - analysisMode: first_order_claim_projection

### Entry ys-iv-024 — The Instrumentality of the Mind (Being-For-Another)

Span:

- sourceFile: `src/logos/ys/ys_iv_24.ts`
- lineStart: 6
- lineEnd: 49

Summary:
Although the mind is highly variegated by countless latent impressions (_asaṁkhyeya-vāsanābhi citram_), it ultimately exists "for another" (_parārtham_), because it functions as a composite aggregate (_saṁhatyā-kāritvāt_).

Key points: (KeyPoint)

- k1. The mind is a machine composed of moving parts (_saṁhati_—conjunction/aggregation) and colored by infinite karmic software (_vāsanās_).
- k2. In Sāṃkhya-Yoga logic, anything that is composite/aggregated cannot be the ultimate Subject. Its existence is necessarily teleological—it serves a higher, non-composite master.
- k3. Therefore, the mind is strictly _parārtham_ ("for another"). It is an instrument constructed for the sake of the Seer (_Puruṣa_). It has no "self-purpose."

Claims: (Claim)

- c1. id: ys-iv-024-c1
  - subject: the_composite_mind
  - predicate: exists_strictly_for
  - object: another_parartham_the_seer
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [25-26] "Citta’s purpose is for puruṣa; not for itself... As composite, citta is not the ultimate subject (no self-end)"

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: ys-iv-025
  - note: Once the mind is definitively recognized as merely an instrument "for another," the false identification with it (the "selfing" process) must logically disintegrate in IV.25.
  - sourceClaimIds: [`ys-iv-024-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: `pending`
  - logicalOperator: deductive_necessary_condition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-025 — The Cessation of Selfing (De-Appropriation)

Span:

- sourceFile: `src/logos/ys/ys_iv_25.ts`
- lineStart: 6
- lineEnd: 42

Summary:
For the practitioner who clearly sees this structural distinction (_viśeṣa-darśina_), there is a total cessation of the cultivation of "self-being" within the mind (_ātma-bhāva-bhāvanā-vinivṛtti_).

Key points: (KeyPoint)

- k1. Discriminative vision (_viśeṣa-darśana_) is the exact realization of IV.22 and IV.24: seeing the divide between the non-transferring Light and the aggregated, form-taking mind.
- k2. When this geometry is grasped, the instinct to map "I" onto the mind's operations (_ātma-bhāva_) is structurally broken.
- k3. This is not ascetic suppression; it is epistemic _de-appropriation_. The mind continues to function, but it is no longer misidentified as the Absolute Subject.

Claims: (Claim)

- c1. id: ys-iv-025-c1
  - subject: discriminative_vision_visesha_darsina
  - predicate: results_in
  - object: cessation_of_self_cultivation_vinivrtti
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [19-20] "Non-appropriation: buddhi’s operations not taken as 'I' (end of selfing)"

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-026
  - note: With the psychological anchor of "selfing" destroyed, IV.26 measures the subsequent gravitational shift of the mind's entire vector.
  - sourceClaimIds: [`ys-iv-025-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: `pending`
  - logicalOperator: sequential_necessity
  - analysisMode: first_order_claim_projection

### Entry ys-iv-026 — The Gravitational Shift toward Kaivalya

Span:

- sourceFile: `src/logos/ys/ys_iv_26.ts`
- lineStart: 6
- lineEnd: 34

Summary:
Then (_tadā_), the mind, now sloping downhill toward discrimination (_viveka-nimnam_), bears the heavy forward-weight toward total isolation/liberation (_kaivalya-prāg-bhāram_).

Key points: (KeyPoint)

- k1. The cessation of "selfing" (IV.25) alters the fundamental teleological vector of the mind.
- k2. _Viveka-nimnam_ indicates that discrimination is no longer an uphill battle; it has become the dominant, gravitational current of the mind.
- k3. The mind enters a "pre-liberative" state (_prāg-bhāra_), where its entire momentum is geared toward _kaivalya_—the absolute isolation of the Seer from the seen.

Claims: (Claim)

- c1. id: ys-iv-026-c1
  - subject: post_realization_mind_citta
  - predicate: enters_gravitational_state_of
  - object: discriminative_weight_toward_liberation_kaivalya
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [20-21] "Pre-liberative mind: instrument aligned 'for another,' trending to isolation"

Relations: (Relation)

- r1. type: finalizes
  - targetEntryId: ys-iv-025
  - note: IV.26 is the physical/teleological consequence of the epistemic realization described in IV.25.
  - sourceClaimIds: [`ys-iv-026-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`ys-iv-025-c1`]
  - logicalOperator: synthetic_conclusion
  - analysisMode: first_order_claim_projection
