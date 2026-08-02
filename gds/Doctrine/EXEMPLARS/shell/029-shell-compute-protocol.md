# Exemplar 029 — Shell Compute Protocol

**Source file**: `gds/examples/shell_compute_protocol.rs`
**Arc position**: Shell as mediated execution boundary between Dataset construction and Procedure realization
**Prior exemplar**: [028 — DataFrame as Intuition](../dataframe/028-dataframe-intuition.md)
**Next exemplar**: [030 — Ideal DataFrame DSL (Design Fiction)](../shell/030-ideal-dataframe-dsl.md)

---

## Principle

This exemplar teaches that Shell is not a decorative front-end scripting layer. It is the
control protocol through which a DataFrame-seeded and Dataset-mediated form becomes a
procedure-bound execution. In the newer doctrine, this matters because Shell is now read as
the immediate ancestor of `OG Script`: a readable authoring surface that begins as Shell Script
but already points beyond itself toward Dataset building, domain model design, and rational
execution discipline.

The core law is simple:

```text
DataFrame -> TaskFrame through shell_ -> proc_ -> eval_
```

This is not only a runtime path. It is the doctrinal path through which the system explains
why an algorithmic result counts as knowledge rather than opaque computation.

---

## What It Does

1. Builds a small relation surface as `00-frame.csv`, making the graph intelligible as a seeded
   DataFrame-style artifact rather than only as hidden runtime state.

2. Materializes an in-memory graph store and persists `00-graph.txt`, so the graph topology is
   inspectable as explicit adjacency and edge structure.

3. Uses `GdsShell.component_plan()` to author a BFS component call and persists `01-shell-plan.txt`.

4. Binds that plan through `ShellProcedureControl`, invokes a real backend graph algorithm, and
   persists `02-shell-result.txt` with the traversal rows.

5. Persists `01-program-features.txt`, `02-shell-return.txt`, and `03-pureform-principle.txt`
   so the runtime path is also readable as a doctrinal topic map.

This is the first shell exemplar in the sequence that should be read as a true backend mediation
surface rather than a purely descriptive shell envelope.

---

## The Arc

This exemplar sits at the transition where mediated semantic form becomes executable process.

It is downstream of DataFrame and Dataset preparation, but upstream of evaluation closure. Its
function is to show that the Shell is the lawful place where a built semantic form is handed into
real procedure execution without losing rational readability.

Read in the canonical movement, it is:

```text
DataFrame seed
-> Dataset / TaskFrame readiness
-> Shell plan authoring
-> Procedure-bound algorithm execution
-> Eval-readable return
-> PureForm principle closure
```

In Kantian terms, this is where the understanding of the built object approaches an end-directed
principle of execution. In the present doctrine, that means the Principle that emerges in the end
is made readable through the persisted fixture path.

---

## The Doctrinal Method Reading

This exemplar should now be read explicitly through the Doctrinal Method:

```text
Doctrine -> Reference -> Exemplar -> Example -> Fixture -> Workbench
```

The file you are reading is the exemplar. The Rust example is the execution surface. The fixture
root is the evidence surface. The workbench makes the sequence replayable for researchers and
developers.

That means this exemplar is not only "about Shell." It is also about how Doctrine itself becomes
an executable developer knowledge base.

---

## OG Script Direction

This example is also the clearest present ancestor of `OG Script`.

OG Script begins from the Shell script surface, but this exemplar already shows why it must become
more than a shell language. Its order of importance is:

1. Dataset Builder
2. Domain Model Design Surface
3. Shell Script Builder
4. ToolChain Coordinator

This exemplar primarily teaches items 3 and 4 today, but it only makes sense because items 1 and 2
already silently govern what the Shell is allowed to coordinate. The shell plan is not free text.
It is meaningful only because Dataset and domain commitments already make the call intelligible.

Everything here should be understood as grounded in Corpus, LM, and Logic artifacts, even where the
current runnable example only exposes the graph/procedure slice explicitly.

---

## Fixture Topic Map

Primary fixture root:

- `gds/fixtures/collections/shell/shell_compute_protocol`

The fixture set is not incidental logging. It is a doctrinal topic map:

- `00-frame.csv`: DataFrame seed of the relation surface
- `00-graph.txt`: explicit graph topology
- `01-shell-plan.txt`: shell component plan
- `01-program-features.txt`: shell/proc/eval mediation statement
- `02-shell-result.txt`: backend procedure result rows
- `02-shell-return.txt`: shell return register
- `03-pureform-principle.txt`: doctrinal closure
- `README.txt`: Kernel / Agent / Logic summary map

The fixture README explicitly names the layered reading:

```text
Kernel -> 00-frame.csv, 00-graph.txt, 02-shell-result.txt
Agent  -> 01-shell-plan.txt, 02-shell-return.txt
Logic  -> 01-program-features.txt, 03-pureform-principle.txt
```

This is why the exemplar now belongs naturally inside the broader Doctrine-as-Workbench direction.

---

## Namespace Discipline

Canonical code surfaces taught here:

- `gds::shell::GdsShell`
- `gds::procedures::shell::ShellProcedureControl`
- `gds::procedures::shell::ShellProcedureControlDependencies`
- `gds::procedures::shell::ShellProcedureResult`

Canonical doctrine surfaces reinforced here:

- `Doctrine/WORKBENCH.md`
- `Doctrine/METHOD-HIERARCHY.md`
- `Doctrine/REFERENCES/gds-kernel/end-state-and-closed-specification.md`

This exemplar should not be read through compatibility shims or old facade vocabulary first.
The doctrinal reading is now shell-to-procedure mediation under a Workbench and knowledge-base frame.

---

## Key Vocabulary

- Shell Compute Protocol — the shell-mediated handoff into real procedure execution
- `OG Script` — the future internal DSL surface that grows from Shell but exceeds it
- Dataset Builder — the highest-priority authoring role implied by OG Script
- Domain Model Design Surface — principled model-language surface before mere execution
- Doctrinal Method — Doctrine -> Reference -> Exemplar -> Example -> Fixture -> Workbench
- Closed Specification Mode — end-oriented discipline for mature kernel work

---

## Notes for Students

**Watch for**: the important change is not just that BFS runs. The important change is that the
algorithm is mediated through Shell and then made readable again through fixtures.

**Do not misread this** as a generic algorithm demo. It is a shell/procedure doctrine exemplar with
algorithmic evidence.

**Watch the inversion**: the shell plan is earlier in runtime order, but the principle of what the
system is doing becomes most explicit at the end, in the persisted return and pureform surfaces.
That is why this exemplar now fits the doctrine of end-oriented specification.

---

## Next Exemplar

**Next**: [030 — Ideal DataFrame DSL](../shell/030-ideal-dataframe-dsl.md)

This next step should be read not as a detached design fantasy, but as the negative space opened by
this exemplar. Once Shell is understood as a true mediation surface and as the ancestor of OG Script,
the question becomes: what should the ideal authoring surface look like when Dataset building and
domain model design are brought to the front?
