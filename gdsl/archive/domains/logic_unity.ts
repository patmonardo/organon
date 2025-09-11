import { atom, rule, bi, sym, v, type Atom, type Var, type Sym, type Program } from '../core'

// Neutral logical predicates (logic:* prefix)
export const triad = (U: Sym | Var, A: Sym | Var, B: Sym | Var, C: Sym | Var): Atom =>
  atom('logic:triad', [U, A, B, C])              // e.g., Existence ← Being, Nothing, Becoming

export const simple = (U: Sym | Var): Atom =>
  atom('logic:simple', [U])                      // Simple unity

export const organic = (U: Sym | Var): Atom =>
  atom('logic:organic', [U])                     // Organic unity

export const member = (X: Sym | Var, U: Sym | Var): Atom =>
  atom('logic:member', [X, U])                   // membership in organic unity

export const duality = (D: Sym | Var, X: Sym | Var, Y: Sym | Var): Atom =>
  atom('logic:duality', [D, X, Y])

export const separate = (X: Sym | Var, Y: Sym | Var): Atom =>
  atom('logic:separate', [X, Y])                 // viyoga

export const together = (X: Sym | Var, Y: Sym | Var): Atom =>
  atom('logic:together', [X, Y])                 // sanyoga

export const presupposes = (D: Sym | Var, U: Sym | Var): Atom =>
  atom('logic:presupposes', [D, U])

export const recognizes = (A: Sym | Var, U: Sym | Var): Atom =>
  atom('logic:recognizes', [A, U])

export const emergesFrom = (U: Sym | Var, O: Sym | Var): Atom =>
  atom('logic:emergesFrom', [U, O])

export const rosary = (U: Sym | Var): Atom =>
  atom('logic:rosary', [U])

export function logicUnityRules() {
  const U = v('U'), A = v('A'), B = v('B'), C = v('C')
  const X = v('X'), Y = v('Y'), D = v('D'), O = v('O')
  const Abs = sym('Absolute')

  return [
    // Simple from triad
    rule(simple(U), [triad(U, A, B, C)], { tags: ['logic:simple-from-triad'] }),

    // Separation from duality
    rule(separate(X, Y), [duality(D, X, Y)], { tags: ['logic:separate-from-duality'] }),

    // Duality presupposes a Simple
    rule(presupposes(D, U), [duality(D, X, Y), triad(U, A, B, C)], { tags: ['logic:duality-presupposes-simple'] }),

    // Togetherness inside an organic unity (X ≠ Y)
    rule(together(X, Y), [member(X, O), member(Y, O), bi('neq', [X, Y]), organic(O)], {
      tags: ['logic:together-membership']
    }),

    // Emergence: a Simple emerges from an Organic that contains its triadic members
    rule(emergesFrom(U, O), [
      triad(U, A, B, C),
      organic(O), member(A, O), member(B, O), member(C, O)
    ], { tags: ['logic:emerges-from-organic'] }),

    // Recognition and collection
    rule(recognizes(Abs, U), [simple(U)], { tags: ['logic:absolute-recognizes'] }),
    rule(rosary(U), [simple(U)], { tags: ['logic:rosary'] })
  ]
}

export function asProgram(facts: Program['facts']): Program {
  return { facts, rules: logicUnityRules(), meta: { tags: ['domain:logic-unity'] } }
}
