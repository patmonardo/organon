# ADR 0005 — Recognizing and Managing Speculative Bubbles

Status: Accepted
Date: 2025-09-07
Applies to: @organon/gds (kernel), @organon/gdsl (SDK/CLI)

Context
- We alternate between Speculative Refinement (useful exploration with guardrails) and Speculative Bubbles (scope drift that risks stability).
- Recent examples: Cypher‑Lite engine, Logogenesis rules — valuable, but must not destabilize the core (schemas, canonicalize, signatures, topic search, AgentKit).
- We need a light process to detect bubbles early and “deflate” them without losing insights.

Decision
- Adopt a Speculation Protocol with explicit states:
  - Core: stable, exported, versioned.
  - Refinement: targeted, test‑backed improvements to Core.
  - Exploration: bounded experiments behind flags, excluded from public exports.
  - Bubble: detected when Exploration breaks guardrails; must be paused or deflated.
- Guardrails are enforced via flags, exports policy, tests, and CI checks.

Signals (early warnings of a Bubble)
- Scope drift: goals change mid‑work without an ADR update.
- Invariant erosion: idempotence, provenance, or schema validation gets bypassed “temporarily”.
- Coupling: Core starts importing experimental modules; circular deps appear.
- Export creep: experimental APIs show up in public barrels or package exports.
- Test gap: new features lack happy‑path tests or reduce overall coverage on Core.
- Churn: repeated rewrites of the same experimental surface without decision to pause or ship.

Guardrails (hard)
- Feature flags: experimental code must require explicit env or config (e.g., GDSL_QUERY_RULES=1).
- Exports policy: experimental modules are never re‑exported from src/index.ts.
- Idempotence: CanonRules remain pure/idempotent; provenance never dropped.
- Schemas first: all persisted artifacts pass GraphArtifactSchema/FacetsSchema validation.
- Timeboxes: each Exploration gets a timebox and explicit exit criteria (ship, pause, or archive).

Process
- Pre‑flight (create Exploration):
  - Open a short ADR or note with: scope, guardrails, timebox, exit criteria, rollback plan.
  - Add tests up front (happy‑path) to define “done”.
- In‑flight (weekly check):
  - Verify: still behind flag? not exported? tests passing? no Core coupling? timebox respected?
  - If any fail, mark “Bubble suspected” and pause for decision.
- Post‑flight (exit):
  - Ship to Core only after: tests, docs, ADR acceptance, no flags required.
  - Otherwise pause (archive branch) or deflate (keep docs/tests, remove code from mainline).

Metrics (simple CI checks)
- Core coverage ≥ 80% (Exploration may be lower but cannot reduce Core coverage).
- Public exports list matches allowlist (CI fails if experimental files surface).
- Dependency guard: no imports from src/query/* inside src/dataset/* or src/schema/*.
- Size guard (optional): diff threshold on bundle size to catch accidental bloat.

Actions now
- Keep query engine and query rules gated by GDSL_QUERY_RULES=1 and out of public exports.
- Maintain CanonRules purity/idempotence; keep provenance in derived edges.
- Keep index.ts export surface Core‑only (schemas, canonicalize, signatures, topic, AgentKit, mock GDS).
- Add ADRs for new rule families or schema changes before implementation.
- Add a CI lint to forbid experimental paths in public barrel.

Checklist (Bubble detector)
- Is it behind a flag?
- Is it excluded from index.ts exports?
- Do happy‑path tests exist and pass?
- Does it avoid importing Core from experimental (no back‑edges)?
- Is the timebox still valid and documented?
- If any “No” → pause and decide: ship/pause/archive.

Consequences
- Faster iteration without Core instability.
- Clear way to pause/deflate without losing insights (tests/docs remain).
- Predictable surface for agents and CLI while we continue research (Cypher‑Lite, Logogenesis).

Related
- ADR 0003 — Rules in GDSL: schema vs. query
- ADR 0004 — GDS⇄GDSL architecture and the Logic–Model–Task
