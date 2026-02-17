# Central Specs Reader Map (v0)

Purpose: fast re-entry guide for the two central artifacts.

## Artifacts

- `logic/doc/COMMON-LANGUAGE-CORE-v0.md`
- `logic/doc/V0-COMPILER-RUNTIME-CONTRACT.md`
- `logic/src/schema/root-shapes.ts`

## 10-Minute Read Order

1. Common Language Core: `The Word`, `Scope`, `Layer Contract`
   - Why: fixes ontology + persistence framing before code details.
2. Common Language Core: `Three-Store Semantics`, `Normative Flow Rules`
   - Why: fixes Fact/Relation/Knowledge order and modeling overlay boundary.
3. Common Language Core: `SDSL and GDSL Roles` (+ many-models/one-GDSL + isomorphic claim)
   - Why: fixes language governance and normalization law.
4. V0 Compiler-Runtime Contract: `In Scope`, `Out of Scope`, `Required Flow`, `Done Criteria`
   - Why: locks execution scope to the semantic middle and prevents overreach.
5. Root Shapes: principle chain (`RootFormShapeSchema` -> `RootContextShapeSchema` -> `RootMorphShapeSchema`)
   - Why: this is FactStore-side semantic progression.
6. Root Shapes: law chain (`RootEntityShapeSchema` -> `RootPropertyShapeSchema` -> `RootAspectShapeSchema`)
   - Why: this is RelationStore-side semantic progression.
7. Root Shapes: bridge + isomorphism contracts
   - Why: enforces shape sharing from SDSL into canonical GDSL roots.

## Mental Model (one line each)

- GDSL: single canonical semantic executor language.
- SDSL: plural modeling/runtime dialects that must normalize to GDSL.
- Principle evolution: Form -> Context -> Morph.
- Law evolution: Entity -> Property -> Aspect.
- Persistence order: FactStore -> RelationStore -> KnowledgeStore.

## File Anchors

### `COMMON-LANGUAGE-CORE-v0.md`

- `Layer Contract`: Being has no store; Essence persists; Concept synthesizes.
- `Morphological Framing`: Form/Context presupposed, Morph operationalizes Ground.
- `Three-Store Semantics`: conjunction + modeling overlay is non-ground.
- `SDSL and GDSL Roles`: many SDSL models, one GDSL; isomorphic claim required.

### `root-shapes.ts`

- `RootPrincipleEvolutionSchema`: validates form/context/morph linkage.
- `RootLawEvolutionSchema`: validates entity/property/aspect linkage.
- `RootPrincipleLawBridgeSchema`: stage correspondence laws.
- `SdslGdslIsomorphicClaimSchema`: runtime-enforceable shape-sharing contract.

### `V0-COMPILER-RUNTIME-CONTRACT.md`

- `In Scope / Out of Scope`: implementation guardrails.
- `Required Flow`: compiler -> runtime -> persistence chain.
- `Boundary Rules`: language vs runtime separation.
- `Done Criteria`: minimum shippable contract.

## Current Decisions (locked)

- Modeling is optional overlay, not semantic ground.
- FactStore is not generic datastore.
- No direct FactStore -> Modeling schema projection.
- SDSL validity is directional: `SDSL -> normalized mapping -> GDSL RootShape`.

## Open Design Slots (next sessions)

1. Define canonical SDSL dialect IDs and versioning policy.
2. Pin stage compatibility defaults for each SDSL adapter family.
3. Add executable fixtures/examples for valid and invalid isomorphic claims.
4. Decide where runtime claim validation lives (`model/src/sdsl` vs compiler boundary layer).

## Suggested Next Working Session (30-45 min)

- 10m: review this map + `SDSL and GDSL Roles` section.
- 15m: add 2-3 concrete `SdslGdslIsomorphicClaim` fixtures.
- 10m: wire one model-side adapter to emit claims.
- 10m: validate with `pnpm --filter @organon/logic build` and a focused model test.
