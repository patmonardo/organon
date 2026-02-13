# ADR 0012 — Absolute Kernel ↔ Relative ToolChain Bridge (BEC:MVC, GDSL/SDSL)

Status: Proposed
Date: 2026-02-12

## Context

- We operate two distinct but coupled domains:
  - **Absolute Logic (Kernel)**: GDS executes scientific/absolute determination.
  - **Relative Logic (ToolChain/UserLand)**: TypeScript layers perform reflective modeling, feature articulation, and discursive rendering.
- The current boundary already exists in code:
  - Absolute boundary and kernel transport live in `logic/src/absolute/form/*` (`kernel-port`, `gds-link.client`, invariants/trace).
  - Relative Form Processor lives in `logic/src/relative/form/*`, with concrete engines for `entity`, `property`, and `aspect`.
- We treat ToolChain as **Logic:Model**, expressed as **BEC:MVC**:
  - **BEC** provides canonical logical encoding.
  - **MVC** provides model/application adapters and userland orchestration.
- ToolChain protocol language is **GDSL/SDSL** encoded as TS-JSON; adapters may project this into React/Next MVC views, but UI projection is not the primary protocol.
- Clarification needed: the **3^3 Dialectic Cube** is a subset of Relative Logic, not the entirety of Relative Logic.
  - Relative Logic includes broader ToolChain concerns, model orchestration, and adapter/runtime boundaries.

## Problem

Without an explicit ADR, architecture drifts in three ways:

1. Relative Logic is mistaken for the Dialectic Cube only.
2. MVC projection is mistaken for the protocol itself (instead of GDSL/SDSL TS-JSON).
3. Feature/model structures are kept too tightly coupled, instead of compiling into independent entity/property/aspect forms that can be agent-consumed.

## Decision

### 1) Define the boundary as Absolute Kernel vs Relative ToolChain

- **Absolute (GDS Kernel)** remains the source of scientific closure (truth-step / formal determination).
- **Relative (TS ToolChain)** remains the source of model authoring, feature articulation, and adapter-specific orchestration.
- Boundary contract remains JSON-first through `KernelPort` and Absolute Form clients.

### 2) Adopt BEC:MVC as the ToolChain architecture contract

- Treat ToolChain as **Logic:Model = BEC:MVC**.
- **Model layer** is the logical client SDK surface for GDSL/SDSL authoring and transformation.
- For GUI delivery, ToolChain flow is: **GDSL authoring -> TS-JSON protocol -> MVC adapter (React/Next)**.
- **MVC adapters** (e.g. React/Next) are one projection target, not the canonical protocol.
- Add/maintain an **Agent Adapter** that consumes and emits GDSL/SDSL TS-JSON directly (without requiring MVC UI adapters).

### 3) Make Relative Form Processor the canonical UserLand compiler surface

Relative logic compiles model/features into forms that are executable and portable:

- `ModelSet -> EntitySet`
- `FeatureSet -> PropertySet`
- `Entity/Property mediation -> AspectSet`

This means:

- Entities are the compiled carriers of model identity/thingness.
- Properties are compiled feature/law structures and may persist independently of any single entity instance.
- Aspects are compiled relational/spectral renderings suitable for discursive and agent-facing outputs.

### 4) Preserve procedure-first and adapter-safe orchestration

- Applications and examples call top-level procedures/facades only; they do not call kernel algorithm internals directly.
- Relative ToolChain prepares program forms and artifacts; Absolute Kernel performs closure and returns determinations.
- Existing GLM algorithm adapter behavior in Absolute Form dispatcher remains valid; Collections DataFrame/Dataset extensions must remain compatible with this boundary.

### 5) Define Design Surface persistence as Entity/Property/Aspect

- ToolChain design surfaces are persisted in Relative Logic storage as **Entity / Property / Aspect** records.
- Neo4j-backed persistence is an accepted implementation for this representation layer.
- Persistence remains protocol-first: stored structures correspond to GDSL/SDSL TS-JSON-level semantics, not to any single UI adapter.

### 6) Clarify dialectic scope metadata

- Keep Dialectic Cube taxonomy explicit as a scoped subsystem of Relative Logic.
- Represent current taxonomy statement for planning/reference:
  - genera space: $3^3 = 27$
  - species space: $3^4 = 81$
  - combined reference cardinality: $108 = 27 + 81$
- These counts are conceptual classification metadata; they are not by themselves runtime limits.

## Architectural mapping (normative)

- **Kernel SDK analogy**: GDS Dataset world acts as middleware/client SDK over DataFrame/Dataset semantics.
- **ToolChain SDK analogy**: MVC world acts as middleware/client SDK over Logic:Model semantics.
- Relative ToolChain and Absolute Kernel are dual SDK surfaces joined by the Application Form boundary.

## Code-grounded rationale

- Absolute boundary already standardizes model IDs and JSON execution (`kernel-port` / `gds-link.client`).
- Relative engines already encode the language runtime we need:
  - `EntityEngine`: compiles and persists concept/thing records plus signatures/facets.
  - `PropertyEngine`: compiles invariants/facticity/mediation structures (feature-law surface).
  - `AspectEngine`: compiles relational/spectral/appearing structures for discursive output.
- Therefore, this ADR codifies existing implementation direction rather than introducing an incompatible design.

## Consequences

- Positive:
  - Unifies ToolChain language around GDSL/SDSL TS-JSON across UI and agent pathways.
  - Prevents conflating Relative Logic with only the Dialectic Cube.
  - Enables model/feature independence through entity/property/aspect compilation.
  - Preserves stable kernel boundary and existing algorithm adapter tests.
- Risks:
  - Vocabulary overlap (model/entity, feature/property, relation/aspect) can create ambiguity.
  - Adapter drift if MVC-only assumptions reappear.
- Mitigations:
  - Keep boundary tests JSON-first.
  - Add explicit Agent Adapter contract tests parallel to MVC adapter tests.
  - Keep terminology mappings documented in package docs and ADR references.

## Adoption checklist

1. Document Agent Adapter contract in `logic/src/absolute/form` docs and examples using GDSL/SDSL TS-JSON payloads.
2. Add/refresh integration examples showing one payload consumed by:
   - MVC adapter (React/Next path), and
   - Agent adapter (direct TS-JSON path).
3. Add a Relative Form compilation walkthrough (`ModelSet/FeatureSet -> Entity/Property/Aspect`) in `logic/doc`.
4. Add invariant tests ensuring property structures can exist independently from a single entity instance while remaining composable.
5. Add architecture guardrail text in docs: Dialectic Cube is subset of Relative Logic.

## Out of scope

- Rewriting kernel internals or changing procedure-first guardrails.
- Redesigning DataFrame/Dataset storage layouts.
- Introducing new UI frameworks or changing existing MVC stack choices.

## References

- `logic/src/absolute/form/kernel-port.ts`
- `logic/src/absolute/form/gds-link.client.ts`
- `logic/src/relative/form/README.md`
- `logic/src/relative/form/entity/entity-engine.ts`
- `logic/src/relative/form/property/property-engine.ts`
- `logic/src/relative/form/aspect/aspect-engine.ts`
- `logic/doc/adr/0011-formshape-program-kernel-unification.md`
