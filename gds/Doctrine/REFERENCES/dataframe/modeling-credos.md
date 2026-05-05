# Modeling Credos

GDS Collections is now in the model creation business. DataFrame and Dataset are
not only convenience wrappers; they are the beginning and mediation of a broader
modeling system.

These credos set review discipline for tabular, relational, and graph-facing
modeling work.

The models of interest are broad: Postgres, Neo4j, GML, TypeScript, Rust, JSON,
Zod, Polars/DataFrame, and selected linguistic model tools. Collections is the
high-level place where those model families become comparable and projectable.

---

## Credos

1. **Begin With A Body**
   Every model must identify the DataFrame body where content first becomes
   inspectable as rows, columns, dtypes, and expressions.

2. **Do Not Hide The Beginning**
   Sources, readers, adapters, SQL tables, and graph projections are not the
   first Dataset object. They must become or produce a DataFrame body before
   Dataset mediation can begin.

3. **Keep Mediation Explicit**
   Dataset must name what it adds to DataFrame: identity, artifact profile,
   Model, Feature, Plan, provenance, and program commitments.

4. **Respect The Plan Boundary**
   Plan controls what is retained for reflection. It does not exhaust the live
   DataFrame body and must state what it selects, projects, excludes, or defers.

5. **Model Across Backends Without Losing The Path**
   Postgres-style relational models and Cypher-style graph models are valid
   model bodies only when the path remains traceable:

   `DataFrame body -> Dataset mediation -> backend projection`

6. **Treat SQL And Cypher As Projections, Not Origins**
   SQL schemas, relational tables, property graphs, nodes, edges, and Cypher
   patterns are powerful projections. They do not replace the DataFrame/Dataset
   path; they specialize it.

7. **Preserve Provenance**
   A model without lineage is not doctrinally trustworthy. Every projection must
   retain source, specification, program, and generation context where possible.

8. **Expose Inspection Surfaces**
   Modeling APIs should produce readable reports: shape, schema, attention,
   modality, image, capability, and Shell trace. Hidden model magic is invalid
   for this system.

9. **Let Shell Govern Integration**
   Shell is the integration protocol. A model becomes platform-ready when Shell
   can locate its immediate body, mediated middle, semantic return, and PureForm
   return.

10. **Borrow Linguistic Tools Without Becoming Linguists**
   Parsers, taggers, tokenizers, and language models are useful published model
   forms, but they are not the center of the system. They enter GDS as tools
   only when they can be located inside the Dataset grammar.

11. **Generated Models Must Conform**
   Generated GDS models must follow the grammar:

   `Model:Feature::Plan -> Corpus:LM::SemDataset`

   A non-conformant external model may be useful as input, adapter, or evidence,
   but it is not yet a GDS model.

---

## Backend Reading

| Backend Style | What It Contributes | Required Discipline |
|---|---|---|
| DataFrame / Polars | immediate analytic body and lazy tabular execution | preserve shape, dtype, expression, and schema visibility |
| Postgres / SQL | relational persistence, constraints, views, joins, transactions | project from Dataset identity without hiding provenance |
| Neo4j / Cypher | property graph persistence and path queries | preserve Dataset feature names, graph labels, and artifact lineage |
| GML / Graph ML | graph-learning bodies, projections, and training surfaces | keep graph compute downstream of Dataset/Shell authority |
| TypeScript | agent/runtime/API-facing structural models | preserve generated schema identity and service boundaries |
| Rust | kernel-native typed models and execution structures | keep types aligned with Dataset grammar and Shell traceability |
| JSON | exchange form and portable artifact body | retain provenance and avoid anonymous untyped blobs |
| Zod | TypeScript validation and schema commitments | generate from, or round-trip to, Dataset model identity |
| NLP / Published LM Tools | tokenizers, taggers, parsers, language models | adapt into `Model:Feature::Plan -> Corpus:LM::SemDataset` rather than treating their native shape as authoritative |

The modeling system may expand across all of these, but the path must stay readable.

Linguistic tools are therefore credentials by usefulness, not by sovereignty.
They help us build the model grammar; they do not define it.

---

## Review Rule

When reviewing a new modeling feature, ask:

1. Where is the DataFrame body?
2. What does Dataset add?
3. What does Plan retain or defer?
4. Which backend projection is being targeted, if any?
5. How is provenance preserved?
6. Which report or Shell trace makes the model inspectable?
7. Does the generated model conform to `Model:Feature::Plan -> Corpus:LM::SemDataset`?

If the answers are not available, the model is not yet ready for doctrine.
