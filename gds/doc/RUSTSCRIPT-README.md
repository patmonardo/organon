# RustScript (Polars-first scripting layer)

RustScript is the emerging, Pythonic facade over Polars within GDS/Organon. The goal is to keep Rust usage expressive and minimal while still compiling to efficient Polars execution.

## Examples

- List namespace: [gds/examples/collections_list_namespace_rustscript.rs](../examples/collections_list_namespace_rustscript.rs)
- String namespace: [gds/examples/collections_string_namespace_rustscript.rs](../examples/collections_string_namespace_rustscript.rs)
- Array namespace: [gds/examples/collections_array_namespace_rustscript.rs](../examples/collections_array_namespace_rustscript.rs)

## Run

- `cargo run -p gds --example collections_list_namespace_rustscript`
- `cargo run -p gds --example collections_string_namespace_rustscript`
- `cargo run -p gds --example collections_array_namespace_rustscript`
