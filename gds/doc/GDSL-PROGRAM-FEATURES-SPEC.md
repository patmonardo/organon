# GDSL Program Features — Knowledge Agent Specification

Date: 2026-05-01

## The Frame

NLP in this platform does not mean general natural language processing. It means
the science of how _scientific knowledge_ is marked, structured, and made
executable. The Knowledge Agent is a specialized slice of NLP whose domain is
epistemic objects: concepts, judgments, inferences, principles, proofs, models,
and their mutual grounding. LLMs handle fluent chit-chat. The Knowledge Agent
handles the harder problem: producing and inspecting the _logical form_ of
knowledge itself.

The GDSL is the program surface where that domain is authored. A GDSL program
defines not what text says but what the text _knows_ — and what executing that
knowledge means as a sequence of Dataset/GDS kernel operations.

This document specifies what a complete GDSL Program Feature system must
consist of, given that framing.

---

## What a Program Feature Is

A `ProgramFeature` is an atomic, named, typed unit of epistemic intent. It is
not a runtime instruction. It is a _semantic commitment_ authored at the GDSL
level and compiled into Dataset artifacts, GDS algorithm invocations, and
ontology image tables.

Three fields define any feature:

| Field    | Meaning                                                                        |
| -------- | ------------------------------------------------------------------------------ |
| `kind`   | The epistemic role this feature plays in the knowledge program.                |
| `value`  | The local name or computed label of this feature instance.                     |
| `source` | The canonical GDSL phrase it was parsed from; preserved for provenance.        |

Features are collected into `ProgramFeatures`, which is the compiler contract:
the thing that Dataset/DataFrame composition must conform to, and the thing that
the GDS kernel receives as a stable specification of what to execute.

---

## Complete Feature Kind Taxonomy

The first implementation used five compact bridge kinds (`ApplicationForm`,
`OperatorPattern`, `Dependency`, `Condition`, `SpecificationBinding`). The
current implementation carries the richer Doctrine taxonomy directly while
preserving `ApplicationForm` and `OperatorPattern` as native PureForm bridge
variants for existing `ProgramSpec` execution plans.

### Tier 1 — Source and Perception

These features define how the agent receives the world.

| Kind                | GDSL keyword        | Role                                                                        |
| ------------------- | ------------------- | --------------------------------------------------------------------------- |
| `Import`            | `use`               | Declares a standard library or peer module dependency.                      |
| `Source`            | `source`            | Names a concrete data source: file, stream, corpus, API feed.               |
| `Observation`       | `appearance`        | Declares the empirical surface: what can be seen, retained, and classified. |

`Observation` replaces the vague `OperatorPattern` assignment for `appearance`.
It is the perceptual gate of the program — the Kantian sensibility layer.

### Tier 2 — Reflection and the Seven Moments

Reflection is the threshold where appearance becomes graspable. It comprises
the 7 moments of the Relative Form Processor.

| Kind            | GDSL keyword   | Role                                                                                     |
| --------------- | -------------- | ---------------------------------------------------------------------------------------- |
| `Reflection`    | `reflection`   | Stages through the 7 moments: identity → opposition → contradiction → ground (×4) → absolute reflection mark. |
| `Logogenesis`   | `logogenesis`  | Names the self-unfolding of essence into a specific natural kind or law form.            |

Critically: the 7th moment is **Absolute Reflection**, which is NOT yet
Concept. It is the preparation. From Absolute Reflection, Principle can be
evaluated, and only on the far side of Principle does Concept emerge.

This is the ZeroCopy junction: both the GDS Kernel (PureForm) and the Agential
(Relative Form Processor) must traverse these 7 moments with zero-copy access to
shape data, so that the evaluation of Principle and emergence of Concept can
happen at the boundary without re-marshaling.

### Tier 3 — Principle: The Nomological Gate

Principle comes before Concept. It determines what _can_ become a concept.

| Kind         | GDSL keyword | Role                                                                                  |
| ------------ | ------------ | ------------------------------------------------------------------------------------- |
| `Principle`  | `principle`  | States a lawful condition that must hold for scientific object status to be granted.  |
| `Condition`  | (existing)   | Sub-unit: a single admissibility condition within a principle or context shape.       |

A `Principle` is not a soft constraint. It is the nomological gate evaluated
within or just after Absolute Reflection. An entity that cannot satisfy its
program's principles does not transition to Concept. Principle is the threshold.

### Tier 4 — Marking and Concept Formation

Once Principle has been satisfied, the entity becomes a Concept. Marks are the
named qualities that make that Concept determinate.

| Kind        | GDSL keyword | Role                                                                               |
| ----------- | ------------ | ---------------------------------------------------------------------------------- |
| `Mark`      | `mark`       | Names a typed quality, measure, or ground property determined on an appearance.    |
| `Concept`   | `concept`    | Selects and identifies a scientific entity formed from Principle + marks.  |

`Mark` is the Dataset-level semantic token for a feature address. It is not a
raw column. It carries a role (quality, measure, ground, determinacy) and a
derivation trace. A `Concept` is what emerges on the far side of Absolute
Reflection and Principle satisfaction.

### Tier 5 — Judgment and Inference

These features define how the agent determines and reasons about what a Concept
is and what it entails.

| Kind         | GDSL keyword | Role                                                                                      |
| ------------ | ------------ | ----------------------------------------------------------------------------------------- |
| `Judgment`   | `judgment`   | States singular/particular/universal determinations over a concept.                       |
| `Syllogism`  | `syllogism`  | Names the middle term and derives lawful conclusions by composing judgments.              |
| `Inference`  | `infer`      | Atomic conditional inference rule; sub-unit of judgment and syllogism bodies.             |

`Judgment` maps to the first-order logical form of the concept. `Syllogism`
maps to the scientific form: middle term + two premises → necessary conclusion.
Both are executed as Dataset filtering/projection operations backed by GDS kernel
evaluation.

### Tier 6 — Query and Procedure

These features define the outputs and the entry into Objectivity.

| Kind                   | GDSL keyword  | Role                                                                                  |
| ---------------------- | ------------- | ------------------------------------------------------------------------------------- |
| `Query`                | `query`       | A named, typed result projection over the program's concept space.                    |
| `Procedure`            | `procedure`   | Entry into Objectivity: emits Dataset artifact stages as Process/realized objects.    |
| `SpecificationBinding` | (existing)    | Binds the program to a named GDSL/SDSL specification in the FactStore.               |

`Procedure` is the only place the program can side-effect the world. It emits
artifacts. Queries read. Procedures emit.

---

## Updated ProgramFeatureKind Enum

The revised taxonomy collapses into this clean Rust enum shape:

```rust
pub enum ProgramFeatureKind {
    // Tier 1 — Source and Perception
    Import,              // use
    Source,              // source
    Observation,         // appearance

    // Tier 2 — Reflection (7 moments: RelativeReflection + Absolute Reflection)
    Reflection,          // reflection
    Logogenesis,         // logogenesis

    // Tier 3 — Principle (nomological gate, evaluated within/after Absolute Reflection)
    Principle,           // principle
    Condition,           // sub-unit of principle / context shape

    // Tier 4 — Marking and Concept (Concept emerges from Principle)
    Mark,                // mark
    Concept,             // concept

    // Tier 5 — Judgment and Inference
    Judgment,            // judgment
    Syllogism,           // syllogism
    Inference,           // infer

    // Tier 6 — Output (entry into Objectivity)
    Query,               // query
    Procedure,           // procedure
    SpecificationBinding,// spec header / module binding

    // Native PureForm bridge variants for ProgramSpec execution plans
    ApplicationForm,
    OperatorPattern,
}
```

  The existing compact kinds are preserved or refined as follows: `Dependency`
  becomes `Import`; `SpecificationBinding` and `Condition` stay; ordinary GDSL
  lowering now uses precise roles such as `Observation`, `Mark`, `Concept`,
  `Judgment`, `Syllogism`, `Inference`, `Principle`, `Reflection`, and
  `Logogenesis`. `ApplicationForm` and `OperatorPattern` remain available for the
  native PureForm bridge, not as the normal GDSL authoring surface.

---

## The Knowledge Agent Pipeline Shape

Every GDSL program is an arc through the following stages, traversed via
ZeroCopy between the GDS Kernel (PureForm) and the Agential (Relative Form
Processor). Each stage produces Dataset artifacts. The arc is the execution shape
of scientific knowing.

```
Source → Observation → Reflection (7 moments) → Principle → Concept → Judgment → Syllogism → Procedure
```

Written in epistemic terms:

1. **Source** — the given world makes itself available (perception stream, corpus, sensor feed).
2. **Observation** — immediacy is determined: the agent decides what to retain, derive, and classify.
3. **Reflection (7 moments)** — the immediate appearance passes through 6 RelativeReflection stages:
   - being → identity → opposition → contradiction → ground → ground-as-condition
   - Then the 7th moment: **Absolute Reflection** (not yet Concept). This is where the kernel and agent must traverse with ZeroCopy access.
4. **Principle** — evaluated at or just after Absolute Reflection. The nomological gate. Does this entity satisfy the program's lawful conditions?
5. **Concept** — emerges from Principle satisfaction. The grounded, named, determinate scientific object.
6. **Judgment** — determines the singularity, modality, and universal status of the Concept.
7. **Syllogism** — establishes the middle term and derives necessary conclusions through composed judgments.
8. **Procedure** — entry into Objectivity. Emits Dataset artifact images as realized Process. For Hegel, Objectivity is Process itself.

LLMs enter this pipeline only at Stage 2 (Observation derivation) or Stage 6
(Judgment inference when the inference is natural-language-based). They do not
own the pipeline. The Knowledge Agent owns the pipeline.

---

## ZeroCopy Kernel-Agent Boundary and the Absolute Reflection Moment

The architectural key: this complete cycle requires **ZeroCopy traversal** across
the kernel/agent boundary.

The GDS Kernel lives on the Rust side: it owns the PureForm shapes and the
graph algorithms that drive the world forward. The Agential side (TS engines)
lives in the Relative Form Processor: it speaks in relative reflection stages
and must evaluate whether each moment prepares the way to the next.

The critical boundary is **Absolute Reflection (Moment 7)**. This is where:

- The kernel has already executed 6 RelativeReflection stages on the entity's
  shape data.
- The Agential must now inspect that shape and evaluate whether Principle
  conditions hold.
- **No marshaling.** No conversion to JSON and back. ZeroCopy: the agent
  reads the shape directly from the kernel's memory, evaluates Principle, and
  signals back whether Concept can emerge.

This is why ZeroCopy is essential: Absolute Reflection is too fine-grained, too
intimate a moment to afford serialization overhead. Both sides must speak in the
same shape language. This is also where the GDS kernel's Form/Context/Morph
triad is most vulnerable to copy cost — and most critical to preserve.

---

## Objectivity as Process: What Procedure Emits

In Hegel, Objectivity is not a static state. It is **Process**.

`Procedure` is the moment where the Knowledge Agent enters Objectivity. It is
not "emit a table." It is "emit a realized Process" — a record of how the
scientific objects came to be, what they determined along the way, what
principles they satisfied, and what next moves are now available.

Every artifact table emitted by `Procedure` is a slice through that Process. The
reflection table shows _how_ essence unfolded. The principle table shows _which
conditions held_. The concept table shows _what identities were fixed_. The
judgment table shows _what was determined_. The syllogism table shows _what was
concluded_.

Taken together, they are the Process record. They are not dead data. They are
the living documentation of how knowledge came to be in this program world.

---

## Scientific Knowledge vs. General NLP

| General NLP task          | Status in this system                                                      |
| ------------------------- | -------------------------------------------------------------------------- |
| Sentiment analysis        | Out of scope. Not a scientific knowledge problem.                          |
| Named entity recognition  | In scope only when the entity is a _concept_ in a scientific program.      |
| Summarization             | Out of scope for the kernel. Delegate to LLM surface layer.                |
| Question answering (open) | Out of scope. General QA is LLM territory.                                 |
| Dependency parsing        | In scope for scientific text when it serves `Reflection` stage marking.    |
| Information extraction    | In scope when extraction = `Observation` into a `mark` or `concept`.      |
| Knowledge graph building  | In scope as `GraphFrame` projection from Dataset concepts and judgments.   |
| Ontology engineering      | In scope as the Dataset Semantic Model Builder function directly.          |
| Scientific fact checking  | In scope as `Principle` evaluation against a compiled concept space.       |
| Citation and provenance   | In scope as mandatory `source` provenance in every artifact table.         |
| Theorem proving / logic   | In scope as `Syllogism` + `Principle` evaluation using GDS kernel logic.   |
| ZeroCopy traversal        | In scope as the architectural requirement for Reflection (7 moments) boundary between kernel and agent. |
| Objectivity / Process     | In scope as what `Procedure` emits: realized scientific objects as Process, not as static snapshots. |

The boundary rule: if the task requires producing or inspecting the logical
form of a piece of knowledge, it belongs here. If the task requires fluent
generation or open-ended retrieval, delegate to LLM.

---

## What Dataset Must Serve for Program Features

Given the complete feature taxonomy, the Dataset layer must be able to:

1. Compile any `ProgramFeatures` set into a `DatasetCompilation` graph where
   every feature kind has a correctly typed `DatasetNode`.

2. Realize each stage as a Dataset artifact table:
   - `source` → raw ingestion record with provenance fields.
   - `observation` → retained schema + derived column definitions.
   - `reflection` → 7-moment sequence table: `(entity_id, moment, stage_name, essence_mark, value)`. Moment 1-6 are RelativeReflection; Moment 7 is Absolute Reflection. Must be ZeroCopy traversable.
   - `principle` → principle evaluation record table: `(entity_id, principle_name, satisfied)`. Evaluated at/after Moment 7.
   - `concept` → concept record table: `(concept_id, identity_field, marks)`. Emerges post-Principle.
   - `judgment` → judgment record table: `(concept_id, judgment_name, value)`.
   - `syllogism` → inference record table: `(concept_id, middle, conclusions)`.
   - `query` → projected result table.
   - `procedure` → emitted artifact manifest as Objectivity/Process realization.

3. Carry provenance across all tables: `(source_id, specification_id, program_name, generated_at)`.

4. Serve the full compiled image through the Applications TS-JSON facade with
   no leakage of internal algo module names.

---

## GDSL Grammar Summary

The current fixture already demonstrates the complete grammar. Annotated here
for specification:

```gdsl
module <qualified.name>;           // SpecificationBinding: program identity

use <module.path>;                 // Import: dependency declaration
source <kind> <name> : <format>;   // Source: data origin

appearance <name> from <source> {  // Observation: perceptual gate
  key <field>;
  retain <fields>;
  derive <mark> = <expr>;
}

reflection <name> for <obs> {      // Reflection: essence path
  stage <stage-name>;              // produces (entity, stage, mark) rows
  culminate mark;
}

logogenesis <name> for <obs> {     // Logogenesis: natural-kind unfolding
  from <reflection>;
  unfold <law-form>;
}

mark <name> on <obs> := <expr>;    // Mark: named typed quality/measure/ground

concept <Name> from <obs> {        // Concept: named scientific object
  identity <field>;
  mark <mark-name>;
}

judgment <name> for <Concept> {    // Judgment: determinate predication
  infer <label> when <condition>;
}

syllogism <name> for <Concept> {   // Syllogism: scientific inference
  middle <term>;
  conclude <label> when <condition>;
}

principle <name> for <Concept> {   // Principle: nomological gate
  require <judgment-label>;
  unify concept, judgment, syllogism;
  infer <label> when <condition>;
}

query <name> :=                    // Query: typed result projection
  select <fields> from <Concept> where <condition>;

procedure <name> {                 // Procedure: artifact emission
  emit <stage> dataset;
}
```

---

## Implementation Status and Remaining Targets

1. **Implemented: expand `ProgramFeatureKind`** to the doctrinal enum above and
  map the lightweight GDSL lowerer in `toolchain.rs` from generic bridge roles
  to precise kinds. The Dataset image path now preserves the feature kind label
  through ontology image rows and compilation tests.

2. **Add per-kind artifact builders** in `compile.rs` so each feature kind
   produces a correctly typed `DatasetNode` and the corresponding artifact
   table schema is known at compile time.

3. **Add a `reflection` stage table as a 7-moment sequence**. Each `reflection` feature
   must produce an artifact table with columns: `(entity_id, moment, stage_name, essence_mark, value)`.
   Moments 1-6 are RelativeReflection stages; Moment 7 is Absolute Reflection. This table is
   the preparation for Principle evaluation and must remain ZeroCopy-traversable at the
   kernel/agent boundary.

4. **Document and enforce the ZeroCopy contract** at Absolute Reflection (Moment 7).
   Both the GDS Kernel (PureForm) and the Agential (Relative Form Processor) must be able
   to traverse the entity's shape data without serialization. This is the architectural
   requirement that justifies building the entire system around form shapes rather than
   JSON records.

5. **Require provenance fields** on every emitted artifact table: `source_id`,
   `specification_id`, `program_name`, `generated_at_unix_ms`. These are not optional.
   They are the Process record.

6. **Write `collections_dataset_gdsl_scientific_inference.rs`** as the
   canonical example that walks the full pipeline from `source` to `procedure`
   using the fixture `absolute-concept-scientific-inference.gdsl`. The example
   must show all 7 reflection moments, principle evaluation, concept emergence,
   judgment, and syllogism in sequence.

The spec is complete. The ZeroCopy requirement is now the first-class architectural
constraint. The agent knows where it ends, the kernel knows where it begins, and
both sides know that Absolute Reflection is where they meet.
