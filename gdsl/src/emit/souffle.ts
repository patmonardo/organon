import type { Program, Atom, Fact, Rule, Term, Var } from '../core'

function t(t: Term): string {
  if ((t as any).kind === 'var') return (t as Var).name
  const v = (t as any).value ?? (t as any).name
  return typeof v === 'number' ? v.toString() : `"${v}"`
}

export function emitSouffle(p: Program) {
  const preds = new Map<string, number>()
  const decl: string[] = []
  const facts: string[] = []
  const rules: string[] = []

  function ensureDecl(a: Atom) {
    if (!preds.has(a.pred)) preds.set(a.pred, a.args.length)
  }

  p.facts.forEach(f => ensureDecl(f.atom))
  p.rules.forEach(r => { ensureDecl(r.head); r.body.forEach(l => (l as any).pred && ensureDecl(l as any)) })

  // .decl lines (use symbol for strings, number for numbers; keep all as symbol for simplicity v0)
  for (const [name, ar] of preds) {
    const cols = Array.from({ length: ar }, (_, i) => `c${i}: symbol`).join(', ')
    decl.push(`.decl ${name}(${cols})`)
    // input/output if desired
    if (name.endsWith('Input')) decl.push(`.input ${name}`)
    if (name.endsWith('Output')) decl.push(`.output ${name}`)
  }

  // facts
  for (const f of p.facts) {
    facts.push(`${f.atom.pred}(${f.atom.args.map(t).join(', ')}).`)
  }

  // rules (no negation/aggregates in v0 emitter)
  for (const r of p.rules) {
    const body = r.body
      .filter(b => (b as any).pred)
      .map((a: any) => `${a.pred}(${a.args.map(t).join(', ')})`)
      .join(', ')
    rules.push(`${r.head.pred}(${r.head.args.map(t).join(', ')}) :- ${body}.`)
  }

  const dl = [decl.join('\n'), facts.join('\n'), rules.join('\n')].filter(Boolean).join('\n\n')
  return { dl }
}
