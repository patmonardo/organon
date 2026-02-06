# Collections Macros Design

Purpose

This document captures design guidance and examples for macros placed under
`gds/src/collections/dataframe/macros.rs`. It focuses on patterns that make
macros ergonomic, maintainable, and safe for building higher-level DSLs (e.g.
GraphFrame / Dataset DSLs that will build on top of the DataFrame primitives).

Goals

- Provide ergonomic call-sites (short, readable syntax) without sacrificing
  compile-time error quality.
- Keep macros focused on _call shaping_ and token normalization; move behavior
  and runtime logic into traits/generic APIs whenever practical.
- Make expansions predictable, small, and auditable for easier debugging.
- Provide clear testing and example material that documents supported forms.

When to use macros vs traits/generics

- Macros: reshape and normalize syntax (identifier -> string, prefix forms,
  alternate argument shapes, concise builder literal forms). Use macros when
  you want to change the surface syntax or accept multiple token forms.
- Traits/Generics: implement actual typed behavior, runtime polymorphism,
  operator semantics, and conversions. Expose a small, well-typed API surface
  that macros expand into.

Design patterns

1. Small arms, orthogonal matching
   - Keep each macro arm focused on one token-shape. Prefer multiple arms over
     complicated single-arm parsing. This improves error localization.
   - Example: `sc!` has two clear arms: bracketed-list and bare args.

2. Clear fallback / compile_error!
   - Add a final fallback arm that emits `compile_error!("...")` so users
     get meaningful messages for unsupported forms rather than confusing
     token-mismatch errors.

3. Hygiene and crate paths
   - Use fully-qualified crate paths (`$crate::...`) inside macro expansions to
     avoid accidental dependency on caller namespace and to ensure the macro
     works from integration tests.
   - Export macros with `#[macro_export]` if they will be used from
     integration tests or other crates.

4. Keep expansions small
   - Macros should expand into calls into the runtime API (traits, builders,
     simple constructors). Avoid putting large amounts of logic into the
     expansion itself.

5. Prefer token-tree (`tt`) arms where needed
   - Use `$tt` or `$($e:tt)+` when you need nested parenthesized forms. Combine
     with small, recursive arms to handle nested expressions (see `s!`).

6. Document accepted forms near the macro
   - Add short examples in comments above the macro and put longer expository
     usage in `gds/doc/` (not doctests if you prefer). This helps contributors
     extend arms without guessing intent.

Example: robust selector macro (patterns)

- Normalized selector list (suitable for `select_columns!` / `sc!`):
  - Accept: `id`, `"name"`, `(starts_with("pre"))`, `by_dtype([Int32, Float64])`
  - Implementation idea: small atom arms (ident, literal), and composite arms
    for `|`, `&`, `-` that build by calling `Selector` combinators.

Example: fallback arm with helpful error

```rust
(@push $vec:ident, $($tok:tt)+) => {{
    compile_error!("invalid selector expression for select_columns!; see docs")
}};
```

Example: small, safe `s!` pattern (already implemented)

- Single-operator prefix form: `( + a b )` -> `(expr!(a) + expr!(b))`
- Parenthesized recursion to allow nested forms.

Testing and examples

- Unit tests (inside crate): put `#[cfg(test)] mod tests {}` in the same
  module/file to exercise macro arms and private helpers. This is the best
  place to test macro forms that rely on private APIs.
- Integration tests: place tests under `gds/tests/` and call macros via the
  public crate API (requires `#[macro_export]`). Good for end-to-end checks.
- Expository examples: keep in `gds/doc/` as markdown examples (these are not
  compiled but are useful for onboarding and design discussion). We created
  `gds/doc/macros_dsl_examples.md` for that reason.

Migration path toward Dataset / GraphFrame DSLs

1. Start with macros that shape calls into well-defined builder/trait calls.
   Example: `dataset!` -> expands to `DatasetBuilder::new().with_schema(...).build()`.
2. Implement `SchemaBuilder` and `Field` helpers as typed APIs (traits + impls)
   so macros don't need to know runtime details.
3. Add small, public trait surfaces for conversions:
   - `impl From<Dataset> for GraphFrame` or `trait ToGraphFrame`.
   - Small adapters should be pure Rust (no macro magic) so they are testable.
4. Use macros for ergonomic convenience only (literals, parentheses, identifier
   conversion). Keep heavier logic in the trait/impl layer.

Examples: small macro -> trait split

- Macro: `schema!(id: UInt32, score: Float64)` expands to `Schema::from_iter(fields!(...))`.
- Trait: `SchemaBuilder` provides builders and validation methods invoked by the
  expanded code.

Extension ideas and constraints

- Compile-time validation: macros can validate shallow token shapes but cannot
  do type-level checks. For deeper checks, run them at runtime in builder
  constructors and emit helpful `Result` errors.
- Better error messages: capture incorrect macro usage quickly with targeted
  fallback arms that call `compile_error!` with actionable advice.
- Backwards compatibility: when evolving macro arms, keep old arms and mark
  deprecated forms in comments. Consider `#[deprecated]` on public helpers if
  there is an obvious runtime surface to attach it to.

Concluding notes

- Use macros to provide the ergonomic entry points your users want, then
  ensure the expanded calls map to a small, well-tested trait/generic API.
- Keep the macro file (`gds/src/collections/dataframe/macros.rs`) readable by
  organizing arms by feature area and documenting each arm with examples.

File references

- Macro definitions: gds/src/collections/dataframe/macros.rs
- Expository examples: gds/doc/macros_dsl_examples.md

---

If you want, I can:

- Add a short runnable example under `gds/examples/` demonstrating `s!` and
  `sc!` without being a test; or
- Add a unit test module inside `gds/src/collections/dataframe/macros.rs` that
  compiles simple forms (not doctests).

Which should I do next?
