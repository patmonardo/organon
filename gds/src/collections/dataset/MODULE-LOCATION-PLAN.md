# Dataset Module Location Plan

Purpose: define which Dataset modules keep a prized top-level location and which should move to folder modules (`mod.rs` + `core.rs`) as the DSL grows.

## Design Rule

- Prefer folder modules for evolving domains with multiple sub-surfaces.
- Keep top-level files only for stable shell surfaces or intentional barrel roles.
- For folder modules, keep `mod.rs` thin (activation + export wiring) and keep implementation in `core.rs`.

## Keep Top-Level (Intentional)

- `mod.rs`: dataset barrel + public surface policy.
- `prelude.rs`: curated stable user surface.
- `namespace.rs`: dataset DSL shell and registry bridge.

## Top-Level To Migrate (Planned)

These are likely to grow and should move to folder modules over time:

- `artifact.rs`
- `catalog.rs`
- `codegen.rs`
- `collocations.rs`
- `compile.rs`
- `dataset.rs`
- `error.rs`
- `grammar.rs`
- `graph.rs`
- `io.rs`
- `metrics.rs`
- `probability.rs`
- `registry.rs`
- `schema.rs`
- `streaming.rs`
- `text.rs`
- `tgrep.rs`
- `toolchain.rs`
- `valuation.rs`

## Already Folderized (Good)

- `corpus/`
- `expr/`
- `expressions/`
- `feature/`
- `frame/`
- `functions/`
- `language/`
- `lazy/`
- `logic/`
- `model/`
- `namespaces/`
- `plan/`
- `series/`
- `stdlib/`
- `utils/`

## Macros Decision

`macros.rs` is main DSL surface and evolving quickly.

Decision:

- Short term: keep `macros.rs` top-level to avoid churn during active DSL edits.
- Near term: migrate to `macros/` with `mod.rs` + `core.rs`.

Suggested future split under `macros/`:

- `core.rs` (shared macro internals + stable macro exports)
- `io.rs` (`io!`, `ds!`)
- `pipeline.rs` (`pipeline!`, `plan!`, `dop!`)
- `tree.rs` (`tree!`, `tleaf!`, `tpos!`, `troot!`, `tspan!`, `ttransform!`)
- `feature.rs` (`fpos!`, `frange!`, `fspec!`, `fspec_tree!`, `fcond!`, `ftemplate!`, `fexpr!`)
- `corpus.rs` (`corpus!`, `split!`)

## Migration Sequence

1. Migrate low-risk utility domains first: `io`, `error`, `metrics`, `registry`.
2. Migrate medium-size semantic domains: `artifact`, `schema`, `streaming`, `valuation`.
3. Migrate heavy compiler/DSL domains: `compile`, `toolchain`, `codegen`, `macros`.
4. Keep `mod.rs`, `prelude.rs`, `namespace.rs` top-level unless a clear architectural reason appears.

## Acceptance Criteria Per Move

- `cargo check -p gds` stays clean.
- Module root remains thin.
- No new ambiguous glob re-export warnings.
- Public API names remain stable unless explicitly renamed in the same PR.
