# @organon/gdsl

Graph Data Science Library (GDSL) — a compact, focused core of reusable graph models, helpers and small DSL primitives used across the Organon monorepo.

This package provides the canonical graph primitives, a small expression/clause language (Cypher‑Lite inspired), and utilities intended to be imported by other @organon packages.

## Key ideas
- Lightweight, test-first primitives for graphs and clauses.  
- Designed to be imported by scoped package name: `@organon/gdsl`.  
- Exports are typed and built to `dist/` — packages consume `dist` at runtime, tests typically point at `src/` for fast iteration.

## Quick usage
Install (workspace-managed)
```bash
# within this monorepo use pnpm workspaces; from repo root:
pnpm install
