# Procedures Subsystem Development Guide

Date: 2026-05-15
Status: Living development contract for the GDS procedures facade layer.

## Why this guide exists

The procedures layer is the practical control surface for the graph algorithm subsystem.
It is where algorithm families become operational APIs with config validation, progress behavior, mode semantics, memory estimation, and store mutation behavior.

This guide defines the mechanics required to drive procedures toward completion with consistent architecture and long-term maintainability.

## Subsystem role

Primary facade root: gds/src/procedures

The procedures subsystem is responsible for:
- user-facing algorithm entrypoints
- config parsing and validation boundaries
- mode orchestration (stream, stats, run, mutate, write, estimate_memory)
- progress/task wiring
- memory estimate exposure
- controller ownership of graph store interactions
- compatibility with spec/pipeline execution paths

## Algorithm family taxonomy

Proposed progression and maintenance tiers:

1. Foundational
- gds/src/procedures/pathfinding
- gds/src/procedures/similarity

2. Classic Core
- gds/src/procedures/centrality
- gds/src/procedures/community

3. Learned/Modeling
- gds/src/procedures/machine_learning
- gds/src/procedures/embeddings

4. Orchestration Meta-Layer
- gds/src/procedures/pipelines

## Required architecture contract

1. Procedure-first controller pattern
- Applications call procedures only.
- Procedures do not bypass storage controllers to access low-level algo internals directly.
- Storage runtime owns graph access, projection materialization, progress tracking, termination checks, and algorithm invocation.
- Computation runtime owns algorithm state transitions and pure compute logic.

2. Pipeline alignment
- Spec execute path and procedure facade path must converge on equivalent algorithm behavior.
- Avoid split semantics where spec and facade return materially different results.

3. Localized change principle
- Upgrade in narrow slices per algorithm/family.
- Avoid introducing broad framework abstractions unless multiple procedures already need them.

## Procedure completeness definition

A procedure is considered complete for a mode family when all applicable items are true:

- Config
  - validates all required and bounded parameters
  - preserves alias compatibility where needed

- Execution
  - stream and stats are implemented and behaviorally valid
  - run exists when algorithm result model naturally supports it
  - mutate/write are implemented when output can be written to node/relationship properties

- Progress
  - progress tasks reflect algorithm phases where applicable
  - controller logs progress at meaningful units (node, relationship, iteration, phase)

- Memory
  - estimate_memory is non-placeholder and tracks core runtime structures
  - concurrency-dependent memory is represented where relevant

- Testing
  - algorithm-focused integration tests cover core graph shapes and edge cases
  - mode behavior (stats/mutate/write where implemented) has coverage

## Implementation readiness matrix

Scoring scale (0-5):
- 0 = absent
- 1 = mostly stubs/placeholders
- 2 = partial and inconsistent
- 3 = broadly implemented with notable gaps
- 4 = strong and mostly consistent
- 5 = complete and production-coherent

Snapshot basis:
- procedure mode inventory in gds/doc/PROCEDURE_MODES_INVENTORY.md
- recent parity/upgrade passes across Community

| Family | Config | Execution | Progress | Memory | Testing | Overall | Notes |
|---|---:|---:|---:|---:|---:|---:|---|
| Foundational: Pathfinding | 3 | 2 | 4 | 3 | 3 | 3 | stream/stats broadly present; mutate/write mostly stubbed |
| Foundational: Similarity | 2 | 2 | 3 | 1 | 2 | 2 | mutate/write sparse, estimate_memory mostly missing |
| Classic Core: Centrality | 3 | 2 | 4 | 4 | 3 | 3 | strong progress + memory coverage, mutate/write largely stubbed |
| Classic Core: Community | 4 | 4 | 4 | 4 | 4 | 4 | active parity work; mode surface and tests now substantially stronger |
| Learned/Modeling: Machine Learning | 2 | 2 | 2 | 2 | 2 | 2 | needs dedicated inventory and parity scoring pass |
| Learned/Modeling: Embeddings | 2 | 2 | 2 | 2 | 2 | 2 | needs dedicated inventory and parity scoring pass |
| Orchestration: Pipelines | 3 | 3 | 3 | 2 | 2 | 3 | rich surface area, but requires explicit consistency audit |

Readiness interpretation:
- Overall 4-5: maintain and harden
- Overall 3: targeted completion work needed
- Overall 0-2: foundational completion work required

Near-term actions from this matrix:
1. Complete remaining Community parity sweep and convert residual stubs.
2. Run a Foundational campaign for Similarity and Pathfinding mutate/write plus estimate_memory coverage.
3. Run a Centrality mode-completion pass focused on mutate/write while preserving current progress quality.
4. Produce explicit mode inventories for Machine Learning, Embeddings, and Pipelines matching the same rubric.

## Standard upgrade workflow

For each target algorithm:

1. Parity baseline
- Compare Java reference behavior/config/progress/memory against current Rust implementation.

2. Boundary check
- Confirm procedure -> storage -> computation control flow is intact and coherent.

3. Focused implementation
- Implement the smallest set of changes to close meaningful parity gaps.

4. Validation
- Run:
  - cargo fmt -p gds
  - cargo check -p gds
  - cargo test -p gds <algorithm-filter>
  - cargo test -p gds community (or relevant family filter)

5. Record decisions
- Update tracking docs with what was aligned, deferred, or intentionally different.

## Priority ordering for near-term completion

1. Finish Community family parity sweep (in progress)
2. Foundational pass on Similarity and Pathfinding
3. Centrality mode and parity uplift
4. Machine Learning and Embeddings consolidation
5. Pipeline compatibility and integration polish

## Practical checklist for each procedure file

- facade docs accurately describe current behavior (no stale placeholders)
- config docs map to actual behavior
- validation rejects invalid values clearly
- progress task shape matches algorithm phase structure
- mutate/write behavior is implemented or explicitly deferred with rationale
- estimate_memory is coherent with runtime structures
- integration tests assert behavior, not only existence

## Related resources

- gds/doc/PROCEDURE_MODES_INVENTORY.md
- gds/doc/ALGORITHMS_UPGRADE_TRACKING.md
- gds/doc/JAVA_PARITY.md
- gds/doc/COMMUNITY-ARCHITECTURAL-REVIEW.md
- gds/doc/CENTRALITY-ARCHITECTURAL-REVIEW.md
- gds/doc/PATHFINDING-ARCHITECTURAL-REVIEW.md
- gds/doc/SIMILARITY_ARCHITECTURAL_REVIEW.md
- gds/doc/PIPELINE_FACADE_ARCH_NOTE.md
