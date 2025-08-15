# @organon/logic

Canonical Logic package providing:
- Schema layer (canon): Zod models and helpers — single source of truth.
- Form layer (six pillars): thin, engine-friendly services that wrap schemas.
- Tests: happy-path suites for services and schema with BaseState defaults awareness.
- Processor (absolute): integrates schemas + forms for world assembly and propagation.

Theory snapshot
- Container (Principles, Qualitative/Quality: Pure Being)
  - Shape → Being (principle, composable)
  - Context → Existence (grounding principle, versioned, immune to Property variations)
  - Morph → Being-for-self (principle of self-related unity/composition; non-concurrent)
- Truth of Quality (Essence-plane: Contained/Appearance, Quantitative)
  - Entity → Essence (determinate content)
  - Property → Reflection (measure within a Context)
  - Relation → Ground (necessary ties; Absolute Relations carry constraints; propagation runs over E/P/R only)
- Truth of Essence (Concept-plane)
  - Concept = Truth of Essence (unified determination)
  - Judgment = Truth of Reflection (measured determination)
  - Syllogism = Truth of Ground (mediating unity)
  - Runtime note: Syllogism is realized via Absolute Relations and the FormProcessor’s composition/propagation over E/P/R — not by Morph itself. Morph remains a principle (Being-for-self) that shapes admissible constructions.

Triad in action
- Qualitative → Quantitative
- Being → Essence → Concept/Judgment/Syllogism (truths)
- Principle → Law
- Pure (Container) → “Impure” with a second (Contained)

Project structure
- src/schema: canonical Zod schemas and helpers
  - form.ts (FormDefinition/FormSchema), shape.ts (FormShape)
  - entity.ts, context.ts, property.ts, morph.ts, relation.ts
- src/form: small services over schemas (engine ergonomics)
  - entity/service.ts, context/service.ts, property/service.ts
  - morph/service.ts, relation/service.ts, shape/service.ts
- src/absolute: processor entry points (FormProcessor)
- test/schema + test/service: happy-path suites
- docs: ADRs, concepts, roadmap, and API (TypeDoc HTML)

Services (Form layer)
Common surface
- on(kind, handler), get(id)
- create, delete, describe, setCore, setState, patchState
Events
- *.created, *.core.set, *.state.set, *.state.patched, *.described, *.deleted
- Relation-only: relation.endpoints.set, relation.direction.set
Create signatures
- Shape/Entity/Context/Morph: { type, name? }
- Property: { type, name?, key, contextId }
- Relation: { type, name?, kind, source:{id,type}, target:{id,type}, direction? }
Persistence
- Repo-first; in-memory fallback. To avoid API mismatches, updates may use delete+create.

Docs
- API (HTML): docs/api/index.html
- ADRs: docs/adr
  - 0001 — Shape as Principle; Entity as Essence
  - 0002 — Container vs. Contained (Context/Morph/Shape as Principles; E/P/R as Essence)
  - 0003 — Absolute Relation and Backpropagation in FormProcessor
  - 0004 — Kriya Triad: Principles (Being) and Contained (Appearance)
- Concepts: docs/concepts
  - absolute-form.md — E/R/G inside a fixed Context
  - erg-in-context.md — E/R/G as the Context’s soul
  - kriya-syllogism.md — Syllogism realized via Absolute Relations and FormProcessor
- Roadmap: docs/ROADMAP.md

Build, test, docs
- Build:
  - pnpm -C /home/pat/VSCode/organon --filter @organon/logic build
- Test (Vitest):
  - pnpm -C /home/pat/VSCode/organon --filter @organon/logic test
- Generate API docs (HTML):
  - pnpm dlx typedoc --tsconfig ./tsconfig.json \
    --entryPoints "./src/schema/*.ts" "./src/form/**/*.ts" \
    --exclude "**/*.test.ts" --readme ./README.md --out ./docs/api
- Open API docs (Linux):
  - xdg-open /home/pat/VSCode/organon/logic/docs/api/index.html

Notes
- BaseState provides defaults; tests use partial assertions for state.
- Repos vary; services validate via Zod and keep persistence simple.
