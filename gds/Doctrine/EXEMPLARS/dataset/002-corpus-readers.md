# Collections Dataset: Corpus Readers

File: `gds/examples/dataset_source_corpus.rs`

## Principle

This exemplar teaches how semantic intent enters the system through **corpus sources**.
You learn the difference between "load data" and "materialize a semantic source."
A corpus is not just rows; it is a named, annotated, provenance-tracked source that
the program depends on. Sources are the first Dataset artifact.

## What It Does

1. Loads text from files or standard corpora
2. Creates a named `Source` that tracks lineage
3. Constructs a corpus as a semantic object (not just raw text)
4. Emits a corpus Dataset artifact with schema metadata
5. Shows how source provenance flows forward through the pipeline

The example moves from abstract DataFrame operations to concrete textual sources.
You see how the Dataset layer gives sources names and keeps track of where they came from.

## The Arc

**Stage 1 in the Knowledge Agent pipeline**: Source → Observation (the threshold, now concretely realized).

Where the previous exemplar was abstract (just showing the DSL surface), this one is concrete.
You see:
- Real sources (text files, corpora)
- How they become named semantic objects
- How provenance is captured from the start
- How a corpus artifact is emitted

## Key Vocabulary

- **Source**: a named data origin (file, stream, corpus, API). See [core-concepts.md#Source](../../REFERENCES/dataset/core-concepts.md#Source)
- **Corpus**: a source organized as annotated, lineage-tracked documents. See [corpus-structures.md](../../REFERENCES/dataset/corpus-structures.md)
- **Provenance**: the lineage metadata attached to every artifact (source_id, specification_id, timestamp). See [provenance.md](../../REFERENCES/dataset/provenance.md)
- **SourceReader**: the adapter interface for ingesting concrete data. See [io-and-readers.md](../../REFERENCES/dataset/io-and-readers.md)

## Next Exemplar

**Next**: `dataset_tree_structures.rs`

This exemplar shows how corpus text is structured into syntactic trees. You'll move from
"what is a source?" to "what structure does language have?" Trees are the first semantic
structure that the Knowledge Agent imposes on raw text. They are generated artifacts of
Observation, preparing the way for Reflection.

## Notes for Students

**Watch for**: Corpus loading is deterministic. The same source should always produce the same
structure and provenance record. Verify this in your own experiments.

**Try this**: Load two different corpora. Compare their artifact records. See how metadata
and schema differ based on source type.

**Understand**: Sources are not just data ingest. They are the first dataset artifact.
Everything downstream depends on source identity and provenance being stable.
