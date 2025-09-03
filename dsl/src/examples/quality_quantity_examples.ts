import { Program, rule, v, str, bi, agg, atom } from '../core'
import { hasProp, opposes, measure } from '../qual_quan'

// Vars
const X = v('X'), P = v('P'), Q = v('Q'), D = v('D'), N = v('N'), V = v('V')

// 1) Quality: tag exclusivity when an entity has opposed properties
const progQuality: Program = {
  facts: [],
  rules: [
    rule(
      atom('tag', [X, str('exclusive')]),
      [hasProp(X, P), hasProp(X, Q), opposes(P, Q)],
      { tags: ['quality:exclusivity'] }
    )
  ]
}

// 2) Quantity: count measurements per dimension (group-by D)
const progQuantity: Program = {
  facts: [],
  rules: [
    rule(
      atom('measureCount', [D, N]),
      [
        agg('count', N, measure(X, D, V), undefined, [D])
      ],
      { tags: ['quantity:count-by-dimension'] }
    ),
    rule(
      atom('totalValue', [D, V]),
      [
        agg('sum', V, measure(X, D, v('Val')), v('Val'), [D])
      ],
      { tags: ['quantity:sum-by-dimension'] }
    )
  ]
}

console.log(JSON.stringify({ q: progQuality, k: progQuantity }, null, 2))
