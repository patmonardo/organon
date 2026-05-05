# Exemplar 023 — String Namespace as Text-Form Observation

**Source file**: `gds/examples/dataset_namespace_string.rs`
**Arc position**: Observation (text-form mark derivation — the NLP entry point)
**Prior exemplar**: [022 — List Namespace as Variable-Arity Observation](../dataset/022-list-namespace-observation.md)
**Next exemplar**: [024 — Catalog Extension as Artifact Persistence](../dataset/024-catalog-extension-persistence.md)

---

## Principle

A string column is the appearance of natural language as a typed entity surface.
String namespace operations are the first Observation marks that the NLP arc derives:
byte count, character count, case normalization, pattern detection.
Before tokens, before parse trees, before embeddings — there is the string.

---

## What This Example Does

It builds a two-column table — `language` and `fruit` — where `fruit` contains
multilingual words (English "pear", Dutch "peer", Portuguese "pêra", Finnish "päärynä"):

```rust
str_ns(gds::col!(fruit)).len_bytes().alias("byte_count")
str_ns(gds::col!(fruit)).len_chars().alias("letter_count")
str_ns(gds::col!(fruit)).to_uppercase().alias("upper")
```

Then demonstrates `GDSSeries + str_ns` directly for Series-level text access.

---

## The Arc Reading

```
Source: a column of text strings (fruit names in four languages)
  → str_ns(...).len_bytes()     [Mark: byte length — encoding-aware]
  → str_ns(...).len_chars()     [Mark: character length — Unicode-aware]
  → str_ns(...).to_uppercase()  [Mark: normalized form — case-insensitive identity surface]
```

These three marks are the entry point of the text observation arc:
- **Byte count** — how the kernel sees the string as raw memory
- **Character count** — how the language sees the string as a sequence of code points
- **Uppercase** — the normalized identity that allows cross-language comparison

For "päärynä" (Finnish):
- `byte_count = 10` (UTF-8 encoding: ä is 2 bytes each)
- `letter_count = 8` (8 code points)
- `upper = "PÄÄRYNÄ"`

The divergence between byte count and letter count is the NLP signal: this entity is non-ASCII.
Any Principle that operates on character-level identity should use `letter_count`, not `byte_count`.

---

## The Multilingual Appearance

The choice of `language`/`fruit` pairing is doctrinal.
It demonstrates that the same concept — a kind of fruit — appears differently across languages:
- Same referent (pear)
- Different string forms ("pear", "peer", "pêra", "päärynä")
- Different byte counts
- Same character-level structure (all 4–8 characters)

In the Shell program arc, each row is a separate entity: English-pear is not the same entity as Dutch-peer.
They are different appearances of the same underlying referent.
Unification across languages is a Concept-level operation (two entities sharing a common identity),
not an Observation-level operation. At Observation, each string is its own entity.
*See*: [Unification](../../REFERENCES/dataset/unification.md)

---

## `str_ns` as the NLP Observation Gateway

The String namespace is the NLP observation gateway. Operations available in `str_ns`:
- `len_bytes` / `len_chars` — structural marks
- `to_uppercase` / `to_lowercase` — normalization marks
- `starts_with` / `ends_with` / `contains` — pattern marks
- `split` / `replace` — structural transformation marks
- `strip_chars` / `zfill` — cleaning marks

All of these are Observation-level: they derive marks from the raw string without semantic interpretation.
Semantic interpretation begins at Reflection, when the marks are worked through as a logical sequence.
The NLP pipeline begins here — in `str_ns`, with deterministic, encoding-aware operations on typed text.

---

## `GDSSeries.str()`: Concept-Level Text Access

```rust
let fruit = GDSSeries::from("fruit", ["pear", "peer", "pêra", "päärynä"]);
let upper = fruit.str().to_uppercase()?;
let bytes = fruit.str().len_bytes()?;
```

The String namespace is accessible at the Series level — Concept-level text access
without building a DataFrame. This is how the kernel inspects string-typed Concepts:
directly, via `.str()`, with no intermediate table required.

In an external `.gdsl` artifact, a `concept` block with a string identity field uses this access path:
```gdsl
concept FruitName from appearances {
  identity fruit;
  mark byte_count;
  mark letter_count;
}
```
The kernel will call `GDSSeries::from("fruit", ...).str().len_bytes()` automatically to derive the marks.

---

## Key Vocabulary

**String namespace (`str_ns`)** — Text-form Observation operations: structural, normalization, and pattern marks on string columns.

**Byte count vs. character count** — The fundamental text measurement distinction. Byte count is kernel-level memory measurement. Character count is Unicode-level semantic measurement. For ASCII text, they are equal. For multilingual text, they diverge.

**Normalization mark** — An Observation derivation that produces a canonical form: `to_uppercase`, `to_lowercase`. Used to create identity surfaces for cross-entity comparison.

**Multilingual appearance** — Multiple string entities that are different appearances of the same conceptual referent. Unification is a Concept-level operation, not an Observation-level one.

**`GDSSeries.str()`** — Direct Concept-level access to String namespace operations.
*See*: [Frame DSL](../../REFERENCES/dataframe/frame-dsl.md)

---

## Student Notes

- The Finnish word "päärynä" has 8 characters but 10 bytes. Build a Principle condition that should apply to character-level identity (not byte-level). Then try building the same condition using `byte_count` instead of `letter_count`. The Finnish entity will behave differently under each. That difference is the doctrinal point.
- `to_uppercase` is not just cosmetic. In a Principle condition like `require identity starts_with("P")`, you must first normalize to uppercase so that "pear" and "Pear" are treated as the same identity. Always apply normalization marks before building case-sensitive Principle conditions.
- The `language` column is metadata about the appearance, not the appearance itself. In an external `.gdsl` `appearance` block: `key language; retain fruit;` — the language is the key that identifies the row; the fruit is the retained content that will be marked and reflected. This distinction matters when building multi-language concept models.

---

## What This Example Does Not Show

- Tokenization and list-form text (splitting a string into a list of tokens — bridges to Exemplar 022)
- Full NLP pipeline: tree structures, dependency graphs, embeddings (see Exemplar 003 and ML exemplars)
- External `.gdsl` `appearance` and `mark` blocks for text columns (see [Shell External Program Grammar](../../REFERENCES/shell/grammar.md))
- Multilingual concept unification (see [Unification](../../REFERENCES/dataset/unification.md))
