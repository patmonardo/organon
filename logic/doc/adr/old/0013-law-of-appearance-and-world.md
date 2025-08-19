---
ad r: 0013
title: Law of Appearance and World — From Kingdom of Laws to Kingdom of Ends
status: proposed
date: 2025-08-16
authors:
  - patmonardo
reviewers:
  - engineering
---

## Context

We have implemented Thing and Property (Concrete Existence) and introduced Reflect (Citta), Ground (Essential Relations), and an optional Action stage. Chapter 2 (Appearance) raises the next layer: appearance as essential concrete existence; the law as the permanent identity within flux; the world as the kingdom of laws; and, by teleology, the passage to a world as a kingdom of ends.

This ADR translates the "Law of Appearance" and "World" into processor design without schema churn, using computed surfaces, signatures, and provenance.

## Problem

- How do we represent Appearance (reflected immediacy) distinctly from immediacy, while keeping it in the system as the higher truth?
- How do we operationalize "law of appearance"—the self-identical content within flux—as a stable, queryable layer?
- How do we model the World as the unity of appearance and law (the kingdom of laws), and point toward the World as a kingdom of ends (teleological planning)?
- How do we keep this non-invasive and idempotent with provenance?

## Decision (Summary)

We introduce two computed, first-class surfaces and clarify their contracts:

- Appearance Surface (A-Surface): a view over EssenceGraph capturing the "reflected immediacy"—the flux—annotated with positedness and reflective identity. It’s dynamic and time-indexed, non-persistent by default.
- Law Surface (L-Surface): the restful, stable content extracted from appearance; a compact set of invariants (equations, correlations, rules) with signatures and proofs (when available). It’s stored as projections (indexes/views) with strong provenance.

World is the unity of A-Surface and L-Surface:

- World of Laws: the identity of content across flux and law (A↔L). Implemented as a WorldView object that binds Appearance snapshots with their corresponding Law set.
- Kingdom of Ends (forward integration): L-Surface informs Control/Plan to generate Ends (goals, tasks) consistent with law; Ends feed back into Action/Actualization.

No schema changes are required. Surfaces are computed and materialized optionally via model/projections and a small World module.

## Detailed Design

### Part I — Idea as Principle/Seed of World

- Seed (Principles) already exists: Shapes, Contexts, Morphs. We treat the "Idea" as the minimal principle that, under Reflect→Ground, generates Appearance and then Law.
- Contract: WorldSeed = { principles, initial conditions (contextId), timeWindow } to parameterize Appearance snapshots.

### Part II — Appearance as Law (Law of Appearance)

- Appearance is concrete existence as essential: the flux annotated by reflective identity and positedness. We operationalize this as:
  - A-Surface Snapshot: { t0,t1, graph: EssenceGraph, reflectFacets, actionHints, evidence }.
  - Extract Law by reducing flux to stable identities (deterministic relationships that remain invariant across the snapshot window).
- Law of Appearance data model (computed):
  - Law: { id, statement, variables, domain, range, conditions, confidence, signature, evidence, proof? }
  - Statement is a normalized rule (predicate, correlation, or equation).
  - Signature = stable hash of normalized statement + conditions + context.
  - Evidence = links to Appearance events/elements used.
- Implementation placement: L-Surface lives under model/projections as `indexes` (for queries) and `views` (for compact law sets). No schema changes.

### Part III — World as Kingdom of Laws (Unity of Appearance and Law)

- WorldView binds a specific A-Surface snapshot with its derived L-Surface:
  - WorldView: { id, appearanceRef, lawsRef, contentSignature, evidence }
  - contentSignature = hash(A.signature, LawSet.signature) for idempotence.
- The “restful copy” is the L-Surface; the “appearing world” is the A-Surface. The unity is established by signatures, timestamps, and bidirectional references.
- Contract: `worldStage(graph, reflect?, action?, window, opts) → WorldView` (pure, computed; materialization optional).

### Part IV — World as Kingdom of Ends (Teleology hook)

- Ends derive from Laws: Control/Plan uses L-Surface to propose Ends that are possible/necessary given constraints.
- Ends schema (computed): { goalId, lawRefs[], expectedEffect, feasibility, priority, signature, evidence }.
- This ADR does not implement Ends; it establishes that Control/Plan are teleological consumers of L-Surface.

## Engine Mapping

- Reflect: provides positing/external/determining facets that inform A-Surface annotations and assist Law extraction (e.g., weighting).
- Ground: supplies essential relations—raw material for both Appearance and Law.
- Model (Projections): hosts L-Surface artifacts (indexes and views). Appearance snapshots can be cached as projections if desired.
- Control/Plan: consumes Laws to propose Ends (Kingdom of Ends) while keeping provenance.
- Action/Actualize/Commit: remain unchanged; they may utilize Ends later.

## Contracts (minimal)

- AppearanceSnapshot
  - input: EssenceGraph, reflect?, window
  - output: { snapshotId, graphRef, reflectRef, stats, signature, evidence }
- LawExtraction
  - input: AppearanceSnapshot
  - output: { laws: Law[], signature, evidence }
- WorldStage (optional helper)
  - input: AppearanceSnapshot, LawSet
  - output: WorldView

All signatures are SHA-1 (or better) stable hashes for idempotence; evidence strings and structured provenance are attached.

## Edge Cases

- Empty or near-empty graphs: Law set is empty with low-confidence placeholders.
- Conflicting candidate laws: keep both with confidence scores; do not collapse without proof.
- Temporal drift: A-Surface windowing must be explicit; include timestamps in signatures.

## Alternatives Considered

- Persisting Laws as first-class schema: rejected for now; projections suffice and keep the core invariant minimal.
- Folding Appearance into Ground outputs: rejected; conflates immediacy and reflected immediacy.

## Impact

- Non-breaking; adds documentation and contracts for future helpers. Enables teleological planning via L-Surface without schema churn.

## Out of Scope / Future Work

- Force and the World of Forces: dedicated ADR (next) to capture mediation via Force and counter-force.
- Proof frameworks for necessity (beyond correlation): optional module for law proofs.

