import type { Program, Atom, Term, Var } from '../core'

function t(x: Term): string {
  if ((x as any).kind === 'var') return (x as Var).name.toLowerCase()
  const v = (x as any).value ?? (x as any).name
  return typeof v === 'number' ? v.toString() : v.toString().toLowerCase()
}

export function emitClingo(p: Program) {
  const lines: string[] = []
  for (const f of p.facts) lines.push(`${f.atom.pred}(${f.atom.args.map(t).join(',')}).`)
  for (const r of p.rules) {
    const head = `${r.head.pred}(${r.head.args.map(t).join(',')})`
    const body = r.body.map((lit: any) => {
      if (lit.kind === 'neg') return `not ${lit.atom.pred}(${lit.atom.args.map(t).join(',')})`
      if (lit.pred) return `${lit.pred}(${lit.args.map(t).join(',')})`
      return '' // skip builtins/aggregates in v0
    }).filter(Boolean).join(', ')
    lines.push(`${head} :- ${body}.`)
  }
  return lines.join('\n')
}
