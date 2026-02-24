# Ratio Compiler Workbook

Section: `COMPILER (Ratio domain scaffold)`
Status: seed_scaffold
Authority: workbook-to-compiler alignment notes

## Purpose

- Track how Ratio workbooks project into compiler-facing structures.
- Keep conceptual-language layer separate from kernel computation semantics.
- Capture deferred compiler decisions pending IDEA pass.

## Current pass assumptions

- TypeScript layer is orchestration/agent language for workbook and graph handling.
- Kernel-level mathematics/physics execution remains in GDS, not in TS workbook logic.
- Ratio seed workbooks are conceptual and source-anchored, not computational implementations.

## Deferred decisions

- Exact projection contract from TopicMap components to singular graph components for Direct/Inverse/Powers ratio.
- Encoding rules for `Genus | Species | Ordinary` entry roles in Ratio after IDEA pass.
- Compiler normalization rules across Quantum -> Ratio -> Measure boundaries.

## Input artifacts

- `RATIO-PART-A-WORKBOOK.md`
- `RATIO-PART-B-WORKBOOK.md`
- `RATIO-PART-C-WORKBOOK.md`
- `RATIO-IDEA-WORKBOOK.md`

## Review outcome

- review_pending
- notes: scaffold only; fill after IDEA pass and contract refinement.
