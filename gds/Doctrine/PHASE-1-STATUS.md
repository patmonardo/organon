# Doctrine Reconstruction — Phase 1 Status

Date: 2026-05-01

## What Has Been Completed

### 1. Doctrine Structure Established ✓

```
gds/
  Doctrine/
    INDEX.md                    ← Navigation hub
    FOUNDING-CHARTER.md         ← Why this exists
    PRINCIPLE-FOUNDATION.md     ← The high principle
    EXEMPLARS/                  ← Canonical texts by fold
      TEMPLATE.md               ← What an exemplar looks like
      form/
      shell/
      dataframe/
      dataset/
    REFERENCES/                 ← Consulted docs
      form/
        pureform.md
        seven-moments.md
      shell/
        program-features.md
        grammar.md
      dataframe/
        dataframe.md
      dataset/
        core-concepts.md
      gds-kernel/
        zeroCopy-boundary.md
```

### 2. Foundational Documents Written ✓

- **FOUNDING-CHARTER.md**: The founding principle and migration plan
- **PRINCIPLE-FOUNDATION.md**: The Knowledge Agent arc and high covenant
- **INDEX.md**: Navigation and quick reference

### 3. Exemplar Companions Written (28/42) ✓

1. 001-frame-dsl.md — The DSL surface
2. 002-corpus-readers.md — How sources enter
3. 003-tree-structures.md — Syntactic and semantic structure
4. 004-featstruct-model.md — Essence preparation and marking
5. 005-compile-ir.md — Compiler-visible IR
6. 006-external-shell-program-artifact.md — External Shell program artifact specimen
7. 007-applications-expository.md — Dataset-first Applications service boundary
8. 008-stdlib-resources.md — Curated resources as source commitments
9. 009-json-semantic-form.md — JSON as observed semantic form
10. 010-xml-html-semantic-form.md — Markup as observed document tree form
11. 011-dataframe-macros-reflection.md — DataFrame macros as Reflection engine
12. 012-select-filter-principle.md — Select/filter as Principle evaluation
13. 013-series-concept-surface.md — Series as Concept surface
14. 014-expr-pipeline-judgment.md — Expr pipeline as Judgment
15. 015-order-group-syllogism.md — Order/group as Syllogism
16. 016-join-operations-inference.md — Joins as Inference
17. 017-streaming-dataset-procedure.md — Streaming Dataset as Procedure
18. 018-streaming-lazy-deferred.md — Streaming lazy as deferred collection
19. 019-expr-basic-judgment-grammar.md — Expr as reusable Judgment grammar
20. 020-framing-chunking-pureform.md — Framing/chunking as PureForm Shape
21. 021-array-namespace-observation.md — Array namespace as fixed-arity Observation
22. 022-list-namespace-observation.md — List namespace as variable-arity Observation
23. 023-string-namespace-observation.md — String namespace as text-form Observation
24. 024-catalog-extension-persistence.md — Catalog extension as artifact persistence
25. 025-graphframe-catalog-write.md — GraphFrame catalog write as graph artifact
26. 026-semantic-meta-pipeline.md — SemDataset as the Corpus:LM Meta Pipeline
27. 027-model-feature-plan-middle.md — Model:Feature::Plan as the Essence Middle
28. 028-dataframe-intuition.md — DataFrame as Intuition and first formed body

Each exemplar:
- States its principle
- Narrates what it does
- Locates itself in the arc
- Defines key vocabulary (with links to references)
- Points to the next exemplar
- Includes notes for students

### 4. Foundational References Written (31 docs)

- **core-concepts.md**: Dataset, DataFrame, Mark, Feature, Pipeline, Compilation, Artifact, Provenance
- **seven-moments.md**: The 7-moment Reflection sequence and the Absolute Reflection threshold
- **zeroCopy-boundary.md**: The kernel-agent contract at Principle evaluation
- **program-features.md / grammar.md**: Shell program envelope and external artifact reading rule
- **principle-evaluation.md / concept-emergence.md**: Principle-before-Concept doctrine
- **hegel-objectivity.md**: Procedure as Process
- **applications-facade.md**: Dataset-first service boundary
- **dataframe / dataset references**: immediate DataFrame body, frame DSL, corpus, readers, provenance, structured sources, stdlib resources, tree structures, feature structures, unification, modality, ontology images, compilation, materialization, and attention reports
- **semantic-pipeline.md**: SemDataset as the Semantic Graph Meta Pipeline
- **model-feature-plan.md**: Model:Feature::Plan as the Essence Middle
- **dataframe-intuition.md**: DataFrame as Intuition and first formed body

---

## What Remains

### Phase 2: Complete Exemplar Coverage (14 exemplars remaining)

- 28+: remaining catalog, selector, GraphFrame, and support surfaces
- ML exemplars remain paused until the ML/NLP layer is refreshed
- Continue updating references as the exemplar sequence expands

**Effort**: ~2 hours per exemplar (write companion, update references)
**Timeline**: 2-3 weeks at steady pace

### Phase 3: Complete Reference Library (remaining focused docs)

**Still to be written or expanded**:
- Relative vs. Absolute Reflection
- Judgment logic
- Syllogism and middle terms
- Procedure service patterns
- Kant: Synthesis in Observation
- PureForm kernel shape contract
- Graph projection as generated body

**Effort**: ~1.5 hours per doc
**Timeline**: 2-3 weeks

### Phase 4: Archive Old Documentation

- Move 100+ existing docs to `doc_archive/`
- Create `doc_archive/INDEX.md` with mapping to new locations
- Do NOT delete; preserve history
- Link from Doctrine when old context is relevant

**Effort**: ~4 hours
**Timeline**: When Phase 2 exemplars are mostly complete

### Phase 5: Quality Pass

- Read the entire exemplar sequence as if learning for the first time
- Fix clarity issues
- Verify all links work
- Add missing references based on questions

**Effort**: ~8 hours
**Timeline**: End of May

---

## Quality Metrics

The Doctrine is successful when:

1. **Exemplar sequence is readable**: A new reader understands the system by reading exemplars 1-12 in order
2. **References are targeted**: Every reference doc answers exactly one question, not multiple
3. **Provenance is visible**: A user can trace any artifact back to its source and Principle decision
4. **ZeroCopy is canonical**: The Absolute Reflection boundary is the strongest part of the system, not the weakest
5. **Voice is singular**: It reads as if written by one mind, not a committee
6. **Arc is inviolable**: All examples and references reinforce the same sequence: Reflection → Principle → Concept → Procedure

---

## High Principles (from FOUNDING-CHARTER.md)

1. **Examples are the canonical exposition**. All other docs serve them.
2. **Each exemplar teaches one principle**. No exemplar tries to do everything.
3. **References are stable definitions**. They do not narrate; they define.
4. **Provenance is mandatory**. Every artifact knows where it came from.
5. **The arc is canonical**. There is only one arc; all programs follow it.
6. **ZeroCopy is sacred**. The kernel-agent boundary is deterministic and auditable.
7. **One voice**. Not scattered notes. Not committee writing. Constitution.

---

## Next Immediate Action

**This week**:
- User review of the 25 exemplar companions and expanded reference shelf
- Revise for clarity and doctrine alignment
- Fix links and references
- Add remaining catalog, selector, GraphFrame, and refreshed ML/NLP exemplars next

**The work begins immediately.**

The docs are no longer scattered. The Doctrine now stands.
