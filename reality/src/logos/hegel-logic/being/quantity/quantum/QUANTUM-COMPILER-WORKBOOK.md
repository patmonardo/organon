# Quantum Compiler Workbook

Section: `COMPILER (Quantum domain scaffold)`
Status: seed_scaffold
Authority: workbook-to-compiler alignment notes

## Purpose

- Track how Quantum workbooks project into compiler-facing structures.
- Keep conceptual-language layer separate from kernel computation semantics.
- Capture deferred compiler decisions pending IDEA pass.

## Current pass assumptions

- TypeScript layer is orchestration/agent language for workbook and graph handling.
- Kernel-level mathematics/physics execution remains in GDS, not in TS workbook logic.
- Quantum seed workbooks are conceptual and source-anchored, not computational implementations.

## Deferred decisions

- Exact projection contract from TopicMap components to singular graph components for Number/Quantum/Infinity.
- Encoding rules for `Genus | Species | Ordinary` entry roles in Quantum after IDEA pass.
- Compiler normalization rules across Quantity -> Quantum -> Ratio boundaries.

## Input artifacts

- `QUANTUM-PART-A-WORKBOOK.md`
- `QUANTUM-PART-B-WORKBOOK.md`
- `QUANTUM-PART-C-WORKBOOK.md`
- `QUANTUM-IDEA-WORKBOOK.md`

## Review outcome

- review_pending
- notes: scaffold only; fill after IDEA pass and contract refinement.
