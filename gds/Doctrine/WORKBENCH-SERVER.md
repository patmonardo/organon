# Doctrine Workbench Server

Purpose: define how the Doctrine Workbench evolves from CLI runner to server-side learning application.

## Intent

The current CLI workbench is valid for immediate researcher onboarding.
The target form is a server-side app that renders Doctrine tracks, runs examples,
captures fixture outputs, and exposes artifact lineage as a topic map.

In other words: the Workbench server is the first operational form of **Doctrine**, the Rational Agent IDE app for users building rational agents.

Workbench should therefore be read not just as a runner, but as the **OG Script Extension Framework** for IDE mode: module developers define workbench modules, and the IDE/server renders, executes, and studies them through one entry surface.

Naming rule:

- **Doctrine**: the product-level name for the future Rational Agent IDE app.
- **OG Script**: shorthand for the internal DSL and script surface used inside that environment.

OG Script should initially grow from the Shell script surface, but it should mature into a broader authoring language with four coordinated functions in this priority order:

- Dataset Builder
- Domain Model Design Surface
- Shell Script builder
- Toolchain coordinator

These functions should be grounded in Corpus, LM, and Logic artifacts, so that the script surface remains a rational modeling surface rather than a free-floating command veneer.

Method rule:

- **Doctrine First Methodology**: doctrine provides the rationale and ordering discipline for the system.
- **Doctrinal Method**: the concrete practice of moving through doctrine, reference, exemplar, example, fixture, and workbench as one executable study path.

## Functional Contract

1. Curriculum API

- List canonical tracks (`quick`, `mediation`, `shell`, `full`).
- Return ordered steps for each track.

2. Run API

- Trigger execution of a track.
- Stream run state (start/progress/complete/failure).

3. Artifact API

- Return generated fixture files for a run.
- Return a canonical session map artifact.

4. Topic Map API

- Return hierarchy view: Doctrine -> References -> Exemplars -> Examples -> Fixtures.
- Return mediation view: DataFrame -> TaskFrame via shell/proc/eval.

5. Knowledge Base API

- Return doctrinal definitions, exemplar links, runnable example links, and fixture roots for one topic.
- Return the smallest study surface needed for a user to continue learning without exploratory search.

6. Script Surface API

- Return OG Script snippets, generated script views, or future editable script forms for a topic or track.
- Preserve traceability from OG Script surface to examples and fixture evidence.
- Preserve enough semantic structure that OG Script can coordinate toolchain steps, dataset-building stages, domain-model design commitments, and Corpus/LM/Logic artifact flow.

7. Method API

- Return the doctrinal rationale for a topic, track, or script surface.
- Return the minimal Doctrinal Method path for reaching executable evidence from that rationale.

## Suggested Server Shape

Recommended initial implementation in TypeScript (Agent world) with Rust process execution:

- `GET /api/workbench/tracks`
- `POST /api/workbench/run/{trackId}`
- `GET /api/workbench/runs/{runId}`
- `GET /api/workbench/runs/{runId}/artifacts`
- `GET /api/workbench/topic-map`
- `GET /api/workbench/knowledge-base/{topicId}`
- `GET /api/workbench/og-script/{topicId}`
- `GET /api/workbench/method/{topicId}`

Server should shell out to:

- `bash gds/Doctrine/workbench.sh <mode>`

and then read fixture outputs under:

- `gds/fixtures/collections/**`
- `gds/fixtures/procedures/**`
- `gds/fixtures/task/workbench/**`

## Data Model (Minimal)

- `WorkbenchTrack { id, title, description, steps[] }`
- `WorkbenchRun { id, trackId, startedAt, finishedAt, status }`
- `WorkbenchArtifact { runId, path, kind, summary }`
- `KnowledgeBaseTopic { id, doctrine, references[], exemplars[], examples[], fixtures[] }`
- `OgScriptSurface { topicId, script, provenance[], fixtureRoots[] }`
- `DoctrinalMethodPath { topicId, doctrine, references[], exemplars[], examples[], fixtures[], workbenchModes[] }`
- `OgScriptSurfaceRole { topicId, datasetBuilder, domainModelDesign, shellBuilder, toolchain, corpusArtifacts, lmArtifacts, logicArtifacts }`

## Layer Alignment

- Kernel target: empirical adequacy (runtime execution and fixture evidence)
- Agent target: conceptual validity (track assembly, orchestration, replay)
- Logic target: rationale coherence (topic-map rendering and doctrinal closure)

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

## Incremental Build Plan

1. Keep CLI as source of truth for track execution.
2. Add session-map output per run (already in CLI path).
3. Build thin server wrapper over CLI execution.
4. Add browser UI for track and artifact inspection.
5. Add topic-oriented knowledge-base views for developers and researchers.
6. Promote server as default researcher entrypoint.
7. Grow the workbench into Doctrine, the Rational Agent IDE app, without abandoning Doctrine-as-source-of-truth at the methodological level.
