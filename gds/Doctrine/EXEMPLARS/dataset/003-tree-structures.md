# Collections Dataset: Tree Structures

File: `gds/examples/dataset_tree_structures.rs`

## Principle

This exemplar teaches **syntactic and semantic trees** as the first structural articulation of text.
Trees embody the NLTK inheritance that grounds the Dataset layer. You learn how parse values,
tree traversal, tree DSL operations, and span indexing work together. Trees are not decoration;
they are the algebra for Observation-level semantic structure.

## What It Does

1. Parses text into dependency or constituent parse trees
2. Constructs trees as first-class Dataset objects (not just structures in memory)
3. Uses the tree DSL to traverse, filter, and project tree data
4. Emits a tree artifact table with spans, labels, and parent-child relationships
5. Shows tree-as-data-object, not tree-as-JSON-serialization

Trees here are Polars-backed. The tree operations are ergonomic projections over
a tabular tree representation. You see how semantic structure becomes inspectable,
filterable, and comparable.

## The Arc

**Stage 1 → Stage 2 in the Knowledge Agent pipeline**: Observation deepens into structural reading.

Where corpus readers showed "what is a source?", this exemplar shows "what structure
does a source have?" Trees are:
- The result of Observation parsing
- The preparation for Reflection (essence is hidden in tree structure)
- The first Dataset artifact that has internal topology

## Key Vocabulary

- **Tree**: a parse or semantic structure object. See [tree-structures.md](../../REFERENCES/dataset/tree-structures.md)
- **Parse value**: a typed tree node (constituent label, dependency label, syntax role). See [tree-structures.md#ParseValue](../../REFERENCES/dataset/tree-structures.md#ParseValue)
- **Span**: a document range `(start, end)` indexed into the corpus. See [spans.md](../../REFERENCES/dataset/spans.md)
- **Tree DSL**: the ergonomic operations on trees as tabular data. See [tree-dsl.md](../../REFERENCES/dataset/tree-dsl.md)

## Next Exemplar

**Next**: `dataset_feature_structures.rs`

This exemplar moves beyond structure into **semantic marking**: feature structures, unification,
and modality stamps. You'll see how the abstract tree becomes a marked, typed semantic object
that can be unified and evaluated. Feature structures are the bridge between syntactic trees
and Reflection essence.

## Notes for Students

**Watch for**: Trees here are not tree-in-memory objects. They are Polars dataframes with tree topology.
The distinction matters: tabular trees are inspectable, composable, and compilable.

**Try this**: Parse two sentences. Build their trees. Project different subtrees. Compare
the resulting artifact tables.

**Key insight**: Trees are the first semantic structure. Reflection will refine them further.
But trees already embody the shape that Reflection will articulate.
