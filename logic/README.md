# @organon/logic

Canonical Logic package providing:
- Schema layer (canon): Zod models and helpers — single source of truth.
- Form layer (six pillars): thin, engine-friendly services that wrap schemas.
- Tests: happy-path suites for services and schema with BaseState defaults awareness.

Key ideas
- Shape = Principle (loose, not bound). Entity = Essence/Dharma of Shape (instantiated).
- Concurrency applies to Entities (Contained), not Shapes (Principle).
- Six pillars: Entity, Context, Property, Morph, Relation, Shape (principle-level FormShape).

Project structure
- src/schema: canonical Zod schemas and helpers
  - form.ts (FormDefinition/FormSchema), shape.ts (FormShape)
  - entity.ts, context.ts, property.ts, morph.ts, relation.ts
- src/form: small services over schemas (engine ergonomics)
  - entity/service.ts, context/service.ts, property/service.ts
  - morph/service.ts, relation/service.ts, shape/service.ts
- test/schema: schema tests (e.g., shape.test.ts)
- test/service: happy-path service tests (entity/context/property/morph/relation/shape)

Naming policy
- In schema: only theory-bearing files use Form* (form.ts, shape.ts). Others use descriptive names.
- In form: public classes/services consistently use Form* naming in code, but filenames may be descriptive; import from the barrel when available.

Install
- Workspace install (from repo root):
  - pnpm install

Build, test, lint
- Build this package:
  - pnpm -C /home/pat/VSCode/organon --filter @organon/logic build
- Test (Vitest):
  - pnpm -C /home/pat/VSCode/organon --filter @organon/logic test
- Single test file:
  - pnpm -C /home/pat/VSCode/organon --filter @organon/logic exec vitest run logic/test/service/entity.test.ts
- Lint:
  - pnpm -C /home/pat/VSCode/organon --filter @organon/logic lint

Services (Form layer)
Common surface:
- on(kind, handler), get(id)
- create, delete, describe, setCore, setState, patchState
Events:
- *.created, *.core.set, *.state.set, *.state.patched, *.described, *.deleted
- Relation-only: relation.endpoints.set, relation.direction.set

Create signatures:
- Shape: { type, name? }
- Entity/Context/Morph: { type, name? }
- Property: { type, name?, key, contextId }
- Relation: { type, name?, kind, source:{id,type}, target:{id,type}, direction? }

Persistence
- Repo-backed when provided, in-memory Map otherwise.
- To avoid API mismatches across repos, services may persist via delete+create on updates.

Schema usage
- All writes validate via Zod (e.g., RelationSchema.parse(...)).
- BaseState provides defaults (e.g., status, tags); tests assert partials instead of exact state equality.

Docs
- See docs/form-theory.md for theory overview.
- See docs/services.md for service/event surface and flows.

Roadmap
- Finalize barrel exports and naming alignment.
- Enhance processor integration with Form wrappers.
- Expand tests with edge cases
