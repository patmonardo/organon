---
adr: 0014
title: A. The Law of Appearance — Jñānavijñāna Translation and Engine Contract
status: proposed
date: 2025-08-16
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

This ADR translates “A. THE LAW OF APPEARANCE” (Appearance as essential concrete existence) into the Logic processor design, integrating the Sāṁkhya Vyakta–Avyakta and Jñāna–Vijñāna (Jñānavijñāna: Science of Knowing) framing.

Key claims from the text:
- Appearance = concrete existence as essential (reflected immediacy), self-mediation via negation.
- Negative side: reciprocal negation and ground as presupposed; positive side: identity of the concrete existent with itself within negation.
- The essential content of appearance has two sides: (1) positedness/external immediacy (flux, accidental), (2) self-identical positedness (permanent content). Their unity = the Law of Appearance.
- Law is not beyond appearance; it is the restful copy within it (kingdom of laws).

Sāṁkhya/Jñānavijñāna mapping:
- Avyakta (Unmanifest) ↔ Essence/Ground (in-itself)
- Vyakta (Manifest) ↔ Appearance (reflected immediacy)
- Jñāna (Nondual Knowing) ↔ Absolute identity (lawful substrate)
- Vijñāna (Discriminative knowing) ↔ Extraction/recognition of law within appearance (normalized invariants)

## Decision

Introduce two computed surfaces and a thin binder:

1) AppearanceSnapshot (A-Surface)
- Purpose: capture the flux (positedness) with annotations that track reflective identity.
- Data: { snapshotId, window, graphRef (EssenceGraph), reflectRef (facets), stats, signature, evidence[] }
- Semantics: represents “positedness referring to positedness” (shine in shine), includes both accidental immediacy and its reflective markers.

2) LawSet (L-Surface)
- Purpose: extract the permanent element across the snapshot — the identity/content that remains self-equal in flux.
- Data: Law[] where Law = { id, statement, variables, conditions, domain, range, confidence, signature, evidence[], proof? }
- Semantics: “unity of the two sides” — the positedness of one is the positedness of the other. Normalized rules/equations/correlations with content-addressed signatures.

3) WorldView (Binder)
- Purpose: bind A-Surface and L-Surface; assert that law is in appearance (not beyond it).
- Data: { worldId, appearanceRef, lawsRef, contentSignature, evidence[] }

No schema changes. These are computed, optionally materialized as projections.

## Implementation Sketch

- Module: appearance
  - function appearanceSnapshot(graph: EssenceGraph, reflect?: ReflectResult, window?: { from: string; to: string }, opts?): AppearanceSnapshot
  - annotations: positedness metrics, volatility, reflective determining scores.
  - signature: sha1(graph.hash, reflect.hash, window)

- Module: law
  - function extractLaws(snapshot: AppearanceSnapshot, opts?): LawSet
  - strategy: reduce manifoldness to simple differences; detect identities/invariants (e.g., proportionalities, equivalences, stable mappings); attach evidence lines linking back to snapshot elements.
  - signature: sha1(sorted(normalizedStatements), context)

- Module: world
  - function worldStage(snapshot: AppearanceSnapshot, lawset: LawSet): WorldView
  - signature: sha1(snapshot.signature, lawset.signature)

- Placement: implement as pure helpers; expose their results via `model.projections` (views/indexes). Orchestrator may call them after `ground`/`action`.

## Engine Contracts (minimal)

- AppearanceSnapshot
  - input: EssenceGraph, ReflectResult?, window
  - output: { snapshotId, signature, evidence[] }

- LawSet
  - input: AppearanceSnapshot
  - output: { laws: Law[], signature, evidence[] }

- WorldView
  - input: AppearanceSnapshot, LawSet
  - output: { worldId, contentSignature, evidence[] }

All outputs include deterministic signatures and provenance evidence; proof is optional and can be added later.

## Edge Cases

- Sparse/volatile flux → few/no laws; return empty LawSet with low confidence and clear evidence.
- Conflicting candidates → keep both with confidence; resolution deferred to proofs or policy.
- Temporal windows → include {from,to} in signatures to avoid false invariants.

## Rationale

This design embodies the text’s logic: appearance contains its essential content; law is the positive essentiality (self-identity) immanent in flux; and law is not beyond appearance. Jñānavijñāna is operationalized as recognizing (Vijñāna) the lawful identity (Jñāna) within the manifest (Vyakta), grounded in the unmanifest (Avyakta).

## Future Work

- Force and World of Forces: capture mediation via force/counter-force as a dynamic complement to law (next ADR).
- Kingdom of Ends: have Control/Plan consume LawSet to propose Ends with feasibility/necessity tags.
