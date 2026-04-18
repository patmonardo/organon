# ADR: Term Logic as Triadic Composition

- Date: 2026-04-15
- Status: Draft
- Scope: GDS form architecture, future FormDB / GDSL planning surfaces, and any runtime or DSL that claims to model composable term relations

## Context

We need a sharper architectural notion of **Term Logic**.

In the weak or degraded sense, "term logic" collapses into taxonomy: a term is treated as a label, and knowing means remembering which list or class the label belongs to. That is not the notion we need.

In the stronger sense needed by this repository, a term is a **seed-form** or condensed intelligible substance. It contains an inner matter, and its marks are extensions of that matter across a field of determinations. A living term is therefore not a dead atom plus attached predicates. It is something that can:

- receive determinations,
- process them according to its own form, and
- re-express the result into further relations.

This raises the architectural question: what is the minimal structure an intelligible substance must have if it is to support **composable relations** rather than merely external adjacency?

The answer emerging across our existing docs is **triadic**:

- `FORM-AS-BUDDHI.md` argues that Form must be triadic because a simple or dyadic structure cannot carry the full movement of Pure Reason.
- `FORM-AS-DYADIC-INFERENCE.md` already states that Form is triadic and precisely thereby enables dyadic relations.
- `CONCEPT-JUDGMENT-BEFORE-SYLLOGISM.md` and related docs insist that concept/judgment processing must happen before syllogistic inference.
- The repo already prefers **Planning/Execution** over Lisp-style **Eval/Apply**, so any borrowing from generic-function systems must be adapted into local terminology rather than imported verbatim.

At the same time, generic dispatch systems provide a useful operational model. In Lisp-style generics, a call can be organized by:

- pre-processing (`before` methods),
- primary realization (main method body),
- post-processing (`after` methods), and
- optional reflective interception (`around` methods).

That pattern is not our public vocabulary, but it exposes a real logical structure: a composable term does not merely relate; it prepares, determines, and propagates.

## Decision

Adopt the following architectural principle:

> **Term Logic in Organon is triadic.**
>
> Any substance, form, node, operator, or protocol that claims to support **composable relations** must be modeled as a threefold process:
>
> 1. **Preparation** - admissibility, normalization, and ingress into the term.
> 2. **Determination** - the term's proper act, where its own form decides the relation.
> 3. **Propagation** - re-expression of the result into a new relation-ready state.

This triad is the minimal architecture of an intelligible substance suitable for compositional systems.

## Rationale

### Why a dyad is insufficient

A dyadic relation can connect `X` and `Y`, but by itself it does not explain:

- how the relation becomes admissible,
- how the term interiorizes what reaches it, or
- how the achieved determination becomes available for further composition.

Without those moments, relation remains external. The system can connect terms, but it cannot explain how a connection becomes a **new ground** for subsequent connections.

### Why the triad is sufficient

The triad solves that problem:

- **Preparation** makes the relation valid for this term.
- **Determination** makes the relation belong to the term's own form rather than remain externally imposed.
- **Propagation** makes the achieved form composable with later relations.

In compact form:

`preparation -> determination -> propagation`

This is the smallest architecture that turns relation into a logical metabolism rather than a static link.

### Why generic dispatch is a useful analogue

Generic-function systems are not the philosophical source of this decision, but they are a useful operational analogue.

- `before` methods model ingress, admissibility, and normalization.
- the primary method models the immanent act of the term.
- `after` methods model consequence, release, and propagation.
- `around` methods model reflection: a higher-order power that can intercept or recontextualize the whole triad.

This is close to what we need for composable term logic, but our preferred public language should remain repo-native.

## Terminology Policy

For Organon documents and APIs, prefer:

- **Preparation** over `before`
- **Determination** over `primary`
- **Propagation** over `after`
- **Reflection** over `around`

Likewise, preserve the existing preference for **Planning/Execution** over Lisp-style **Eval/Apply**.

The point is not to hide the analogy to Lisp generics, but to translate it into the repo's own architectonic vocabulary.

## Term Logic Model

### 1. Term as seed-form

A term is a condensed form whose matter is inwardly held. Its marks are not accidental attachments but unfoldings of that matter across a field.

### 2. Relation as internal process

A relation is not only "A connected to B". It is the passage by which a term:

- admits an incoming determination,
- processes it according to its own form,
- emits a new determination fit for further composition.

### 3. Composition as triadic relay

Composition means that the propagated result of one triad can become the prepared input of another triad.

This yields a genuine chain of living terms rather than a flat graph of labels.

### 4. Reflection as meta-control

Reflection is not a fourth internal moment of substance itself. It is a higher-order control layer that can:

- inspect the triad,
- wrap it,
- redirect it, or
- suspend and resume it.

This maps well to the repo's broader concern with controller/runtime separation.

## Consequences

### Positive

- Gives a precise definition of **Term Logic** beyond taxonomy or list-memory.
- Explains why triadic structures recur across Form, Concept/Judgment, and inference docs.
- Provides an operational bridge between philosophical architecture and runtime/API design.
- Clarifies why dyadic inference alone is derivative: dyads are enabled by a triadic substrate.
- Makes "composability" a structural property rather than an accidental API feature.

### Negative / Trade-offs

- Adds conceptual discipline: not every convenient hook chain counts as term logic.
- Risks over-generalization if every three-step process is casually called triadic.
- Requires translation work when borrowing from Lisp, CLOS, or other generic systems.

## Non-Goals

- Mandating Common Lisp, CLOS, or any specific generic dispatch framework in runtime code.
- Replacing existing stable repo vocabulary with imported terminology.
- Forcing speculative triadic language into hot-path implementation details where a simpler description suffices.
- Claiming that every data structure must expose explicit pre/core/post hooks in the public API.

## Implementation Guidance

When designing a new form, protocol, DSL surface, or execution pipeline that claims composability:

1. Identify the **preparation** step.
   - What normalizes or admits the incoming relation?

2. Identify the **determination** step.
   - What is the term's own act?

3. Identify the **propagation** step.
   - What new relation-ready state is emitted?

4. Decide whether a reflective wrapper is needed.
   - If yes, keep it conceptually distinct from the core triad.

5. Prefer repo-native vocabulary in public APIs and docs.
   - Use the Lisp generic model as an explanatory analogue, not as the default naming scheme.

## Relation to Existing GDS Structures

This ADR does not replace existing triads. It gives them a more general logical interpretation.

- `Shape / Context / Morph` can be read as a domain-specific triadic realization of the same principle.
- `Concept / Judgment / Syllogism` already shows the same order: processing before inference.
- Procedure-first controller patterns also exhibit a related separation between ingress control, core act, and emitted result.

The present ADR therefore functions as a unifying explanation rather than a competing structure.

## References

- `gds/doc/FORM-AS-BUDDHI.md`
- `gds/doc/FORM-AS-DYADIC-INFERENCE.md`
- `gds/doc/CONCEPT-JUDGMENT-BEFORE-SYLLOGISM.md`
- `gds/doc/PRELINGUAL-KERNEL-COGITO-MEDIATION.md`
- `reality/src/logos/hegel-logic/concept/subject/syllogism/SYLLOGISM-IDEA-WORKBOOK.md`
