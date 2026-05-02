# Provenance

Provenance is the lineage record carried by every Dataset artifact.

---

## Required Fields

Every emitted artifact should carry:

- `source_id`
- `specification_id`
- `program_name`
- `generated_at_unix_ms`

Additional fields may include document span, parser version, feature source, principle reference, or service run ID.

---

## Doctrine

Provenance is not metadata decoration. It is the answer to: how did this object come to be?

Without provenance, an artifact is not inspectable. Without inspection, the Knowledge Agent cannot claim scientific traceability.

---

## Stage-Specific Provenance

- Source: origin path or resource ID
- Observation: tokenizer/parser and retained schema
- Reflection: moment and essence mark
- Principle: condition and evidence reference
- Concept: principle reference
- Judgment: concept reference
- Syllogism: middle term reference
- Procedure: emitted stage and manifest reference

---

## Review Rule

A new artifact table is incomplete until it has provenance fields or a documented reason why it cannot yet carry them.
