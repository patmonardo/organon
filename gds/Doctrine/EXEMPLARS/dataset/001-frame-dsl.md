# Collections Dataset: Frame DSL

File: `gds/examples/dataset_frame_dsl.rs`

## Principle

This exemplar teaches the **modern Dataset DSL surface** and the lightweight wrapper pattern.
You learn that Dataset is not merely a Polars wrapper, but a semantic boundary where names
are fixed and pipeline intent is authored _before_ runtime. The `.ds()` namespace reads like
modern Polars while keeping the Dataset semantic layer distinct.

## What It Does

1. Imports the Dataset prelude
2. Creates a named Dataset with `.ds()` namespace access
3. Constructs a pipeline with metadata expressions
4. Compiles the pipeline into a `DatasetCompilation` graph
5. Prints artifact records with metadata

The example is intentionally simple: no Reflection yet, no Principle evaluation, no Concept
formation. Just the ergonomic entry point where semantic programs begin.

## The Arc

**Stage 1 in the Knowledge Agent pipeline**: Source → Observation.

This exemplar lives at the threshold. You see:
- How a program is named
- How the Dataset namespace makes operations readable
- How pipeline metadata is captured
- How the program becomes a compiled artifact before execution

There is no logical depth here yet. That comes in later exemplars. This one teaches surface.

## Key Vocabulary

- **Dataset**: the semantic model builder layer above DataFrame. See [core-concepts.md](../../REFERENCES/dataset/core-concepts.md)
- **DatasetPipeline**: the named, typed pipeline envelope. See [core-concepts.md#Pipeline](../../REFERENCES/dataset/core-concepts.md#Pipeline)
- **Namespace pattern**: `.ds()` as ergonomic sugar on the modern Polars API. See [frame-dsl.md](../../REFERENCES/dataframe/frame-dsl.md)
- **DatasetCompilation**: the compiled graph of all pipeline stages. See [core-concepts.md#Compilation](../../REFERENCES/dataset/core-concepts.md#Compilation)

## Next Exemplar

**Next**: `dataset_source_corpus.rs`

This exemplar shows how the abstract DataFrame idea becomes concrete through corpus sources.
You'll see how real data sources (text files, streams, corpora) become named semantic objects
that carry lineage and source metadata. The difference between "load a file" and "materialize
a corpus" will become clear.

## Notes for Students

**Watch for**: The `.ds()` namespace is ergonomic sugar on top of the DataFrame DSL. The actual
computation is still Polars-backed expressions. The Dataset layer adds semantic boundaries and
metadata, not new algorithms.

**Try this**: Modify the metadata expressions in the pipeline. Build a second pipeline in the
same example. See how the compiled graph changes when you add more stages.

**Remember**: This exemplar is intentionally minimal. It is the surface. The depth comes next.
