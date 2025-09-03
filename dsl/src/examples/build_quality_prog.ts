import { type Program, atom, fact, rule, v, sym, str } from '../core'

// tag(X,"exclusive") :- hasProp(X,P), hasProp(X,Q), P != Q, opposes(P,Q).
const X = v('X'), P = v('P'), Q = v('Q')
const prog: Program = {
  facts: [
    fact(atom('hasProp', [sym('alice'), sym('hot')])),
    fact(atom('hasProp', [sym('alice'), sym('cold')])),
    fact(atom('opposes', [sym('hot'), sym('cold')]))
  ],
  rules: [
    rule(
      atom('tag', [X, str('exclusive')]),
      [
        atom('hasProp', [X, P]),
        atom('hasProp', [X, Q]),
        { kind: 'builtin', op: 'neq', args: [P, Q] },
        atom('opposes', [P, Q])
      ],
      { tags: ['quality:exclusivity'] }
    )
  ],
  meta: { tags: ['example'] }
}

console.log(JSON.stringify(prog, null, 2))
