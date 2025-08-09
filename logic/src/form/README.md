# @organon/logic — Form layer (six pillars)

This folder contains the Form layer: small, engine-friendly TypeScript classes that wrap the canonical Zod schemas in `logic/src/schema`. Think of these as the “immediate-with-its-mediacy” adapters between schema (Being/Essence) and engines/processors.

Everything is a Form — but we only use the `Form*` prefix for the schemas that implement the Form theory itself. In schema, those live in `schema/form.ts` and `schema/shape.ts` (FormDefinition/FormSchema/FormShape). The six pillars of the Form theory are:

- Form:Entity
- Context:Property
- Morph:Relation

Together, these give us the Thing:World:Essence platform. A seventh “Processor” integrates them into the Relational Form Processor (lives under `logic/src/processor`).

## What lives here

- Thin wrappers over schema types that:
	- Expose ergonomic getters/setters for engines.
	- Preserve schema invariants (updates go through schema helpers where available).
	- Provide `toSchema()` for persistence/transport and `toJSON()` including engine metadata when useful.
	- Keep engine-only concerns (metadata/context) out of the schema domain.

Current exports (via `form/index.ts`):

- `FormEntity` — wraps `schema/entity`
- `FormContext` — wraps `schema/context`
- `FormProperty` (+Definition) — wraps `schema/property`
- `FormMorph` (+Pipeline) — wraps `schema/morph`
- `FormRelation` — wraps `schema/relation`

Import from the barrel so future renames don’t ripple:

```ts
import { FormEntity, FormContext, FormProperty, FormMorph, FormRelation } from "@organon/logic/src/form";
```

## Naming policy

- In `logic/src/schema`:
	- Only theory-bearing files use `Form*` prefixes: `schema/form.ts`, `schema/shape.ts`.
	- Other schemas (entity, context, property, morph, relation) can remain descriptive without the `Form` prefix or use it where we’ve standardized recently.
- In `logic/src/form` (this folder):
	- Public classes use `Form*` names consistently: `FormEntity`, `FormContext`, `FormProperty`, `FormMorph`, `FormRelation`.
	- Files and symbols may be renamed during cleanup; import from the barrel.

## Quick examples

Create a FormEntity from parameters, later serialize for persistence:

```ts
const thing = new FormEntity({ id: "system:engine", type: "System::Engine" });
// ... mutate via setters or update helpers ...
persist(thing.toSchema());
```

Work with a context and relations using schema-safe helpers underneath:

```ts
const ctx = FormContext.create({ name: "World", type: "domain" });
ctx.addEntities([{ entity: "system.Entity", id: thing.id }]);

const rel = FormRelation.create({
	source: { entity: thing.type, id: thing.id },
	target: { entity: thing.type, id: thing.id },
	type: "related_to",
});
```

Define morphs and pipelines that seed EssentialRelations:

```ts
const morph = FormMorph.define({ id: "normalize", transformFn: "Normalize.transform" });
```

## Processor integration

- The Relational Form Processor lives under `logic/src/processor` and integrates these pillars into a cohesive runtime (Goal/Logic Process Engine). It consumes the Form layer wrappers and emits world/shape/essence computations.

## Migration notes

- Legacy engine code here was experimental; it’s okay if we break it while upgrading form+schema together.
- Some older files (e.g., a previous `property.ts`) may be replaced by the new schema-backed wrappers; imports should target the barrel.
- We’re on NodeNext; TypeScript source uses extensionless imports, emit resolves `.js`.

## Roadmap (short)

- Complete the wrappers’ tests (happy path + 1-2 edges each).
- Finalize naming alignment across `schema/*` and `form/*`.
- Bridge wrappers into the processor end-to-end (world/form/entity/property/morph/relation flows).
