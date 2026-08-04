# GraphFeatureGrammarForm v1 (Provisional Name)

Date: 2026-08-04
Status: Speculative Working Spec

This reference specifies the first concrete form under the current GraphForm taxonomy:
GraphFeatureGrammarForm.

The name is provisional. The semantic content is primary.

---

## Task Framing

Layer:

- Kernel: enforces executable typing and scope checks.
- Agent: compiles feature intent into graph mediation plans.
- Logic: constrains admissible feature commitments and invalid collapses.

Target:

- empirical adequacy
- conceptual validity
- rationale coherence

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

---

## Purpose

GraphFeatureGrammarForm declares the legal feature language for graph mediation.

It answers:

- Which feature strata are allowed (Graph/Node/Edge)?
- Which types and constraints are valid at each stratum?
- Which derivations are legal across strata?
- Which collapses are prohibited?

It does not execute algorithms by itself. It governs what execution is allowed to mean.

---

## Dataset Grounding

This form is valid only as a determinate articulation of pre-existing Dataset components:

- Feature: named typed semantic address
- Feature structures: typed symbolic constraints and unification discipline
- Model: scope and relation commitments
- Plan: deferred execution route
- Provenance: traceability of feature commitments

Compact mapping:

GraphFeatureGrammarForm = Feature + FeatStruct + Model + Plan + Provenance

---

## Minimal Record Shape (Doctrine-Level)

```text
GraphFeatureGrammarForm {
  name: String
  version: String
  model_scope: GraphModelScope
  strata: [FeatureStratumRule]
  derivations: [FeatureDerivationRule]
  prohibitions: [FeatureProhibitionRule]
  evaluation_norms: [FeatureNormRule]
  provenance_policy: FeatureProvenancePolicy
}
```

Support shapes:

```text
GraphModelScope = graph_id + allowed_relation_kinds + boundary_rule

FeatureStratumRule = {
  stratum: Graph | Node | Edge,
  feature_name: String,
  value_type: Scalar | Vector | Symbolic | Distribution,
  required: bool,
  cardinality: One | Many,
  constraints: [ConstraintRule]
}

FeatureDerivationRule = {
  from: [feature_ref],
  to: feature_ref,
  derivation_kind: Aggregate | Propagate | Compose | Infer,
  monotonicity: Preserving | Expanding,
  trace_required: bool
}

FeatureProhibitionRule = {
  code: String,
  description: String,
  violation_class: ScopeCollapse | TypeCollapse | ProvenanceBreak
}
```

---

## First Normative Laws (v1)

1. Scope law

- Every feature declaration must bind to one explicit stratum: Graph, Node, or Edge.

2. Irreducibility law

- Graph-scope features cannot be represented only as node/edge summaries.

3. Typed-address law

- Every feature must declare type, cardinality, and constraint policy.

4. Derivation trace law

- Cross-strata derivations must retain explicit provenance links.

5. Plan-compatibility law

- Grammar declarations must compile into plan-legal operations.

---

## Integration with Existing Artifact Kinds

GraphFeatureGrammarForm should emit or enrich existing artifacts only:

- FeatureMap: stratum rules, types, constraints
- ProgramPlan: derived executable commitments
- ProgramImage: grammar version, checks, and gate outcomes
- SemanticSubgraph/Table artifacts: realized values with stratum trace

No opaque or standalone graph grammar container is introduced in v1.

---

## Gate Checks (Promotion to Stable)

Promote this form only if all are true:

1. Grammar replay determinism

- Same input declaration yields identical rule graph.

2. Strata correctness

- Graph/Node/Edge features stay explicitly separated in artifacts.

3. Plan realizability

- A declared grammar compiles to executable plan surfaces.

4. Provenance continuity

- Derivations are traceable from declaration to realized output.

5. Regression discipline

- No empirical adequacy regressions in baseline graph procedures without rationale gain.

---

## Example Mini Declaration (Conceptual)

```text
grammar citation_graph_v1:
  Graph:
    - density: Scalar(required)
    - component_count: Scalar(required)
  Node:
    - pagerank: Scalar(optional)
    - role: Symbolic(required)
  Edge:
    - weight: Scalar(optional)
    - relation_type: Symbolic(required)

  derive:
    - Graph.density <- Aggregate(Edge.weight)

  prohibit:
    - P001: Graph.density_as_node_field
```

This declaration is valid only if derivation trace and strata laws hold.

---

## Naming Generalization Matrix (For Next Pass)

Current name:

- GraphFeatureGrammarForm

Candidate family A (grammar emphasis):

- GraphGrammarForm
- GraphFeatureLanguageForm

Candidate family B (constitution emphasis):

- GraphFeatureConstitutionForm
- GraphDeterminationForm

Candidate family C (neutral mediation emphasis):

- GraphFeatureForm
- GraphFeatureSchemaForm

Selection rule:

- choose the name that best survives future extension beyond grammar-only concerns
- keep v1 content unchanged while renaming

---

## Practical Rule for This Cycle

Keep the current name for v1 implementation notes.
Generalize naming only after one runnable exemplar and one focused test path are green.

See also:

- intelligent-wrappings.md
- graphfeaturegrammar-formal.md
