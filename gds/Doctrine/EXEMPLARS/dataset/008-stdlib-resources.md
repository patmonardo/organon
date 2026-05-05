# Collections Dataset: Standard Library Resources

File: `gds/examples/dataset_source_stdlib.rs`

## Principle

This exemplar teaches curated resources as semantic source material. A Dataset stdlib is not a bag of downloads. It is a controlled resource ecology where commonly used corpora, archives, and reference material can enter the Knowledge Agent pipeline with stable names and reproducible local paths.

## What It Does

1. Lists configured Dataset stdlib resources
2. Chooses either a requested resource or the first available resource
3. Fetches the resource through the stdlib resource interface
4. Prints archive path, data directory, and byte count

The example is small because the doctrine is simple: reliable sources come before reliable Observation.

## The Arc

**Stage 1: Source.**

Before Observation, the system must know what its sources are and where they came from. The stdlib provides curated source availability. It is the supply layer for later scientific workflows.

## Key Vocabulary

- **Stdlib resource**: a named, reusable source package. See [stdlib-resources.md](../../REFERENCES/dataset/stdlib-resources.md)
- **Source provenance**: the stable origin and local materialization record. See [provenance.md](../../REFERENCES/dataset/provenance.md)
- **Resource report**: the fetch result that records archive path, data dir, and size. See [stdlib-resources.md#ResourceReport](../../REFERENCES/dataset/stdlib-resources.md#ResourceReport)

## Next Exemplar

**Next**: `dataset_io_json.rs`

The stdlib tells us how curated sources enter. JSON parsing shows how structured sources are lowered into semantic form rather than treated as opaque bytes.

## Notes for Students

**Watch for**: Fetching a resource is not yet Observation. It only makes a stable source available. Observation begins when the source is parsed, retained, derived, and marked.

**Key insight**: A scientific pipeline is only as trustworthy as its sources. Stdlib resources are part of doctrine because they discipline what counts as reusable material.
