# Polars Upstream Sync Protocol

Purpose: keep DataFrame (UR DSL) aligned with fast-moving Polars Python surface,
then decide Dataset conformance from a stable DataFrame baseline.

## Why this matters

- Upstream Polars Python moves quickly.
- DataFrame aims to remain isomorphic to Polars Python DSL.
- Dataset should conform to DataFrame protocol (functions + expr + namespace layering),
  then extend explicitly where needed.

## Tool

- Run: `tools/polars_surface_diff.sh`
- Output: markdown diff on stdout.
- Example:
  - `./tools/polars_surface_diff.sh > /tmp/polars_surface_diff.md`

## Weekly cadence (minimum)

1. Refresh `ref/polars`.
2. Run surface diff script.
3. Classify changes as:
   - parity-critical (DataFrame must update now),
   - parity-neutral (naming/layout only),
   - out-of-scope (intentional divergence).
4. Apply DataFrame updates first.
5. Re-evaluate Dataset compatibility matrix after DataFrame baseline changes.

## Change classification rubric

- parity-critical:
  - expression namespace behavior changed,
  - function constructor signatures changed,
  - lazyframe/expr coupling changed.
- parity-neutral:
  - package/file movement with same public semantics.
- out-of-scope:
  - optional integrations not represented in UR DSL.

## Dataset rule during upstream churn

- Do not invent Dataset namespace/function semantics that conflict with DataFrame protocol.
- Keep namespaces thin.
- Prefer function helpers returning typed objects/Expr.
- Mark any semantic extension explicitly in FRAME-COMPATIBILITY-MATRIX.

## Related docs

- `MODULE-LOCATION-PLAN.md`
- `FRAME-COMPATIBILITY-MATRIX.md`
