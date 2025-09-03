import { atom, rule, bi, sym, v, str, type Atom, type Var, type Sym, type Str, type Program, type Term } from '../core'

// Accept plain strings as sugar for symbolic qualities
type Qual = Sym | Str | Var
type QualLike = Qual | string
const asQual = (x: QualLike): Qual => (typeof x === 'string' ? sym(x) : x)

// Predicates
export const being = (X: Var): Atom => atom('qual:being', [X])
export const quality = (X: Var, Q: QualLike): Atom => atom('qual:quality', [X, asQual(Q)])
export const opposes = (P: QualLike, Q: QualLike): Atom => atom('qual:opposes', [asQual(P), asQual(Q)])
export const dominates = (X: Var, Q: QualLike): Atom => atom('qual:dominates', [X, asQual(Q)])
export const existence = (X: Var): Atom => atom('qual:existence', [X])
export const self = (X: Var): Atom => atom('qual:self', [X])

// Domain rules
export function qualityRules() {
  const X = v('X'), P = v('P'), Q = v('Q')
  return [
    // Existence: something exists in the quality domain if it has any quality
    rule(existence(X), [quality(X, P)], { tags: ['qual:existence'] }),

    // Self: dominated quality implies a notion of “self” (minimal scaffold)
    rule(self(X), [dominates(X, Q)], { tags: ['qual:self'] }),

    // Tag exclusive: conflicting qualities present together
    // tag(X, 'exclusive') :- quality(X,P), quality(X,Q), P != Q, opposes(P,Q).
    rule(atom('tag', [X, str('exclusive')]), [
      quality(X, P),
      quality(X, Q),
      bi('neq', [P, Q]),
      opposes(P, Q)
    ], { tags: ['qual:exclusive'] }),
  ]
}

// Helper to wrap facts with the domain rules
export function asProgram(facts: Program['facts']): Program {
  return { facts, rules: qualityRules(), meta: { tags: ['domain:quality'] } }
}
