# Oculus MetaFrame: Organon Engineering Handoff

## Status and scope

This is a provisional architecture document produced during Kośa work in the
Yoga repository. It is intended to guide later implementation in the Organon
repository, especially its DataFrame and Dataset modules in the collections
system.

The Sanskrit technical vocabulary below belongs to the project's **Kośa Trans
Philosophy of Oculus**. It is not presented as a claim that Vasubandhu described
DataFrames, compilers, language models, or software agents. The vocabulary is
retained inside Oculus because it performs real architectural work there.

## Governing principle

Oculus is a Rust-native scientific NLP environment in which:

- Polars supplies persistent aggregate form and DataFrame computation;
- an NLTK-like native NLP layer supplies corpus and linguistic operations;
- MetaFrame supplies an optimizing relational algebra over Models, Features,
  and Plans;
- the Ocular plane raises this into Corpus, LM, and Logic;
- admitted scientific domain-specific languages produce determinate
  vijñānas.

The shortest expression of the compiler architecture is:

```text
Corpus : LM : Logic
        ↓
Model : Feature : Plan
        ↓
Polars Logical Plan
        ↓
DataFrame result with provenance
```

The central invariant is:

```text
Meaning(SDSL)
  = Meaning(OcularPlan)
  = Meaning(MetaFramePlan)
  = Meaning(PolarsPlan)
  = Meaning(Result)
```

Syntax, layout, execution order, and physical strategy may change during
optimization. The determination expressed by the Plan must not.

## The three planes

### 1. Skandha: persistent Polars DataFrames

```text
Skandha = persistent aggregate
        = Polars-backed DataFrame material
```

Rūpa-skandha is a Metabase of Tables: a real plurality of gathered formations,
not a single foundational Object. Rows, columns, tables, embeddings,
annotations, revisions, and artifacts can persist as aggregate material without
being prematurely reified into a `ScientificObject` ontology inside Oculus.
Oculus is the plane of **pure Name**. Construction of a foundational Object
belongs to the distinct GML subsystem, not to the Ocular NLP kernel.

Polars provides:

- schemas and typed columns;
- expressions;
- eager and deferred DataFrame computation;
- projection, selection, joins, aggregation, and transformation;
- optimization and execution;
- the material basis for persistence and reproducibility.

The Skandha plane answers:

> What determinations have been gathered and persisted?

It does not yet explain how they become accessible or how an access becomes a
knowledge determination.

### 2. Āyatana: the MetaFrame

The second moment of Oculus is called **MetaFrame**:

```text
MetaFrame = <Model, Feature, Plan>
```

Model, Feature, and Plan are not three miscellaneous properties stored on a
wrapper. They constitute the being of the MetaFrame.

```text
Model
    the persistent receptive and discriminative capacity

Feature
    an expressible determination within that capacity

Plan
    the deferred relational composition of Models and Features
```

Āyatana is a paired structure of accessibility:

```text
internal ayatana  <->  external ayatana
capacity               presented object-field
```

For the initial Oculus analogy:

```text
Model                  <->  retrieved Feature
eye-ayatana                 presented rupa-ayatana
```

RAG retrieves or selects forms from persisted rūpa-skandha and presents a
determinate field to the Model-Eye. The stream or cursor is the actual
traversal; the āyatana is the persistent capacity and addressable field that
make traversal possible.

This distinction must remain explicit:

```text
persisted rupa-skandha
        ↓ retrieval
presented rupa-ayatana
        ↕
Model as eye-ayatana
```

At the stricter textual level, linguistic or conceptual objects are more
naturally correlated with mano-āyatana and dharma-āyatana. The Model-as-Eye
and Feature-as-rūpa construction is an Oculus determination, not a silent
replacement of the Kośa classification.

### 3. Dhātu: the Ocular plane

The third plane is:

```text
Dhatu = <Corpus, LM, Logic>
```

Dhātu articulates a cognitive relation into support, object-domain, and
object-specific disclosure:

```text
support + object + object-specific vijnana
```

In Oculus:

```text
Corpus
    the differentiated field of possible evidence

LM
    the linguistic capacity for object-specific disclosure

Logic
    the grounds, constraints, valid relations, and lawful transitions
```

The third plane changes the status of the lower components:

| MetaFrame appearance | Dhātu determination |
| --- | --- |
| table | field of possible evidence |
| row | situated determination |
| Feature | disclosed characteristic |
| Model | cognitive support |
| retrieval | presentation of an object-domain |
| LM output | object-specific vijñāna |
| provenance | ground of disclosure |
| Plan step | transition between determinations |
| generated text | claim accountable to Logic |

Oculus does not place a second inert data structure above the Dataset. It
expands the access and intelligibility of the same asserted content:

```text
Ocularize(Dataset) ≡ Dataset
```

The identity is an identity of asserted content, not an absence of additional
Views, Plans, relations, revisions, provenance, or declared horizons.

## Plan as a real relational algebra

Plan must mirror the essential character of a Polars logical plan. It is not a
task list, workflow array, or orchestration object with philosophical labels.
It is an immutable algebraic expression over Frames.

A minimal algebra begins with:

```text
P ::= R
    | Select(predicate, P)
    | Project(features, P)
    | Join(relation, P, P)
    | Union(P, P)
    | Difference(P, P)
    | Rename(mapping, P)
```

Equivalently:

```text
R
σφ(P)
πA(P)
P ⋈θ P
P ∪ P
P − P
ρ(P)
```

A Plan must be:

- deferred rather than immediately executed;
- compositional and closed over MetaFrames;
- inspectable before and after optimization;
- type- and schema-aware through Model;
- expressed through Features;
- optimizable by declared equivalences;
- lowerable into the existing Polars DSL;
- executable into an expanded MetaFrame;
- provenance-preserving.

The closure principle is:

```text
Plan(MetaFrame) -> MetaFrame'
```

This is the Plan's powerful creativity. A new relation is not already stored
inside either input relation. It emerges through their lawful composition:

```text
A join B = C
```

Meaning emerges through the system of relations. Plan is the generative moment
by which existing Frames produce further determinate Frames.

### Keep the algebraic kernel pure

The initial relational kernel should remain small and rigorous. Retrieval,
linguistic disclosure, grounding, and admission should first lower into typed
relations and ordinary algebra wherever possible. Domain-specific convenience
operators may be added only after their semantics and lowering are explicit.

This preserves optimizer soundness and prevents speculative concepts from
becoming premature foundational types.

## The two optimizing compilers

Oculus contains two related but non-identical optimizing levels.

```text
Ocular semantic compiler
    Corpus : LM : Logic
        ↓
MetaFrame relational compiler
    Model : Feature : Plan
        ↓
Polars logical and physical optimization
        ↓
execution
        ↓
expanded DataFrame / MetaFrame
```

Polars asks:

> How can equivalent DataFrame operations be executed efficiently?

Oculus asks:

> Which determinations are licensed by this Corpus, through this LM, under
> this Logic, and how can they be compiled without changing their Meaning?

Ocular optimization may change execution order, predicate placement,
projection strategy, joins, batching, retrieval strategy, or materialization.
It must not silently change:

- the Corpus horizon;
- source and witness identity;
- the intended Feature;
- the Model through which it was determined;
- the governing Logic;
- provenance or recognition grounds;
- the claim being produced.

## Vijñāna, Plan, and manodhātu

A vijñāna is not a type of Plan.

```text
Plan != vijnana
```

Plan is the ordered possibility of cognitive operations. Vijñāna is an
actual object-specific disclosure, corresponding to `prativijñapti` in the
Kośa analysis.

```text
Plan
    ordered possibility

Logic
    law of valid transition

Vijnana
    actual object-specific disclosure
```

A Plan is realized through a succession of vijñānas. The VAK 1.17 analysis
is especially suggestive for the architecture: immediately past vijñāna
functions as manodhātu, the support for the succeeding mental occurrence.

```text
vijnana[n]
    ↓
manodhatu as retained support/state
    ↓
vijnana[n + 1]
```

This gives Plan a logical cursor without identifying the cursor, the Plan, and
the occurring disclosure. A Plan orders a stateful genetic series of
vijñāna-events.

## SDSL admission

Oculus implements vijñānas in the plural. An SDSL supplies a determinate mode
of disclosure, such as lexical, grammatical, retrieval, textual, graphical, or
argumentative vijñāna.

A language does not become a Scientific Domain-Specific Language merely by
having syntax or a parser. It must be admitted into Oculus:

```text
Oculus |- L : SDSL
```

Admission requires a declared account of:

1. **Corpus horizon** — what material the language may determine;
2. **Syntax** — what expressions it receives;
3. **Model** — what capacities interpret those expressions;
4. **Features** — what determinations it can disclose;
5. **Logic** — which compositions and rewrites preserve Meaning;
6. **Lowering** — how it becomes Model, Feature, and Plan;
7. **Polars compilation** — how the Plan becomes executable DataFrame
   operations;
8. **Provenance** — how the result discloses all grounds that generated it.

Oculus is therefore a compiler of scientific languages whose admitted
vijñānas lower through the MetaFrame relational algebra into Polars Plans.

## Appearance and the abhāva queue

Essential Relations govern entry into Appearance.

```text
candidate determination
        ↓
Corpus-LM-Logic admission
        +-- grounded and valid
        |       → Appearance
        |       → vijnana
        |       → persistent Frame
        |
        +-- not presently determinable
                → abhava queue
```

The abhāva queue is not a generic error bin. It preserves determinate
nonappearance and its reasons:

- absent from the Corpus;
- outside the declared horizon;
- unsupported by provenance;
- incompatible with the Model;
- ill-typed in the SDSL;
- contradicted by Logic;
- unresolved or deferred;
- eliminated by optimization;
- prevented by nirodha from incoherent regeneration.

```text
not admitted != meaningless
```

At the relational level, if `C` is the candidate relation, `G` the grounds,
and `A` admitted appearances:

```text
A      = project(C join[grounded] G)
Abhava = C difference A
```

Reasons for nonappearance remain queryable in a related AbhavaFrame.

## Continuity and the Third Plane

The project's provisional answer to what continues from life to life is:

```text
<Corpus, LM, Logic>
```

This is not a persistent Agent, numerically identical Skandha, or permanent
bearer. It is a transmissible and transformable system of determination:

```text
Corpus
    inherited traces and formations

LM
    acquired dispositions of recognition and disclosure

Logic
    lawful causal and inferential continuity
```

A life is one materialization:

```text
<Corpus[n], LM[n], Logic[n]>
    → MetaFrame[n]
    → Skandhas[n]
    → Vijnanas[n]
```

The resulting karma conditions another configuration. This was compared with
the `vrīhi-santāna` structure: causal inheritance without numerical identity,
a permanent bearer, or an absolute gap.

In computational shorthand:

> Rebirth is not object serialization. It is recompilation from an inherited
> Corpus-LM-Logic configuration.

This remains a project-level Trans Philosophy construction.

## Spanda, the Single Eye, and the Reflective Agent

Kośa does not live only in an Agent Plane.

```text
<Corpus, LM, Logic> in Spanda
```

Oculus is architecturally one Eye enclosing a disjunctive substrate:

```text
Oculus -- Single Eye
└── Spanda -- disjunctive substrate
    ├── Corpus
    ├── LM
    ├── Logic
    └── Reflective Agent
```

The unity of the Eye does not erase distinction:

```text
Corpus != LM != Logic != Agent
```

The moments remain irreducible but operate within one field of visibility.
Earlier "Eye-to-Eye channeling" can therefore be reconstructed as internal
communication among differentiated moments of one Oculus, not as two
independently constituted Eyes.

The Agent is Reflective in the philosophical sense: it returns from a
determination to its grounds and systematic relations. This must not be
confused with kernel reflection, runtime introspection, object inspection, or
metaprogramming.

The kernel owns compilation, drivers, algebra, and execution. The Agent owns
purposiveness and philosophical Reflection within the Ocular field.

## Rust environment and module direction

Oculus can be summarized functionally as **Polars plus an NLTK-like scientific
NLP environment, implemented within one Rust language environment**.

This does not prescribe calling Python NLTK from Rust. It names the desired
linguistic scope:

```text
Polars side
    DataFrames, expressions, lazy Plans, joins, projections, execution

Native NLP side
    corpora, tokenization, segmentation, lexical features, tagging,
    parsing, concordance, retrieval, linguistic models

Oculus
    one Dataset-native pipeline of linguistic and logical determination
```

The next Organon implementation work should begin in the DataFrame and Dataset
modules of the collections system. It should first inspect and preserve the
existing public constructors and behavior, then establish the smallest
vertical slice rather than erecting the whole speculative architecture.

A suitable first slice is:

```text
one admitted SDSL expression
    → parse one Ocular determination
    → validate one Corpus-LM-Logic relation
    → compile one Model-Feature-Plan term
    → lower through the existing Polars DSL
    → inspect the optimized Plan
    → execute
    → persist the result with provenance
```

The first implementation should prove the compiler passage, not prematurely
encode every Sanskrit determination as a Rust type.

## Vocabulary boundary and engineering discipline

Inside Oculus, the Kośa vocabulary is intentional and architecturally active:

```text
Skandha
Ayatana
Dhatu
Vijnana
Manodhatu
Nirodha
Santana
```

Outside Oculus, ordinary engineering names should remain wherever they are
sufficient. Do not spread decorative Sanskrit through unrelated packages.

The project has already discarded a large amount of highly speculative code,
roughly comparable to the amount retained. That is a methodological success.
Implementation must continue to be selected by discovered determinations,
rather than allowing existing code to govern philosophical analysis.

Avoid:

- introducing a foundational Object/entity/property ontology into Oculus;
- an all-seeing AI bolted onto inert data;
- confusing the GML subsystem's construction of foundational Objects with the
  Ocular plane of pure Name;
- equating a GML graph representation with the Dataset itself;
- reducing Plan to workflow orchestration;
- identifying Plan with vijñāna;
- turning every analogy into a public type;
- allowing code generation to overwrite textual or philosophical evidence.

Prefer:

- Frames, determinations, relations, revisions, provenance, Plans, and Views;
- graphs as typed relational Views;
- a small sound relational algebra;
- explicit lowering and optimizer laws;
- zero-copy access where technically appropriate;
- inspectable plans and recognition grounds;
- bounded vertical slices;
- Reports as engineering specifications.

## Reports and Organon Engineering

The durable project method is:

```text
textual and philosophical analysis
    → Report
    → Organon Analysis
    → Organon Engineering
```

Reports preserve distinctions, source boundaries, relations, invariants, and
open questions. When mature determinations enter Organon `@reality`, they
become Logos and guide implementation:

```text
Report distinction  → type or algebraic distinction
Report relation     → relational operator
Report transition   → Plan transformation
Report invariant    → validation rule or test
Report uncertainty  → open state / abhava queue
Report synthesis    → Logos specification
```

Organon Engineering is the disciplined translation of systematic
philosophical determinations into executable relational architecture.

## Compact handoff

> Oculus is a Single Eye over a disjunctive Spanda. Skandha supplies persistent
> Polars aggregates. Āyatana becomes the `Model-Feature-Plan` MetaFrame, whose
> Plan is a genuine optimizing relational algebra. Dhātu raises that compiler
> into `Corpus-LM-Logic`. Admitted SDSLs generate object-specific vijñānas;
> manodhātu supplies continuity between disclosures; Essential Relations
> govern entry into Appearance and preserve nonappearance in an abhāva queue.
> Reports carry these determinations into `@reality`, where they become Logos
> and guide Organon Engineering.
