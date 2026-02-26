# LOGIC COMPILER WORKBOOK

Part: compiler
Status: seed
Authority: `logic/src/compiler/idea/IDEA-WORKBOOK-PROCESS.md`

## Objective

Compile `logic-idea.txt` argument blocks into stable principle-network artifacts.

## Inputs

- Source text: `logic-idea.txt`
- Ledger: `IDEA-PRINCIPLES-LEDGER.md`
- Key tables: `IDEA-KEY-TABLES.md`
- Taxonomy map: `../genera-species-table.md`
- Idea workbook: `LOGIC-IDEA-WORKBOOK.md`

## Outputs

- Verified and expanded `pr-log-*` principle rows
- Verified and expanded `e-log-*` edge rows
- Coverage updates for `logic-idea`

## Block-Enclosed Compilation Units

| Block ID       | Source Scope                           | Kernel Targets         | Pedagogy Targets | Status |
| -------------- | -------------------------------------- | ---------------------- | ---------------- | ------ |
| log-intro-a    | General concept of logic               | pr-log-001, pr-log-002 | pe-log-001       | seed   |
| log-method-b   | Contradiction and determinate negation | pr-log-003, pr-log-004 | pe-log-002       | seed   |
| log-division-c | Division into Being/Essence/Concept    | pr-log-005             | pe-log-001       | seed   |

## Compilation Checklist

- Segment source into argument units.
- Map units to existing principle IDs where possible.
- Create new IDs only when no existing node fits.
- Add relation edges using allowed vocabulary.
- Record evidence anchors.
- Update coverage status.

## Build Log

| Step | Action             | Result                                                  |
| ---- | ------------------ | ------------------------------------------------------- |
| 1    | Segment            | Seed blocks created in workbook.                        |
| 2    | Map IDs            | Existing `pr-log-*` and `pe-log-*` linked.              |
| 3    | Add edges          | Existing `e-log-*` acknowledged; no new edge added yet. |
| 4    | Reconcile pedagogy | Pedagogy nodes mapped to blocks.                        |
| 5    | Final audit        | Pending detailed line-anchor pass.                      |

## Session

- Decisions: compiler workbook follows block-enclosed units as local operational surface.
- Open Questions: whether to split `log-intro-a` into two blocks after first annotation pass.
- Next Steps: annotate each block with precise line ranges and add second-order nodes.
