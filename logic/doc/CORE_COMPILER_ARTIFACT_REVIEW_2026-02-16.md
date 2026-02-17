# Core Compiler Artifact Review (2026-02-16)

## Scope

Focused review of three concerns in `logic` core/compiler artifacts:

1. Runtime schemas should focus on **Shapes**.
2. Language compiler should focus on **Definitions**.
3. Repository architecture should remain explicit and directional.

## Current Boundary (Observed)

### Runtime Shape schemas

- Primary module: `logic/src/schema/root-shapes.ts`
- Exports canonical runtime shape families:
  - Principle-side: `RootFormShape`, `RootContextShape`, `RootMorphShape`
  - Law-side: `RootEntityShape`, `RootPropertyShape`, `RootAspectShape`
- Includes SDSL↔GDSL shape isomorphism schemas (runtime mapping-oriented).

### Compiler definitions and lowering

- Definition language forms: `logic/src/relative/core/compiler/gdsl-definitional-forms.ts`
- Definition compiler: `logic/src/relative/core/compiler/gdsl-definitional-compiler.ts`
- Compiler currently lowers Definitions into Root Law Shapes (`entity/property/aspect`) and defers rule/constraint/vocabulary/provenance lowering.

### Repository architecture artifacts

- Boundary contract: `logic/src/relative/core/compiler/three-store-boundary.ts`
- Store prototypes:
  - `fact-store-prototype.ts`
  - `relation-store-prototype.ts`
  - `knowledge-store-prototype.ts`
- Direction is explicit and consistent:
  - FactStore → RelationStore → KnowledgeStore
  - RelationStore → Modeling layer
  - FactStore does not directly feed modeling layer.

## Where concerns are still blended

1. **Naming leak inside runtime shapes**
   - `RootProcessorDefinitionSchema` in `root-shapes.ts` uses compiler-centric wording (`Definition`) inside runtime schema space.
   - This is a language leak, even though the structure is generic (`record<string, unknown>`).

2. **Compiler output naming could be more directional**
   - `compileGdslDefinitionalPackage` returns `rootLawShapes`.
   - The shape is correct, but the intent “Definitions lowered into Shapes” is implicit rather than explicit in naming.

3. **Compiler barrel mixes distinct concerns without grouping cues**
   - `logic/src/relative/core/compiler/index.ts` exports shape compilers, store prototypes, and definitional compiler together.
   - This is functional but makes architectural boundaries harder to read at a glance.

## Suggested minimal sequence (no behavioral break)

### Step 1 — Naming alignment only

- In `root-shapes.ts`, introduce `RootProcessorSpecSchema` (or `RootProcessorShapeSpecSchema`) and keep `RootProcessorDefinitionSchema` as compatibility alias.
- Intent: runtime schema language says “Shape/Spec”, compiler language says “Definition”.

### Step 2 — Compiler artifact clarity

- In `gdsl-definitional-compiler.ts`, add a clearer field alias (e.g., `loweredRootLawShapes`) while preserving `rootLawShapes` for compatibility.
- Intent: make lowering direction explicit without breaking existing callers.

### Step 3 — Barrel readability (optional, non-breaking)

- Add grouped comments/exports in compiler `index.ts`:
  - shape compilers
  - definitional compilers
  - repository/store boundary prototypes
- Intent: improve architectural scanning, no API break.

## Recommendation for next focused PR

Create a small PR containing only **Step 1 + Step 2** (naming and aliasing, zero runtime behavior changes), plus tests/typecheck updates if needed.

This keeps today’s decision crisp:

- Runtime schemas = Shapes/Specs.
- Compiler = Definitions and lowering.
- Repository architecture = directional boundary contract.
