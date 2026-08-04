# GraphForm Taxonomy (Speculative, Dataset-Grounded)

Date: 2026-08-04
Status: Speculative Doctrine Reference

This reference defines the first GraphForm taxonomy using pre-existing Dataset formal components as the ground.

GraphForm is not a replacement for Dataset formalism. It is a graph-specific articulation of it.

---

## Task Framing

Layer:

- Kernel: graph/table execution discipline.
- Agent: graph intent orchestration and mediation.
- Logic: rationale for why graph outcomes count as knowledge.

Target:

- empirical adequacy
- conceptual validity
- rationale coherence

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

---

## Ground Rule: No New Ontology Without Dataset Ground

GraphForm must be authored as a determinate extension of existing Dataset components:

- structural substrate: Frame, Series, Expr
- semantic substrate: Model, Feature, Plan
- rational substrate: Corpus, LM, Logic

Compactly:

GraphForm = <Frame,Series,Expr><Model,Feature,Plan><Corpus,LM,Logic>

If a proposed GraphForm kind cannot be located in this triad, it is out of scope for this phase.

---

## Taxonomy Axes

GraphForm kinds are classified by two orthogonal axes.

Axis A: Scope of determination

- Graph scope: whole-graph determination.
- Node scope: per-node determination.
- Edge scope: per-edge determination.
- Subgraph scope: bounded local whole.
- Path scope: ordered relational determination.

Axis B: Moment of mediation

- Constitution: what the graph is.
- Featureing: what determinations it carries.
- Planning: how determinations will be executed.
- Realization: how results are emitted as artifacts.
- Evaluation: how validity is assessed.

---

## Canonical GraphForm Kinds (v0)

These are the first canonical kinds, grounded in Dataset formal components.

1. GraphConstitutionForm

- Moment: Constitution
- Required components: Model + Corpus + Logic
- Duty: declare graph identity, boundaries, and admissible relations.

2. GraphFeatureGrammarForm

- Moment: Featureing
- Required components: Feature + Feature structures + Logic
- Duty: declare legal Graph/Node/Edge feature strata and typing rules.
- Working spec: graphfeaturegrammarform-v1.md

3. GraphMediationPlanForm

- Moment: Planning
- Required components: Plan + Expr + Model
- Duty: compile semantic feature commitments into executable graph/table plans.

4. GraphExecutionFrameForm

- Moment: Realization
- Required components: Frame + Series + Expr
- Duty: realize nodes/edges/graph artifacts and execution receipts.

5. GraphEvaluativeNormForm

- Moment: Evaluation
- Required components: Logic + LM + provenance-bearing artifacts
- Duty: evaluate outputs against empirical, reflexive, and dialectical norms.

6. GraphServiceContractForm

- Moment: Realization/Evaluation boundary
- Required components: Plan + artifact profile + provenance
- Duty: stabilize user-facing GraphStore and analytics surfaces while internals evolve.

---

## Feature-Strata Law

Graph features must remain first-class and irreducible.

Required split:

- Graph Features: whole-graph properties (density, components, global constraints, graph-level embeddings).
- Node Features: per-node properties.
- Edge Features: per-edge properties.

Prohibited collapse:

- deriving Graph Features only as summaries without preserving graph-scope semantic status.
- encoding graph-scope commitments as ad hoc node or edge fields.

---

## Mapping to Existing Dataset Artifacts

GraphForm must emit existing artifact families rather than inventing opaque containers.

- graph constitution records -> ModelView / ProgramImage artifacts
- graph feature grammar records -> FeatureMap artifacts
- mediation plans -> ProgramPlan artifacts
- execution outputs -> Table + SemanticSubgraph artifacts
- evaluative outcomes -> attention/report artifacts with provenance

This preserves continuity with the current DatasetCatalog and compilation surfaces.

---

## Minimal Compilation Circuit

Program Feature -> GraphForm(kind) -> GraphFrame -> GraphExecutionIntent -> TaskFrame -> Shell/Daemon -> artifacts

Interpretation:

- GraphForm is the typed middle that prevents direct collapse from intent into runtime machinery.
- GraphFrame is the first high-level application species that carries this middle into work.

---

## Gate Checks for Any New GraphForm Kind

A proposed GraphForm kind is admissible only if all checks pass:

1. Dataset grounding check

- Which of <Frame,Series,Expr><Model,Feature,Plan><Corpus,LM,Logic> does it concretize?

2. Scope check

- Is scope explicit (Graph/Node/Edge/Subgraph/Path)?

3. Artifact check

- Which existing artifact kinds does it emit or transform?

4. Trace check

- Can provenance be followed from source commitment to graph outcome?

5. Invariant check

- Does it preserve empirical adequacy, reflexive consistency, and dialectical mediation?

If one check fails, keep the proposal as speculative note; do not promote to taxonomy.

---

## Near-Term Implementation Posture

1. Keep this taxonomy as reference doctrine while types mature.
2. Introduce one kind at a time, starting with GraphFeatureGrammarForm.
3. Validate by runnable exemplars and focused tests.
4. Promote only kinds that survive gate checks.

This keeps GraphFrame ambition high without losing method discipline.
