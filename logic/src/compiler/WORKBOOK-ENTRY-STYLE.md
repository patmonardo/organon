# Workbook Entry ID + Title Style (Normalization Spec)

## Scope

This spec applies to Markdown headings in workbooks that use:

- `### Entry <id> — <title>`

The goal is stable IDs and Outline-friendly titles.

## Canonical heading form

Required:

- `### Entry <id> — <core title>`

No placeholder forms:

- `### Entry (Topic) ...`
- `### Entry ... <id> ...`
- headings without a concrete ID

## ID grammar

### Terms

- **Domain mnemonic**: exactly 3 lowercase letters for the major logical domain.
  - examples: `bei`, `exi`, `qty`, `qtm`, `rat`, `spq`, `rm`, `sub`, `abs`, `act`
- **Part mnemonic**: exactly 3 lowercase letters for the workbook part/family.
  - examples: `par`, `ref`, `nes`, `con`, `spec` (project chooses actual values)
- **Index**: exactly 3 digits, zero-padded (`001`..`999`).
- **Subentry letter**: one lowercase letter from `a|b|c`, only when the part has subentries.

### Canonical ID patterns

1. Regular entry (no subentries):

- `<domain>-<part>-<index>`
- regex: `^[a-z]{3}-[a-z]{3}-\d{3}$`

2. Subentry group marker (optional, if used):

- `<domain>-<part>-<sub>`
- regex: `^[a-z]{3}-[a-z]{3}-[abc]$`

3. Subentry item:

- `<domain>-<part>-<sub>-<index>`
- regex: `^[a-z]{3}-[a-z]{3}-[abc]-\d{3}$`

### Prohibited ID forms

- Mixed mnemonic lengths (`2`, `4+`, etc.)
- Parentheses in IDs (`c(a)`, `c-b(1)`, etc.)
- Variant delimiters (`_`, `.`, spaces)
- Double hyphens
- Uppercase letters

## Title rules (Outline-first)

### Core principle

Use **content-first** titles with minimal prefix noise so Outline reveals meaning immediately.

### Required

- Keep structural bookkeeping out of title starts (move it to body text if needed).
- Prefer short, browseable core titles (target 2–5 words; 3 words preferred in most cases).

### Disallowed title prefixes

- `Section N:`
- `Block N:`
- `Paragraph N:`
- `Marker ...`
- Numbering wrappers like `a.1`, `b.2`, `c-003`, `1.` at start

## Examples

Good:

- `### Entry bei-par-001 — Pure being immediacy`
- `### Entry jud-nes-b-002 — Conditional relation mediation`
- `### Entry qtm-dif-a-003 — Degree self-reference continuity`

Good with subentries:

- `### Entry exi-fin-a — Finitude immediacy`
- `### Entry exi-fin-a-001 — Non-being finite nature`
- `### Entry exi-fin-a-002 — Finite obstinacy`
- `### Entry exi-fin-a-003 — Perishing of perishing`

Bad:

- `### Entry b-c-a-001 — c(a). Immediacy of finitude I: ...`
- `### Entry obj-chem-b-001 — Section 1: affinity, communication...`
- `### Entry (Topic) <id> — <title>`

## Migration guidance

- Keep existing conceptual sequence; normalize only surface form.
- If a part currently uses ad hoc IDs, map each old ID to a canonical ID in a temporary migration table.
- Normalize title starts first (highest impact for Outline browsing), then backfill IDs.
