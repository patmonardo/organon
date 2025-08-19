---
adr: 0016
title: World Synthesis — Essential World, Opposition, and Inversion
status: proposed
date: 2025-08-16
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

This ADR completes the World concept by translating “2. The world which is in and for itself …” into engine contracts. It formalizes the dual worlds (appearance vs. in‑itself), the essential connection as opposition, and the inversion relation: the world‑in‑itself is the inversion of the world of appearance.

Prior ADRs 0013/0014 defined Law of Appearance; 0015 introduced dual worlds and Law with negative unity (Law±). This ADR specifies the synthesis glue and implementable helpers.

## Decisions

We define three synthesis artifacts and a binder:

- OppositionGraph (OG): a graph of essential oppositions among content determinations derived from Law±. Each side contains and repels the other; identity is mediated via negative unity.
- InversionIndex (Inv): a total mapping from appearance determinations to their essential counterparts (x ↦ x^†), constructed via closure over Law± and OG.
- EssentialWorld (W_ess): the world in‑and‑for‑itself as a totalized content graph (every determinateness connected), containing explicit Opposition and Inversion edges.
- WorldDual (Binder): binds AppearanceWorld (W_app) and W_ess with the InversionIndex and signatures; asserts identity as opposition.

All are computed surfaces (no schema churn) and are idempotent via content‑addressed signatures with provenance.

## Implementation Sketch

Inputs
- W_app: from deriveAppearanceWorld(graph, reflect?, window)
- Law±: from upgradeLawsToNegativeUnity(LawSet)

Steps
1) buildOppositionGraph(Law±):
   - For each law with sides A,B, add OG edges A ↮ B (opposition) with evidence and optional weights (from reflect determining scores and law confidence).
   - Stitch via shared variables/terms into a connected graph.

2) buildInversionIndex(W_app, Law±, OG):
   - For each appearance determination x, find essential counterpart x^† via Law± normalization and OG closure.
   - inv[x] = x^†; record evidence with path traces through laws/oppositions; compute stable signature sha1(x, x^†, lawIds, path).

3) deriveEssentialWorld(W_app, Law±, OG, Inv):
   - Nodes are essential determinations (terms/entities under law normalization).
   - Edges: Opposition from OG; Inversion from Inv; Plus totalizing links from Law±.
   - Compute contentSignature = sha1(sorted(nodeSigs), sorted(edgeSigs)).

4) bindWorlds(W_app, W_ess, Inv):
   - WorldDual = { worldId, appRef, essRef, inversionIndexRef, contentSignature, evidence } with signature sha1(W_app.sig, W_ess.sig, Inv.sig).

Placement
- Implement as pure helpers under an absolute/model world module. Surface via `model.projections`.

## Contracts (minimal)

- buildOppositionGraph(lawpm: LawSetNegativeUnity, opts?): Promise<OppositionGraph>
- buildInversionIndex(app: AppearanceWorld, lawpm: LawSetNegativeUnity, og: OppositionGraph, opts?): Promise<InversionIndex>
- deriveEssentialWorld(app: AppearanceWorld, lawpm: LawSetNegativeUnity, og: OppositionGraph, inv: InversionIndex, opts?): Promise<EssentialWorld>
- bindWorlds(app: AppearanceWorld, ess: EssentialWorld, inv: InversionIndex): Promise<WorldDual>

Each returns { signature: string, evidence: string[], … }.

## Edge Cases & Policies
- Non‑bijective inversions: allow many‑to‑one; include scores; do not force total bijection unless a policy demands it.
- Conflicting inversions: keep multiple candidates with confidence; downstream consumers choose per policy.
- Time windows: include window bounds in signatures.

## Rationale
- Matches the text: totality, opposition, and inversion as the logic of identity between the two worlds.
- Keeps the core schemas stable; everything is computed with provenance and idempotence.

## Future Work
- Force mediation ADR: dynamic realization of opposition as force/counter‑force.
- Ends/Teleology: generate Ends using W_ess + Inv, ensuring goals respect essential connections.
