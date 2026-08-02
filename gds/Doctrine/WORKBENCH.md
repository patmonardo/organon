# Doctrine Workbench

Purpose: turn Doctrine into a runnable learning surface for researchers.

This module already has canonical exposition (`EXEMPLARS/` and `REFERENCES/`).
The workbench adds executable movement so the doctrine can be studied as a
topic map of software design, not only as static text.

There are now two workbench surfaces:

- `gds/Doctrine/workbench.sh` for Doctrine-first curricular paths
- `cargo run -p gds --bin workbench_cli -- ...` for unified domain-level workbench access
- `cargo run -p gds --bin og_cli -- ...` as the emerging single Organon/OG entry surface

The deeper architectural reading is now explicit: Workbench is becoming the **OG Script Extension Framework**.
Subsystem and module developers define workbench modules, and `og_cli` becomes the single entry
surface that discovers and runs them.

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
bash gds/Doctrine/workbench.sh full
bash gds/Doctrine/workbench.sh map
bash gds/Doctrine/workbench.sh list
bash gds/Doctrine/workbench.sh example shell_compute_protocol
bash gds/Doctrine/workbench.sh cheap
bash gds/Doctrine/workbench.sh cheap-mediation
bash gds/Doctrine/workbench.sh cheap-shell
bash gds/Doctrine/workbench.sh cheap-example shell_compute_protocol
```

Unified workbench CLI:

```bash
cargo run -p gds --bin workbench_cli -- domains
cargo run -p gds --bin workbench_cli -- list shell
cargo run -p gds --bin workbench_cli -- run shell shell-compute-protocol
cargo run -p gds --bin workbench_cli -- run dataframe 1,3-4
cargo run -p gds --bin workbench_cli -- list ml-nlp
```

OG CLI alias:

```bash
cargo run -p gds --bin og_cli -- domains
cargo run -p gds --bin og_cli -- modules
cargo run -p gds --bin og_cli -- list shell
cargo run -p gds --bin og_cli -- run shell shell-compute-protocol
```

Mode intent:

- `quick`: short onboarding arc (DataFrame -> Dataset middle -> Shell control)
- `mediation`: explicit `shell_ -> proc_ -> eval_` teaching path
- `shell`: shell-focused curriculum slice before deeper procedure/eval descent
- `full`: systematic spine across Doctrine layers with taskframe endpoint
- `map`: write the Doctrine session map without running examples
- `list`: show low-cost modes and recommended single targets
- `example <name>`: run one exact example only
- `cheap`: built-in low-cost version of `quick`
- `cheap-mediation`: built-in low-cost version of `mediation`
- `cheap-shell`: built-in low-cost version of `shell`
- `cheap-example <name>`: built-in low-cost single-example run

Unified CLI intent:

- collapse scattered bin entrypoints into one workbench surface
- unify example-backed domains: `dataframe`, `dataset`, `shell`, `eval`, `task`
- retain command-backed domains: `ml-models`, `ml-nlp`, `tsjson`
- let `og_cli` become the named single-entry surface as Doctrine matures into the Rational Agent IDE app
- treat workbench modules as the extension units for IDE mode

Each run also emits a session map artifact:

- `gds/fixtures/collections/doctrine/doctrine_workbench_session/00-session-map.txt`

## Budget Controls

To reduce repeated compile cost:

- default behavior prebuilds examples once per run and then executes binaries directly
- set `DOCTRINE_WORKBENCH_BUILD=0` to skip prebuild and run existing binaries only
- set `DOCTRINE_WORKBENCH_REFRESH=0` to skip artifact listing output

Examples:

```bash
DOCTRINE_WORKBENCH_BUILD=0 bash gds/Doctrine/workbench.sh quick
DOCTRINE_WORKBENCH_BUILD=0 DOCTRINE_WORKBENCH_REFRESH=0 bash gds/Doctrine/workbench.sh mediation
bash gds/Doctrine/workbench.sh map
DOCTRINE_WORKBENCH_BUILD=0 DOCTRINE_WORKBENCH_REFRESH=0 bash gds/Doctrine/workbench.sh example shell_compute_protocol
bash gds/Doctrine/workbench.sh cheap-mediation
bash gds/Doctrine/workbench.sh cheap-example shell_compute_protocol
```

For the broader operating method under monthly budget pressure:

- [BUDGET-METHOD.md](BUDGET-METHOD.md)

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

## Server-Side Direction

The CLI workbench is the current execution surface. The target form is a server-side
application that exposes tracks, run state, and artifact/topic-map APIs.

Architecture and rollout notes:

- [WORKBENCH-SERVER.md](WORKBENCH-SERVER.md)
