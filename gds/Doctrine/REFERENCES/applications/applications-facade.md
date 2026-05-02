# Applications Facade

The Applications facade is the serviceable boundary for users who should not call kernel internals directly.

---

## Rule

Applications and examples call procedures or services, never internal `algo` modules.

The service layer is responsible for:

- validation and normalization
- catalog access
- schema and preview responses
- feature evaluation
- attention reports
- GDSL compilation
- materialization of Dataset artifacts

---

## Collections

The Collections facade is Dataset-first. It serves named semantic datasets, not anonymous frames.

Typical operations:

- `datasetIngestTable`
- `datasetListCatalog`
- `datasetSchema`
- `datasetPreview`
- `datasetEvalFeature`
- `datasetFeatureAttentionReport`
- `datasetCapabilities`
- `datasetCompileGdslSource`
- `datasetMaterializeCompilation`

---

## Doctrine Rule

Applications does not invent semantic meaning. Dataset and GDSL own the meaning. Applications makes it serviceable.
