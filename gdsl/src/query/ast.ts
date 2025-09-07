export type Var = string

// Predicates allowed in rule bodies (keep small and practical)
export type EdgePred = { kind: 'edge'; type?: string; from: Var; to: Var }            // edge(?a -[:TYPE]-> ?b) or any type if type=undefined
export type LabelPred = { kind: 'label'; v: Var; label: string }                      // has label
export type PropContainsPred = { kind: 'propContains'; v: Var; path: string[]; value: string } // contains substring

export type BodyPred = EdgePred | LabelPred | PropContainsPred
export type Literal = { pred: BodyPred; negated?: boolean }                            // negated is reserved (not implemented yet)

// Rule head: derive a typed edge from two vars
export type EdgeHead = { kind: 'edge'; type: string; from: Var; to: Var }

export type Rule = {
  name?: string
  head: EdgeHead
  body: Literal[]
}
