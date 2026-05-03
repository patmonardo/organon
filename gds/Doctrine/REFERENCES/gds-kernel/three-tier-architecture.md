# Three-Tier Architecture

This note defines the product-architecture split for GDS as a Knowledge Agent kernel.
It keeps semantic authority local while allowing large compute delivery at scale.

---

## Tier 1: Core Kernel (Rational Control)

Canonical line:

```text
DataFrame -> Dataset -> Shell -> PureForm
```

Role:

- DataFrame: immediate body
- Dataset: mediated identity, feature commitments, plan surface
- Shell: living doctrine runtime and traceability authority
- PureForm: executable return contract

This tier defines validity and method discipline. It is the rational system for
working with Collections.

---

## Tier 2: SemDataset Discourse Platform (NLP-First)

Canonical line:

```text
Dataset -> Model:Feature::Plan -> Corpus:LanguageModel -> SemDataset
```

Role:

- Home base for discourse, interpretation, and provenance-bearing semantics
- NLTK-inspired semantic processing (token/parse/form) inside typed Dataset surfaces
- Mathematical logic entry through ProgramFeature commitments and SemForm parsing

This tier is where "Dataset learns something" and becomes SemDataset.

---

## Tier 3: RealityFabric Delivery Layer (Scale and Extensions)

Role:

- Collections as configurable substrate for specialized storage
- Huge-array and graph-heavy extension patterns
- High-scale GML/GNN projection/eval pipelines for external clients

This tier can be enormous in compute magnitude and graph-store size.

---

## Governing Law Across Tiers

Scale does not become authority.

Even for massive GML/GNN delivery, MetaPipeline authority is defined by the containing
Dataset and validated by Shell traceability.

```text
Frame -> Model:Feature::Plan -> SemDataset -> PureForm return
```

Compactly:

- `HugeGraphStore != MetaPipelineAuthority`
- `ContainingDataset + ShellTrace = MetaPipelineAuthority`

---

## Strategic Reading

- Tier 1 protects method and validity.
- Tier 2 grows semantic intelligence and discourse capability.
- Tier 3 monetizes scale without surrendering semantic governance.

This is the stable split: principled kernel, learning semantic center, scalable delivery edge.
