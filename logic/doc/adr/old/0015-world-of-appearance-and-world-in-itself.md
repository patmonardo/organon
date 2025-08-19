---
adr: 0015
title: World of Appearance and World‑in‑Itself — Dual World, Opposition, and Inversion
status: proposed
date: 2025-08-16
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

This ADR translates “B. THE WORLD OF APPEARANCE AND THE WORLD‑IN‑ITSELF.”
Key theses to operationalize:
- The appearing world raises itself to a kingdom of laws; appearance’s changing is also enduring; its positedness is law (law as simple identity in appearance).
- Law is the substrate of appearance (not its ground) yet also its negative unity: the sides of law are not only different but negatively refer to each other; each contains the other in itself.
- With negative unity posited, law becomes essential negativity and totality: its content is “every determinateness in general,” essentially connected in a totalizing connection.
- Thus appearance reflected‑into‑itself discloses a world above the world of appearance: a world that exists in‑and‑for‑itself (suprasensible), the true concrete existence.
- The totality splits: a sensible/posited world (appearance) and an essential world (in‑itself). Their identity is a connection of opposition; the essential world is the inversion of the world of appearance.

## Decision

Introduce a dual‑world computed representation with explicit opposition and inversion, built from Appearance and Law:

- AppearanceWorld (W_app):
  - The posited/flux world (A‑Surface + grounds/conditions among appearing things).
  - Data: { nodes (entities/properties/relations), grounds (dependency edges), snapshot window, signature, evidence }.

- LawSet with Negative Unity (L±):
  - Laws as identities across flux, upgraded to include negative unity: each law’s sides both imply and negate/contain one another.
  - Data: Law± = { id, statement, sides: [A,B], relations: { identity, opposition }, totalizingLinks[], signature, evidence, proof? }.

- EssentialWorld (W_ess):
  - The world in‑and‑for‑itself: total reflection that includes the negative form (opposition) and totalizing connection.
  - Constructed by closing L± over content and introducing explicit Opposition and Inversion edges.
  - Data: { nodes (essentials), oppositions (A ↮ B), inversions (x ↦ x^†), totalizingGraph, signature, evidence }.

- WorldDual (Binder):
  - Binds W_app and W_ess; asserts their identity as connection of opposition; records inversion mapping.
  - Data: { worldId, appRef, essRef, inversionIndex, contentSignature, evidence }.

No schema changes. All are computed projections/views with content‑addressed signatures and provenance.

## Implementation Sketch

1) Build W_app
- Input: EssenceGraph, reflect?, window.
- Collect appearing nodes and their grounds/conditions (from relations and property dependencies). Attach reflect facets and volatility metrics.
- Signature: sha1(graph.hash, window, reflect.hash).

2) Upgrade Law to Law±
- Input: LawSet from ADR‑0013/0014 extraction.
- For each law, compute:
  - identity(A,B) (positedness of one is positedness of the other), and
  - opposition(A,B): negative unity link such that each contains the other while repelling it.
- Add totalizingLinks by stitching laws through shared variables/terms; compute a content closure graph.

3) Build W_ess
- Input: W_app, Law±.
- Produce essential nodes (abstracted variables/terms/entities under law closure), add:
  - Opposition edges from Law±, and
  - Inversion edges mapping each W_app determination x to its essential inverse x^† in W_ess (the inversion of the world of appearance).
- Compute totalizingGraph (transitive closure over Law± links) and contentSignature.

4) Bind Worlds
- worldDual = { appRef, essRef, inversionIndex } with signature sha1(W_app.sig, W_ess.sig).

Placement: helper modules under absolute/model; surface via `model.projections`.

## Contracts (minimal)

- deriveAppearanceWorld(
  graph: EssenceGraph,
  reflect?: ReflectResult,
  window?: { from: string; to: string },
  opts?: Record<string, unknown>
): Promise<AppearanceWorld>

- upgradeLawsToNegativeUnity(
  laws: LawSet,
  opts?: Record<string, unknown>
): Promise<LawSetNegativeUnity>

- deriveEssentialWorld(
  app: AppearanceWorld,
  lpm: LawSetNegativeUnity,
  opts?: Record<string, unknown>
): Promise<EssentialWorld>

- bindWorlds(
  app: AppearanceWorld,
  ess: EssentialWorld
): Promise<WorldDual>

Each returns { …, signature: string, evidence: string[] }.

## Edge Cases
- Sparse laws → W_ess degenerates to thin identity graph; still produce inversion mapping for traceability.
- Conflicting laws → maintain multiple oppositions with confidence/proofs; do not collapse without mediation.
- Window drift → include time bounds in all signatures to avoid pseudo‑invariants.

## Rationale

- Matches the text: Law as substrate and negative unity; totalizing connection; dual worlds with identity as opposition; essential world as inversion of appearance.
- Non‑invasive: computed projections with strong provenance; idempotent via signatures.

## Future Work
- Force & World of Forces ADR: mediate opposition dynamically (force/counter‑force), integrate with W_ess.
- Kingdom of Ends: Control/Plan consume W_ess and Law± to synthesize Ends aligned with the totalizing connection (teleology).
