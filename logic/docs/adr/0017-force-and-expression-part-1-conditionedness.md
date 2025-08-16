# ADR 0017 — Force and its Expression, Part 1: Conditionedness

Status: Proposed

Context

- We conclude the Science of Appearance by introducing Force as the “negative unity” that resolves the earlier whole/parts contradiction. Here we scope Part 1 to Force-as-conditioned: force presupposes immediacy (a “thing”/matter), yet is itself reflection (positedness). The “thing” as indifferent immediacy is no longer explanatory; instead, force posits externality as its own moment. This frames force as active unity that becomes external manifoldness from itself, but (in Part 1) still conditioned by an immediate.
- Historically, this sits between mechanical aggregation (Newtonian-style articulation as laws over parts) and a fully internal dynamic unity (rationalist reflection). Kant’s Transcendental Analytic mediates these by grounding the conditions of possible experience (and calculus-like synthesis) rather than choosing one side. Our engine analog: keep the schema stable; surface Force as computed conditions/projections over what already appears.

Decision

- Represent “Conditionedness of Force” as a pure analysis over reflective/contextual data, with no schema churn:
  - Inputs: a minimal “force candidate” record with flags for immediacy (external, concrete), reflection (positedness), and any externality evidence tags.
  - Output: a ForceConditionReport capturing: presupposition of a thing, appearance as positedness, transient immediacy, designated-as-matter tags, negative unity heuristic, a stable signature, and evidence strings.
- Provide a tiny helper in `src/essence/relation/force-conditionedness.ts` and a minimal unit test. This complements reflect/action without introducing persistence or new relations.

Consequences

- We can discuss Force concretely and cheaply: it remains a computed view with provenance, not a stored entity.
- Part 2 may extend with solicitation/reciprocity/infinity (forces mutually condition and elicit each other), potentially bridging to the Action stage and World of Forces without schema changes.

Implementation Notes

- Helper produces deterministic signatures (sha1 over normalized inputs) for idempotence.
- “Designated as matter” is modeled as tags derived from evidence tokens (e.g., magnetic, electric, ether), matching the textual motif without hard-coding physics.
- Tests cover the happy path and signature stability.
