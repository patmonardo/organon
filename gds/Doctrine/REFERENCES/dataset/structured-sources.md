# Structured Sources

Structured sources are inputs whose syntax already carries explicit form: JSON, XML, HTML, and related formats.

---

## Doctrine

Structured input is still only Observation. A parsed JSON tree or markup tree is not yet Concept. It is appearance made explicit.

---

## Pipeline

```text
source text -> tokens -> parse forest -> tree artifact -> marks -> reflection
```

- tokenizer: splits input into typed tokens
- parser: builds parse tree or forest
- tree artifact: stores the observed structure
- marks: attach semantic roles
- reflection: begins essence articulation

---

## JSON

JSON enters as objects, arrays, primitives, and field names. These become tree nodes. Field names are not concepts yet; they are observed structural labels.

---

## XML and HTML

Markup enters as tags, attributes, text nodes, and nesting. These become a document tree. Tags are not concepts yet; they are observed structure.

---

## Review Rule

When adding a structured-source parser, ensure it:

1. Tokenizes deterministically
2. Emits inspectable tree form
3. Preserves source provenance
4. Leaves semantic marking to later stages
