# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `REFLECTION/ESSENCE` (YS IV.2 - IV.4)
Status: active
Authority: TS source files (`ys_iv_02.ts`, `ys_iv_03.ts`, `ys_iv_04.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

## Working decisions

- Converting the dense HLOs and Ontologies from the TS objects into formal line-anchored graph claims.
- Lines referenced under "evidence" correspond to the declarative blocks in the `.ts` files.

---

### Entry ys-iv-002 — Reclassification through matrix saturation

Span:

- sourceFile: `src/logos/ys/ys_iv_02.ts`
- lineStart: 4
- lineEnd: 15

Summary:
Transformation to a distinct categorical configuration (_jāti-antara-pariṇāma_) occurs through the saturation of the potential envelope of the constitutive matrix (_prakṛti-āpurāt_).

Key points: (KeyPoint)

- k1. _Jāti-antara-pariṇāma_ represents a threshold shift to a distinct configuration.
- k2. This transformation is driven by _prakṛti-āpurāt_ (the filling up / saturation of potential).
- k3. Functionally, a dharma stream reclassifies when its active lattice fully discloses its modality envelope.

Claims: (Claim)

- c1. id: ys-iv-002-c1
  - subject: jati_antara_parinama
  - predicate: is_caused_by
  - object: prakriti_apurat_saturation
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [15-15] "jāti-antara-pariṇāma prakṛti-āpurāt ... occurs when the conditioning matrix reaches saturation..."

- c2. id: ys-iv-002-c2
  - subject: reclassification_trigger
  - predicate: requires
  - object: full_disclosure_of_potential_envelope
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [15-15] "...the dharma stream reclassifies because its active pratyaya lattice fully discloses a modality"

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ys-iv-003
  - note: Saturation defines the _what_ of change; sutra 3 defines the subtractive _how_ of that causal change.
  - sourceClaimIds: [`ys-iv-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`ys-iv-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-003 — Subtractive causality and enabling removal

Span:

- sourceFile: `src/logos/ys/ys_iv_03.ts`
- lineStart: 4
- lineEnd: 17

Summary:
The instrumental condition (_nimitta_) does not efficaciously drive the natural streams (_aprayojakam prakṛtīnām_); rather, it acts by removing obstacles (_varaṇa-bheda_), like a farmer opening an irrigation channel.

Key points: (KeyPoint)

- k1. _Nimitta_ (instrumental condition) is non-impelling (_aprayojakam_); it lacks productive physical force here.
- k2. Transformation occurs via subtractive catalysis—the removal of an obstruction (_varaṇa-bheda_).
- k3. The natural tendencies (_prakṛtīnām_) self-differentiate and flow inherently once the dam is broken, analogous to water in a field (_kṣetrikavat_).

Claims: (Claim)

- c1. id: ys-iv-003-c1
  - subject: nimitta_occasion
  - predicate: is_determined_as
  - object: non_impelling_aprayojakam
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [7-9] "nimittam: instrumental condition... a-prayojakam: not an impelling / driving efficient cause"

- c2. id: ys-iv-003-c2
  - subject: varana_bheda_clearing
  - predicate: enables
  - object: self_differentiation_of_prakriti_streams
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [17-17] "Citi does not propel prakṛti-forces; it discloses by removing impediments. The plurality of natural tendencies self-differentiate once obstruction is cleared"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-002
  - note: Clarifies that the saturation shift in IV.2 is NOT pushed by an external creator, but released by internal necessity.
  - sourceClaimIds: [`ys-iv-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ys-iv-002-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ys-iv-004
  - note: Having established self-differentiation through negative removal, the text must explain _what_ primary parameter differentiates the plurality of minds.
  - sourceClaimIds: [`ys-iv-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ys-iv-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry ys-iv-004 — The Cogito Modulation Law

Span:

- sourceFile: `src/logos/ys/ys_iv_04.ts`
- lineStart: 4
- lineEnd: 25

Summary:
The plurality of constructed cognition-streams (_nirmāṇa-cittāni_) arises solely from the internal differentiation of the pure I-ness parameter (_asmitā-mātra_).

Key points: (KeyPoint)

- k1. _Nirmāṇa-cittāni_ denotes constructed, derivative populations of cognition streams.
- k2. _Asmitā-mātra_ ("I-ness alone") is the minimal structural parameter from which these arise.
- k3. The law of cogito modulation asserts that apparent existential plurality is simply the internal indexed folding of a single parameter.

Claims: (Claim)

- c1. id: ys-iv-004-c1
  - subject: constructed_cittas_plurality
  - predicate: arise_from
  - object: asmita_matra_parameter
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [5-5] "nirmāṇa-cittāni asmitā-mātra"
    - [32-32] "Constructed cognition-streams are nothing but pure I-parameter (asmitā-mātra) modulations."

- c2. id: ys-iv-004-c2
  - subject: asmita_matra
  - predicate: guarantees
  - object: internal_underlying_identity_continuum
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [17-17] "Underlying self-sameness across constructed multiplicity"

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ys-iv-003
  - note: Translates the "self-differentiation" of prakriti streams in sutra 3 strictly into the psychological parameter of "asmitā" constructing multiple cittas.
  - sourceClaimIds: [`ys-iv-004-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ys-iv-003-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
