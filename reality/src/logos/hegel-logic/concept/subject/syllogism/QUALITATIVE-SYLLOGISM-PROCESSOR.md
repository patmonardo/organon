# Qualitative Syllogism Processor

Status: draft
Authority: derivative from Distillations + Workbooks (non-authoritative)

## Purpose

- Define the immediate implementation target for the Subject-side logical processor.
- Keep the focus on Concept, Judgment, and Syllogism as the sufficient infrastructure for a first executable logical knowledge base.
- Treat the Qualitative Syllogism as the first serious stress-test of Judgment in execution.

## Strategic Decision

- Do not rush into Objectivity for implementation sequencing.
- Treat Objectivity as a later consequence of successful Subject-side mediation, not as a prerequisite for building the first logical processor.
- Use Distillation files to align interpretation.
- Use Workbook files to formalize graph structure, claims, relations, and operations.

## Implementation Thesis

We are not translating odd Hegelian language into modern engineering metaphors after the fact. We are implementing the power of judgment itself as graph-mediated logical execution. In this repository's implementation perspective, the "Absolute Form in execution" is a GraphDB-capable logical platform whose RDF/TopicMap dataset can carry:

- term-positions,
- predicative relations,
- mediating middle terms,
- figure-rotations,
- contradiction detection,
- premise recursion,
- and transition triggers into richer forms of mediation.

## Subject-Side Architecture

### 1. Concept

Concept supplies the term space.

- Universal: whole rule or schema horizon.
- Particular: differentiating region or determinacy used for mediation.
- Singular: concrete node or this-instance where determinacy is explicit.

Implementation role:

- define term kinds,
- define role-mobility across U/P/S,
- define the conditions under which a determination can function as an extreme or a middle.

### 2. Judgment

Judgment supplies the predicative split.

- Subject and predicate are not pre-given inert labels.
- Judgment posits them as opposed yet related sides.
- The copula is the first execution of concept into determinately asserted relation.

Implementation role:

- define assertion edges,
- define subject/predicate polarity,
- define qualitative, reflective, and necessary judgment contexts,
- define when a judgment is immediate, mediated, contradictory, or transition-forcing.

### 3. Syllogism

Syllogism supplies explicit mediation.

- The middle term is the processor's essential operator.
- The figures are not optional variants; they are failure-driven reorganizations of mediation.
- The syllogism of existence is the first executable but unstable mediation engine.

Implementation role:

- define triadic mediation,
- define figure-specific role assignment,
- define conclusion-generation,
- define formal breakdown conditions,
- define transition from existence to reflection.

## Why Start with the Qualitative Syllogism

The qualitative syllogism is the first place where the logical machine meets concrete content and therefore the first place where a dataset-backed logical processor can genuinely fail in an intelligible way.

Its value is architectural:

- it exposes arbitrary middle-term selection,
- it reveals how formally valid inference can still be semantically contingent,
- it forces premise-validation,
- it generates the bad infinite regress of ungrounded mediation,
- and it compels figure-rotation as an internal repair attempt.

That makes it the correct first implementation target.

## Processor Primitives

The first processor pass should normalize the following primitives across Subject workbooks.

### Core entities

- Term
- Extreme
- MiddleTerm
- Figure
- Premise
- Conclusion
- JudgmentLink
- ContradictionSet
- TransitionTrigger

### Core roles

- singular
- particular
- universal
- subject
- predicate
- middle
- extreme_left
- extreme_right

### Core operations

- instantiate_first_figure
- select_middle_term
- derive_conclusion
- detect_middle_term_contingency
- request_premise_grounding
- expand_premise_regress
- rotate_figure
- detect_formal_exhaustion
- close_mediation_circle
- transition_to_reflection

## RDF / TopicMap Projection

The processor should be representable as a dataset-first Knowledge Graph.

### Dataset view

- Terms are nodes.
- Role-assignments are typed edges.
- Premises and conclusions are assertion records.
- Figures are named mediation contexts.
- Contradictions are explicit conflict relations.
- Transition triggers are event-like relations marking why one form sublates into another.

### TopicMap view

- Workbook entries remain the authoritative source for semantic structure.
- Distillations remain the interpretive source for stable conceptual reading.
- Compiler or DB projections remain derivative.

## Immediate Work Sequence

### Phase 1. Stabilize Subject protocol vocabulary

- Extract the minimum shared processor vocabulary from Concept, Judgment, and Syllogism workbooks.
- Prefer stable operator names over expressive but one-off narrative phrasing.
- Preserve the current workbook schema while tightening semantic reuse.

### Phase 2. Build the Qualitative Syllogism execution map

- Start from `SYLLOGISM-PART-A-WORKBOOK.md`.
- Normalize the first-figure protocol as an executable graph pattern.
- Mark the exact points where arbitrariness, contradiction, and regress are produced.
- Model the rotation into second and third figures as explicit transition operations.

### Phase 3. Back-propagate requirements into Judgment

- Identify which Judgment structures are required for the syllogism engine.
- Tighten the Existence-Judgment workbook around immediate predicative assertion and premise status.
- Make the Subject/Predicate polarity machine-readable for later processor compilation.

### Phase 4. Back-propagate requirements into Concept

- Tighten the U/P/S role grammar so role rotation in syllogistic figures is explicit.
- Ensure singularity is modeled as the point where determinacy becomes executable.
- Ensure particularity is modeled as the first unstable middle, not merely a taxonomic category.

## Concrete Next Artifact

The next artifact should not be an Object workbook. It should be a focused extraction pass over `SYLLOGISM-PART-A-WORKBOOK.md` that converts its active cognitive protocol into a reusable processor contract for:

- first-figure execution,
- middle-term contingency detection,
- premise-grounding recursion,
- figure-rotation,
- and transition to reflection.

## Start Here

Start with `SYLLOGISM-PART-A-WORKBOOK.md`, not `SYLLOGISM-IDEA-WORKBOOK.md`.

Reason:

- `SYLLOGISM-IDEA-WORKBOOK.md` already functions as the sphere-level coordinator and framing layer.
- `SYLLOGISM-PART-A-WORKBOOK.md` is the first authoritative execution surface for the Qualitative Syllogism itself.
- The final product we are trying to see is the machine in operation, and Part A is where the machine first becomes operationally visible.
- Once Part A is normalized as a processor contract, the IDEA workbook can be tightened afterward to reflect the stabilized operator vocabulary and architectural outcome.

Working sequence:

- read `EXISTENCE-DISTILLATION.md` to complete interpretive alignment,
- normalize `SYLLOGISM-PART-A-WORKBOOK.md` as processor contract,
- back-propagate required structures into Judgment and Concept,
- then revise `SYLLOGISM-IDEA-WORKBOOK.md` as the coordinator of the now-stabilized machine.

## Working Rule

Until the qualitative syllogism is modeled as a processor, Subject remains the implementation frontier. Objectivity should be entered only after the mediation engine is stable enough to show why facticity must arise from the logical processor rather than being assumed in advance.
