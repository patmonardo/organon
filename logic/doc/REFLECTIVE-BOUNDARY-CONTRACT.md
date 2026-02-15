# Reflective Boundary Contract (TS ↔ Kernel)

Status: quick reference for the current operational split.

## Purpose

Define the handoff between:

- TS Agent Logic (`epistemicProcessor`)
- Rust GDS Kernel (`transcendentalKernel`)

This keeps the system specification-driven while making runtime realization auditable.

## Contract Summary

- epistemicProcessor
  - runtime: `ts-agent-logic`
  - processor: `reflective-form`
  - mode: `epistemic`
  - authority: `sdsl/zod`

- transcendentalKernel
  - runtime: `gds-rust-kernel`
  - processor: `program-form-evaluate-apply-print`
  - mode: `transcendental-logic`
  - role: `cache`

- handoff
  - substrate: `cypher-driven`
  - invariants (minimum)
    - program-features-precede-kernel-compilation
    - specification-bindings-are-explicit
    - graph-refs-resolve-via-store-contract
  - proofObligations (minimum)
    - artifact-hooks-validated
    - program-form-print-materialized

## Data Substrate Note

For transcendental realization, the FactStore carries a dual data substrate:

- DataFrame aspect (structured compilation substrate)
- living Dataset aspect (actual evolving facticity)

FactStore implementation profile for current architecture:

- single Prisma MetaModel template (`single-factstore-metamodel`)
- Postgres grounds integration for unbounded grounds/conditions
- Postgres role is single-purpose (`grounds-and-conditions-maintenance`), not
  a general-purpose model-to-SQL destination
- media/blob grounding routes to filesystem (`image`, `audio`, `video`,
  `binary`, `document`)
- kernel polyglot dataset cache support (multiple dataset backends)
- DragonSeed SDK augmentation for SDSL dataset analyses
- access matrix:
  - Polars: partial-direct access for Postgres/filesystem
  - Kernel: direct access for Neo4j, Postgres, filesystem
- agent monitoring: enabled for factstore structure, dataset health, and cache health

The FormCatalog (Root Table of Forms) remains pure and does not require a Dataset store.

The Root DataFrame of GDSL is compiler-derived from specification and is not
produced by Dataset SDK execution.

## Reflective Grounding Transition

- Reflective evaluation of essential relations is grounded via Postgres
  (`grounds-and-conditions-maintenance`).
- Facts-in-themselves remain in Neo4j; grounds/conditions are modeled in
  TS-Zod-Prisma as the reflective substrate.
- Legacy prompt wording of "overflow" is interpreted as this grounding
  transition, not as a generic spillover mechanism.

## Hegelian Essence Embedding

- Relative Form Processor architecture is explicitly aligned to Hegelian Essence logic.
- Operational triad: `Entity-Property-Aspect`.
- Essence mapping: `Thing-World-Law-Essential Relations`.
- The boundary contract treats this as a runtime invariant for reflective reasoning,
  not as optional narrative metadata.

## Six-Pillar Form Engine Mapping

- The six pillars are treated as Form-engine capture of Hegelian Essence development.
- Reflection chapter mapping: `Shape-Context-Morph`.
  - This is the reflective determination layer (form determination before full appearing).
- Appearance chapter mapping: `Entity-Property-Aspect`.
  - This is the appearing/essential-relation layer where law and relation become operational.
- Together these establish the active axis for runtime reasoning:
  **Facticity ↔ EssentialRelation**.
- Current contracts treat this axis as implemented foundation, not future intent.

## Mapping Obligation

At SDSL generation time, persist explicit substrate mappings for each
Model/Feature + FeatureStruct:

- DataFrame definition reference
- Dataset definition reference

This mapping is carried in compiler artifacts as `substrateMapping` so the
handoff remains auditable and reversible.

## Where It Is Emitted

The boundary contract is emitted into `form_eval` artifacts as `boundaryContract` by TS compilation, and validated/echoed as kernel artifact hooks in `form_eval` proof output.
