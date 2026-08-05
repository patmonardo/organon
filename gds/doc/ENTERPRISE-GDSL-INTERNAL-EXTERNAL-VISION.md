# Enterprise GDSL Vision: Internal First, External Next

Date: 2026-08-05
Status: Active implementation guide (speculative, no compatibility guarantees)

## Task Framing

Layer:

- Kernel: execution mediation and route/policy control
- Agent: request determination from generic form to executable graph plan
- Logic: rationale for why outcomes count as knowledge

Target:

- empirical adequacy
- conceptual validity
- rationale coherence

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

## Architectural Thesis

Enterprise GDSL is the canonical ingress language for graph requests.

The request is first generic, then determinately shaped through stored graph forms in the GML Dataset, then executed through GraphFrame -> TaskFrame -> TaskDaemon.

Compactly:

FormRequest -> GMLDataset(StoredGraphForms) -> GraphFrame Plan -> TaskFrame -> TaskDaemon

## Internal GDSL (Current Authority)

Internal GDSL is the in-kernel contract surface and currently authoritative.

Required properties:

- typed transmission contract with explicit execution surfaces
- explicit dataset framing metadata
- route selection and policy enforcement in GraphFrame TaskDaemon adapter
- deterministic trace events for begin/end, policy blocks, and contract mismatches

Current realization:

- GraphFrame transmission contract as enterprise graph intent carrier
- initial GML Dataset pipeline builder from transmission spec
- runtime profiles and strict component governance at daemon boundary

## External GDSL (Next Authority)

External GDSL will be a stable boundary protocol over the internal contract.

Required properties:

- explicit contract version field and feature/capability negotiation
- deterministic parse/lower pipeline into internal GraphFrame contract
- explicit error surface with policy/routing validation classes
- transport-neutral payloads (TS-JSON first, others optional)

Non-goal:

- no direct procedure invocation as public API

Rule:

- external requests must lower to TaskDaemon-mediated execution

## PureForm Cleanup Posture

PureForm language is now optional doctrine, not GraphFrame runtime authority.

Cleanup rule:

- remove PureForm-oriented naming from GraphFrame public surface
- retain Shell internals temporarily only where needed for migration sequencing
- do not add compatibility shims unless strictly temporary and isolated

Immediate cleanup sequence:

1. GraphFrame public API naming aligned to GDSL reciprocity and contract flow
2. Replace PureForm-centric comments/examples in GraphFrame-facing docs
3. Isolate residual Shell PureForm return terms behind Shell-internal boundaries
4. Introduce External GDSL boundary types as first-class ingress contracts

## Boundary Law

Procedure < Pipeline < Shell < TaskDaemon Gateway

Operationally:

- all shell-mediated graph execution should pass through TaskDaemon
- GraphFrame is the determinate mediation layer for enterprise graph requests

## Product Lanes

The architecture supports multiple lanes without forcing one worldview:

- application-server lane: enterprise graph analytics and store operations
- semantic lane: stat learning and dialectical learning extensions
- reflective lane: rational-agent overlays as opt-in, not mandatory

## Governance Rule

No backward compatibility is assumed during this speculative phase unless a compatibility shim is:

- temporary
- isolated from primary surfaces
- removed once migration window closes

This keeps architecture velocity high while preserving method discipline.
