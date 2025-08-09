# @organon/logic — Schema layer (canon)

This folder defines the canonical Zod schemas for the Logic system. These are the source of truth for types and invariants. The Form layer (`../form`) wraps these schemas with small classes for engine ergonomics; the Processor integrates both into the runtime.

## Pillars and theory-bearing schemas

The Form theory is captured at schema level by:

- FormDefinition / FormSchema — in `schema/form.ts`
- FormShape — in `schema/shape.ts`

Six pillars (domain primitives) are modeled with dedicated schemas:
- form — `schema/form.ts`
- entity — `schema/entity.ts`
- context — `schema/context.ts`
- property — `schema/property.ts`
- morph — `schema/morph.ts`
- relation — `schema/relation.ts`
- shape — `schema/shape.ts`

The wrappers in `logic/src/form` mirror these pillars with runtime-friendly classes: `FormEntity`, `FormContext`, `FormProperty`, `FormMorph`, `FormRelation`.

## Naming policy

- “Everything is a Form”, but only the theory-bearing schemas use the `Form*` prefix at schema level (`form.ts`, `shape.ts`).
- Domain schemas can be descriptive (entity/context/property/morph/relation) or prefixed if standardized; the codebase is converging on consistent names.
- Wrapper classes always use `Form*` in `logic/src/form`, regardless of the schema file name.

## Usage patterns

Schemas are Zod models used to:

- Validate and infer types for data moving through the system.
- Provide creation/update helpers that preserve invariants (e.g., `createEntity`, `updateEntity`, `createContext`, `createRelation`).
- Serve as the canonical interface; wrappers call these helpers and expose `toSchema()` for persistence.

Example (schema level):

```ts
import { createEntity, createEntityRef } from "./entity";
import { createRelation } from "./relation";

const e1 = createEntity({ type: "system.Entity" });
const e2 = createEntity({ type: "system.Entity" });

const r = createRelation({
	source: createEntityRef(e1),
	target: createEntityRef(e2),
	type: "related_to",
});
```

## Relationship to the Form and Processor layers

- Form layer wraps these schemas to add engine ergonomics (metadata, context handles, small mutators) while keeping schema as the single source of truth.
- Processor layer (Relational Form Processor) consumes the Form wrappers and schema-derived shapes to compute world state, properties, and essential relations.

## Special schemas

This folder also contains:

- GUI-oriented schemas (e.g., card, list, table, image, button, display, chart) for predefined UI Forms.
- Conceptual signposts (being, essence, concept, thing, world) for integrating Logic into Forms. They act as namespaces and defined terms today; deeper integration is planned.

## Node/TypeScript notes

- NodeNext is used; import schema modules without extensions in TS source; emitted JS will use `.js`.
- Only a subset of files may be compiled while the system evolves; the build can be constrained via the package tsconfig.
