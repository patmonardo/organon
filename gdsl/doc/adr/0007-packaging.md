# ADR 0007: Packaging (dataset-first, colocated manifests)

- Status: Accepted
- Date: 2025-09-06
- Owners: @pat

## Context
We need installable artifacts (“packages”) for datasets and, later, apps/pipelines. Authoring lives in domain packages (@reality, @logic, @model, @task, @core). GDSL is the Single Source of Truth (schema/validators/CLI), not the home of datasets.

## Decision
- Colocate manifests with their datasets in source packages.
- Use a minimal DatasetPackageManifest (id/name/version/kind/datasetId/units/assets/meta).
- GDSL ships the schema/validator and pack CLI; it does not import authoring packages.
- Views remain secondary projections and live under GDSL schema/views.

## Consequences
- + Clean dependency direction (GDSL upstream-only).
- + Reproducible, installable .tgz packages.
- - Assembly (collecting unit objects) happens in CLIs, not library code.

## CLI
- pack-dataset: validate manifest, stage manifest+assets, emit gdsl/dist/pkg/*.tgz.
- install (later): extract tgz into a packages/ folder for runtime use.

## References
- @organon/gdsl/src/packaging/dataset-manifest.ts
- @organon/gdsl/src/cli/pack-dataset.ts
