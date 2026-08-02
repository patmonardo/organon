# Budget Method

Purpose: define a lower-cost operating method for using Doctrine, the Workbench, and the coding assistant under tight monthly limits.

## Main Rule

Do not pay for broad exploration when the repository already has named doctrinal surfaces.

Use this order:

1. Doctrine
2. Reference
3. Exemplar
4. Example
5. Fixture
6. Workbench

That means: name the exact layer first, then ask for one narrow action.

## The Cheap Method

Prefer manual, anchored requests over Auto mode.

Good prompts:

- "Read Doctrine exemplar 029 and summarize only the mediation law."
- "Run `bash gds/Doctrine/workbench.sh example shell_compute_protocol` and inspect only `02-shell-return.txt`."
- "Compare `shell_compute_protocol` and `proc_pathfinding_procedure` at the fixture level only."
- "Patch only `gds/Doctrine/WORKBENCH.md` to add one new low-cost workflow note."

Avoid prompts like:

- "Explore the repo and tell me what to do next."
- "Audit the whole doctrine system."
- "Run everything."

## Runtime Cost Ladder

From cheapest to most expensive:

1. Read Doctrine text only
2. `workbench.sh map`
3. `workbench.sh list`
4. `workbench.sh example <name>` with `DOCTRINE_WORKBENCH_BUILD=0`
5. `workbench.sh quick` with `DOCTRINE_WORKBENCH_BUILD=0`
6. `workbench.sh mediation` with `DOCTRINE_WORKBENCH_BUILD=0`
7. `workbench.sh full`
8. open-ended Auto mode

## Commands

Lowest-cost commands:

```bash
bash gds/Doctrine/workbench.sh map
bash gds/Doctrine/workbench.sh list
DOCTRINE_WORKBENCH_BUILD=0 DOCTRINE_WORKBENCH_REFRESH=0 bash gds/Doctrine/workbench.sh example shell_compute_protocol
```

Medium-cost commands:

```bash
DOCTRINE_WORKBENCH_BUILD=0 DOCTRINE_WORKBENCH_REFRESH=0 bash gds/Doctrine/workbench.sh quick
DOCTRINE_WORKBENCH_BUILD=0 DOCTRINE_WORKBENCH_REFRESH=0 bash gds/Doctrine/workbench.sh mediation
```

High-cost command:

```bash
bash gds/Doctrine/workbench.sh full
```

## Assistant Protocol

When using the coding assistant under budget pressure, constrain each turn to one of these:

- one file
- one example
- one fixture root
- one doctrinal question
- one validation command

Best pattern:

1. Name the exact file or example.
2. State the exact outcome wanted.
3. State the cheapest acceptable validation.

Template:

```text
Work only on <file or example>.
Goal: <single outcome>.
Validation: <single command or fixture read>.
Do not widen scope.
```

## Session Discipline

For a new session:

1. Start with `bash gds/Doctrine/workbench.sh map`
2. Choose one example with `bash gds/Doctrine/workbench.sh list`
3. Run one target with build disabled if binaries already exist
4. Read one fixture file
5. Ask one bounded follow-up question

This is slower than Auto mode, but it is sustainable.

## Strategic Principle

Under budget limits, systematic narrowness beats exploratory breadth.
The correct adaptation is not "do less thinking".
It is "spend thinking only where the hierarchy already gives an anchor".
