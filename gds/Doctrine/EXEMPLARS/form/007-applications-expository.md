# Collections Dataset: Applications Expository

File: `gds/examples/form_applications_expository.rs`

## Principle

This exemplar teaches the Dataset-first Applications facade. The system does not expose internal algorithms as the user's first vocabulary. It serves named Semantic Datasets through catalog, schema, preview, feature evaluation, attention report, capabilities, external `.gdsl` artifact compilation, and materialization operations.

## What It Does

1. Creates a temporary Collections catalog root
2. Ingests a named semantic table as `corpus.semantic_notes`
3. Serves catalog, schema, and preview operations
4. Evaluates a feature and emits an attention report
5. Compiles a small external `.gdsl` source and materializes its Dataset artifacts
6. Removes the dataset and shows the catalog cleanly updated

The example narrates the service boundary. It is intentionally verbose because Applications is where doctrine becomes user-facing platform.

## The Arc

**Service surface for the whole arc**.

Applications is not a new ontology. It is the served face of Dataset doctrine. The arc remains the same:

Source enters through ingest. Observation appears through schema and preview. Feature evaluation prepares marks. external `.gdsl` artifact compilation materializes program intent. Procedure-like materialization emits derived artifacts.

## Key Vocabulary

- **Applications facade**: the service boundary for non-Rust users. See [applications-facade.md](../../REFERENCES/applications/applications-facade.md)
- **Collections service**: the Dataset-first TS-JSON surface. See [applications-facade.md#Collections](../../REFERENCES/applications/applications-facade.md#Collections)
- **Attention report**: a user-facing inspection record for feature evaluation. See [attention-reports.md](../../REFERENCES/dataset/attention-reports.md)
- **Materialization**: turning compilation images into served artifacts. See [artifact-materialization.md](../../REFERENCES/dataset/artifact-materialization.md)

## Next Exemplar

**Next**: `dataset_source_stdlib.rs`

After service comes supply. The stdlib exemplar shows curated resources entering the Dataset world as reliable source material for scientific workflows.

## Notes for Students

**Watch for**: Every request uses a service operation name, not a direct algorithm module. This follows the procedure/service boundary required by the architecture.

**Key insight**: Applications is not where doctrine is invented. It is where doctrine is made serviceable.

**Try this**: Change the ingested table columns and inspect how `datasetSchema`, `datasetPreview`, and `datasetEvalFeature` respond. The service surface should remain stable even as data shape changes.
