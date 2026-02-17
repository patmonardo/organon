# Being Artifact Contract

This document defines the canonical artifacts produced for the Being pass and the exact role of each artifact family.

## Scope

- Doctrine of Being / Quality / Being-Nothing-Becoming
- Generator entrypoint: `logic/src/tools/generate-being-ir.ts`
- Package script: `pnpm --filter @organon/logic codegen:being:ir`

## Artifact families

### 1) TopicMap-integrated artifacts (graph-loading and query artifacts)

Generated files:

- `logic/src/relative/being/quality/being/integrated-topicmap-ir.ts`
- `logic/src/relative/being/quality/being/sources/generated/integrated-topicmap-ir.debug.ts`
- `logic/src/relative/being/quality/being/sources/generated/integrated-topicmap-ir.cypher`
- `logic/src/relative/being/quality/being/sources/generated/integrated-topicmap-query-pack.cypher`

Purpose:

- Materialize TopicMap chunks into a connected integrated graph representation.
- Emit Cypher for KG load and inspection.
- Preserve chunk/source provenance and trace relations.

Primary consumers:

- Neo4j load workflows.
- Cypher research and graph diagnostics.

### 2) Dialectical States artifacts (state-machine IR)

Generated/maintained files in Being path:

- `logic/src/relative/being/quality/being/being-ir.ts`
- `logic/src/relative/being/quality/being/nothing-ir.ts`
- `logic/src/relative/being/quality/being/becoming-ir.ts`

Format:

- `DialecticIR` containing `DialecticState[]` (schema in `logic/src/schema/dialectic.ts`).

Purpose:

- Encode dialectical progression as explicit states, forces, invariants, and transitions.
- Provide canonical logical/state representation for dialectical composition.

Primary consumers:

- Dialectic registry/codegen.
- Higher-level chapter or section composition in Dialectical States format.

## Terminology (to prevent drift)

- Use **TopicMap artifact** for integrated TopicMap/Cypher outputs.
- Use **Dialectical States artifact** for `DialecticIR` files.
- Do **not** use “DialecticMap” as a separate artifact class. In this repository, the canonical term is `DialecticIR`.

## Canonical vs supporting outputs

- For KG construction and Cypher delivery, the canonical outputs are the `integrated-topicmap-*` artifacts.
- For dialectical logic/state composition, the canonical outputs are the `*-ir.ts` `DialecticIR` files.
- These two families are complementary and both are required in the Being workflow.

## Required generation flow (Being)

1. Update Being source analysis/chunk/TopicMap files.
2. Run `pnpm --filter @organon/logic codegen:being:ir`.
3. Verify all four integrated TopicMap outputs exist and are updated.
4. Run `pnpm --filter @organon/logic build`.
5. Ensure build is green and generated files remain connected/provenance-preserving.

## Acceptance checks

- No broken IDs or missing references in generated integrated outputs.
- `integrated-topicmap-ir.cypher` and `integrated-topicmap-query-pack.cypher` regenerate successfully.
- Being `DialecticIR` files remain schema-valid and build passes.
- No methodology drift to unrelated chapters when executing Being-only scope.
