# Exemplar 011 — DataFrame Macros as Reflection Engine

**Source file**: `gds/examples/collections_dataframe_macros_expository.rs`
**Arc position**: Reflection (Moments 1–6)
**Prior exemplar**: [010 — XML/HTML Semantic Form](010-xml-html-semantic-form.md)
**Next exemplar**: [012 — Select/Filter as Principle Evaluation](012-select-filter-principle.md)

---

## Principle

The DataFrame macro system is the Reflection engine.
Reflection is how essence articulates itself through the appearance data.
Each macro corresponds to a moment in that articulation.

---

## What This Example Does

It builds a student table (`tbl_def!`), then works through it systematically:

1. **`mutate!`** — derives new marks from existing fields (`score_x2`, `passed`, `honors`)
2. **`where_!`** — retains only entities meeting a compound condition
3. **`arrange!`** — orders by a derived mark
4. **`summarize!`** — consolidates the surviving columns into a working surface
5. **`join!`** — extends the surface with a label table (middle term introduction)
6. **`pipe!`** — chains further filtering and ordering as a single passage
7. **`q!`** — applies lazy expr logic to derive a final grade classification

Each step does not replace the prior one. Each step works the data further into determinacy.

---

## The Arc Reading

```
Appearance (students table)
  → mutate!     [Moment 1: Being → enriched marks]
  → where_!     [Moment 2: Identity → which entities hold together]
  → arrange!    [Moment 3: Opposition → ordering by contrast]
  → summarize!  [Moment 4: Contradiction resolved → consolidated surface]
  → join!       [Moment 5: Ground → external label joined as middle term]
  → pipe!       [Moment 6: Ground-as-condition → passage through further gates]
  → q!          [Absolute Reflection threshold → lazy grade classification]
```

After `q!`, the entity is at the threshold. It has been worked through six moments.
It now awaits Principle evaluation to determine whether it may become a Concept.

---

## The Macro System as Doctrinal Surface

Each macro in this example is a stage-level operation:

| Macro | Reflection Moment | What It Does |
|---|---|---|
| `tbl_def!` | (pre-Reflection, Observation) | Fixes the appearance surface |
| `mutate!` | Moment 1–2 | Derives marks, determines identity |
| `where_!` | Moment 2–3 | Filters to those that hold together |
| `arrange!` | Moment 3 | Orders by opposition/contrast |
| `summarize!` | Moment 4 | Resolves contradiction into a working summary |
| `join!` | Moment 5 | Introduces the ground via external label |
| `pipe!` | Moment 6 | Passes through ground-as-condition |
| `q!` (lazy) | Moment 7 | Absolute Reflection threshold |

This mapping is not forced. The DataFrame macro system was designed as an expressive pipeline;
its stages happen to reproduce the Reflection moments because Reflection is the general logic
of how any analytic surface works through its material.

---

## Key Vocabulary

**Reflection** — The movement of essence through its own determinations (Being → Identity → Opposition → Contradiction → Ground → Ground-as-condition → Absolute Reflection).
*See*: [Seven Moments of Reflection](../REFERENCES/form-processor/seven-moments.md)

**Mark** — A derived field fixed by a `mutate!` expression. Marks are the outputs of Reflection moments 1–2.
*See*: [Core Concepts — Mark](../REFERENCES/collections-dataset/core-concepts.md)

**Absolute Reflection threshold** — The point where the entity has been fully worked through and now awaits Principle evaluation.
*See*: [Absolute Reflection](../REFERENCES/form-processor/absolute-reflection.md)

**`q!` (lazy query)** — The lazy expression pipeline that executes at the Absolute Reflection boundary. It does not materialize until Principle has been checked.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

---

## Student Notes

- The example opens: "This walkthrough treats the DataFrame as the analytic body of a richer semantic object." That sentence is the doctrinal statement. Hold it.
- The `q!` macro is the pivot. Before `q!`, all operations are eager. After `q!`, we are in lazy evaluation — the kernel defers materialization. This deferral is not a performance trick. It is the architectural expression of Absolute Reflection: the shape is inspectable but not yet committed.
- The student who wants to understand Reflection should write a `mutate!` → `where_!` → `arrange!` sequence manually and trace which properties of the original data survive at each step. What disappears is as important as what remains.

---

## What This Example Does Not Show

- Principle evaluation (that is Exemplar 012)
- Concept emergence (Exemplar 013)
- The ZeroCopy boundary at Moment 7 (see [ZeroCopy Boundary](../REFERENCES/gds-kernel/zeroCopy-boundary.md))
