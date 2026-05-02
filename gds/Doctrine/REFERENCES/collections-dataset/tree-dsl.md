# Tree DSL

The Tree DSL provides operations over tree-shaped Dataset artifacts.

---

## Doctrine

Tree operations are still Dataset operations. They should remain inspectable as tabular tree transformations, not hidden recursive logic.

---

## Common Operations

- traverse children
- filter by label
- project spans
- collect subtrees
- map nodes to marks

---

## Review Rule

A tree operation should preserve node identity and source span unless explicitly producing an aggregate.
