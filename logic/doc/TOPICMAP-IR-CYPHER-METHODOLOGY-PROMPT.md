# TopicMap → IR → Cypher Methodology Prompt

Use this prompt verbatim when running a section pass. The goal is deterministic regeneration of graph artifacts, with Cypher outputs treated as the primary research deliverable.

Artifact taxonomy and naming are defined in `logic/doc/BEING-ARTIFACT-CONTRACT.md`.
Use `DialecticIR` / “Dialectical States artifact” as the state-IR term (not “DialecticMap”).

## Fixed constraints

- Keep analysis language plain and source-faithful.
- Do not introduce Sanskrit terminology in analysis prose.
- Do not skip regeneration stages after TopicMap edits.
- Preserve stable chunk IDs and referential integrity.
- Prefer minimal, surgical edits.

## Execution order (must not drift)

1. Source analysis + chunk map updates
2. TopicMap updates
3. IR/codegen script run for the section
4. Generated artifact verification (`generated/`)
5. Package build validation

If any stage fails, fix the root cause and rerun from the failed stage onward.

## Canonical operator prompt

```text
You are updating a Logic section with strict methodology.

Mission:
- Produce source-faithful TopicMap updates and regenerate downstream artifacts.
- Treat Cypher artifacts as the prime output for KG construction.

Hard constraints:
- No Sanskrit words in analysis prose.
- Keep IDs stable unless explicitly instructed otherwise.
- Preserve source alignment and line-range traceability.
- No workflow drift: always run TopicMap -> IR -> Cypher generation.

Required workflow:
1) Update source analysis/chunk files for the target section.
2) Update TopicMap files for the same section.
3) Run the section IR generator script.
4) Confirm these outputs exist in the section generated folder:
   - integrated-topicmap-ir.cypher
   - integrated-topicmap-query-pack.cypher
   - integrated-topicmap-ir.debug.ts
5) Run package build and resolve relevant errors.
6) Report changed files, generated outputs, and validation status.

Acceptance criteria:
- TopicMap changes and generated Cypher artifacts are both present.
- Build succeeds.
- Output summary includes exact file paths and command results.
```

## Command template

Run from repo root. Replace `<section-script>` with the target script.

```bash
pnpm --filter @organon/logic <section-script>
pnpm --filter @organon/logic build
```

Example for being-for-self:

```bash
pnpm --filter @organon/logic codegen:being-for-self:ir
pnpm --filter @organon/logic build
```

## Generated artifact checklist

For section path:

`logic/src/relative/.../sources/generated/`

Required files:

- `integrated-topicmap-ir.cypher`
- `integrated-topicmap-query-pack.cypher`
- `integrated-topicmap-ir.debug.ts`

## Legacy artifact handling

If older integrated-cit artifacts exist in the same section:

- Keep them unless there is an explicit migration/removal request.
- Do not silently delete legacy files.
- Prefer generating the current canonical `integrated-topicmap-*` set in parallel.

## Completion report format

- Scope: section path and files edited
- Generation: commands run + output file paths
- Validation: build status
- Notes: blockers, assumptions, follow-up actions
