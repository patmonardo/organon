# Organon Definitional Forms

Status: working draft (single canonical GDSL document).

## Intent

Define a single Organon semantic core (GDSL) as **one formal language**.

## Canonical Design Rule

- Compiler semantics: `CLOS + Prolog` (Hegel-leaning compilation model).
- Bridge definitions: `Form / Context / Morph`.
- Runtime/target language: `Cypher`.

Strategic note:

- Hegelian Objective Logic is treated as the higher-order semantic frame.
- Lisp/CLOS remains a major design influence, but not the final semantic limit.
- Selected general standard-library patterns may be encoded into Organon DSLs
  where they preserve form-semantic discipline.

Operational reading:

- Form Engines are effectively Cypher interpreters/executors over FormDB.
- Specs compile to Cypher-oriented graph operations (read/write/query).
- Entity facets may carry embedded props for local convenience.
- Operational truth is graph state resolved through Cypher queries of
  `Property` states and `Entity` states.

- Primary meta language: `Form` / `Context` / `Morph`.
- Class language: `Entity` / `Property` / `Aspect`.

Interpretation for this project:

- `Form/Context/Morph` = **MetaClass surface**.
- `Entity/Property/Aspect` = **Class surface**.

Direction rule:

- Classes instantiate metaclasses.
- Metaclasses are first-class and are not reconstructed from classes.

`Entity/Property/Aspect` are retained as familiar class realizations,
instantiated from `Form/Context/Morph`.

Projection to external standards is intentionally out of scope for this file
and is tracked in `ORGANON-FORMS-NOTES.md`.

## Idea Status

- `Concept`: `Form/Context/Morph` as first-class metaclass surfaces.
- `Concept`: classes instantiate metaclasses; `Property` is context-anchored.
- `Experimental`: CLOS + Prolog + Cypher unified reading.
- `Deferred`: detailed form engine behavior, FormDB physical schema, and full execution planner semantics.

## Unified Language Core

Every form-definition is a declarative unit:

- `id`
- `kind` (`form-def` | `context-def` | `morph-def`)
- `subject`
- `clauses[]`
- `justification`
- `priority` (optional)
- `provenance` (optional)

Definitions are authored in the `Form/Context/Morph` language first.

## CLOS-Inspired Reading

This model is closer to CLOS than to simple class taxonomies:

- `Form/Context/Morph` behave as first-class metaclass-level objects.
- Behavior and change are organized through `Morph`-level operations
  (generic-function style), not only inheritance trees.
- Classes (`Entity/Property/Aspect`) are concrete instantiations of those
  metaclass-level commitments.

## Primary Surfaces

### 1) Form Definition

Purpose: definitions governing identity-bearing determinacy.

Typical clause operators:

- `require(field)`
- `prohibit(field)`
- `compatibleWith(formId)`
- `incompatibleWith(formId)`

### 2) Context Definition

Purpose: forms governing conditions, scope, and admissibility horizon.

Typical clause operators:

- `within(scope)`
- `assume(assertion)`
- `requiresEvidence(kind)`
- `defeasibleBy(condition)`

### 3) Morph

Purpose: forms governing transition, mediation, and conditional origination.

Typical clause operators:

- `from(statePattern)`
- `to(statePattern)`
- `preserve(invariant)`
- `transform(field, rule)`

## Minimal Syntax (Draft)

```txt
formdef <id> : <kind> for <subject> {
  clause <operator>(<args>)
  clause <operator>(<args>)
  because "<justification>"
}
```

Example:

```txt
formdef d.grounding : morph-def for Morph {
  clause from(resolved)
  clause to(grounded)
  clause preserve(identityKeys)
  because "Grounding must not destroy identity continuity."
}
```

## Architectural Boundary

- Form Engines are invoke/orchestrate/persist controllers.
- Repos (Fact/Relation/Knowledge strategies) are engine-internal dependencies.
- Compiler owns dialectical semantics and proof/constraint logic.
- Root processor schemas remain engine-oriented (not ontology-theory-complete).

## Universal Metadata Presupposition

Universal metadata standards (for example Dublin Core and PROV-style provenance)
are presupposed as cross-cutting envelopes, not intrinsic root-shape semantics.

- Root runtime shapes carry only minimal execution trace metadata.
- Full metadata vocabularies are attached at persistence and export boundaries.
- Signature/Facet-style envelopes remain FormDB/runtime infrastructure concerns.

## Class Instantiation System

The following class forms are instantiated from the
primary `Form/Context/Morph` form surfaces.

Important clarification:

- `Property` is context-anchored and may exist independently of any specific
  `Entity` binding.
- `Entity` binding for a property is optional/situational and used when a
  context-level property is specialized for a concrete entity class.

### 1) Entity Definition

Purpose: define identity-bearing classes/types and inheritance.

Required fields:

- `id`
- `label`
- `identityKeys[]`

Optional fields:

- `extends[]`
- `disjointWith[]`
- `equivalentTo[]`
- `facets{}`

### 2) Property Definition

Purpose: define admissible determinations in a Context (optionally specialized
to an Entity).

Required fields:

- `id`
- `contextTypeId`
- `valueKind` (`datatype` | `object`)

Optional fields:

- `subjectTypeId` (optional entity specialization)
- `datatype`
- `objectTypeId`
- `cardinality{min,max,exact}`
- `functional`
- `defaultValue`

### 3) Aspect Definition

Purpose: define relation-level coherence and polarity commitments.

Required fields:

- `id`
- `relationType`
- `subjectTypeId`

Optional fields:

- `objectTypeId`
- `predicate`
- `polarity` (`affirmed` | `negated` | `mediated`)
- `constraints[]`

### 4) Constraint Definition

Purpose: data/shape validity constraints (operational profile).

Required fields:

- `id`
- `target` (type/property/aspect reference)

Optional fields:

- `severity` (`info` | `warning` | `violation`)
- `expression`
- `message`
- `closed`

### 5) Rule Definition

Purpose: derivation, enrichment, and inference hooks.

Required fields:

- `id`
- `kind` (`derive` | `infer` | `normalize` | `validate`)
- `body`

Optional fields:

- `target`
- `dependsOn[]`
- `priority`

### 6) Vocabulary Definition

Purpose: controlled terms and concept systems.

Required fields:

- `id`
- `scheme`
- `prefLabel`

Optional fields:

- `altLabels[]`
- `broader[]`
- `narrower[]`
- `exactMatch[]`

### 7) Provenance Definition

Purpose: traceability and scientific auditability.

Required fields:

- `id`
- `source`
- `recordedAt`

Optional fields:

- `agent`
- `confidence`
- `method`
- `lineage[]`

## State Semantics for Processor Flow

- `resolved` means an Entity satisfies Property admissibility (schema-level determination).
- `grounded` means an Entity supports Aspect-level relational commitments.
- Semantics:
  - resolved -> property support enabled
  - grounded -> aspect support enabled

## Conditional Genesis Interpretation (Morph-First)

These forms are interpreted for Conditional Genesis as follows:

- `Form` captures determinacy of what is.
- `Context` captures conditions under which determination is valid.
- `Morph` captures structured transition/origination (Becoming under constraints).

Operational reading:

- No grounded commitment without a valid conditioning context.
- No morph transition that destroys identity continuity unless explicitly declared.
- Every transition must preserve declared invariants or provide a declared
  transformation rule.

## Source/Target Language Correspondence

To reduce the terminology quandary, use this correspondence in reviews:

| Source (dialectical) | Target (Organon) | Primary form surface |
| -------------------- | ---------------- | -------------------- |
| Thing                | Form             | Entity               |
| World                | Context          | Property             |
| Relation             | Morph            | Aspect               |

This keeps Organon target language stable (`Form`/`Context`/`Morph`) while
allowing source-theory expression in dialectical terms.

## Instantiation Matrix (MetaClass -> Class)

This is the operative instantiation order used by ORG-FORMS:

| Primary Essence Surface | Class Instantiation Surface                 | Notes                                         |
| ----------------------- | ------------------------------------------- | --------------------------------------------- |
| `Form`                  | `Entity`                                    | Determinacy and identity anchors              |
| `Context`               | `Property` + `Constraint` + `Rule`          | Conditions, admissibility, and determinations |
| `Morph`                 | `Aspect` + `Rule` (+ transition properties) | Mediation, transition, and becoming           |

Interpretation rule:

- Author in `Form/Context/Morph` first.
- Instantiate `Entity/Property/Aspect` (and associated `Constraint/Rule`) second.

## CLOS + Prolog Lens

This model can be read as:

- `CLOS`: first-class metaclass surfaces and instantiation semantics.
- `Prolog`: declarative conditions, admissibility, and rule-style resolution.

### Example 1 — Form as metaclass commitment (CLOS-leaning)

```txt
formdef f.identity.core : form-def for Form {
  clause require(identityKeys)
  clause incompatibleWith(anonymous)
  because "Identity-bearing forms require stable keys."
}
```

### Example 2 — Context as admissibility logic (Prolog-leaning)

```txt
formdef c.measurement.scope : context-def for Context {
  clause within(empirical-context)
  clause requiresEvidence(observation)
  because "Claims are valid only under explicit evidential context."
}
```

### Example 3 — Morph as transition with preservation

```txt
formdef m.grounding.transition : morph-def for Morph {
  clause from(resolved)
  clause to(grounded)
  clause preserve(identityKeys)
  because "Transition to grounded must preserve identity continuity."
}
```

## CLOS + Prolog + Cypher Lens

Add a graph execution view on top of the same model:

- `CLOS`: metaclass objects (`Form`, `Context`, `Morph`) and instantiation.
- `Prolog`: admissibility and transition clauses.
- `Cypher`: graph persistence and traversal over instantiated structures.

Minimal graph perspective:

- Nodes: `Form`, `Context`, `Morph`, `Entity`, `Property`, `Aspect`.
- Edges: `(:Entity)-[:INSTANTIATES]->(:Form)`,
  `(:Property)-[:SCOPED_BY]->(:Context)`,
  `(:Aspect)-[:MEDIATES]->(:Morph)`.

Tiny Cypher sketch:

```cypher
MATCH (e:Entity)-[:INSTANTIATES]->(f:Form)
MATCH (p:Property)-[:SCOPED_BY]->(c:Context)
MATCH (a:Aspect)-[:MEDIATES]->(m:Morph)
WHERE m.state = 'resolved'
RETURN e.id, f.id, p.id, c.id, a.id, m.id
```

Transition-path Cypher sketch:

```cypher
MATCH (m1:Morph {state: 'resolved'})-[:TRANSITIONS_TO]->(m2:Morph {state: 'grounded'})
OPTIONAL MATCH (a:Aspect)-[:MEDIATES]->(m2)
RETURN m1.id AS fromMorph, m2.id AS toMorph, collect(a.id) AS groundedAspects
```

Interpretation:

- Form gives identity semantics.
- Context gives admissibility scope.
- Morph gives transition state.

## Formal Design Note — Container-Governed Worlds

This section formalizes the CLOS-inspired extension for Organon:

- Universal supertype is interpreted as truth-bearing existence (`T`) at the
  meta-level, not as a concrete domain class.
- `Container` is the governing metaobject that determines which
  `Form/Context/Morph` commitments are active in a world.
- `Entity` identity is stable across containers; meaning and admissibility are
  container-scoped.

### Terminology

- `Container`: world-governing metaobject (activation, scope, policy).
- `Entity`: identity-bearing subject with stable identity keys.
- `Form`: determinacy commitments for what an entity can be.
- `Context`: admissibility commitments for when claims are valid.
- `Morph`: meta-aspect commitments for how aspectual state can change.
- `Aspect`: class-level relational interpretation governed by `Morph`.
- `Claim`: a property/aspect assertion interpreted under
  `(Container, Form, Context, Morph)`.

### Core Invariants

1. Identity stability:
   - `Entity.id` and declared `identityKeys` are container-invariant unless a
     declared morph explicitly performs identity transformation.
2. Contextual property resolution:
   - Property truth is resolved from
     `(Entity, Property, Container, Form, Context)`, never from
     `(Entity, Property)` alone.
3. Morph-governed aspect interpretation:
   - Aspect truth is resolved from `(Entity, Aspect, Container, Form, Morph)`.
   - `Aspect` semantics are primary under `Morph`; contexts may constrain
     admissibility but do not replace morph-level mediation semantics.
4. Container sovereignty:
   - A container may activate/deactivate admissibility rules without mutating
     canonical entity identity.
5. Traceability:
   - Every operational claim must retain provenance to originating
     `Form/Context/Morph` definitions.
6. Conflict visibility:
   - Contradictory same-priority claims must be represented explicitly, not
     overwritten.

### Truth Status Lattice (Operational)

Recommended statuses for claim interpretation:

- `asserted`
- `inferred`
- `conflicted`
- `suspended`
- `retracted`

Monotonicity guideline:

- provenance should be monotonic (never lost), while status may be revised by
  higher-priority context/container rules.

### Metaobject Control Surface

`Container` governs the active interpretation profile:

- active contexts and admissibility horizon;
- rule priorities and defeasibility strategy;
- allowed morph transitions;
- conflict handling policy (block, branch, or annotate).

This yields world-building without identity fragmentation: shared entities,
container-specific interpretation.

### Canonical Axiom Block (Objective Logic)

Organon is treated as a **Theory of Relational Spectrum** grounded in a
**Theory of Grounds** and **Conditional Genesis**, with the Hypothetical
syllogism (`if ... then ...`) as Reason's primary movement.

Axiomatic reading:

- `Form` is the category of identity-preserving determination.
- `Context` is admissibility fibration/scope over determination.
- `Morph` is `MetaAspect`: the meta-level organizer of aspect composition,
  transition, and mediation constraints.
- `Aspect` is class-level realization of `Morph` in a container-scoped world.

Grounding thesis:

- relations are graded/structured modes of grounding, not merely static links;
- `Morph-Aspect` is the control axis of the relational spectrum;
- claims hinge on hypothetical structure and are valid when hypothetical
  conditions are satisfiable in the active container profile.

Practical evaluation rule:

- evaluate property claims under `(Container, Form, Context)`;
- evaluate aspect claims under `(Container, Form, Morph)`;
- compose both for full claim interpretation under
  `(Container, Form, Context, Morph)`.

Interpretive consequence:

- evaluation centers on grounded hypothetical transitions in a
  container-governed world, not on isolated predicates.

### Processor Boundary (Hypothetical vs Categorical)

The Form Processor is Essence-oriented and primarily evaluates hypothetical
grounds (`if X then Y`).

- `X -> Y` belongs to hypothetical mediation (ground-governed transition).
- `X | Y` and `X is Y` are categorical/being-level forms.
- Categorical forms are treated as encoding/decoding boundaries:
  - input encoding may arrive in categorical syntax;
  - processor semantics remain hypothetical;
  - output decoding may project into categorical concepts.

Operational law thesis:

- hypothetical ground is what gives lawful structure to contained appearance
  ("law to nature") in container-scoped world interpretation.

### Claim Interpretations as Objects (Algebraic-Category Reading)

Treat claim interpretations themselves as typed objects in an algebraic category:

- objects: interpreted claims (property-claims, aspect-claims);
- morphisms: admissible transformations/derivations between claims;
- composition: chained derivations preserving declared invariants/provenance;
- identity morphism: stable reinterpretation under unchanged
  `(Container, Form, Context, Morph)` commitments.

Operational implication:

- property-objects are context-indexed;
- aspect-objects are morph-indexed;
- both are container-scoped and form-disciplined;
- conflict appears as non-composable (or branching) morphism structure rather
  than silent overwrite.

## Prolog + Cypher Synthesis (Best-of Extraction)

Use Prolog where declarative resolution is strongest, and Cypher where graph
materialization/traversal is strongest.

### Keep from Prolog

- Horn-clause style rule authoring for admissibility and derivation.
- Unification-style variable binding for pattern-level rule reuse.
- Explicit negation/defeasibility discipline at rule level.
- Explainability via proof traces (`why` a claim is admissible/inadmissible).

### Keep from Cypher

- Native graph pattern matching for world-state retrieval.
- Efficient path and neighborhood traversal over Entity/Aspect structures.
- Declarative write operations for materializing derived graph facts.
- Operational fit with FormDB runtime and indexed graph persistence.

### Division of Labor

- Prolog layer: decides admissibility, derivation, and conflict classification.
- Cypher layer: stores claims, relations, transitions, and executes retrieval.
- Bridge contract: Prolog emits normalized claim deltas; Cypher applies and
  indexes them as graph updates.

Interpretation emphasis:

- property claim derivation is context-driven;
- aspect claim derivation is morph-driven;
- container policy composes both into one world-state.

### Normalized Claim Delta (Draft)

```txt
claim_delta {
  claimId,
  entityId,
  predicate,
  value,
  containerId,
  contextId,
  status,        // asserted|inferred|conflicted|suspended|retracted
  justification, // rule/proof reference
  priority,
  provenance[]
}
```

### Minimal Execution Loop

1. Load active `(Container, Context)` profile.
2. Load active `Form` and `Morph` commitments for that container.
3. Evaluate Prolog-style rules over current graph facts.

- property rules over `(Container, Form, Context)`
- aspect rules over `(Container, Form, Morph)`

4. Produce normalized claim deltas with status/provenance.
5. Apply deltas to graph store via Cypher writes.
6. Query resulting world-state via Cypher reads/traversals.

## Non-Goals

- Replace all standards with a new wire protocol.
- Encode full theorem proving inside root processor schemas.
- Collapse engine/repo and compiler concerns into one layer.

## Validation Requirements

- A complete definitional package must include at least one `Form`, one
  `Context`, and one `Morph` form-surface declaration (directly or by
  instantiable equivalent).
- Every `Entity/Property/Aspect` artifact must be traceable back to
  `Form/Context/Morph` origin semantics.
- Every definition must parse as one of `form-def` | `context-def` | `morph-def`.
- Definition conflicts (direct contradiction at same priority) must be reported.
- Each definition must validate as independent schema instance.
- A complete definitional package must include at least one Entity Definition.
- Every Property Definition must point to an existing Context definition;
  `subjectTypeId` is optional and only required when entity specialization is intended.
- Every Aspect Definition subject/object references must resolve to Entity Definitions.
- Every Rule Definition target (if present) must resolve to an existing definition id.

## Next Step

Define a compact forms appendix that maps each definition to an originating form kind,
and then to its class instantiation surface:

- `form-def` (determinacy)
- `context-def` (conditions)
- `morph-def` (origination/becoming)
