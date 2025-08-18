# ADR 0018 — Force, Parts 2–3: Solicitation and Infinity

Status: Proposed

Context

- Following Part 1 (Conditionedness), Force advances to: (b) Solicitation (reciprocal stimulus; active/passive unity) and (c) Infinity (identity of outwardness with inwardness). The text frames: each force presupposes/externalizes the other; stimulus solicits but is mediated; activity is reactive and returns-to-self; true expression is that reference-to-other equals reference-to-self.
- Goal: represent these as cheap, computed projections with deterministic signatures, no schema churn, and optional inputs from reflect/action surfaces.

Decision

- Provide two pure helpers:
  1) deriveSolicitationPairs(forces): produce symmetric pairings where each “solicits” the other; include mediated flags and a stable signature.
  2) expressForceInfinity(pair): compute an identity score asserting the equivalence of outward solicitation and inward self-reference; emit signature + evidence.
- Keep them independent of orchestration; they can be imported by higher-level stages later.

Consequences

- We operationalize Parts 2–3 with provenance, ready to compose with Action or World-of-Forces later.
- No persistence or relation schema required; results are ephemeral projections.

Implementation Notes

- Signatures: sha1 over normalized, sorted components.
- Evidence strings: pair:id:sig:..., mediated, symmetric flags; infinity: identity:outward=inward.
- Tests: happy path + symmetry + signature stability.
