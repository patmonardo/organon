import { atom, rule, bi, sym, v, str, type Atom, type Var, type Sym, type Program } from '../core'

// Core predicates
export const triad = (U: Sym | Var, A: Sym | Var, B: Sym | Var, C: Sym | Var): Atom =>
  atom('meta:triad', [U, A, B, C])                 // e.g., Existence ← Being, Nothing, Becoming

export const simpleUnity = (U: Sym | Var): Atom =>
  atom('meta:simpleUnity', [U])                    // “Simple Unity” (Viyoga’s presupposition)

export const organicUnity = (U: Sym | Var): Atom =>
  atom('meta:organicUnity', [U])                   // “Organic Unity” (Sanyoga)

export const memberOf = (X: Sym | Var, U: Sym | Var): Atom =>
  atom('meta:memberOf', [X, U])                    // part membership in an organic unity

export const duality = (D: Sym | Var, X: Sym | Var, Y: Sym | Var): Atom =>
  atom('meta:duality', [D, X, Y])                  // named duality of X vs Y

export const viyoga = (X: Sym | Var, Y: Sym | Var): Atom =>
  atom('meta:viyoga', [X, Y])                      // separation

export const sanyoga = (X: Sym | Var, Y: Sym | Var): Atom =>
  atom('meta:sanyoga', [X, Y])                     // conjunction within an organic unity

export const presupposes = (D: Sym | Var, U: Sym | Var): Atom =>
  atom('meta:presupposes', [D, U])                 // a duality presupposes a Simple Unity

export const expresses = (X: Sym | Var, G: Sym | Var): Atom =>
  atom('meta:expresses', [X, G])                   // map Guna ↔ unity mode

export const recognizes = (A: Sym | Var, U: Sym | Var): Atom =>
  atom('meta:recognizes', [A, U])

export const emergesFrom = (U: Sym | Var, O: Sym | Var): Atom =>
  atom('meta:emergesFrom', [U, O])

// Optional: treat the set of simple unities as a “rosary”
export const rosary = (U: Sym | Var): Atom =>
  atom('meta:rosary', [U])

// Rules
export function metaphysicsRules() {
  const U = v('U'), A = v('A'), B = v('B'), C = v('C')
  const X = v('X'), Y = v('Y'), D = v('D'), O = v('O')
  const Abs = sym('Absolute')

  return [
    // Emergence: Simple Unity via triad (Existence from Being/Nothing/Becoming)
    rule(simpleUnity(U), [triad(U, A, B, C)], { tags: ['meta:simple-from-triad'] }),

    // Viyoga: any duality witnesses separation of its poles
    rule(viyoga(X, Y), [duality(D, X, Y)], { tags: ['meta:viyoga-from-duality'] }),

    // Duality presupposes a Simple Unity (pick any triad’s U; typically Existence)
    rule(presupposes(D, U), [duality(D, X, Y), triad(U, A, B, C)], { tags: ['meta:duality-presupposes-simple'] }),

    // Sanyoga: two distinct members within the same organic unity
    rule(sanyoga(X, Y), [memberOf(X, O), memberOf(Y, O), bi('neq', [X, Y]), organicUnity(O)], {
      tags: ['meta:sanyoga-membership']
    }),

    // Emergence: a Simple Unity emerges from an Organic Unity that contains its triadic members
    rule(emergesFrom(U, O), [
      triad(U, A, B, C),
      organicUnity(O), memberOf(A, O), memberOf(B, O), memberOf(C, O)
    ], { tags: ['meta:emerges-from-organic'] }),

    // Recognition: Absolute recognizes every Simple Unity
    rule(recognizes(Abs, U), [simpleUnity(U)], { tags: ['meta:absolute-recognizes'] }),

    // Rosary: collect all Simple Unities
    rule(rosary(U), [simpleUnity(U)], { tags: ['meta:rosary'] }),
  ]
}

// Small helper to build a program with these rules
export function asProgram(facts: Program['facts']): Program {
  return { facts, rules: metaphysicsRules(), meta: { tags: ['domain:metaphysics'] } }
}
