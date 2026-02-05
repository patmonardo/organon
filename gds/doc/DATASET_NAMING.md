Dataset naming policy

Summary

- Use `Dataset` (PascalCase / UpperCamelCase) for internal type names (e.g., `pub struct Dataset { ... }`).
- Use `dataset` (snake_case) for module/file names (e.g., `dataset.rs`).
- Preserve external/quoted spellings (vendor docs, data releases) verbatim when quoting. Do not change third-party source text.

CI enforcement

- A small check script (`tools/check_dataset_naming.sh`) runs on push/PR to ensure forbidden spellings (`DataSet`, `dataSet`, `GdsDataFrame`, `gdsDataFrame`) do not appear in source files. Note: `GDSDataFrame` is an allowed exception.
- Documentation directories (e.g., `gds/doc/`, `logic/doc/`) are excluded so `DATASET` or other verbatim text can remain.

Exception

- `GDSDataFrame` (acronym "GDS" in all-caps) is a deliberate exception and should be used as-is. Avoid using `GdsDataFrame` or other casings in source files.

Guidance for contributors

- If you integrate data from an external API that uses `DataSet` or `dataset_object`, provide an adapter (e.g., `impl From<ExternalDataSet> for Dataset`) rather than adopting their spelling in our codebase.
- If you must reference an external brand or release that uses a different casing, quote it literally (e.g., "The FBI release used 'DataSet'").

If you'd like different patterns or stricter rules (e.g., block text in `.md`), say so and the check will be adjusted.
