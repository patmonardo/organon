# GDSL Plan Language Proposal

Date: 2026-04-12

## Goal

Define a real GDSL plan language for ontological and epistemological programs.
The language should be symbolic-first, Dataset-native, and compiler-oriented.
It should not treat RDF/OWL syntax as the primary user surface, and it should
not collapse Features into numeric vectors or large-language-model embeddings.

The immediate architectural target is this:

- store language artifacts as source programs
- compile them into semantic IR
- lower them into Dataset/DataFrame artifacts
- preserve raw data as sensation while deriving appearances, concepts, marks,
  principles, and executable plans

This keeps the current Dataset SDK as the base platform and treats GraphFrame as
an analytic extension layer rather than the fundamental semantic substrate.

## Design stance

### 1. Symbolic-first, not LLM-first

Our system is not fundamentally a Large Language Model system.
LLMs may assist with proposing models, features, or object networks, but the
core language remains symbolic and plan-based.

- A Feature is not fundamentally a vector.
- A Feature is a Mark, a ground of cognition.
- A Model is not just a fitted parameter bundle.
- A Model is an articulated organization of appearances, concepts, and laws.

### 2. Standard language approach, not RDF serialization worship

OWL and SPARQL remain useful reference points, but they should not dictate the
surface syntax or the runtime substrate.

Borrow from them at the semantic level:

- OWL: ontology-level class/property/constraint semantics
- SPARQL: query patterns over structured semantic artifacts
- SHACL: validation/constraint language

But draw the surface language more heavily from:

- Datalog / Souffle: rule and derivation structure
- SQL / Malloy: data modeling and query ergonomics over tabular substrates
- MLIR / compiler IR practice: explicit staged lowering
- module-based language design: imports, typed declarations, scoped names

The practical rule is simple:

- triples are not the native mental model
- tables and typed artifact families are the native runtime substrate
- rules, constraints, and plans are first-class symbolic structures

### 3. Dataset-first ontology

The Dataset layer should remain the immediate semantic home.
Raw datastreams can persist as data qua sensation.
From them we derive empirical appearances.
From appearances we derive marks, concepts, objects, and principles.

This yields a cognitive stack:

1. Sensation: raw signals, streams, logs, measurements, text, traces
2. Appearance: structured empirical appearance abstracted from sensation
3. Mark: a feature or ground of cognition extracted from appearance
4. Concept/Object: a unity of marks under a determinate schema
5. Principle: lawful judgment about objects, relations, or conditions
6. Plan: an executable articulation of appearances, concepts, and principles

Kantian framing:

- the Analytic of Concepts and the Analytic of Principles provide the starting
  distinction
- but in the Hegelian development these are not final, separate regions
- the language should therefore be designed so ontology and epistemology are
  two moments of one self-mediating Absolute Concept

## Proposal: GDSL as a plan language

GDSL should be a textual language whose canonical meaning is a compiled plan.
JSON should remain a serialization and interchange format, not the primary
authoring surface.

### Core source-level units

A GDSL module should be able to declare:

- `source`: raw inputs and sensation-preserving assets
- `appearance`: empirical appearances abstracted from raw data
- `reflection`: provisional storage of the path from Being through the determinations of reflection
- `logogenesis`: explicit genetic pathway by which the plan stores and unfolds scientific inference
- `mark`: grounds of cognition extracted from appearances
- `concept`: typed unities of marks
- `judgment`: determinate articulation of concepts in appearance
- `syllogism`: mediated inferential movement that binds judgments into science
- `relation`: lawful relations among concepts or appearances
- `principle`: rules, constraints, and inferential laws
- `query`: interrogations over the semantic space
- `procedure`: executable orchestration or compilation targets
- `emit`: declared artifact outputs

### Minimal conceptual grammar

```text
module org.phenomenology.perception;

use std.dataset;
use std.logic;

source sensation sensor_stream : parquet("s3://bucket/stream.parquet");

appearance frame from sensor_stream {
  key observation_id;
  retain timestamp, channel, raw_value;
  derive signal_kind = classify(channel);
}

mark intensity on frame := normalize(raw_value);
mark confidence on frame := calibrate(channel, raw_value);

concept AppearanceUnit from frame {
  identity observation_id;
  mark intensity;
  mark confidence;
  mark signal_kind;
}

principle perceptual_coherence for AppearanceUnit {
  require confidence > 0.80;
  infer stable when intensity != null;
}

query stable_units :=
  select observation_id, intensity
  from AppearanceUnit
  where stable;

procedure compile_perception {
  emit ontology image;
  emit dataset artifacts;
}
```

This is only a sketch, but it shows the intended balance:

- declarative source structure
- explicit appearance layer
- explicit reflection and logogenesis storage
- explicit marks/features
- concept/object definitions
- judgment and syllogistic mediation
- rule/principle layer
- query and procedure layers

### Hegelian epistemology progression

For the Hegelian language direction, the epistemology side should not stop at
feature extraction.
The semantic progression should be legible in the language itself:

1. sensation
2. appearance
3. reflection
4. logogenesis
5. mark
6. concept
7. judgment
8. syllogism
9. absolute concept

This is the path by which marks become syllogistic through dialectical
evolution.
In implementation terms, this means the language should eventually be able to
compile not only marks and concepts, but also reflections, logo-genetic
sequences, judgments, and syllogistic mediations as typed artifacts.

### Provisional reflection and logo-genesis doctrine

We should implement this conservatively.

We do not yet need to claim a final and exhaustive executable account of how
Reflection flows in all its precision.
But we do need to store it explicitly.

That means the language should already have a place to record, at minimum, a
provisional genetic pathway such as:

- Being
- Identity
- Opposition
- Contradiction
- Ground
- Condition
- Mark

and then the further movement toward:

- Thing
- Law
- EssentialRelation

For now, these can be treated as stored semantic stages and provenance-bearing
plan artifacts rather than fully formalized evaluators.
The important thing is that logo-genesis itself is not left implicit.

## Ontology and epistemology as one Absolute Concept

The language should not ultimately split ontology and epistemology into two
separate worlds.

At the source and tooling level, it is useful to distinguish:

- what kinds of objects may count as objects of cognition
- by what marks, appearances, and principles they are known

But the semantic truth of the language should be stronger than that practical
distinction.

Ontology and epistemology should be treated as two moments of one Absolute
Concept.

### Ontological moment

The ontological moment says what the object is in its conceptual determinacy.

- concept declarations
- relation declarations
- identity and cardinality constraints
- type hierarchies
- domain/range and composition rules

### Epistemological moment

The epistemological moment says how the object is given, marked, judged, and
validated in appearance.

- appearance definitions
- reflection pathways
- logogenesis pathways
- mark derivations
- principle blocks
- evidence rules
- confidence, provenance, and support
- procedural strategies for judgment, filtering, or validation

### Why the unity matters

These are not two independent DSLs glued together afterward.

The same GDSL plan should say both:

- what an object is
- how it appears
- by what marks it is known
- under what principles it is judged

That is the Hegelian correction to a merely Kantian split.
The language may preserve separate declaration forms for clarity, but the IR
should join them into one semantic object: an Absolute Concept in execution.

## What is a DSL Knowledge Graph?

We should be careful not to collapse several distinct things into one phrase.

A "DSL Knowledge Graph" is not simply one pure store that holds everything at
once in the same way.
For Organon, the more accurate architecture is a coupled system with at least
two analytically distinct but genetically inseparable graph-like surfaces.

### 1. Planning FormDB

This is the more ontological side.
It is the planning and rational-structure store.

Its job is to store:

- stable forms and form-shapes
- concept and relation blueprints
- signatures, facets, tags, and classification metadata
- handles and references for appearance-bearing entities
- planning-level links between universals, contexts, morphs, and operations

This is close to what the TS Agent platform already means by a knowledge graph.
It is not primarily an execution trace and not primarily a compiled DataFrame
artifact store.

In this sense, the planning FormDB is more ontological: it stores what the
system takes to be structurally and rationally there.

### 2. Semantic Plan IR

This is the more epistemological side.
It is the compiled plan graph that expresses how the object is known,
constructed, judged, and mediated in execution.

Its job is to store:

- sources and sensation assets
- appearance derivations
- mark extraction logic
- concept bindings inside a plan
- judgment and syllogistic mediations
- principle blocks, evidence, and provenance
- query plans and emitted artifact targets

This is what the current Dataset/GDSL compiler direction is already beginning to
do. It is a top-level graph of semantic planning, but it is not yet the whole
of the ontological system.

### Why both are needed

If we store only the planning FormDB, we lose the executable epistemic pathway.
We know what the object is supposed to be, but not how the system actually
comes to know and validate it.

If we store only the Semantic Plan IR, we lose the more stable ontological
surface. We know the plan of knowing, but not the persistent rational blueprint
that the TS agent platform needs as its living KG.

So we need both:

- Planning FormDB: ontological and structural
- Semantic Plan IR: epistemological and inferential

The right architecture is not one graph instead of the other.
It is their disciplined coupling.

### Genetic method

That disciplined coupling is another way of expressing the genetic method.

The point is not merely that two layers happen to coexist.
The point is that the epistemological plan must be generated from, refer back
to, and progressively determine the ontological structure, while the ontological
structure must already contain the conditions under which that inferential plan
can arise.

So the distinction is real, but only as a distinction internal to one living
process.

- FormDB without Semantic Plan IR is static blueprint without genesis.
- Semantic Plan IR without FormDB is inferential motion without stable objectivity.

The genetic method therefore means:

- analytically distinguish the moments
- preserve their internal generation of one another
- refuse to flatten them into one schema
- refuse to separate them into unrelated stores

### Working formula

The simplest formula is:

- FormDB stores the planned objectivity of the language
- Semantic Plan IR stores the planned subjectivity of scientific inference
- Dataset/DataFrame artifacts store the materialized execution surface

In Hegelian terms, these are not alien regions.
They are different moments of one Absolute Concept, but they should not be
flattened into one physical store or one undifferentiated schema.

They are implementation details only in the strongest sense: the details are
where the Absolute Concept actually has to work.

### Storage doctrine

For current implementation purposes, we should treat the persistence picture as:

1. `FormDB` stores planning-level ontological structures and rational handles.
2. `SemanticPlanIR` stores executable epistemological plans.
3. `DatasetCompilation` and materialized dataset artifacts store lowered runtime
   images for inspection and execution.

That means the question "what do we store?" has a layered answer, not a single
answer.

- In the TS agent platform, we store the planning graph.
- In the compiler/toolchain path, we store semantic plan IR.
- In the Dataset runtime, we store materialized artifacts.

### Bridge requirements

The bridges between these layers should be explicit.

At minimum we will eventually need:

- `FormDbPlanRef` or equivalent handle from plan IR into FormDB shapes
- `SemanticPlanRef` or equivalent handle from FormDB planning nodes into the
  compiled epistemic plan
- provenance links from dataset artifacts back to both the FormDB node set and
  the semantic plan nodes that generated them

This is the safer answer than trying to call the whole thing "the DSL KG" and
letting the store boundaries blur.

### Implementation consequence

This is difficult engineering precisely because the distinction and the unity
must both be preserved at once.

The hard parts are not decorative.
They are the core requirements:

- identity discipline between planning nodes, semantic plan nodes, and runtime artifacts
- provenance discipline across source, plan, FormDB, and Dataset materialization
- partial evolution, where ontology can deepen and force plan recompilation
- inferential feedback, where execution can enrich or revise planning metadata
- versioning and migration rules so the coupled layers do not drift apart

If there is a "devil in the details" here, it is exactly in these mediations.
This is why the implementation problem is so hard: we are not building a single
parser or a single graph store, but a genetic architecture for the Absolute
Concept in execution.

## Proposal: staged IR pipeline

The current codebase already has the beginning of this in:

- `ProgramFeatures`
- `ProgramCompileIr`
- `DatasetCompilation`
- `OntologyDataFrameImage`

But the larger architecture should now be understood as:

- Planning FormDB above and beside these compiler structures
- Semantic Plan IR as the bridge from source language to runtime artifacts
- DatasetCompilation as the lowering into dataset-native executable images

Those are useful, but they are still thin.
We should make the lowering pipeline explicit.

### Stage 0: Source text

The authored `.gdsl` file.

Purpose:

- human-readable plan source
- stable module system
- comments, imports, declarations, procedures

### Stage 1: Surface AST

Parse the textual program into a syntax tree.

Suggested top-level nodes:

- `ModuleDecl`
- `UseDecl`
- `SourceDecl`
- `AppearanceDecl`
- `ReflectionDecl`
- `LogogenesisDecl`
- `MarkDecl`
- `ConceptDecl`
- `JudgmentDecl`
- `SyllogismDecl`
- `RelationDecl`
- `PrincipleDecl`
- `QueryDecl`
- `ProcedureDecl`
- `EmitDecl`

This stage should preserve source spans and comments for editor tooling.

### Stage 2: Semantic Plan IR

Resolve names, types, scopes, imports, and declaration references.

Suggested structs:

- `SemanticPlan`
- `AbsoluteConceptPlan`
- `PlanSource`
- `PlanAppearance`
- `PlanReflection`
- `PlanLogogenesis`
- `PlanMark`
- `PlanConcept`
- `PlanJudgment`
- `PlanSyllogism`
- `PlanRelation`
- `PlanPrinciple`
- `PlanQuery`
- `PlanProcedure`

This is the first IR that knows the distinction between ontology and
epistemology while also binding them together as one executable semantic plan.

Key properties:

- names resolved
- types assigned
- dependencies explicit
- diagnostics attached
- ontology and epistemology linked inside one plan graph
- no backend-specific DataFrame operators yet

### Stage 3: Cognitive IR

This stage captures the repository-specific semantic categories more directly.

Suggested artifact families:

- `AbsoluteConceptArtifact`
- `SensationAsset`
- `AppearanceSchema`
- `ReflectionPathSchema`
- `LogogenesisSchema`
- `MarkSchema`
- `ConceptSchema`
- `JudgmentSchema`
- `SyllogismSchema`
- `RelationSchema`
- `PrincipleSchema`
- `EvidenceRule`
- `JudgmentPlan`
- `ProcedurePlan`

This is where the language stops being generic compiler structure and becomes a
language of cognition.

This is also the stage where the unity of ontology and epistemology should no
longer be implicit. The IR should contain an explicit Absolute Concept artifact
that gathers appearances, marks, concepts, relations, and principles into one
semantic whole.

This is also the right stage for distinctions like:

- sensation vs appearance
- reflection vs explicit logo-genesis
- appearance vs object
- mark vs measure
- concept vs empirical instance
- judgment vs syllogistic mediation
- principle vs query

### Stage 4: Dataset Compilation IR

Lower the cognitive IR into the existing Dataset plan/image infrastructure.

Current anchors in code:

- `DatasetNode`
- `DatasetNodeKind`
- `DatasetCompilation`
- `DatasetCompilationArtifacts`
- `OntologyDataFrameImage`

This stage should remain substrate-oriented and artifact-oriented.

Recommended extension:

- keep `DatasetNodeKind` for generic compiler machinery
- add richer metadata families for the ontological and epistemological moments
  of one Absolute Concept plan
- represent concepts, marks, principles, and appearances as typed dataset
  artifacts, not only as ad hoc metadata strings

### Stage 5: Execution/DataExpr lowering

Lower selected portions of the Dataset compilation into:

- DataExpr pipelines
- DataFrame transformations
- validation plans
- query plans
- search plans
- optional graph or GNN projections

This stage is operational, not source-semantic.

## What the IR needs to represent

At minimum, the IR needs to carry more than the current `ProgramFeatureKind`
set.

The current kinds:

- `ApplicationForm`
- `OperatorPattern`
- `Dependency`
- `Condition`
- `SpecificationBinding`

are too thin for a real language.

We likely need explicit representation of:

- source assets and sensation schemas
- appearance schemas
- reflection pathways
- logogenesis pathways
- mark definitions
- concept/object schemas
- relation schemas
- law/principle blocks
- evidence/provenance support
- query definitions
- procedure plans
- artifact emission targets

That does not mean the runtime has to explode in complexity.
It means the IR must stop hiding these distinctions inside generic strings.

## Recommended standard influences

If we want a standard language approach without becoming an RDF clone, the best
blend is probably:

### Best semantic sources

- OWL 2: ontology constraints, classification ideas
- SHACL: constraint validation model
- SPARQL: graph-pattern query lessons

### Better surface and rule sources

- Datalog / Souffle: rules, inference, derivation, recursion
- SQL / Malloy: query ergonomics and model-to-query flow
- dbt-style project structure: modular data declarations and build targets

### Better compiler sources

- MLIR: multi-stage lowering and dialect discipline
- typed AST + semantic analysis patterns from Rust/Scala compilers

In plain terms:

- ontology semantics from OWL/SHACL
- rule language from Datalog
- query ergonomics from SQL/Malloy
- compiler architecture from MLIR and standard typed-language practice

## Proposed artifact families in Dataset

The Dataset layer should eventually materialize at least these artifact kinds:

- absolute concept tables
- sensation tables
- appearance tables
- reflection tables
- logogenesis tables
- mark tables
- concept tables
- relation tables
- principle tables
- query specifications
- procedure specifications
- ontology images
- provenance and evidence tables

This preserves the existing artifact-profile direction already present in the
Dataset SDK while making it semantically richer.

## Planning FormDB and Semantic Plan IR

The proposal now needs one more explicit distinction.

### Planning FormDB should eventually materialize planning-side artifacts such as:

- form shapes
- concept blueprints
- relation blueprints
- context/property/morph planning nodes
- signatures and facets
- planning references into empirical/model systems

### Semantic Plan IR should eventually materialize epistemic-plan artifacts such as:

- appearance plans
- mark plans
- judgment plans
- syllogism plans
- principle plans
- evidence and provenance plans
- compilation and emission plans

These should be linked, but not conflated.

## First build target: Absolute Concept pilot

To test the current DataFrame and Dataset SDK, we should build a small vertical
slice instead of waiting for the full language.

The first pilot should compile one modest GDSL module into Dataset artifacts
that make the Absolute Concept visible as data.

### Proposed pilot flow

1. Ingest one raw source as sensation.
2. Derive one appearance table from it.
3. Derive a small set of marks from the appearance.
4. Bind those marks into one concept/object schema.
5. Apply one or two principles that validate or infer determinations.
6. Emit dataset artifacts for inspection.

### Concrete emitted artifacts

The pilot should materialize at least:

- one sensation dataset
- one appearance dataset
- one reflection dataset
- one logogenesis dataset
- one mark dataset
- one concept dataset
- one judgment dataset
- one syllogism dataset
- one principle/evidence dataset
- one absolute concept manifest dataset

### Why this is the right pilot

This directly exercises the platform we already have:

- DataFrame as substrate
- Dataset as semantic carrier
- compile IR as plan/image layer
- materialized artifact tables as inspection surface

And it tests the right philosophical claim in executable form:

- ontology is not somewhere else than epistemology
- the object is known only in and through its appearing and its marks
- the compiled plan should therefore manifest their unity as one Absolute
  Concept artifact family

## Why this is better than TS-JSON alone

TS-JSON is still useful, but only as one layer.

JSON alone is weak for:

- human authorship
- modular imports
- long-lived language ergonomics
- rule writing
- diagnostics and source spans
- readable conceptual structure

The right path is:

1. textual GDSL source for authorship
2. JSON schema for interchange and storage when useful
3. typed semantic IR for compilation
4. Dataset/DataFrame artifacts for execution and inspection

## Immediate implementation direction

This proposal is intentionally conservative.
It does not require a complete language implementation now.

The next practical steps could be:

1. Define a minimal textual grammar for `source`, `appearance`, `mark`,
  `concept`, `principle`, `query`, and `procedure`.
2. Introduce a richer semantic IR in front of `ProgramFeatures`, centered on
  an `AbsoluteConceptPlan`.
3. Keep `ProgramFeatures` as a transitional compact artifact, not the final
   semantic language.
4. Extend `DatasetCompilation` metadata so appearances, reflections,
  logogenesis stages, marks, and principles become typed artifact families
  within one Absolute Concept artifact graph.
5. Build a small pilot module that compiles source -> appearance -> reflection
  -> logogenesis -> mark -> concept -> judgment -> syllogism -> principle
  into materialized dataset artifacts.
6. Keep JSON as serialization of AST or semantic IR, not as the sole authored
   language.

## Concrete pilot module

The first epistemology-language pilot for this proposal is captured in:

- `gds/fixtures/gdsl/absolute-concept-scientific-inference.gdsl`
- `gds/fixtures/gdsl/absolute-concept-scientific-inference.expected.json`

That pair should be treated as the initial reference surface for exercising the
Dataset/DataFrame stack with a Hegelian source program.

## Summary

The right GDSL is not primarily:

- an RDF syntax
- a JSON blob format
- an LLM prompt language
- a vector feature pipeline

The right GDSL is:

- a symbolic plan language
- ontology and epistemology unified as moments of one Absolute Concept
- source-authored, compiled, and materialized
- Dataset-native in runtime posture
- capable of preserving sensation, deriving appearance, and organizing marks,
  concepts, and principles into executable cognitive plans

That is the language architecture most consistent with the current Dataset SDK,
the current `compile_ir` direction, and the repository's broader symbolic-first
aim.
