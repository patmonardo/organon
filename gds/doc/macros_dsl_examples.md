# Macros DSL Examples

This short, expository document demonstrates the `s!` (Scheme-style prefix) and
`sc!` (Python-like select columns) macros defined in
`gds/src/collections/dataframe/macros.rs`.

These examples are illustrative only — they are not doctests or unit tests.
They show how the macros expand conceptually and how to use them in your code.

## `s!` — Scheme-style prefix expressions

`s!` is a small convenience wrapper around the existing `expr!` macro that
accepts prefix operator syntax. It expands to the same `Expr`-building calls
used elsewhere in the crate.

Examples:

```rust
// Arithmetic
let add = s!(+ score 10.0);     // expands to (expr!(score) + expr!(10.0))
let scaled = s!(* ( + score 2.0 ) 0.5); // nested: multiply the sum by 0.5

// Comparisons (use Expr methods)
let ok = s!(> score 20.0);      // expands to expr!(score).gt(expr!(20.0))
let eq = s!(== name "Alice"); // expr!(name).eq(expr!("Alice"))
```

Notes:

- Tokens may be identifiers, literals, or nested parenthesized forms.
- `s!` simply forwards to `expr!` and `Expr` operator/methods; use it for
  concise, Lisp-like expression composition.

## `sc!` — select columns shorthand

`sc!` is a thin shorthand for `select_columns!`. It accepts either a bracketed
selector list or bare selector arguments, e.g. `sc!(table, id, score)` or
`sc!(table, [id, "name"])`.

Examples:

```rust
// Using bare selectors
let view = sc!(my_table, id, score, "group");

// Using a bracketed selector list
let same = sc!(my_table, [id, score, "group"]);
```

Notes:

- `sc!` is exported with `#[macro_export]` and forwards to
  `select_columns!`, which uses the `selector!` helper to normalize identifiers
  and string literals into `&'static str` column names used by the runtime API.
- Use `sc!` when you want a compact, Python-like call site for selecting columns.

## Testing and usage tips

- Macros are expanded at compile time; to verify forms compile, either place
  small `#[cfg(test)]` unit tests alongside the macro (inside the same crate)
  or create integration tests under `gds/tests/` that call the macros via the
  crate API.
- If a macro arm should reject invalid input with a clear message, add a
  fallback arm that uses `compile_error!` to surface a friendly message.

## Where to look in the codebase

- Macro definitions: `gds/src/collections/dataframe/macros.rs`
- Table/selector APIs: `gds/src/collections/dataframe` (see `TableBuilder`,
  `select_columns`, and selector helpers)

---

If you want, I can also add a short `gds/examples` Rust file that shows these
macros in action as a small, runnable example (non-test). Want that next?
