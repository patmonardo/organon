# Welcome to GDS Doctrine

**The exemplars are the canonical exposition. Runnable examples and fixtures are their executable evidence.**

This is the constitution of the GDS Knowledge Agent system. It is not scattered working notes.
It is Doctrine.

---

## The Structure (What You See Today)

```
gds/Doctrine/
  ├── INDEX.md                    ← You are here. Navigation hub.
  ├── FOUNDING-CHARTER.md         ← Why this exists and how it replaces old /gds/doc
  ├── PRINCIPLE-FOUNDATION.md     ← The high principle: Source → Observation → Reflection → Principle → Concept → Judgment → Syllogism → Procedure
  ├── PHASE-1-STATUS.md           ← What's done, what remains
  │
    ├── EXEMPLARS/                  ← Canonical texts grouped by kernel fold
  │   ├── TEMPLATE.md             ← How to write an exemplar
    │   ├── form/                   ← PureForm and principle-gated return
    │   ├── shell/                  ← Runtime connector and DataPipeline protocol
    │   ├── dataframe/              ← Immediate Frame:Series::Expr body
    │   └── dataset/                ← Mediated Model:Feature::Plan and SemDataset
  │
    └── REFERENCES/                 ← Consulted definitions by the same roots
      ├── form/                   ← PureForm and reflection/principle concepts
      ├── shell/                  ← Runtime program grammar and feature envelope
      ├── dataframe/              ← Immediate tabular body and expression surface
      ├── dataset/                ← Mediated semantic controller and artifacts
      ├── gds-kernel/
      │   └── zeroCopy-boundary.md← Kernel-agent contract at Principle
      ├── philosophy/
      └── (more to come)
```

---

## Read This First

1. **[INDEX.md](INDEX.md)** (5 min) — Navigation and quick reference
2. **[PRINCIPLE-FOUNDATION.md](PRINCIPLE-FOUNDATION.md)** (10 min) — The one-page high principle
3. **[EXEMPLARS/001-frame-dsl.md](EXEMPLARS/dataset/001-frame-dsl.md)** (15 min) — Start the exemplar sequence

Then continue through the current exemplar sequence in order.

---

## The Arc (The Only Arc)

```
Source → Observation → Reflection (7 moments) → Principle → Concept → Judgment → Syllogism → Procedure
```

Every Shell program artifact is read through this arc.
Every Dataset artifact embodies this arc.
Every exemplar teaches one stage of this arc.

The top-level fold is fixed everywhere:

```text
form -> shell -> dataframe -> dataset
```

Form names the PureForm return. Shell connects the runtime. DataFrame supplies
the immediate body. Dataset supplies the mediated semantic controller.

---

## What Makes This Different from Old /gds/doc

**Old structure** (100+ scattered docs):
- Working notes, proposals, project reviews scattered by task
- No canonical sequence
- Easy to contradict yourself
- No single voice
- No clear entry point

**New Doctrine** (Index -> Exemplars -> References -> executable fixtures):
- Exemplars are the texts; runnable examples and fixtures prove them
- One canonical sequence (the arc)
- Consistent voice
- Clear entry points
- All references tied to principles

This is higher principle at play. The docs are no longer scattered notes.

---

## Doctrine Hygiene

The Doctrine must keep three layers distinct:

1. **Exemplars** explain a settled doctrinal moment.
2. **Runnable examples** execute that moment through the internal Rust DSL unless explicitly marked external.
3. **Fixtures** persist the generated artifact evidence.

Do not let an exemplar become a scratch note, a fixture manifest become the doctrine, or an external program artifact masquerade as the internal RustScript surface. When a runnable example is renamed, update the exemplar source line, [INDEX.md](INDEX.md), and any namespace registry in the same change.

---

## The High Covenant

> The Doctrine is written in a single voice: the voice of the system explaining itself through
> canonical examples and settled reference material.
>
> This is not scattered notes. This is not unfinished proposals. This is the constitution.

---

## Quick Links

- **Navigation**: [INDEX.md](INDEX.md)
- **High Principle**: [PRINCIPLE-FOUNDATION.md](PRINCIPLE-FOUNDATION.md)
- **Exemplar Template**: [EXEMPLARS/TEMPLATE.md](EXEMPLARS/TEMPLATE.md)
- **First Exemplar**: [EXEMPLARS/001-frame-dsl.md](EXEMPLARS/dataset/001-frame-dsl.md)
- **External Shell Program Artifact**: [EXEMPLARS/006-external-shell-program-artifact.md](EXEMPLARS/shell/006-external-shell-program-artifact.md)
- **Applications Surface**: [EXEMPLARS/007-applications-expository.md](EXEMPLARS/form/007-applications-expository.md)
- **Status**: [PHASE-1-STATUS.md](PHASE-1-STATUS.md)

---

## Start Learning

If you are new to GDS:

1. Read PRINCIPLE-FOUNDATION.md
2. Study EXEMPLARS in order
3. When an exemplar links to a REFERENCE, consult it
4. Build something using the same pattern
5. You now own the system

---

Welcome to Doctrine.
