# Collections Dataset: Compilation IR and Program Features

File: `gds/examples/collections_dataset_compile_ir.rs`

## Principle

This exemplar teaches how **Program Features become Dataset artifacts**. You learn the
compiler boundary: GDSL features (ProgramFeature) enter as semantic intent, and Dataset
artifacts (records, relations, properties) exit as compiled images. The compilation graph
is the visible compiler-IR that binds GDS kernel work to Dataset materialization.

## What It Does

1. Constructs a `ProgramFeatures` set from GDSL sources
2. Builds a `DatasetCompilation` graph with typed nodes
3. Traverses the compilation to show dependencies
4. Realizes artifact tables: records, relations, properties
5. Emits the final ontology image

This is where the abstract `ProgramFeature` (Import, Source, Observation, Reflection, etc.)
becomes the concrete artifact that can be stored, queried, and served through Applications.

## The Arc

**Threshold between Stage 2 (Observation/marking) and Stage 3 (Reflection/essence)**: The compiler view.

You are seeing the other side of what the form processor showed you. Where the form processor
showed how features are stamped, the compiler shows how those stamps become artifact records.
The two views are the same process, seen from different angles:

- Form processor view: "How do we mark this entity?"
- Compiler view: "What artifact records does that marking produce?"

## Key Vocabulary

- **ProgramFeature**: an atomic semantic unit (Import, Source, Observation, ...). See [program-features.md](../REFERENCES/gdsl/program-features.md)
- **DatasetCompilation**: the compiled graph of all features and their dependencies. See [compilation.md](../REFERENCES/collections-dataset/compilation.md)
- **DatasetNode**: a typed node in the compilation graph (Image, Model, Feature, Frame, ...). See [compilation.md#Nodes](../REFERENCES/collections-dataset/compilation.md#Nodes)
- **Artifact materialization**: the realization of features as tables (records, relations, properties). See [artifact-materialization.md](../REFERENCES/collections-dataset/artifact-materialization.md)

## Next Exemplar

**Next**: `collections_dataset_gdsl_absolute_concept.rs` (to be written)

This exemplar will show the **complete arc from GDSL to emitted procedure**. You'll see a
full scientific knowledge program: from source through reflection to principle evaluation to
procedure emission. The exemplar here showed the compiler machinery. The next one shows it
serving a complete knowledge program.

## Notes for Students

**Watch for**: The compilation graph is deterministic. The same ProgramFeatures input always
produces the same graph structure. Verify this by building the same program twice and comparing
artifact IDs.

**Key insight**: The compilation graph is NOT the execution plan. It is the _declaration_ of what
the program means. Execution happens later, driven by the GDS kernel. The compilation graph is
the bridge between declarative GDSL intent and executable kernel operations.

**Try this**: Modify a program feature (add a mark, change a dependency). Rebuild the compilation.
See how the artifact graph changes. This teaches you how GDSL changes flow into artifact structure.

**Critical**: Every artifact record carries provenance (source_id, specification_id, program_name,
generated_at). This is not a convenience. It is the foundation of absolute interpretability.
