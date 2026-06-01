# Frame Definition Template

Use this template to define a Frame for a special object without collapsing mediator logic, manifest logic, or trait surfaces.

This template is designed for the current architecture commitments:

- distributed Component Model
- traitless composition at the substrate level
- mediator and manifest are distinct moments
- Return to PureForm is explicit
- Logic may return to Corpus through a Logical Corpus path
- Mathematical Logic is scoped to the Known Field, not the Knower

---

## 0) Header

- Frame Name:
- Object Name:
- Moment:
- Kind: mediator | manifest | bridge
- Owner Module:
- Status: draft | active | deprecated
- Related Frames:

## 1) Identity Claim

What this object is, in one sentence.

- Identity:
- Not this object (negative boundary):

## 2) Boundary Contract

What is inside this frame and what is outside it.

- Inside:
- Outside:
- Upstream dependencies:
- Downstream dependents:

## 3) Moment Placement

Place the object in the dialectical motion so it is not mistaken for another moment.

- Position in arc: beginning | middle | end
- If middle: which mediation does it perform?
- If end: which manifestation does it stabilize?
- If bridge: which two moments does it connect?

## 3.5) Observer Frame and Relativity Contract

Make the framing assumptions explicit so behavior is not confused with absolutes.

- Observer position (who or what evaluates this frame):
- Reference frame (context, schema, runtime, or corpus scope):
- Frame-dependent outputs (what changes across observers/contexts):
- Frame-invariant outputs (what must remain stable):
- Transform rule between frames (how translation is performed):
- Information horizon (what this frame cannot see):

## 3.6) Known Field and Knower Separation Contract

Prevent confusion between object-logic and subject-logic.

- Known Field statement (what is formally knowable here):
- Mathematical Logic scope (which structures/inferences are validated):
- Knower exclusion (which subjective/agentive states are out of scope):
- Allowed bridge to knower-level reasoning (if any):
- Misclassification risk (where knower terms could leak into field logic):

Knower logic doctrine:

- Logic of the Knower is Reflective Logic.
- Reflective Logic is oriented to the unknown (hypothesis, revision, re-framing), not direct field-validity proof.
- Reflective outcomes can guide field inquiry, but they do not count as Known Field validity until field validation is passed.
- Speculation belongs to reflective knower activity and is never sufficient by itself for mathematical-logic validity.

## 3.7) DSL Externality and Copula Contract

Declare the DSL as the explicit point where internal component commitments become externally legible and executable.

- Externality statement (what the DSL exposes across boundaries):
- Copula role (how DSL joins component moments into a runnable relation):
- Analytic presupposition (simple first moment the copula starts from):
- Dialectical point (where negation/mediation is introduced):
- Internal commitments projected (model/feature/plan/logic forms):
- External commitments accepted (schema, corpus, runtime, protocol):
- 3^3 Dialectical Cube residency (which cube coordinates are represented here):
- Re-entry rule (how external execution returns to internal form):

Principle:

- The DSL is the Point of Externality of Frame Theory.
- The DSL is the operational copula of the Component Model.
- The dialectical copula presupposes an analytic first moment (simple being).
- The analytic moment is immediate division by a higher concept (simple being as first determination).
- The negative moment introduces dialectical/accidental differentiation against that immediacy.
- The positive return is the mediated copular unity produced through that negation.
- Dialectical movement begins when that simple moment is negated/mediated at a defined transition point.
- Cubical/dialectical variation belongs at this interface because becoming is coordinated at the boundary, not in isolated components.

Tri-moment quick prompt:

- Analytic placement (simple/immediate being as certainty):
- Negative placement (dialectical determination):
- Positive return placement (mediated copular unity):

## 4) Component Stance

Declare how the object participates in the distributed Component Model.

- Component role:
- Distribution unit (service/module/node/process):
- Coupling policy: loose | constrained | strict
- Trait policy: traitless core | trait adapter | trait-backed
- Why traitless (if chosen):

## 5) Invariants

Rules that must remain true for identity to hold.

1.
2.
3.

## 6) Degrees of Freedom

What may vary without breaking identity.

- Allowed variation dimensions:
- Forbidden variation dimensions:
- Versioning note:

## 7) Data and Shape Surface

Define the minimal shape and persistence surface.

- Primary frame/table/form:
- Required fields:
- Optional fields:
- Provenance fields:
- Validation fields:

## 8) Transformation Grammar

Legal transitions this frame can undergo.

- Input states:
- Allowed transforms:
- Output states:
- Illegal transforms:

## 9) Interface Contract

How other components read, write, or invoke this frame.

- Read surface:
- Write surface:
- Execution surface:
- Event or message contract:

## 10) Failure Semantics

How breakdown appears and how it is recorded.

- Failure classes:
- Detection method:
- Error representation:
- Recovery or rollback policy:
- Framing failures (wrong observer/frame assumptions):
- Knower leakage failures (subjective claims encoded as field logic):

## 11) Mediator/Manifest Separation Check

Prevent category mistakes.

- Mediator responsibility:
- Manifest responsibility:
- Explicit non-goals for mediator:
- Explicit non-goals for manifest:

## 12) Return to PureForm Contract

Record how execution returns as reflective evidence.

- Return envelope type:
- Execution identity fields:
- Formal context fields:
- Result summary fields:
- Reflective signals:
- Unknown markers captured for reflection:
- DSL boundary address used for return routing:

## 13) Logic Return to Corpus Contract

Use this section when Logic must return to Corpus.

- Return path statement:
- Logic artifact kinds produced:
- Corpus artifact kinds updated:
- Provenance link keys:
- Validation gate before corpus write:

Logical Corpus constraint:

- Logical Corpus is a logic-manifest evidentiary body.
- Logical Corpus is not a language-feature table.
- Language features may support logic, but do not define Logical Corpus identity.

## 14) Plan and Logic Open Questions

Use this section to keep unresolved work explicit instead of implicit.

Plan domain boundary:

- Plan, in this architecture, means mediation of inferential commitments for a Knowledge Agent.
- General-purpose task planning (for example, everyday procedural tasks) is out of domain unless translated into inferential commitments and traceable logic artifacts.

- Plan unknowns:
- Logic unknowns:
- Required experiments:
- Decision deadline:
- Temporary doctrine until resolved:

## 15) Test and Evidence Checklist

- Unit tests for invariants
- Integration tests for transform grammar
- Provenance round-trip tests
- Failure-path tests
- Return to PureForm record tests
- Logic-to-Corpus write/read verification

## 16) Minimal Example Stub

```text
Frame Name: LogicElementFrame
Object Name: Logic Element
Moment: end
Kind: manifest

Identity:
Stores normalized logic elements as durable manifest artifacts with provenance.

Observer frame:
Logic validity is evaluated in a declared corpus and schema context; no context-free validity claims.

Frame-invariant output:
Provenance and proof trace identity remain stable across context projections.

Known Field statement:
The Known Field is the corpus-bounded logic artifact space and its validated inferential relations.

Mathematical Logic scope:
Rule validity, entailment traces, and transformation correctness inside the declared field.

Knower exclusion:
No claims about agent consciousness, intention, or subjective certainty are part of logic validity.

Speculation note:
Speculative insight may originate in the mathematician as reflective guidance, but it enters logic only as a testable field claim.

Reflective bridge note:
Unknown-facing reflective judgments may be recorded as guidance signals, but they must be re-expressed as field-testable claims before entering Known Field logic.

DSL copula note:
The DSL surface is where internal commitments are bound to external execution contexts and then routed back as PureForm-return evidence.

Mediator responsibility:
Plan selects and organizes inferential commitments.

Manifest responsibility:
LogicElementFrame stores resulting logic artifacts and trace links.

Logic Return to Corpus:
ProofTraceFrame rows update Logical Corpus index entries by proof_id and source_span.

Constraint:
Logical Corpus is not a language-feature table.
```

---

## Fast Fill Order

If you are moving quickly, fill sections in this order first:

1. Header
2. Identity Claim
3. Invariants
4. Mediator/Manifest Separation Check
5. Return to PureForm Contract
6. Logic Return to Corpus Contract

Then complete all remaining sections.
