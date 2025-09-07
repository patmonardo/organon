# ADR 0002 — Testing strategy: "Happy‑path" unit tests with Vitest

Status: Proposed
Date: 2025-09-07

Context
- @organon/gdsl is a foundational library (schemas, Dataset, Signatures/Facets, Topic/Lens primitives).
- Tests are an architectural concern: they protect the public API, document intended behavior, and enable safe evolution.
- We keep the core small; tests should be readable and fast so they become part of every edit cycle.

Decision
- Use Vitest as the test runner for unit tests.
- Place tests in `test/` alongside `src/` (project-root: `gdsl/test/`) with a small directory structure:
  - test/happy/ — "happy‑path" unit tests (the starting point)
  - test/edge/ — later: edge cases and error paths
  - test/integration/ — later: integration/smoke tests that may use small fixtures
- Start with happy‑path tests that assert canonical, expected behaviour for public primitives:
  - computeTermSignatures returns normalized facets for simple fixtures
  - findTerms returns expected ranked results for straightforward queries
  - Dataset.fromArtifact produces usable Dataset wrappers

Happy‑path defined
- Tests that cover the common, intended, success scenarios.
- Minimal mocking; use small plain fixtures resembling real GraphArtifact/Topic shapes.
- Fast execution (< 100ms per test typically), deterministic, and independent.

Guidelines
- Tests should import the public internal APIs (e.g., `src/dataset/signatures`, `src/topic/search`).
- Keep tests small and focused: arrange/act/assert with clear fixtures.
- Use Zod validation in code; tests should use simple fixtures that reflect expected shapes.
- Prefer explicit assertions over snapshot tests for core primitives.
- Add edge/failure tests after the happy‑path base is stable.

CI and local workflow
- Run `npx vitest` (or `npm test`) locally during development.
- CI should run `npm run build && npm test` to catch type and runtime regressions.
- Keep test runs fast; mock slow IO in unit tests.

Consequences
- Quick developer feedback loop for core library changes.
- Clear documentation of expected behaviour in the tests.
- Future expansions (fuzzy search, complex canonicalize flows) can add integration tests under test/integration.
