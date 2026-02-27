# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/FOUNDATION` (YS IV.5 - IV.6)
Status: active
Authority: TS source files (`ys_iv_05.ts`, `ys_iv_06.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

## Working decisions

- Converting the dense HLOs and Ontologies from the TS objects into formal line-anchored graph claims.
- Lines referenced under "evidence" correspond to the declarative blocks in the `.ts` files.

---

### Entry ys-iv-005 — The Unity of the Determining Cogito

Span:

- sourceFile: `src/logos/ys/ys_iv_05.ts`
- lineStart: 4
- lineEnd: 24

Summary:
One determining Cogito (_cittam ekam_) acts as the immanent functional instigator (_prayojakam_) across a plurality of diverse operations (_anekeṣām_) within the identity-asserting differentiation phase (_pravṛtti-bhede_).

Key points: (KeyPoint)

- k1. _Cittam ekam_ represents a numerically and structurally singular determining consciousness.
- k2. This single consciousness is the immanent determinant (_prayojakam_) of many derivative operation-vectors (_anekeṣām_).
- k3. _Pravṛtti-bhede_ denotes the objective dialectic where a primary identity-thrust disperses into an articulated field of difference.

Claims: (Claim)

- c1. id: ys-iv-005-c1
  - subject: single_cogito_cittam_ekam
  - predicate: determines
  - object: plurality_of_operations_anekesam
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [16-16] "One determining Cogito (cittam ekam) functions as the immanent determinant (prayojakam) across a multiplicity of operation-vectors (anekesam)"

- c2. id: ys-iv-005-c2
  - subject: difference_bheda
  - predicate: is_determined_as
  - object: articulated_dispersion_of_single_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [16-16] "Difference (bheda) is not opposed externality but the articulated dispersion of a single identity-thrust."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-006
  - note: Evaluates how the dialectical resolution of this identity/difference generation yields residue-free cognition.
  - sourceClaimIds: [`ys-iv-005-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-006-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-006 — Residue-Free Dialectic Cognition

Span:

- sourceFile: `src/logos/ys/ys_iv_06.ts`
- lineStart: 4
- lineEnd: 23

Summary:
Within that dialectical field (_tatra_), the cognition generated actively through synthesis (_dhyāna-jam_) is structurally contradiction-free and without latent karmic residue (_anāśayam_).

Key points: (KeyPoint)

- k1. _Tatra_ refers back to the identity-difference field established in IV.5.
- k2. _Dhyāna-jam_ denotes a generative event of cognition arising from true dialectic synthesis (not quietist trance).
- k3. _Anāśayam_ asserts the absence of any latent, un-assimilated contradiction deposit (_āśaya_).

Claims: (Claim)

- c1. id: ys-iv-006-c1
  - subject: dhyana_generated_cognition
  - predicate: is_qualified_as
  - object: residue_free_anasayam
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [14-15] "the cognition-stream it generates (dhyāna-born citta) carries no residual latency... leaving no un-assimilated remainder (no āśaya deposition)."

- c2. id: ys-iv-006-c2
  - subject: anasayam_state
  - predicate: requires
  - object: proactive_resolution_at_generation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [14-14] "contradictions (virodha) are integrated at generation"
    - [22-22] "dialectic synthesis prevents residue production"

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: ys-iv-005
  - note: The residue-free dialectic necessarily occurs within the structural unity of the identity-determining Cogito mapped previously.
  - sourceClaimIds: [`ys-iv-006-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`ys-iv-005-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection
