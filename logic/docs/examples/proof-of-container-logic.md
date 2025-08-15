# Proof of Container Logic (by eating)

Goal
- Demonstrate: principles are fixed inputs; all mutation/propagation lives in E/P/R.

Recipe
1) Take a Context snapshot (version v1).
2) Create Entities and Properties keyed to { contextId, contextVersion: v1 }.
3) Create Essential Relation(s) (typed endpoints), mutate E/P/R; verify Context unchanged.
4) Bump Context version to v2; verify prior Property variations invalidate (recompute under v2).

Run (existing tests)
- pnpm -C ../../.. --filter @organon/logic test

Manual checks (suggested)
- Add a test that:
  - creates Context v1 → creates Property with { contextId, v1 } → asserts describe() shows v1
  - bumps Context to v2 → asserts prior Property describe() invalidates/recomputes for v2
```// filepath: /home/pat/VSCode/organon/logic/docs/examples/proof-of-container-logic.md
# Proof of Container Logic (by eating)

Goal
- Demonstrate: principles are fixed inputs; all mutation/propagation lives in E/P/R.

Recipe
1) Take a Context snapshot (version v1).
2) Create Entities and Properties keyed to { contextId, contextVersion: v1 }.
3) Create Essential Relation(s) (typed endpoints), mutate E/P/R; verify Context unchanged.
4) Bump Context version to v2; verify prior Property variations invalidate (recompute under v2).

Run (existing tests)
- pnpm -C ../../.. --filter @organon/logic test

Manual checks (suggested)
- Add a test that:
  - creates Context v1 → creates Property with { contextId, v1 } → asserts describe() shows v1
  - bumps Context to v2 → asserts prior Property describe() invalidates/recomputes
