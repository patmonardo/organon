# SANSKRIT COMPILER Workbook (TopicMap Format)

Part: `ACTUALITY/SUBSTANCE` (YS IV.34)
Status: active
Authority: TS source files (`ys_iv_34.ts`)

## Authority + format lock

- Contract reference: `WORKBOOK-CONTRACT-V1.md` (Logic Schema)
- This workbook markdown is the authoritative Knowledge Graph artifact for this part's Sanskrit baseline claims.
- Any generated Cypher projection is derivative.
- Maintains strict `<id>`, `Key points: (KeyPoint)`, `Claims: (Claim)`, and `Relations: (Relation)` blocks.

---

### Entry ys-iv-034 — Absolute Kaivalya

Span:

- sourceFile: `src/logos/ys/ys_iv_34.ts`
- lineStart: 4
- lineEnd: 27

Summary:
_Kaivalya_ is defined either as the absolute counter-flow/return (_pratiprasavaḥ_) of the _guṇas_ to their source—since they are now completely devoid of any teleological purpose for the Seer (_puruṣārtha-śūnyānāṁ_)—or (_vā_) as the power of pure consciousness (_citi-śaktir_) solidly established in its own inherent nature (_svarūpa-pratiṣṭhā_).

Key points: (KeyPoint)

- k1. The sutra defines the ultimate goal of Yoga: _Kaivalya_ (radical independence/isolation).
- k2. It offers two symmetrical definitions joined by "or" (_vā_). One is physical (the perspective of the _guṇas_), and the other is metaphysical (the perspective of the Seer).
- k3. From the physical side: the _guṇas_ stop mutating and return to their unmanifest state (_pratiprasava_) because there is literally no reason for them to act anymore (_puruṣārtha-śūnya_).
- k4. From the metaphysical side: pure consciousness (_citi-śakti_) no longer falsely identifies with fluctuations (_vṛttis_) and simply rests in its own pristine form (_svarūpa-pratiṣṭhā_).

Claims: (Claim)

- c1. id: ys-iv-034-c1
  - subject: absolute_liberation_kaivalya
  - predicate: is_physically_defined_as
  - object: the_terminal_involution_of_the_gunas
  - modality: asserted
  - confidence: 0.99
- c2. id: ys-iv-034-c2
  - subject: absolute_liberation_kaivalya
  - predicate: is_metaphysically_defined_as
  - object: consciousness_established_in_its_own_nature
  - modality: asserted
  - confidence: 0.99

Relations: (Relation)

- r1. type: is_the_final_conclusion_of
  - targetEntryId: ys-iv-033
  - note: Once absolute necessity is grasped at the terminal limit of time (IV.33), the system formally closes. IV.34 is the statement of this closure.
  - sourceClaimIds: [`ys-iv-034-c1`, `ys-iv-034-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - logicalOperator: final_teleological_closure
