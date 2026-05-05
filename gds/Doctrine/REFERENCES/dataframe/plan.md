# Doctrine of the Plan

`Plan` is the deferred DataFrame recipe inside the Dataset Meta Pipeline. It is
the ordinary Polars-facing form of reflection: a sequence of lazy tabular steps
held up before execution.

This reference begins with the ordinary sense of Plan. The larger ontological,
epistemological, and mathematical readings must remain grounded in this simple
fact: a Plan is a deferred recipe over a DataFrame body.

---

## Ordinary Definition

In current code, `Plan` is a Dataset-side wrapper over Polars-shaped lazy work:

```rust
pub struct Plan {
    name: Option<String>,
    source: Source,
    steps: Vec<Step>,
}
```

Its source is either:

- a bound Dataset variable (`Source::Var`)
- an inline Dataset value (`Source::Value`)

Its steps are ordinary tabular directions:

- `Filter`
- `Select`
- `WithColumns`
- `Item`
- `Split`
- `Batch`
- `DataOp`

The core ordinary behavior is this:

`Plan` receives a DataFrame body through Dataset and applies lazy tabular steps
without becoming a new runtime beside Polars.

---

## Plan Is Deferred Reflection

Rows and columns in the sphere of Being are live, extensional, and variable.
They behave as immediate data.

Plan changes the sphere. It does not capture every aspect of that immediacy. It
selects, orders, projects, and suspends part of the body for reflective use.

Compactly:

`Plan = deferred reflection over DataFrame immediacy`

This means Plan is powerful, but bounded:

- It retains what mediation needs.
- It excludes or defers what mediation does not yet know how to use.
- It must remain traceable to the DataFrame body it reads.

---

## Model Feature Plan As Mutual Ground

`Model:Feature::Plan` is not three independent modules that happen to appear
together. They derive from one another, the way Subject and Predicate in the
determinate Judgment derive from the Judgment itself as their matter — not as
two ready-made parts joined by a copula, but as moments that the Judgment
itself produces as its own articulation.

In the same sense:

- `Model` is not a naming convention applied to any DataFrame. It is a moment
  that has meaning only because Feature can bind a role to it and Plan can
  govern what survives into mediation.
- `Feature` is not a standalone descriptor. It is what Model body becomes when
  Plan reflects it under a specific retention commitment.
- `Plan` is not just a Polars recipe floating free. It is what completes the
  Essence middle by saying: this, and not that, enters mediation.

Remove any one of the three and the Essence middle collapses into either raw
DataFrame immediacy or premature Semantic result. They hold the middle by
holding one another.

This is why passing a Plan to Dataset is not merely a convenience. It is the
act that constitutes the Essence triad.

---

## Plan's Triple Duty

Plan does triple duty. Acknowledging all three is what keeps the doctrine honest.

### Duty One: Polars Plan At Face Value

Plan is a Polars deferred recipe. This is accepted entirely and without
qualification. Steps become `LazyFrame` operations; `eval_dataset` and
`apply_to_lazyframe` are real Polars calls. Plan does not add a runtime beside
Polars — it uses Polars.

No doctrinal reading of Plan may contradict this first duty.

### Duty Two: Middle Moment Of Essence

Inside Dataset mediation, Plan is the third moment of `Model:Feature::Plan`. It
is the specific form of deferred reflection that closes the Essence triad. It
retains the body that Feature has bound and Model has named. It excludes or
defers the rest.

Plan in this duty is not a passive recipe. It is an active commitment: this
body, in this form, under this attention, is what Essence retains.

### Duty Three: The Telic Step Toward Semantics

Plan is placed on the path toward Semantics — toward `Corpus:LM::SemDataset` —
by design. This placement is not forced on Plan by technical necessity. It is
an act of free will grounded in purpose: our purpose is Semantic Datasets.

This means Plan is not neutral as to where it leads. A Plan that leads nowhere
beyond its own tabular steps is technically correct but doctrinally incomplete.
A doctrinally complete Plan either reaches Semantics directly (through
`SemDataset` construction downstream) or names the stage at which it hands off
to the body that does.

Compactly:

`Plan carries the telos of Semantics as the reason it exists in Dataset at all.`

### The Triple Duty In One Statement

```text
Plan = Polars deferred recipe
     + Essence-middle commitment (Model:Feature::[Plan])
     + telic step toward Corpus:LM::SemDataset
```

All three duties are simultaneously active. Optimizing for one while ignoring
the other two produces a distorted Plan.

---

## Ordinary Polars Relation

Plan belongs near Polars because its first meaning is lazy tabular computation.
It can evaluate into a Dataset, apply itself to a `LazyFrame`, or become a
streaming transform.

Current surfaces:

| Surface | Meaning |
|---|---|
| `eval_dataset(env, mode)` | run the Plan against a Dataset environment |
| `apply_to_lazyframe(lf)` | apply tabular steps directly to a Polars `LazyFrame` |
| `to_streaming_transform()` | reuse the same tabular steps over streaming batches |
| `attention_report(env, mode)` | inspect intended and observed tabular behavior |

This is the ordinary ground. Any higher reading of Plan must preserve it.

---

## The Item Projection

`Step::Item` is the canonical point where Plan idealizes the DataFrame body into
a record-shaped artifact.

Convention:

`item` is a Polars `Struct` column that gathers selected columns or expressions
into one feature-map body.

The `item` column is not the whole DataFrame. It is what the Plan chooses to
hold up for mediation.

---

## Three Demands On Plan

Plan sits under three demands. The ordinary Polars demand comes first.

| Demand | Question | Current Anchor |
|---|---|---|
| Ontological | What body does the Plan retain from Being? | `Source`, `Step`, `item` projection |
| Epistemological | What can the user inspect about the Plan's attention? | `PlanAttentionReport` |
| Mathematical | Which transformations can be composed, evaluated, or later compiled? | `chain`, `eval_dataset`, `apply_to_lazyframe` |

The desired Singleton is not a mystical extra object. It is the disciplined
unity of these demands in one Plan surface:

`retained body + inspectable attention + composable transformation`

---

## Evaluation Modes

Plan currently distinguishes two modes:

| Mode | Meaning |
|---|---|
| `Preview` | inspect a bounded sample before full commitment |
| `Fit` | evaluate as the committed training/processing pass |

This matters doctrinally because Plan is not only a recipe. It is a recipe with
degrees of commitment.

Preview is reflection before full mediation. Fit is reflection accepted into the
mediating path.

---

## Attention Report

`PlanAttentionReport` is the ordinary visibility surface of Plan.

It can report:

- plan name
- evaluation mode
- source summary
- ordered steps
- planned columns
- observed columns
- row and column counts when evaluated
- batch and split hints

This is why Plan is not a blind lazy computation. It can explain what it is
holding up for reflection.

---

## Boundary Rule

Do not make Plan responsible for everything in `Model:Feature::Plan`.

Plan does not by itself:

- create Model essence
- bind all Feature roles
- evaluate admissibility predicates
- exhaust all DataFrame variation
- produce SemDataset meaning

Plan gives Feature a runtime body and gives Dataset mediation a deferred
tabular recipe. Model, Feature, FeatStruct, Modality, Corpus, LanguageModel,
and SemDataset carry the other moments.

---

## Review Rule

When reviewing a Plan, ask:

1. Which DataFrame/Dataset body does it read?
2. Which steps does it defer?
3. Which `item` or output body does it intend?
4. What does its attention report make visible?
5. Which variation is excluded, deferred, or left in the immediate body?
6. Can the same transformation be evaluated or applied to a `LazyFrame`?

If these answers are unclear, the Plan is not yet doctrinally ready.
