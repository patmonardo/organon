# Doctrine of the DataFrame

`DataFrame` is the immediate analytic body of the Collections system. It is the
first formed place where contents appear as rows, columns, dtypes, and
expressions before Dataset mediation begins.

This reference defines DataFrame in the new doctrinal method. It starts from the
ordinary Polars-facing body and then names why that body becomes EssentialBeing
for Dataset.

---

## Ordinary Definition

In current code, the primary body is `GDSDataFrame`: a Rust wrapper around a
Polars `DataFrame` with lazy access, expression helpers, construction helpers,
serialization, grouping, sorting, joins, IO, and table-formatting surfaces.

Its ordinary responsibilities are:

- construct tabular bodies from rows, columns, records, and series
- preserve column names and dtypes
- expose row and column shape
- provide eager operations where appropriate
- produce lazy Polars plans through `LazyFrame`
- serialize, deserialize, and write table artifacts

This is the first truth of DataFrame: it is a practical analytic table body.

---

## The Immediate Triad

The DataFrame beginning appears as three immediate terms:

| Term | Ordinary Meaning | Doctrinal Meaning |
|---|---|---|
| `Frame` | whole table body | enclosure of live data |
| `Series` | named typed column | line of appearance |
| `Expr` | executable expression | operation still in Being |

Compactly:

`DataFrame = Frame:Series::Expr as immediate analytic body`

This triad remains in the sphere of Being. Rows and columns still behave as
extensional data; expressions still behave as executable operations. Nothing has
yet been held up as Dataset mediation.

---

## DataFrame As Enclosure

DataFrame encloses live data without yet interpreting it as Dataset meaning.
This enclosure is not dead storage. It is formed immediacy: contents have enough
shape to be addressed, counted, typed, filtered, selected, joined, grouped, and
written.

The doctrinal word for this first enclosure is Intuition. From the Dataset side,
the same enclosure is EssentialBeing: determinate enough to mediate, not yet
mediated.

---

## Construction Discipline

Construction is where raw contents become a Frame.

Current construction surfaces include:

- columns
- rows
- records
- series
- schema pairs
- schema overrides
- orientation and strictness options

Construction is therefore not a minor utility. It is the threshold where loose
content receives tabular form.

Review rule:

If construction hides column order, dtype, schema, or null behavior, it weakens
the beginning of the path.

---

## Ordinary Operations

The ordinary DataFrame surface includes operations such as:

- `select`
- `filter`
- `with_columns`
- `sort`
- `group_by`
- `join`
- horizontal aggregations
- mapping over rows or columns
- `to_struct`
- table IO (`csv`, `json`, `ipc`, `parquet`)

These operations are not yet Dataset doctrine. They are how the immediate body
moves in its own sphere.

When Dataset later reflects DataFrame, some of these operations become material
for Plan. But inside DataFrame they remain ordinary tabular transformations.

---

## Relation To Plan

Plan does not live inside DataFrame as doctrine. DataFrame offers lazy and eager
tabular behavior; Plan is Dataset's reflective holding of that behavior.

The passage is:

```text
Expr in DataFrame Being
  -> Plan in Dataset Essence
```

This matters because a DataFrame expression can execute without becoming a Plan.
A Plan begins only when Dataset mediation retains, names, and orders an
expression sequence for reflective use.

---

## Relation To Dataset

The relation is asymmetrical:

- DataFrame can exist, execute, serialize, and write artifacts without Dataset.
- Dataset requires a DataFrame body in order to mediate anything concrete.

So Dataset is not a replacement for DataFrame. Dataset is a higher-order reading
of the same body.

This is why the first Dataset question is always:

`Where does the raw content become a DataFrame body?`

---

## Relation To Shell

Shell reads DataFrame as the immediate register.

The minimum Shell needs from DataFrame is:

- column names
- dtypes
- row count
- column count
- a schema seed
- a path into the mediated Dataset register

`ShellDataFrameKnowledge` is deliberately smaller than the full DataFrame API.
It records what the Shell needs to know about the immediate body, not everything
Polars can do.

---

## Boundary Rule

Do not make DataFrame responsible for Dataset mediation.

DataFrame does not by itself:

- name semantic intent
- prepare Model essence
- bind Feature roles
- govern Plan commitments
- produce SemDataset meaning
- validate PureForm return

DataFrame supplies the formed body. Dataset mediates. Shell governs the path.

---

## Doctrine-First Implementation Horizon

For the current phase, doctrine leads implementation. We write the required
shape first, then implement in Rust in small, testable batches.

This means:

- do not force premature API churn just to satisfy every doctrinal sentence now
- keep implementation TODOs explicit and local to the doctrine
- treat doctrine as the ideal contract for upcoming code work

### Planned TODO Bands

1. DataFrame beginning clarity TODO
  - tighten construction-time schema and orientation diagnostics
  - standardize shape and dtype reporting surfaces across wrappers

2. Expr passage discipline TODO
  - make the handoff from immediate Expr operations to mediated Plan usage
    more explicit in examples and helper APIs

3. Shell seed reliability TODO
  - ensure every DataFrame entry path preserves the minimum seed contract:
    columns, dtypes, row_count, column_count, schema projection

4. Projection backends TODO
  - keep Postgres/Neo4j/GML/TS/Rust/JSON/Zod projections downstream of
    DataFrame/Dataset grammar instead of introducing bypass surfaces

None of these TODOs requires breaking the doctrine-first method. They are
implementation consequences of it.

---

## Review Rule

When reviewing a DataFrame change, ask:

1. Does it preserve the ordinary Polars-facing body?
2. Are columns, dtypes, rows, schema, and shape still inspectable?
3. Does it clarify or obscure the `Frame:Series::Expr` triad?
4. Does it keep DataFrame operations distinct from Dataset Plan mediation?
5. Can Shell still seed its immediate register from the resulting body?

If these answers are unclear, the beginning of the Dataset path has become
unclear.

---

## The DataFrame Method

The DataFrame Method is the doctrinal procedure for working with DataFrame at
any scale: new API, new example, projection path, or code review. It is a
five-step cycle. Each step names the governing doctrine, the practical question,
and the acceptable answer form.

Run each step in order. A step that cannot be answered halts the work until it
is clarified.

---

### Step 1 — Locate the Beginning (EssentialBeing)

Governing doctrine: DataFrame is EssentialBeing. It is immediate and determinate
enough to mediate, but not yet mediated.

Question:

> Where does the raw content become a `Frame:Series::Expr` body?

Acceptable answer form:

- name the construction surface used (columns / rows / records / series / IO)
- state the schema: column names and dtypes at construction time
- confirm that no Dataset concept has entered before this point

If construction is ambiguous or the schema is not stateable, the Beginning is
not yet a Beginning. Resolve it before proceeding.

---

### Step 2 — State the Asymmetry

Governing doctrine: DataFrame executes alone. Dataset cannot mediate without
DataFrame. Mediation is a higher-order reading, not a second body.

Question:

> Can this DataFrame body exist and execute independently of Dataset?

Acceptable answer form:

- yes: the body constructs, filters, selects, and writes without Dataset
- if no: identify which Dataset concept has leaked into DataFrame and remove it

This step catches the most common architectural error: making DataFrame
dependent on Dataset awareness it should not carry.

---

### Step 3 — Name the Passage (Vanishing Rule)

Governing doctrine: Being vanishes as independent sphere inside Essence. The
immediate `Frame:Series::Expr` does not remain as an independent object inside
Dataset mediation. It persists only as mediated form:

```text
Frame  ->  Frame as Model
Series ->  Series as Feature
Expr   ->  Expr as Plan
```

Question:

> What becomes of this DataFrame body when Dataset mediates it?

Acceptable answer form:

- name which columns become Model body
- name which Series become Feature candidates
- name which Expr operations become Plan steps (or remain raw and unmediated)

If the full immediate body is assumed to persist unchanged inside Dataset, the
Vanishing Rule has been violated. Correct it by making the mediation explicit.

---

### Step 4 — Check the Plan Boundary

Governing doctrine: Plan is deferred reflection over DataFrame immediacy. It is
not a copy of everything in DataFrame. A Plan retains, names, and orders — it
does not claim to exhaust all immediate variation.

Question:

> Where does this work make a Plan, and what does the Plan explicitly exclude?

Acceptable answer form:

- name the Plan: `{ name, source, steps }` at minimum
- name at least one thing the Plan does not retain (excluded columns, raw IO,
  uninterpreted Expr operations)
- confirm the three demands are satisfiable: ontological (body?),
  epistemological (inspectable?), mathematical (composable?)

If no exclusion can be named, the Plan is implicitly claiming to be total
Dataset representation. That claim is false and must be corrected.

---

### Step 5 — Verify the Shell Seed

Governing doctrine: Shell reads DataFrame as the immediate register. The minimum
Shell seed contract must survive every DataFrame entry path.

Question:

> After this work, can Shell still produce a valid seed from the DataFrame body?

Acceptable answer form:

- column names: present and non-empty
- dtypes: all columns typed
- row_count: stateable (may be approximate for lazy bodies)
- column_count: stateable
- schema projection: addressable by `ShellDataFrameKnowledge`

If Shell cannot produce a seed, the DataFrame body has lost enough determinacy
that the path into Dataset and PureForm return is severed. This is a hard stop.

---

### Method Summary

| Step | Governing Doctrine | Central Question |
|---|---|---|
| 1 | EssentialBeing | Where does raw content become Frame? |
| 2 | Asymmetry | Can this body execute without Dataset? |
| 3 | Vanishing Rule | What becomes of Frame as Dataset mediates? |
| 4 | Plan Boundary | What does the Plan retain and exclude? |
| 5 | Shell Seed | Can Shell still read the minimum register? |

Each step is a constraint, not a checkbox. A step answered with a vague
affirmative is not answered. The Method earns its result only when each
question receives a stateable, falsifiable answer.
