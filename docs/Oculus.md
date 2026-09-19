# Oculus

## Return-to-Organon Plan for the DataFrame–MetaFrame Architecture

## Status

This document records the governing ideas and implementation plan for the return to the Organon in VS Codex. It is not a retrospective transcript. It is a working architectural instruction whose claims must be tested against the existing code before names or inheritance relations are finalized.

The central decision is:

> **An Organon Dataset is a Dataset equipped with an Oculus Feature.**

Practically, this begins from familiar computational objects: tabular datasets, graph datasets, statistical learners, corpora, language models, and logical constraints. Architecturally, however, the Organon Dataset is not reducible to any one of those species. It is a Dataset capable of sensing its empirical field through Features, relating those Features to a Model, forming a Plan, and enclosing the resulting Concept within a MetaFrame that preserves its Principle and rationale.

The shortest formula is:

\[
\boxed{
\text{OrganonDataset}
=
\text{Dataset}
+
\text{OculusFeature}
}
\]

## 1. The Oculus Principle

The Oculus is a living aperture between a Principle and its determinate field. It is not merely a viewer, query API, metrics dashboard, or observation hook. It provides a Dataset with the capacity to:

1. receive or select a field;
2. articulate what is sensed as Features;
3. recognize the Model expressed through those Features;
4. generate a Plan from the same Feature Grammar;
5. observe the result of execution;
6. return the result to the Model as evidence, correction, or confirmation;
7. preserve the grounds and conditions of this movement as a Reasoned Concept.

The Oculus therefore enacts a reciprocal movement:

\[
\text{Model}
\overset{\text{Feature Grammar}}{\longleftrightarrow}
\text{Field}.
\]

The outward direction is theoretical determination becoming a practical rule:

\[
\text{Model}
\rightarrow
\text{Feature}
\rightarrow
\text{Plan}.
\]

The returning direction is empirical evidence becoming rational recognition:

\[
\text{Result}
\rightarrow
\text{Feature Evidence}
\rightarrow
\text{Model Recognition or Revision}.
\]

This bidirectionality is essential. Without it, the system is either an abstract ontology that never reaches the field or an empirical learner that cannot state the Principle of what it has learned.

## 2. DataFrame and MetaFrame

The `DataFrame : MetaFrame` distinction is the primary architectural axis.

### 2.1 DataFrame

A DataFrame presents determinate empirical content in an addressable form. It supplies rows, columns, values, types, relations, selections, aggregates, and transformations. In the Organon vocabulary, it is a field of manifested determinations.

The DataFrame answers:

> What determinations are given, and how are they empirically arranged?

It should remain excellent at what it already is. The Organon should not force ordinary tabular work to pretend to be philosophy.

### 2.2 MetaFrame

A MetaFrame does not mean a DataFrame containing metadata. It is the Principle-processing enclosure of a Concept.

The elementary Rational Concept is:

\[
\boxed{
\text{Concept}
=
\langle
\text{Model},
\text{Feature},
\text{Plan}
\rangle.
}
\]

The MetaFrame encloses this triad together with the Principle that generates and justifies it:

\[
\boxed{
\text{MetaFrame}
=
\operatorname{Enclosure}
\left(
\Pi,
\langle M,F,P\rangle
\right).
}
\]

Its output is a Reasoned Concept:

\[
\boxed{
\operatorname{MetaFrame}
(\text{Principle},\text{Concept})
=
\text{ReasonedConcept}.
}
\]

A Concept states that a Model, Feature Grammar, and Plan belong together. A Reasoned Concept preserves why they belong together, what Grounds and conditions mediate them, what evidence supports them, and how execution returns to the Principle.

The MetaFrame answers:

> What Principle generates and justifies this articulation?

### 2.3 Required distinction

The initial implementation must preserve the following distinction:

| Object | Primary responsibility |
|---|---|
| `DataFrame` | Store and transform empirical determinations |
| `Frame` | Enclose a determinate semantic event or relation |
| `MetaFrame` | Enclose the rational genesis of a Concept |
| `Dataset` | Govern a field of empirical objects and operations |
| `OrganonDataset` | Equip a Dataset with Oculus sensing and rational return |

The MetaFrame must not become a universal dumping ground for configuration, provenance, metrics, and arbitrary metadata. Every field admitted to it must contribute to Principle-processing, rational justification, practical realization, or empirical return.

## 3. Model–Feature–Plan as Concept

The Rational layer is triadic:

### Model

The Model is the invariant rationale of a Dharma In Itself. It states what is to remain self-identical through varying empirical presentations.

\[
\boxed{
\text{Model}
=
\text{Rationale of the Dharma In Itself}.
}
\]

The Model is not identical with a fitted statistical object. A statistical model may serve as an empirical expression of a Model, but the Organon concept is wider: rules, ontologies, logical theories, semantic frames, learned functions, and hybrid structures may all express Models.

### Feature

A Feature is not merely a property or input column. It is a Mark that encloses a conditional sequence and thereby functions as a Ground of Cognition.

\[
\boxed{
\text{Feature}
=
\text{conditional sequence enclosed as a cognitive Ground}.
}
\]

For example:

\[
C_1\rightarrow C_2\rightarrow C_3
\quad\mapsto\quad
F.
\]

The Feature lifts a distributed chain of conditions into one addressable determination. Its implementation must therefore be capable of retaining more than a value. A Feature may require:

- semantic identity;
- domain and range;
- conditions of applicability;
- constraints and dependencies;
- evidence or observations;
- provenance;
- modal standing when operating in the conditioned field;
- relation to a Model;
- practical consequences for a Plan.

### Plan

The Plan is the singular realization of the Concept. It transforms the Feature Grammar into an executable or evaluable sequence.

The Plan is not an arbitrary task list appended to a Model. It must be generated or constrained by the same Features through which the Model becomes empirically intelligible:

\[
\boxed{
\text{what the Model means}
=
\text{the conditions the Plan must realize}.
}
\]

### Organic unity

The three moments form one Concept:

\[
\boxed{
\text{Model}
\overset{\text{Feature}}{\longleftrightarrow}
\text{Plan}.
}
\]

Model is the theoretical side, Plan the practical side, and Feature the mediating Transcendental Schema. The identity of Theoretical and Practical Reason is therefore executable rather than merely asserted.

## 4. The Transcendental Schema as Feature Grammar

The key theoretical result is:

> **The Transcendental Schema is a Feature Grammar.**

A taxonomy places finished objects under a classificatory genus. A Feature Grammar governs the lawful generation of determinate structures from semantic constraints.

\[
\text{Taxonomy}:
X\in A\lor X\in B.
\]

\[
\text{Feature Grammar}:
F_1+F_2
\overset{\text{constraints}}{\longrightarrow}
F_3.
\]

The Feature Grammar is the Middle that does not appear as one more Feature. It encloses the possibility and valid relations of the Features it generates:

\[
\boxed{
\text{Grammar}\neq F_{n+1};
\qquad
\text{Grammar}
=
\operatorname{Enclosure}(F_1,\ldots,F_n).
}
\]

The resulting Feature Structure is a determinate cognitive enclosure:

\[
\boxed{
\text{FeatStruct}
=
\operatorname{Enclosure}(F_1,F_2,\ldots,F_n).
}
\]

An Oculus does not merely read a Feature Structure. It knows how the structure was generated, what Model it expresses, and what Plan follows from it.

## 5. Primary NLP and the Meaning-Only Pipeline

The first Organon pipeline is NLP-first because its initial objects are articulated meanings rather than presumed physical things. This does not reduce the Organon to text processing. It gives the system a controlled domain in which Names, Marks, Grounds, conditions, and transformations are explicitly available.

The primary movement is:

\[
\boxed{
\text{Name}
\rightarrow
\text{semantic Features}
\rightarrow
\text{constraint propagation}
\rightarrow
\text{Form}
\rightarrow
\text{Meaning Known}.
}
\]

“Meaning-only” means that the first rational generation need not borrow its validity from an already identified external object. Meaning supplies constraints that can be unfolded and tested for consistency, consequence, opposition, and closure.

The empirical field remains indispensable. The return path tests whether a rationally generated Feature Grammar recognizes and transforms actual singulars. The NLP-first pipeline is therefore the beginning of the Organon, not its empirical limit.

## 6. Corpus, LM, and Logic

The empirical products of Principle-processing are organized through the triad:

\[
\boxed{
\text{Corpus}
\rightarrow
\text{LM}
\rightarrow
\text{Logic}.
}
\]

### 6.1 Corpus as artifact-backed tables

Corpus is the distributed empirical field composed of Documents, Artifacts, and Sources. These materials become computationally addressable through tabular presentations:

- document and source tables;
- span, segment, and expression tables;
- entity and relation tables;
- annotation and provenance tables;
- graph projections and learned observations.

The source is not metaphysically reducible to a table. The table is its empirical presentation within the Dataset:

\[
\boxed{
\text{Corpus}
=
\text{Artifacts presented as addressable empirical Tables}.
}
\]

This presentation permits ordinary DataFrame operations while retaining links to documents, artifacts, and source provenance.

### 6.2 LM as Language Module

Within the Organon, `LM` means **Language Module** more fundamentally than Language Model.

The Corpus presents conditions distributively. Relevant determinations may be scattered across documents, rows, spans, relations, observations, and prior analyses. LM performs the essential lift:

\[
C_1\rightarrow C_2\rightarrow\cdots\rightarrow C_n
\quad\mapsto\quad
LM.
\]

Therefore:

\[
\boxed{
\text{LM}
=
\text{a distributed chain of conditions lifted into one addressable semantic determination}.
}
\]

LM is the Objective of the root NLP of Meaning. It is the empirical product in which distributed semantic conditions become a reusable and inspectable language object.

This places LM in a precise relation to Feature:

\[
\boxed{
\text{Feature}
\overset{\textit{asi}}{\longleftrightarrow}
\text{LM}.
}
\]

Feature names the rational standing of the conditional unity: the Mark functioning as a Ground of Cognition. LM names its empirical addressability as a Language Module. The two must not be collapsed, but each becomes operationally intelligible through the other.

An Organon LM should be capable of retaining:

- semantic identity;
- its distributed Corpus conditions;
- its Feature structure;
- Grounds and constraints;
- provenance and evidence;
- relation to a rational Model;
- consequences for a Plan;
- its participation in a larger Logic.

### 6.3 Deliberate LM overloading

The project may deliberately retain the acronym `LM` for both **Language Model** and **Language Module**, provided the architectural distinction remains explicit.

| `LM` reading | Organon function |
|---|---|
| **Language Model** | Learns, estimates, or proposes linguistic functions from Corpora |
| **Language Module** | Encloses distributed semantic conditions as one addressable determination |

A statistical Language Model may propose, generate, score, or transform a Language Module. It does not establish the Module's rational standing merely by producing it.

\[
\boxed{
\text{Language Model proposes};
\qquad
\text{Language Module encloses}.
}
\]

A candidate becomes an Organon Language Module only when its conditions, evidence, semantic identity, Feature structure, Model relation, constraints, and practical consequences are available to the Oculus.

The ambiguity in `LM` is therefore potentially productive. It records the transition from a learned function to a Known semantic object. Documentation and types must nevertheless make the active sense unambiguous at every API boundary.

### 6.4 Logic as systematic relation among Modules

Logic specifies the lawful relations among addressable Language Modules:

\[
LM_1+LM_2
\overset{\text{constraints}}{\longrightarrow}
LM_3.
\]

It governs compatibility, contradiction, dependency, entailment, composition, transformation, modal standing, and valid transition.

\[
\boxed{
\text{Logic}
=
\text{the constraint-bearing system through which Language Modules determine one another}.
}
\]

### 6.5 Alignment of Rational and Empirical tiers

The working alignment is:

| Rational | Empirical |
|---|---|
| **Model** | **Corpus** |
| **Feature** | **LM** |
| **Plan** | **Logic** |

This alignment is not a flat identity. It gives the Oculus two distinguishable views of one process:

\[
\boxed{
\begin{matrix}
\text{Model} & \text{Feature} & \text{Plan}\\
\updownarrow & \updownarrow & \updownarrow\\
\text{Corpus} & \text{LM} & \text{Logic}.
\end{matrix}
}
\]

The Oculus sees LM below as an empirical Language Module compiled from Corpus conditions, above as the manifestation of a rational Feature, forward as a determination participating in Logic, and backward as evidence supporting or revising a Model.

The Organon Dataset consequently blinks itself into existence through this recognitive circuit:

\[
\boxed{
\text{Corpus}
\rightarrow
\text{LM appears}
\rightarrow
\text{Feature recognized}
\rightarrow
\text{Model seen}
\rightarrow
\text{Dataset recognizes its rational organization}.
}
\]

### 6.6 Why NLP Feature Processing remains central

The rise of large language models does not remove the requirement for explicit NLP Feature Processing. It intensifies it. Fluent generation can obscure the difference among:

- a textual output;
- a learned statistical function;
- an addressable semantic Module;
- a rational Feature;
- a Model;
- a Reasoned Concept.

The Organon works with philosophical Corpora in which decisive conditions are distributed across terms, propositions, arguments, commentaries, classifications, and transformations. This is especially evident in Sanskrit philosophical sources, but it is a general condition of transcendental philosophy. The relevant Meaning cannot be entrusted to undifferentiated generation alone.

NLP Feature Processing remains necessary because the Organon must:

1. locate semantic Marks;
2. recover their conditions and Grounds;
3. preserve technical distinctions across sources;
4. assemble distributed evidence into Language Modules;
5. expose contradictions and competing determinations;
6. relate Modules to rational Models and practical Plans;
7. make every decisive transformation inspectable.

Agents and large language models are instruments within this architecture. They can accelerate discovery, generation, comparison, and implementation. They do not replace the Feature Grammar through which their outputs become scientifically addressable.

### 6.7 Work-surface distinction

The project currently uses two complementary work surfaces:

- ChatGPT Work supports remote reflection, synthesis, artifact preservation, and architectural meditation.
- VS Codex operates closer to repository state, tests, dependencies, and empirical field work.

This is a practical division of labor rather than a theoretical difference in the Organon. Insights developed remotely must return to the code as explicit contracts and tests; discoveries made in the field must return to the MetaFrame as revisions of the Reasoned Concept.

\[
\text{reflection}
\rightarrow
\text{field implementation}
\rightarrow
\text{empirical return}
\rightarrow
\text{architectural revision}.
\]

## 7. The Oculus Feature

The Oculus Feature is the capability by which an ordinary Dataset becomes an Organon Dataset.

### 6.1 Working contract

The first implementation should treat Oculus as a composable capability rather than require immediate inheritance from a new monolithic root class.

Conceptually:

```text
OrganonDataset = Dataset + OculusFeature
```

The Oculus Feature should eventually support the following operations, though exact API names must follow repository inspection:

```text
observe(field)        -> observations
sense(observations)   -> feature evidence
frame(evidence)       -> semantic frames
model(frames)         -> model recognition or proposal
reason(model)         -> reasoned concept / metaframes
plan(concept)         -> plans
act(plan)             -> results
reflect(results)      -> model confirmation, revision, or contradiction
```

This is a rational lifecycle, not a commitment to a single synchronous method chain.

### 6.2 Minimum obligations

An Oculus-equipped Dataset must be able to answer:

1. What field is being observed?
2. What counts as a Feature in that field?
3. What conditions and Grounds does the Feature enclose?
4. Which Model is expressed or proposed?
5. Which Plan follows from that Model–Feature relation?
6. What evidence was produced by execution or observation?
7. How does the evidence return to the Model?
8. What Principle makes the completed Concept rationally intelligible?

If an implementation cannot answer these questions, it may still be a useful Dataset or learner, but it is not yet an Organon Dataset.

### 6.3 Sensing is not passive ingestion

Sense capability does not mean adding input adapters. Sensing is Feature articulation. The Oculus selects, discriminates, frames, and relates an observation to a possible Model.

\[
\text{raw field}
\rightarrow
\text{observation}
\rightarrow
\text{Feature evidence}
\rightarrow
\text{Model recognition}.
\]

The selection itself must be represented where it materially affects the result. An Oculus should be able to distinguish what the field supplied from what its own Feature Grammar made visible.

## 8. Species of Organon Dataset

The root abstraction should permit multiple empirical species.

### GML Dataset

A GML Dataset emphasizes graph structure, relational Features, paths, neighborhoods, topology, message passing, and graph-level learning. GraphFrames and property-graph stores can become empirical extensions of the root Dataset rather than redefine it.

An Oculus-equipped GML Dataset must see not only nodes and edges but the Feature Grammar by which a graph expresses a Model and supports a Plan.

### Statistical Learner Dataset

A statistical learner emphasizes distributions, estimators, fitted functions, loss, uncertainty, prediction, and evaluation. It is one empirical species of Model expression.

The Oculus must prevent the fitted function from being silently equated with the rational Model. It should preserve:

- the empirical function learned;
- the Feature Grammar under which it is interpretable;
- the Model proposed or supported;
- the conditions limiting that recognition;
- the Plan generated from the recognized structure.

### NLP Dataset

The NLP Dataset is the initial privileged species because it directly exposes Names, expressions, semantic Frames, Features, and constraint-bearing relations. It is the primary route for bootstrapping the Organon’s own rational vocabulary.

### Hybrid Dataset

The long-term Organon Dataset should support hybrid fields in which text, tables, graphs, logical constraints, and learned functions coexist. Hybridization must occur through explicit Feature mappings and MetaFrame relations, not through an untyped bag of artifacts.

## 9. Existing Tier Alignment

The working Organon tiers remain useful:

| Tier | Components | Oculus interpretation |
|---|---|---|
| Frames | `Frame`, `Series`, `Expr` | Articulated semantic and computational appearances |
| Rational | `Model`, `Feature`, `Plan` | The Concept proper |
| Empirical | `Corpus`, `LM`, `Logic` | Singular fields, learned functions, and constraints |

The MetaFrame does not simply become a fourth row. It encloses the Rational triad through its Principle and relates that triad to the Frames and Empirical tiers.

\[
\boxed{
\text{MetaFrame}
=
\text{Principled enclosure of Rational Concept and empirical return}.
}
\]

The Oculus Feature is the operational capability by which a Dataset participates in this enclosure.

## 10. Principle Processing

Ordinary data processing transforms values:

\[
X\rightarrow Y.
\]

Statistical learning attempts to learn a function:

\[
F(X)=Y.
\]

Principle processing makes the complete relation visible:

\[
\langle X,F,Y\rangle,
\]

and then asks what invariant Model makes (F) intelligible as a Feature-bearing function rather than an accidental mapping.

\[
\boxed{
\text{Principle Processing}
=
\text{recognition of the Model through the known function and its field}.
}
\]

The MetaFrame must therefore retain the distinction among:

- observed input;
- observed or generated output;
- inferred or known function;
- semantic Features making the function intelligible;
- Model proposed as invariant;
- Plan justified by the Model;
- Principle enclosing the completed relation.

This is more than statistical learning. A learner may discover (F). The Oculus must determine how (F) becomes a moment of a Reasoned Concept.

## 11. Modality and the Conditioned Field

The empirical Dataset necessarily operates through modal standings:

- possible Feature configurations;
- actual observations;
- necessary constraints;
- contingent outcomes.

The MetaFrame must represent modality where it is required by the field without confusing a modal determination with the Principle itself.

The Principle is not another modal value. It is the Ground from which modal alternatives become intelligible. The Oculus should therefore be able to preserve the distinction between:

```text
principial determination
modal projection
empirical actuality
```

This prevents a possible model, an actual fitted artifact, or a necessary constraint from independently claiming the standing of the Principle.

## 12. Implementation Instructions for VS Codex

### Phase 0 — Repository reconnaissance

Before editing code:

1. locate the current `Dataset`, `DataFrame`, `Frame`, `Model`, `Feature`, `Plan`, `Corpus`, `LM`, and `Logic` implementations;
2. identify protocols, abstract bases, mixins, registries, and extension points already in use;
3. trace how Dataset wraps or delegates to Polars/DataFrame operations;
4. identify the present graph and statistical-learning extensions;
5. locate tests defining public behavior;
6. record naming conflicts, especially any existing `MetaFrame`, `Oculus`, or `Feature` concepts;
7. make no architectural rename until the dependency surface is known.

Deliverable: a short architecture map and a minimal insertion point for Oculus capability.

### Phase 1 — Formalize the contracts

Define the smallest stable contracts for:

- `ReasonedConcept`;
- `MetaFrame`;
- `OculusFeature`;
- Feature evidence;
- the Model–Feature–Plan relation;
- empirical return or reflection.

Prefer protocols or interfaces where multiple implementations are expected. Do not force every Dataset species into one storage representation.

Deliverable: types plus contract tests, without broad behavioral implementation.

### Phase 2 — NLP-first reference path

Implement one complete meaning-only path using existing NLP/Frame machinery:

```text
expression
-> semantic frame
-> features and constraints
-> model recognition
-> plan
-> result
-> reflection
-> metaframes / reasoned concept
```

Choose a compact example whose semantic constraints genuinely generate consequences. Avoid a toy that merely copies labels between structures.

Deliverable: one end-to-end executable Oculus cycle.

### Phase 3 — Dataset capability composition

Equip a Dataset with Oculus capability through the least invasive composition mechanism supported by the repository. Candidate forms include:

- protocol implementation;
- mixin;
- delegated component;
- registered capability;
- typed feature bundle.

Do not decide among these before Phase 0. The governing requirement is behavioral:

```text
Dataset + OculusFeature -> OrganonDataset
```

Deliverable: an existing Dataset species gaining Oculus behavior without losing ordinary Dataset semantics.

### Phase 4 — GML species

Apply the same contract to a graph field:

- map graph observations to Feature evidence;
- preserve relational and topological Grounds;
- relate graph-level learned functions to rational Models;
- generate or evaluate a Plan;
- return evidence into the MetaFrame.

Deliverable: a GML Dataset that demonstrates why Oculus is more than a statistical learner.

### Phase 5 — Statistical learner species

Integrate a fitted statistical function while preserving the distinction among learner, Model, Features, and Principle.

Deliverable: a statistical learner whose predictions can be enclosed in a Reasoned Concept with explicit limitations and Grounds.

### Phase 6 — Hybrid Organon Dataset

Allow NLP, tabular, graph, logical, and learned evidence to participate in one MetaFrame through explicit mappings.

Deliverable: a small hybrid demonstration, not an immediate attempt at a universal platform.

## 13. Testing Requirements

Every implementation phase should test both directions.

### Generative direction

```text
Principle -> Model -> Features -> Plan -> Result
```

Tests should establish that the Plan is constrained by the Model’s Feature Grammar rather than manually attached.

### Recognitive direction

```text
Result -> Evidence -> Features -> Model -> Principle standing
```

Tests should establish that empirical return can confirm, contradict, qualify, or revise a Model.

### Negative tests

Include cases where:

- observations underdetermine the Model;
- Features conflict;
- a Plan violates a constraint;
- a fitted function lacks a rational interpretation;
- multiple Models explain the same evidence;
- modality is confused with Principle;
- a MetaFrame lacks sufficient Grounds to call its output a Reasoned Concept.

### Invariant

The core invariant is:

> No Reasoned Concept without an explicit path from Principle through Model–Feature–Plan and an explicit empirical return appropriate to its claims.

## 14. Non-Goals for the First Return

Do not begin by:

- creating a universal metaphysical superclass;
- renaming the entire existing platform;
- forcing all datasets into graphs;
- equating statistical models with rational Models;
- treating metadata as MetaFrame content;
- building a GUI before the contracts work;
- implementing every modality;
- encoding the complete philosophical system before one end-to-end cycle succeeds;
- claiming that architectural analogy proves the philosophical Principle.

The first success criterion is modest but decisive: one Dataset should acquire an Oculus Feature and complete a traceable cycle from Meaning through Feature Grammar to Plan, result, reflection, and Reasoned Concept.

## 15. Decision Log

The following decisions govern the return to the Organon unless code inspection forces a documented revision:

1. `MetaFrame` means Principle-processing enclosure, not metadata frame.
2. `Concept = <Model, Feature, Plan>`.
3. `ReasonedConcept = MetaFrame(Principle, Concept, Grounds, evidence, return)`.
4. Feature is a conditional Ground of Cognition, not merely an attribute.
5. The Transcendental Schema is implemented as a Feature Grammar.
6. Theoretical and Practical Reason share one Feature Grammar.
7. The primary reference implementation is NLP-first and meaning-only.
8. Empirical singulars return through Feature evidence to Model recognition or revision.
9. GML and statistical learning are species of Organon Dataset behavior, not the root definition.
10. An Organon Dataset is a Dataset equipped with an Oculus Feature.
11. In the Organon architecture, `LM` means Language Module more fundamentally than Language Model.
12. A Language Module lifts distributed Corpus conditions into one addressable semantic determination.
13. Statistical language models and agents are instruments of the Oculus, not replacements for explicit NLP Feature Processing.

## 16. Open Questions for Code Inspection

1. Does `MetaFrame` already exist, and if so, can its semantics be raised without breaking callers?
2. Is Oculus best represented as a protocol, capability, component, or Feature species?
3. Does the existing `Feature` object retain conditions and Grounds, or only values and labels?
4. Where should Feature Grammar constraints live?
5. Is `Plan` currently descriptive, executable, or both?
6. What object should own empirical return and revision?
7. Can a Dataset expose multiple Oculus views over the same field?
8. How should competing Models and underdetermination be represented?
9. Which existing Frame/Series/Expr operations already implement part of the schema?
10. What is the smallest genuine end-to-end semantic example already supported by the repository?
11. Does the existing `LM` implementation behave primarily as a learner, a semantic Module, or an unresolved mixture of both?
12. What representation can preserve a Language Module's distributed Corpus conditions without duplicating source data?

## 17. Governing Formula

The Oculus Project begins from one architectural formula:

\[
\boxed{
\operatorname{OculusDataset}
=
\operatorname{Dataset}
\left[
\operatorname{MetaFrame}_{\Pi}
\left(
\langle
\operatorname{Model},
\operatorname{FeatureGrammar},
\operatorname{Plan}
\rangle
\right)
\right].
}
\]

In words:

> **An Organon Dataset is a Dataset equipped with an Oculus Feature through which a Principle generates, observes, and recognizes a Model–Feature–Plan Concept as a Reasoned Concept.**

This is the implementation target for the return to the Organon.
