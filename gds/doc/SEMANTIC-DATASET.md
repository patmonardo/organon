# Semantic Dataset

What we are building: an **NLP extension of Polars DataFrame**. Same
hacking we have been doing, simpler framing, fewer new ideas.

Three layers, named honestly:

- **Semantic DataFrame** (proxy) — access objects that mirror Polars'
  top-level objects: our frames + `Plan`. No new concepts; pure
  NLP-shaped hosting of what Polars already has.
- **Semantic machinery** (NLP, *not* Polars) — `model*` and `feature*`
  modules. `Model` and `Feature` are our own additions; they are not
  Polars notions. This is where the NLP extension actually lives.
- **Semantic Dataset** (synthesis) — `EvidenceCorpus` and
  `LanguageModel`. A Semantic Dataset *is* the Corpus + LanguageModel
  pairing over the machinery.

The directory is named `dataset/` for the synthesis layer, but it
houses all three. Keep the distinction in mind when reviewing: the
proxy layer should stay boring, the machinery is where NLP design
lives, and the synthesis is the user-facing artifact.

## The whole pitch

Polars already gives us everything we need: `LazyFrame`, `DataFrame`,
`Series`, `Expr`, and — via `LazyFrame` — its own query **Plan**. A
typical Polars client just reaches for the macros, expressions, and
functions directly. **We do not.** We host the Polars top-level
objects inside proxy access objects and expose them as an NLP-shaped
DSL. Nothing we add is new Polars; it is NLP framing over the real
Polars objects.

`Plan` in our code is not a new concept either — it is our handle on
Polars' own `LazyFrame` plan, plus the NLP-shaped telemetry and
eval-mode scaffolding around it.

It is simultaneously:

- an **SDK** — typed Rust handles, construction-time validation
- a **DSL** — a small language of nouns that resolves into Polars
  plans

Same modules, two readings.

## The pieces

### Semantic DataFrame (proxy over Polars)

**Frames** (thin typed wrappers over `GDSDataFrame`):

- `SourceFrame` — byte-stream identity rows
- `DocumentFrame` — source + span rows
- `AnnotationFrame` — typed observations with provenance
- `ValuationFrame` — schema-aware value rows

**Polars-plan proxy:**

- `Plan` (`plan.rs`) — our access object over Polars' `LazyFrame` plan.
  Adds eager/lazy execution, NLP-shaped step telemetry, and a named
  dataset env on top of what Polars already does. Already ~560 lines
  and mostly right. The one clear upgrade: make Eager/Lazy strategy
  explicit alongside `EvalMode { Preview, Fit }` so the proxy
  faithfully exposes what Polars already distinguishes.

### Semantic machinery (NLP extension, not Polars)

`Model` and `Feature` are **our** concepts; Polars has no equivalent.
This is where the NLP extension actually lives.

- `FeatureDescriptor` + `FeatureRole` (Projection / Binder / Reentrancy /
  Annotation). A feature with Binder or Reentrancy roles points at
  other features — an addressable-node graph shape over named features.
- `Model` trait (Tagger, Parser, …) — compute-side "model."
- `Valuation` — data-side "model" (partial map `FeatureName → cell`).

### Semantic Dataset (the syntheses)

- `EvidenceCorpus` — SourceFrame + DocumentFrame + AnnotationFrame
- `LanguageModel` — schema + ValuationFrame + binding contexts, with
  stubbed admissibility predicates returning `Verdict::Unknown`

A Semantic Dataset in use is an `EvidenceCorpus` and a `LanguageModel`
together — the evidence and the meaning theory over it.

## What changes when we upgrade

Almost nothing structural. The phases already landed the frames,
schema vocabulary, and composites. Remaining work is detail-level:

- Eager/Lazy as a first-class `Plan` concern
- Fill in the admissibility predicate bodies (or decide they stay stubs)
- Optional: end-to-end smoke test wiring a corpus through Plan into a
  language model

If it starts expanding beyond that, we have drifted off the pitch.

## Naming survivals (two of each, no renames)

- `model.rs` (compute trait) and `valuation.rs` (data side)
- `corpus.rs` (text wrapper) and `evidence_corpus.rs` (composition)
- `language_model.rs` (old stub) and `language_model_product.rs` (new)

Kept both because nothing broke; not worth churning.

## Rule of thumb

If a change makes `plan.rs` or the frames *simpler*, it is probably
right. If it adds a new concept that is not already a Polars concept,
it probably is not. This is hosting Polars through a proxy, not
building a second Polars.
