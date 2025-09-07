import { atom, rule, bi, v, type Atom, type Var, type Sym, type Program } from '../core'

// Predicates (logic:* namespace)
export const complex = (C: Sym | Var): Atom => atom('logic:complex', [C])            // a unity with parts
export const part = (C: Sym | Var, X: Sym | Var): Atom => atom('logic:part', [C, X]) // membership
export const center = (C: Sym | Var, X: Sym | Var): Atom => atom('logic:center', [C, X]) // One as center

export const one = (X: Sym | Var): Atom => atom('logic:one', [X])                     // the One
export const forSelf = (X: Sym | Var): Atom => atom('logic:forSelf', [X])             // Being‑for‑Self

// One-and-Many: repulsion/attraction
export const repels = (X: Sym | Var, Y: Sym | Var): Atom => atom('logic:repels', [X, Y])     // exclusion
export const attracts = (X: Sym | Var, Y: Sym | Var): Atom => atom('logic:attracts', [X, Y]) // relating

// Optional aliases specialized for Body–Mind complexes
export const bodyMind = (C: Sym | Var): Atom => atom('logic:bmComplex', [C])
export const body = (X: Sym | Var): Atom => atom('logic:body', [X])
export const mind = (X: Sym | Var): Atom => atom('logic:mind', [X])

export function beingForSelfRules() {
  const C = v('C'), X = v('X'), Y = v('Y')

  return [
    // One: any center of a complex counts as a One
    rule(one(X), [center(C, X)], { tags: ['bfs:one-from-center'] }),

    // Repulsion (exclusion): the One excludes its others within the same complex
    rule(repels(X, Y), [center(C, X), part(C, Y), bi('neq', [X, Y])], { tags: ['bfs:repulsion'] }),

    // Attraction (relating): the One relates to every member (including possibly itself)
    rule(attracts(X, Y), [center(C, X), part(C, Y)], { tags: ['bfs:attraction'] }),

    // Being‑for‑Self: a One that both excludes and relates (self‑reference via mediation)
    rule(forSelf(X), [one(X), repels(X, Y)], { tags: ['bfs:forSelf'] }),

    // Convenience: any body–mind complex is a complex
    rule(complex(C), [bodyMind(C)], { tags: ['bfs:bm-is-complex'] })
  ]
}

export function asProgram(facts: Program['facts']): Program {
  return { facts, rules: beingForSelfRules(), meta: { tags: ['domain:being-for-self'] } }
}
