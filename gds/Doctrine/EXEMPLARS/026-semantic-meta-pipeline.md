# Exemplar 026 — Semantic Meta Pipeline as End-Stage Concept

**Source file**: `gds/examples/collections_semantic_meta_pipeline.rs`
**Arc position**: End-stage Concept: Corpus:LM receiving Form Program Features
**Prior exemplar**: [025 — GraphFrame Catalog Write as Graph Artifact](025-graphframe-catalog-write.md)
**Next exemplar**: [027 — Model:Feature::Plan as Essence Middle](027-model-feature-plan-middle.md)

---

## Principle

The Semantic Meta Pipeline is the End moment of the Dataset system. DataFrame remains the real runtime body, Dataset supplies the Meta Plan, and `SemDataset` holds the Corpus:LM pair where Form Program Features become logical forms. This is the agential access path for semantic graph work.

---

## What This Example Does

It builds the smallest complete semantic end-view:

1. Creates a `Corpus` from three sentences
2. Fits an `MLE` LanguageModel through `SemDataset::fit`
3. Ingests three `ProgramFeature` values: a Judgment, an Inference, and one malformed Principle form
4. Runs `parse_forms()` through the NLP semantic logic parser
5. Prints the parsed expressions and the retained parse error

The example is deliberately small. It shows the boundary, not the full future inference engine.

---

## The Arc Reading

```text
Text sources
  -> Corpus                         [evidence: documents and source identity]
  -> LanguageModel                  [meaning distribution over evidence]
  -> SemDataset                     [Corpus:LM end-view]
  -> ProgramFeature                 [Form Program Feature enters]
  -> SemForm                        [feature as semantic form]
  -> LogicParser                    [logical expression or parse error]
```

`SemDataset` is not a raw computation engine. It is the semantic controller where evidence, fitted language state, and logical form meet.

---

## Semantic Graph and Compute Graph

This exemplar names the platform split:

| Aspect | Platform | Role |
|---|---|---|
| Nama | Semantic Graph | meaning, forms, plans, provenance |
| Rupa | Compute Graph | GLM/GNN scale, projection, training |

The Semantic Graph platform can summon the Compute Graph platform through DirectCompute. It does not become DirectCompute. This preserves the agential access method: the Form Program Feature.

---

## Why This Comes After GraphFrame Catalog

Exemplar 025 showed graph artifacts entering the Dataset catalog as named retrievable structures.
Exemplar 026 turns inward to the End-stage semantic controller: the place where the Dataset pipeline can read its own Program Features as logical forms.

GraphFrame teaches graph artifact body.
`SemDataset` teaches semantic graph meaning.

---

## Key Vocabulary

**`SemDataset`** — The Corpus:LM end-view plus semantic forms. See [Semantic Pipeline](../REFERENCES/collections-dataset/semantic-pipeline.md).

**`SemForm`** — A `ProgramFeature` inside the Semantic Pipeline, with parse result or parse error retained.

**`ProgramFeature`** — The atomic GDSL commitment. See [GDSL Program Features](../REFERENCES/gdsl/program-features.md).

**`Corpus`** — The evidence object of Dataset. See [Corpus Structures](../REFERENCES/collections-dataset/corpus-structures.md).

**`LanguageModel`** — The intensional counterpart fitted over Corpus text. See [Semantic Pipeline](../REFERENCES/collections-dataset/semantic-pipeline.md).

---

## Student Notes

- The malformed form is intentional. A semantic pipeline must retain failures as visible state. Silent failure would break provenance.
- `parse_forms()` is not proof. It is the first gate: logical readability.
- Future stages will connect this surface to model-theoretic evaluation and theorem proving. Those belong after the form is readable.
- When this pipeline needs GLM/GNN scale, it should dispatch to DirectCompute. The semantic controller should remain small and inspectable.

---

## What This Example Does Not Show

- Proof search through `ml::nlp::inference`
- Tarski-model evaluation through `ml::nlp::sem::evaluate`
- DirectCompute dispatch into GLM/GNN routines
- Code generation from a stable semantic Meta Plan
