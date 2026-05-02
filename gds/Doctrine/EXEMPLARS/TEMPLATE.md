# Exemplar Template

Use this template for each example. Each exemplar should answer these questions clearly and preserve canonical namespace language.

---

# [Title of Example]

File: `gds/examples/[example_name].rs`

## Principle

State the high principle this example teaches. One paragraph. What does learning this teach you about the Knowledge Agent?

Example:
> This exemplar teaches the modern Dataset DSL surface and the lightweight wrapper pattern.
> You learn that Dataset is not just a Polars wrapper, but a semantic boundary where names
> and pipeline intent are fixed before runtime.

## What It Does

Narrate the example in 3-5 steps. What is the input? What stages does it traverse? What is the output?

Example:
> 1. Creates a named Dataset
> 2. Uses the `.ds()` namespace to access frame operations ergonomically
> 3. Constructs a pipeline with metadata and operations
> 4. Compiles the pipeline into a DatasetCompilation graph
> 5. Prints the artifact records

## The Arc

Locate this exemplar within the Reflection→Principle→Concept→Procedure arc. Which stage or transition does it exemplify?

Example:
> This exemplar is **Stage 1 in the Knowledge Agent pipeline**: Source → Observation.
> It shows the surface where programs begin authoring semantic intent, before any Reflection.
> There is no Reflection here yet; there is no Principle. The exemplar teaches the ergonomic
> entry point.

## Namespace Discipline

State canonical module homes when naming code surfaces. Prefer canonical paths over compatibility shims unless the exemplar is explicitly teaching backwards compatibility.

Reference:
- `Doctrine/REFERENCES/collections-dataset/namespace-invariants.md`

Example:
> Canonical path: `dataset::model::prep`.
> Compatibility path (legacy): `dataset::model_prep`.
> This exemplar uses canonical paths unless noted otherwise.

## Key Vocabulary

List 3-5 terms this exemplar teaches. Each term links to a reference doc.

Example:
- Dataset (see REFERENCES/collections-dataset/core-concepts.md)
- DatasetPipeline (see REFERENCES/collections-dataset/core-concepts.md#DatasetPipeline)
- Namespace access pattern (see REFERENCES/collections-dataset/frame-dsl.md)

## Next Exemplar

Name the exemplar the reader should study next, and why.

Example:
> **Next**: `collections_dataset_corpus_readers.rs`
>
> This exemplar shows how Observation moves from simple DataFrame loading into semantic
> corpus marking. You'll see how sources become named corpus objects that carry semantic intent.

## Notes for Students

Optional: special observations that help learning. Warnings about common mistakes. Hints about
what to look for.

Example:
> **Watch for**: the `.ds()` namespace is ergonomic sugar on top of the DataFrame API.
> The actual work is still Polars-backed expressions. The Dataset layer adds semantic boundaries, not computation.
>
> **Try this**: modify the metadata expressions. See how the compiled graph changes.
