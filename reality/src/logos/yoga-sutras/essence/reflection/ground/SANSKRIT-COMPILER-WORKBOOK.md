# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/GROUND` (YS IV.7 - IV.11)
Status: active
Authority: TS source files (`ys_iv_07.ts` through `ys_iv_11.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry ys-iv-007 — The Colorless Ground

Span:

- sourceFile: `src/logos/ys/ys_iv_07.ts`
- lineStart: 4
- lineEnd: 28

Summary:
For the yogin who has achieved residue-free cognition, iterative causal throughput (_karma_) becomes colorless (_aśukla-akṛṣṇam_), acting as a pure, neutral ground-function. For non-integrated standpoints (_itareṣām_), unassimilated residues differentiate the causal stream into a triplex typology (white, black, mixed).

Key points: (KeyPoint)

- k1. _Aśukla-akṛṣṇam_ (neither white nor black) indicates a double qualitative privation, signifying an uncolored karmic ground.
- k2. This colorless ground requires the preceding contradiction-free closure established in IV.6 (_anāśayam_).
- k3. For "others" (_itareṣām_), a residual color channel maps causal operations into an active triadic polarity schema (white, black, or mixed synthesis).

Claims: (Claim)

- c1. id: ys-iv-007-c1
  - subject: karma_throughput
  - predicate: is_qualified_as
  - object: colorless_unpolar_ground
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [15-16] "For the yogin, 'karma' = colorless ground-function... removing latent seed-coloration."

- c2. id: ys-iv-007-c2
  - subject: unresolved_residue
  - predicate: generates
  - object: triplex_karma_typing
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [17-19] "For others, the causal throughput appears triple-typed (trividham) due to unresolved residue stratification... otherwise it differentiates into a triadic appearance taxonomy."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-008
  - note: Evaluates how the Ground state directly determinates the specific ripening/fruition profiles of latency.
  - sourceClaimIds: [`ys-iv-007-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: `pending`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-008 — Congruent Manifestation (Ripening)

Span:

- sourceFile: `src/logos/ys/ys_iv_08.ts`
- lineStart: 4
- lineEnd: 21

Summary:
From the preceding conditions (_tataḥ_), the manifestation (_abhivyakti_) into appearance is strictly selected based on congruency (_anuguṇānām_) with the immediate maturation vector (_tad-vipāka_) of the vāsanās.

Key points: (KeyPoint)

- k1. _Tad-vipāka_ defines the maturation vector or "ripening profile" computed from the prior Ground ensemble.
- k2. _Anuguṇānām_ serves as a strict compatibility predicate/filter between the ripening profile and the latent form-trace packets (_vāsanās_).
- k3. Manifestation is not a miraculous external production; it functions as an internal selection law based on congruency.

Claims: (Claim)

- c1. id: ys-iv-008-c1
  - subject: vasana_manifestation
  - predicate: is_constrained_by
  - object: congruence_with_ripening_profile
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [16-17] "Only those vāsanas whose determination-signatures are congruent with the present ripening profile produced by the prior ground actually manifest."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-007
  - note: Clarifies how the Ground condition (colored or uncolored) yields the specific selection filtering for Appearance.
  - sourceClaimIds: [`ys-iv-008-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`ys-iv-007-c1`, `ys-iv-007-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

### Entry ys-iv-009 — Continuity via One-Form Identity

Span:

- sourceFile: `src/logos/ys/ys_iv_09.ts`
- lineStart: 4
- lineEnd: 19

Summary:
Despite massive separations (_vyavahitānām_) across class, space, and time vectors (_jāti-deśa-kāla_), succession remains unbroken (_ānantaryam_) because memory (_smṛti_) and latent traces (_saṁskāra_) share a singular invariant form signature (_eka-rūpatvāt_).

Key points: (KeyPoint)

- k1. _Ānantaryam_ (unbroken continuity) does not rely on spatiotemporal adjacency to function.
- k2. Continuity is sustained by _eka-rūpatva_ (one-form-ness), rendering memory as an exact structural readout of the latent trace packet.
- k3. This protects the phenomenological stream from structural fragmentation regardless of intervening temporal or locational gaps.

Claims: (Claim)

- c1. id: ys-iv-009-c1
  - subject: unbroken_succession
  - predicate: requires
  - object: one_form_identity
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [18-18] "Continuity is secured by form-identity (eka-rūpatva), not by spatiotemporal adjacency."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-008
  - note: Validates that the "congruent selection" observed in IV.8 is fundamentally maintained by form-invariance across all apparent separation parameters.
  - sourceClaimIds: [`ys-iv-009-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-008-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

### Entry ys-iv-010 — The Condition of Beginninglessness

Span:

- sourceFile: `src/logos/ys/ys_iv_10.ts`
- lineStart: 4
- lineEnd: 16

Summary:
The latent impression traces (_tāsām_ / the vāsanās) are temporally beginningless (_anāditvam_) precisely because the deep desiderative aim/vector (_āśiṣ_) that drives them is utterly constant and eternal (_nityatvāt_).

Key points: (KeyPoint)

- k1. _Anāditvam_ removes the possibility of an absolute first efficient cause within the temporal series itself.
- k2. _Nityatva_ of _āśiṣ_ indicates that the foundational normative/aiming vector is a constant condition for the form-trace packets.

Claims: (Claim)

- c1. id: ys-iv-010-c1
  - subject: latent_impression_set
  - predicate: is_qualified_as
  - object: beginningless_anaditvam
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [14-14] "The latent impressions (vāsanā/saṁskāra) are beginningless because the desiderative vector (āśiṣ) is constant."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-011
  - note: By establishing beginninglessness instead of first-causation, IV.10 necessitates the structural dependent-origination logic of IV.11.
  - sourceClaimIds: [`ys-iv-010-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: `pending`
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

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

- r1. type: presupposes
  - targetEntryId: ys-iv-010
  - note: Originates structurally rather than temporally because the temporal chain is designated as beginningless in IV.10.
  - sourceClaimIds: [`ys-iv-011-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-010-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection
