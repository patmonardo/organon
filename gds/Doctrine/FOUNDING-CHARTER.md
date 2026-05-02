# GDS Doctrine — Founding Charter

Date: 2026-05-01

## The Move

The `/gds/doc` directory has become a scattered archive of working notes, proposals, and project-specific reviews. It is no longer functional as a system for learning or reference.

This Doctrine reconstruction rests on a single high principle:

**The examples are the canonical exposition of the system. All other documentation serves the examples.**

This is not inverted. The examples are not decorative. They are constitutional. Every reader should be able to learn the entire system by studying the examples in sequence and consulting the reference docs only when needed.

---

## Structure

The new Doctrine lives under `Doctrine/` with this shape:

```
gds/
  Doctrine/
    FOUNDING-CHARTER.md          ← This file
    PRINCIPLE-FOUNDATION.md      ← The high principle (Reflection→Principle→Concept→Procedure)

    EXEMPLARS/
      001-frame-dsl.md           ← What this example teaches
      002-corpus-readers.md      ← What this example teaches
      003-tree-structures.md
      ...
      (41 exemplars, each with a .md companion)

    REFERENCES/
      collections-dataset/
        core-concepts.md         ← Reference: what is a Dataset?
        artifact-kinds.md        ← Reference: what are artifact kinds?
        marks-and-features.md    ← Reference: mark semantics
        feature-structures.md    ← Reference: FeatStruct algebra

      form-processor/
        relative-reflection.md   ← Reference: 6 RelativeReflection moments
        absolute-reflection.md   ← Reference: 7th moment, the threshold
        principle-evaluation.md  ← Reference: nomological gate
        concept-emergence.md     ← Reference: post-Principle objects

      gdsl/
        program-features.md      ← Reference: feature kind taxonomy
        grammar.md               ← Reference: GDSL syntax

      gds-kernel/
        pureform.md              ← Reference: Form/Context/Morph
        zeroCopy-boundary.md     ← Reference: kernel-agent contract

      philosophy/
        hegel-objectivity.md     ← Reference: Procedure as Process
        kant-synthesis.md        ← Reference: Observation stage

    doc_archive/
      (old scattered docs moved here with index)
```

---

## What an Exemplar Document Is

Each exemplar `.md` companion file answers four questions:

1. **Principle**: What high principle does this example teach?
2. **Flow**: What does the example do, stage by stage?
3. **Doctrine**: How does this example embody the Reflection→Principle→Concept→Procedure arc?
4. **Next**: What exemplar should the reader study next?

Example template:

```markdown
# Collections Dataset: Frame DSL (`collections_dataset_frame_dsl.rs`)

## Principle
This example teaches the modern DataFrame DSL surface: how to name a Dataset,
author pipeline stages, and compile them into executable plans.

## What It Does
- Creates a named Dataset
- Uses the `.ds()` namespace to access frame operations
- Demonstrates the lightweight Dataset wrapper
- Compiles the result into a dataset image

## The Arc
This is **Stage 1 in the Knowledge Agent pipeline**: Source → Observation.
The example shows the ergonomic surface where users begin authoring semantic programs.

## Next
Study `collections_dataset_corpus_readers.rs` to see how Observation moves from
immediate data loading into semantic corpus marking.
```

---

## What a Reference Document Is

Reference docs are consulted, not read. They are organized by topic:

- **`core-concepts.md`**: What is Dataset? What is DataFrame? What is a mark?
- **`relative-reflection.md`**: What are the 6 moments? How do they sequence?
- **`program-features.md`**: All 16 feature kinds; when to use each.
- **`zeroCopy-boundary.md`**: The kernel-agent contract at Absolute Reflection.

Reference docs are:
- Topic-focused, not project-focused
- Stable and definition-centric
- Linked from exemplars when deeper context is needed
- Never narrative or scattered across multiple files

---

## Migration Plan

### Phase 1: Create Doctrine Structure (This Week)
- Create `Doctrine/` directory with the above shape.
- Write `PRINCIPLE-FOUNDATION.md` as the executive summary.
- Write 5-10 exemplar companions for the most important examples.
- Write 8-10 foundational reference docs.

### Phase 2: Complete Exemplar Coverage (Next Two Weeks)
- Add exemplar companions for all 41 examples.
- Each companion links to relevant reference docs.
- Each companion states "next exemplar" clearly.

### Phase 3: Archive Old Docs (End of May)
- Create `doc_archive/` with an index.
- Move all 100+ existing docs to `doc_archive/`.
- Keep a `doc_archive/INDEX.md` mapping old doc names to new Doctrine locations.
- Do NOT delete. Archive preserves history.

### Phase 4: Quality Pass (Early June)
- Read through all exemplar sequences as if learning for the first time.
- Fix doctrine clarity issues.
- Add missing reference docs based on reader questions.
- Ensure links are correct.

---

## High Principle

The Doctrine should read as if written in a single voice. Not collage. Not scattered notes.
One voice explaining the system through canonical examples, with reference material in the wings.

That voice is: "Here is how you know. Here is how the Knowledge Agent reasons. Here is what
Principle means. Here is where Concept emerges. Here is what Procedure emits."

Not: "We considered doing X, but Y might work better, and here's an unfinished proposal."

The old docs are not wrong. They are scaffolding. They belonged during construction. Now
the building stands. The Doctrine is what stands.

---

## Near-Term Targets

1. Write `PRINCIPLE-FOUNDATION.md` (1 hour)
2. Write exemplar companions for the 5 exemplars recommended in DATASET-SEMANTIC-MODEL-BUILDER-DOCTRINE.md
3. Write 8 foundational reference docs
4. Test the exemplar sequence: can a new reader understand the system by reading them in order?

The work begins now.
