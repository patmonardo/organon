# Dataset Compilation

DatasetCompilation is the visible compiler graph for Dataset programs.

---

## Doctrine

Compilation is declaration, not execution. It says what the program means and how artifacts depend on one another.

---

## Nodes

A node carries:

- id
- name
- kind
- dependencies
- metadata

---

## Review Rule

Compilation graphs should be deterministic. The same program should produce the same node IDs and dependency shape.
