# Idea Principles Ledger

Canonical registry of principle nodes for the Idea workbooks and their KG joints.

## Usage

- Keep one row per principle node.
- Keep `Principle ID` stable once assigned.
- Use `Kernel` for top-level logical commitments.
- Use `Pedagogy` for explanatory teaching frames linked to a Kernel node.

## Ledger Table

| Principle ID | Layer    | Claim                                                                                                            | Role in Network                                                   | Depends On         | Transitions To   | Workbook Refs | Status |
| ------------ | -------- | ---------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------------------ | ---------------- | ------------- | ------ |
| pr-log-001   | Kernel   | Logic must begin from the Sache itself and cannot presuppose external rules of thought.                          | Entry condition for pure science; grounds self-developing method. | phen-abs-knowledge | pr-log-002       | logic-idea    | seed   |
| pr-log-002   | Kernel   | The form/content split is inadequate; thought determinations are themselves content.                             | Refutes formalism; establishes logical content as objective.      | pr-log-001         | pr-log-003       | logic-idea    | seed   |
| pr-log-003   | Kernel   | Contradiction is immanent in determinations and drives transition, not mere collapse into nothing.               | Engine of dialectical development.                                | pr-log-002         | pr-log-004       | logic-idea    | seed   |
| pr-log-004   | Kernel   | Determinate negation preserves content and generates a richer successor concept.                                 | Defines progression rule for concept network growth.              | pr-log-003         | pr-log-005       | logic-idea    | seed   |
| pr-log-005   | Kernel   | Logic divides into Being, Essence, Concept as immanent determinations of one concrete unity.                     | Top-level architecture for workbook decomposition.                | pr-log-004         | being-idea-001   | logic-idea    | seed   |
| pe-log-001   | Pedagogy | Beginners first encounter logic as abstract form; significance deepens after scientific experience.              | Teaching sequencing principle for workbook pedagogy.              | pr-log-005         | pe-log-002       | logic-idea    | seed   |
| pe-log-002   | Pedagogy | Workbook work should pair conceptual rigor with guided practice to move from abstract to concrete understanding. | Bridges Principle Ledger and educational track.                   | pe-log-001         | concept-idea-001 | logic-idea    | draft  |

## ID Rules

- Kernel: `pr-<domain>-NNN`
- Pedagogy: `pe-<domain>-NNN`
- Domain examples: `log`, `bei`, `ess`, `con`, `ide`
- Use left-padded sequence numbers (`001`, `002`, ...)

## Linking Rules

- `Depends On` lists prerequisite principle IDs.
- `Transitions To` lists primary successor IDs.
- `Workbook Refs` points to workbook scope tags (`logic-idea`, `being-idea`, `concept-idea`, etc.).
