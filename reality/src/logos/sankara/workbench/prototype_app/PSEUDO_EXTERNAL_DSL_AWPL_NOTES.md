# Sankara Pseudo External DSL Notes

Purpose: keep a research-safe planning layer for the Sankara workbench while the corpus artifacts remain research-grade.

Classification: TS agential programmer workbench, not a generic ML workbench.

## Scope Boundary

- Corpus truth: passage records, translation JSONL, record manifest, generated markdown records.
- Management meta: viewer steps, macro targets, function targets, and planning traces.
- Rule: management meta may describe corpus workflow but must not redefine corpus content.

## Analysis Policy

- Analysis mode: technical vocabulary only.
- Vocabulary sources: record manifest, translation notes, domain glossary.
- Unknown term handling: flag for curation.
- Minimum translation note level: lexical notes.

## Touch Boundary Policy

- Do not touch: translation literal/technical/interpretive text, provenance objects, source track.
- Keep source track explicit; preferred source track remains sringeri.net.
- Review exclusions for routine passes: QA uncertainty notes and provenance review.
- Safe structuring lane: record_kind, analysis.tags, analysis.lexical_notes, analysis.transition_notes, analysis.logical_notes.
- Curated knowledge files are treated as high-value prompt material for agents.

## Logical Notes Lane

- Enabled, but semantically open/undefined for now.
- Accepted note forms: gloss, axiom, bridge-note, question.
- Interpretation rule: human-curated disambiguation.
- Meaning is intentionally provisional until a logical-notes taxonomy stabilizes.

## Current Contract Surfaces

- Component model root: frame + model + view + controller.
- Workbench recording: viewer step stream and recoding targets.
- Schema extension: manifest-backed proto language model.
- Shell instance: namespaces, macros, functions, expr forms.
- Curation profile: domain curation lanes and review queues.
- Agent knowledge profile: Sankara corpus + Sanskrit transition signals + Kant-Hegel dialectics + editorial policy.

## Proto Language Model

- Name: proto language model.
- Source: record manifest.
- Role: pseudo external DSL.
- Planning language: AWPL prototype.

## Corpus Document and Annotation DSL

The first prototype app now carries a minimal Input and Output DSL shape:

- Input kind: corpus-document-input
- Output kind: analytical-output
- Analysis mode: technical-vocabulary-only
- Annotation labels: mantra, bhasya, preamble, transition, technical-term, logical-note

Reference examples live in:

- corpus_document_annotations.examples.json

These examples are intentionally simple and focused on corpus-document plus annotation flow, not full translation regeneration.

The shell instance is currently defined as:

- Base: Rust internal DSL.
- Namespaces:
  - sankara.<work>.corpus
  - sankara.<work>.workbench
  - sankara.<work>.planning
- Macro seeds:
  - open_sutra_context
  - trace_transition_window
- Function seeds:
  - select_workbench_records
  - filter_workbench_semantics
  - emit_awpl_plan
- Expr forms:
  - select
  - filter
  - anchor
  - compose
  - plan

## Viewer Step Recording -> Recoding Path

Viewer steps are recorded first, then recoded in two directions:

1. Macros (higher-level shell actions)
2. Functions (dataset selection/filter kernels)

Planned recoding chain:

1. viewer steps
2. normalized step graph
3. macro/function candidate extraction
4. AWPL plan emission
5. optional shell materialization in Rust modules

## AWPL Sketch (Prototype)

This is a descriptive sketch, not a fixed grammar.

```txt
PLAN SankaraStudy
  SELECT work=BS
  ANCHOR chapter=1 section=1 sutra=1
  FILTER kind IN [sutra, bhasya]
  FILTER transition CONTAINS [therefore, thus]
  COMPOSE co_located=true
  EMIT target=awpl-plan
```

## Upgrade Track

- manifest-language-model
- shell-language-instance
- awpl-notes
- corpus-annotation-examples
- curated-agential-profile
- logical-notes-taxonomy
- intent-dsl
- anchor-schema
- topic-schema
- transition-schema
- agent-projection

## Research Grade Reminder

All planning and management artifacts are derivative. Corpus artifacts remain authoritative.
