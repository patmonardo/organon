import type { Builtin, Literal } from '../core'

export function emitZ3Guards(lits: Literal[]) {
  const guards = lits.filter((l: any) => l.kind === 'builtin') as Builtin[]
  const z: string[] = ['(set-logic ALL)']
  for (const g of guards) {
    const [a, b, c] = g.args
    const A = (a as any).name ?? (a as any).value
    const B = (b as any).name ?? (b as any).value
    switch (g.op) {
      case 'lt': z.push(`(assert (< ${A} ${B}))`); break
      case 'le': z.push(`(assert (<= ${A} ${B}))`); break
      case 'gt': z.push(`(assert (> ${A} ${B}))`); break
      case 'ge': z.push(`(assert (>= ${A} ${B}))`); break
      case 'eq': z.push(`(assert (= ${A} ${B}))`); break
      case 'neq': z.push(`(assert (not (= ${A} ${B})))`); break
      default: break
    }
  }
  z.push('(check-sat)', '(get-model)')
  return { smt2: z.join('\n') }
}
