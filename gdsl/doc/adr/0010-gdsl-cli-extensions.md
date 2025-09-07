# ADR 0010: GDSL CLI — noun-verb with per-package extensions

- Status: Accepted
- Date: 2025-09-06
- Owners: @pat

## Context
We want a single CLI: gdsl <noun> <verb>. Each package (@reality, @logic, @model, @task, @core) contributes its own commands without coupling the core.

## Decision
- Core nouns: dataset (alias ds). Verbs: pack, validate, info, list.
- Extensions: packages register new nouns/verbs via small modules loaded by the GDSL CLI at startup.
- Conventions: nouns are singular; verbs imperative; support --json; nonzero exit on error.
- Reserved nouns (initial): dataset, pipeline, agent, workflow, app, model.

## Consequences
- + One CLI surface; packages add capabilities safely.
- + Script-friendly, composable.
- - Small plugin loader in CLI.

## Examples
- gdsl dataset pack reality/src/packages/vitarka.pkg.ts
- gdsl workflow run task/src/pipelines/some.flow.ts
- gdsl logic morph @organon/logic/relative/...
