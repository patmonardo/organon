# Namespace Invariants (Nine-Moment Dataset SDK)

This reference defines the namespace contract used by Doctrine exemplars and generator inputs.

---

## Canonical Pipeline

```text
Frame:Series::Expr -> Model:Feature::Plan -> Corpus:Language::Semantics
```

The nine moments are not only conceptual. They are canonical module homes and should be treated as normative in examples and generated text.

---

## Canonical Module Homes

### Beginning

- `dataset::frame` (root module at `gds/src/collections/dataset/frame/mod.rs`)
- `dataset::series` (root module at `gds/src/collections/dataset/series/mod.rs`)
- `dataset::expr` (root module at `gds/src/collections/dataset/expr/mod.rs`)

### Essence

- `dataset::model`
  - `dataset::model::prep`
  - `dataset::model::exec`
  - `dataset::model::image`
- `dataset::feature`
  - `dataset::feature::featstruct`
  - `dataset::feature::role`
- `dataset::plan`

### Concept

- `dataset::corpus`
- `dataset::lm`
- `dataset::sem`

---

## Compatibility Policy

Compatibility shims remain for legacy callers where canonical and legacy names differ.

Examples:

- `dataset::model_prep` -> `dataset::model::prep`
- `dataset::model_exec` -> `dataset::model::exec`
- `dataset::model_image` -> `dataset::model::image`
- `dataset::featstruct` -> `dataset::feature::featstruct`
- `dataset::feature_role` -> `dataset::feature::role`
- `dataset::semantic` -> `dataset::sem`

Important exception:

- For same-name root modules (`frame`, `series`, `expr`), public paths already remain stable after folderization, so no separate same-name shim files are used.

---

## Generator Rule

Doctrine generators must emit canonical paths by default and should emit shim paths only when targeting explicit backward-compatibility examples.

Default references should prefer:

- `dataset::model::prep` over `dataset::model_prep`
- `dataset::feature::role` over `dataset::feature_role`
- `dataset::sem` over `dataset::semantic`

---

## Review Rule

When adding a new module or exemplar:

1. Place it in its canonical moment first.
2. Add a shim only if canonical and existing public path differ.
3. Keep prelude exports stable while migration is in progress.
4. Update Doctrine references when canonical homes change.
