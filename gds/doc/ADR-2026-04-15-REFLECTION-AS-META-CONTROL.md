# ADR: Reflection as Meta-Control

- Date: 2026-04-15
- Status: Draft
- Scope: GDS form architecture, controller/runtime boundaries, future FormDB / GDSL planning surfaces, and any execution layer that wraps triadic term operations

## Context

The Term Logic ADR establishes that composable relations require a triadic internal architecture:

- Preparation
- Determination
- Propagation

That ADR also notes that some systems exhibit a higher-order wrapper around the triad. We now need to state that wrapper precisely, because without it two errors recur:

1. Reflection is collapsed into one of the internal moments of the term.
2. Reflection is treated as vague introspection instead of an architectural control layer.

This repository already contains the relevant ingredients for a stronger position:

- `CONCEPT-JUDGMENT-AS-REAL-MECHANISM.md` treats Context/Judgment/Reflection as the active determination surface rather than as an ornament added afterward.
- `CONCEPT-JUDGMENT-BEFORE-SYLLOGISM.md` insists that reflection-bearing concept/judgment work precedes syllogistic deployment.
- `KANTIAN-PURE-MECHANICS-AS-CONCEPT-JUDGMENT.md` explicitly treats dynamics as reflection and therefore as the admissibility/constraint surface for what can follow.
- `PRELINGUAL-KERNEL-COGITO-MEDIATION.md` preserves a critical boundary between lawful kernel execution and the discursive interpretation layer.
- the new Term Logic ADR already states that reflection should be conceptually distinct from the internal triad.

We therefore need a separate ADR that defines Reflection as the layer that governs, wraps, and recontextualizes triadic activity without being reducible to one more step inside the triad.

## Decision

Adopt the following architectural principle:

> **Reflection in Organon is meta-control.**
>
> Reflection is not a fourth internal moment of a term's substance.
> Reflection is the higher-order control layer that:
>
> 1. inspects a triadic process,
> 2. constrains or qualifies its admissibility,
> 3. wraps or reorders its execution,
> 4. records the grounds, conditions, and active judgments under which it runs, and
> 5. decides what the triad means for subsequent planning or execution.

Reflection therefore stands **over** triadic term logic as its judgmental and controlling envelope.

## Rationale

### Why reflection is not an internal fourth step

The internal triad already says what a composable term must do to process a relation:

- prepare
- determine
- propagate

If reflection were simply appended as a fourth internal step, the distinction between substance and control would collapse. The term would no longer be distinguishable from the conditions under which it is being inspected, qualified, or redirected.

That would make it impossible to state cleanly:

- whether a process is valid,
- under what presuppositions it is valid,
- how it should be wrapped or suspended,
- what evidence or provenance is attached to it, and
- when a result should count as judgment, law, or action.

These are reflective tasks, not internal moments of the term's proper act.

### Why reflection must be meta-control

Reflection performs the work of judgment over the triad. It decides the space in which the triad counts.

Concretely, reflection is where a system can:

- establish scope,
- declare admissibility conditions,
- bind presuppositions,
- attach provenance or evidence,
- suspend, resume, or redirect execution,
- compare outcomes against invariants,
- and promote a result into a durable control state.

This makes reflection architecturally closer to **Context/Judgment** and to **controller surfaces** than to the internal metabolism of the term itself.

### Why the generic-dispatch analogy still helps

In CLOS-style systems, `around` methods are the nearest operational analogue. They can intercept the primary flow, decide whether and how it runs, and alter the conditions of continuation.

That analogy is useful, but only if translated carefully:

- Reflection is not merely a wrapper convenience.
- Reflection is not arbitrary middleware.
- Reflection is the control and judgment surface that determines how the inner triad is to be taken.

## Terminology Policy

For Organon documents and APIs, prefer:

- **Reflection** for the higher-order control layer
- **Context** when the reflective state is stored as active judgments, conditions, and presuppositions
- **Control** or **controller** when the reflective layer actively governs runtime flow

Avoid defaulting to imported names like:

- `around`
- middleware
- interceptor

Those analogies may appear in explanations, but the repo-native terms should remain primary.

## Reflection Model

### 1. Reflection as judgmental envelope

Reflection is the envelope that states the conditions under which a triadic process is valid and meaningful.

It is where the system says:

- what counts,
- what is allowed,
- what is presupposed,
- and what follows if the process succeeds or fails.

### 2. Reflection as stored active judgment

When reflection becomes durable, it appears as Context:

- active judgments,
- declared conditions,
- admissibility constraints,
- grounding metadata,
- and references to the operations that activated them.

This matches the repository's treatment of Context as a reflective determination surface rather than a passive bag of metadata.

### 3. Reflection as control over execution

Reflection is where a planning or runtime layer can:

- run validation before execution,
- assert invariants during execution,
- interrupt or resume execution,
- translate low-level outcomes into discursive or planning artifacts,
- and decide whether a propagated result should activate a next step.

This is why reflection maps naturally to controller/runtime separation.

### 4. Reflection as promotion surface

A propagated output is not yet automatically a new law, judgment, or active control state.

Reflection is the layer that promotes or refuses promotion.

It decides whether a result becomes:

- a stored judgment,
- a next-step presupposition,
- a control signal,
- a provenance-bearing artifact,
- or merely an ephemeral outcome.

## Consequences

### Positive

- Clarifies the boundary between the term's internal triad and the system's reflective control surface.
- Gives a precise home to Context/Judgment language already present in the repo.
- Prevents overloading triadic process models with responsibilities that belong to planning, validation, or governance.
- Makes controller/runtime patterns philosophically legible rather than ad hoc.
- Creates a stable concept for future FormDB, plan reflection tables, and provenance/control metadata.

### Negative / Trade-offs

- Requires discipline to avoid calling every validation hook "reflection."
- Introduces another architectural distinction that must be taught clearly.
- Can be misused if reflection is allowed to absorb the proper act of the term itself.

## Non-Goals

- Declaring that every API needs an explicit reflective wrapper.
- Replacing the internal triad with a four-stage pipeline.
- Treating reflection as mere logging, tracing, or narration.
- Smuggling user-space explanation requirements into the kernel's lawful compute layer.

## Implementation Guidance

When designing a pipeline, protocol, or execution surface:

1. First identify the internal triad.
   - What prepares?
   - What determines?
   - What propagates?

2. Then identify the reflective layer.
   - What declares admissibility?
   - What stores presuppositions?
   - What can suspend, resume, reorder, or promote?

3. Keep the two concerns separate.
   - The triad is the term's proper act.
   - Reflection is judgment over that act.

4. Prefer Context-shaped representations for durable reflection.
   - Conditions, active judgments, invariants, provenance, activation references.

5. Do not force reflection into hot-path implementation details unless control really requires it.
   - Many core compute paths should remain lawful and boring.

## Relation to Existing Structures

This ADR extends, but does not replace, the Term Logic ADR.

- `ADR-2026-04-15-TERM-LOGIC-TRIADIC-COMPOSITION.md` defines the internal architecture of composable terms.
- The present ADR defines the higher-order layer that governs and qualifies those terms in planning and execution.

It also aligns with existing repo structures:

- **Context** as active reflective judgment
- **Concept/Judgment before Syllogism** as the priority of reflection-bearing processing
- **validation before/after execution** as a practical reflection surface
- **controller/runtime separation** as an engineering form of reflective governance

## References

- `gds/doc/ADR-2026-04-15-TERM-LOGIC-TRIADIC-COMPOSITION.md`
- `gds/doc/CONCEPT-JUDGMENT-AS-REAL-MECHANISM.md`
- `gds/doc/CONCEPT-JUDGMENT-BEFORE-SYLLOGISM.md`
- `gds/doc/KANTIAN-PURE-MECHANICS-AS-CONCEPT-JUDGMENT.md`
- `gds/doc/PRELINGUAL-KERNEL-COGITO-MEDIATION.md`
- `gds/doc/GDSL-PLAN-LANGUAGE-PROPOSAL.md`
