# Doctrine Workbench

Purpose: turn Doctrine into a runnable learning surface for researchers.

This module already has canonical exposition (`EXEMPLARS/` and `REFERENCES/`).
The workbench adds executable movement so the doctrine can be studied as a
topic map of software design, not only as static text.

## Core Claim

The canonical mediation path is:

```text
DataFrame -> TaskFrame through shell_ -> proc_ -> eval_
```

The shell track is the center of that mediation because it binds declarative
plan language to procedure runtime and then publishes an eval-readable return.

Primary artifact root for this path:

- `gds/fixtures/collections/shell/shell_compute_protocol`

## Runnable Modes

Run from repo root:

```bash
bash gds/Doctrine/workbench.sh quick
bash gds/Doctrine/workbench.sh mediation
bash gds/Doctrine/workbench.sh shell
```

Mode intent:

- `quick`: short onboarding arc (DataFrame -> Dataset middle -> Shell control)
- `mediation`: explicit `shell_ -> proc_ -> eval_` teaching path
- `shell`: shell-focused curriculum slice before deeper procedure/eval descent

## Reading Contract

Treat generated fixtures as curricular evidence:

1. `00-*` files: immediate scene and frame seeding
2. `01-*` files: shell plan and program feature commitments
3. `02-*` files: procedure result and return register
4. `03-*` files: pureform principle closure

This preserves the architectural framing:

- Kernel target: empirical adequacy (real runtime outputs)
- Agent target: conceptual validity (coherent shell/procedure coordination)
- Logic target: rationale coherence (readable doctrinal closure)

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

## For Researchers New to the Platform

Suggested first sequence:

1. Read `INDEX.md` and `PRINCIPLE-FOUNDATION.md`
2. Run `bash gds/Doctrine/workbench.sh quick`
3. Inspect `gds/fixtures/collections/shell/shell_compute_protocol/README.txt`
4. Run `bash gds/Doctrine/workbench.sh mediation`
5. Read `EXEMPLARS/shell/029-shell-compute-protocol.md`

This workflow is intentionally notebook-like, but backed by real executable
examples and persisted fixtures.
