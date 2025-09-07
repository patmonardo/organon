import fs from 'fs'
import path from 'path'

type Edge = { type: string; from: string; to: string; props?: any }
type Node = { id: string; labels?: string[]; props?: any }
type Artifact = { nodes: Node[]; edges: Edge[] }

function load(file: string): Artifact {
  return JSON.parse(fs.readFileSync(path.resolve(file), 'utf8'))
}

// Collect HLO tags
function collectTags(art: Artifact) {
  const tags = new Map<string, Record<string,string>>()
  for (const e of art.edges) {
    if (e.type === 'HLO_TAGS') {
      const key = e.props?.key
      const value = e.props?.value
      if (!key) continue
      const t = tags.get(e.from) ?? {}
      t[String(key)] = String(value)
      tags.set(e.from, t)
    }
    if (e.type === 'HLO_TAGS_TERM') {
      const key = e.props?.key
      const termId = String(e.to).startsWith('term:') ? String(e.to).slice(5) : String(e.to)
      if (!key) continue
      const t = tags.get(e.from) ?? {}
      t[String(key)] = termId
      tags.set(e.from, t)
    }
  }
  return tags
}

// Map HLO -> asserted tokens (e.g., equalsTruth)
function collectTokensByHlo(art: Artifact) {
  const byHlo = new Map<string, Set<string>>()
  const clausesByHlo = new Map<string, string[]>()
  for (const e of art.edges) if (e.type === 'HLO_HAS_CLAUSE') {
    const arr = clausesByHlo.get(e.from) ?? []
    arr.push(e.to)
    clausesByHlo.set(e.from, arr)
  }
  const tokenOfClause = new Map<string, string[]>()
  for (const e of art.edges) if (e.type === 'CLAUSE_ASSERTS_SYMBOL') {
    const tok = String(e.to).startsWith('token:') ? String(e.to).slice(6) : String(e.to)
    const arr = tokenOfClause.get(e.from) ?? []
    arr.push(tok)
    tokenOfClause.set(e.from, arr)
  }
  for (const [hlo, clauses] of clausesByHlo) {
    for (const c of clauses) {
      const toks = tokenOfClause.get(c) ?? []
      const set = byHlo.get(hlo) ?? new Set<string>()
      toks.forEach(t => set.add(t))
      byHlo.set(hlo, set)
    }
  }
  return byHlo
}

function asNum(x?: string) { const n = Number(x); return Number.isFinite(n) ? n : NaN }

type CycleReport = {
  cycle: string
  okOriginal: boolean
  okJudgment: boolean
  okReflection: boolean
  okReconstruct: boolean
  okStageOrder: boolean
  okEqualsTruthPlacement: boolean
  minimality: { removeOriginalBreaks: boolean; removeJudgmentBreaks: boolean; removeReconstructBreaks: boolean }
  orders: { original?: number; judgment?: number; reflections: number[]; reconstruction?: number }
  violations: string[]
}

function checkCycle(cycle: string, ids: string[], tags: Map<string, Record<string,string>>, tokensByHlo: Map<string, Set<string>>): CycleReport {
  const has = (id: string, k: string, v: string) => (tags.get(id)?.[k] ?? '') === v
  const filter = (k: string, v: string) => ids.filter(id => has(id,k,v))

  const originals = ids.filter(id => has(id,'phase','savitarka') && has(id,'role','original'))
  const judgments = ids.filter(id => has(id,'phase','nirvitarka') && has(id,'role','judgment'))
  const reflects  = ids.filter(id => has(id,'phase','savicara') && has(id,'role','reflection'))
  const reconstructs = ids.filter(id => has(id,'phase','nirvicara') && has(id,'role','reconstruction'))

  const orderOf = (id: string) => asNum(tags.get(id)?.order)
  const ordersRef = reflects.map(orderOf).filter(Number.isFinite).sort((a,b)=>a-b) as number[]
  const ordOriginal = originals[0] ? orderOf(originals[0]) : undefined
  const ordJudgment = judgments[0] ? orderOf(judgments[0]) : undefined
  const ordReconstruct = reconstructs[0] ? orderOf(reconstructs[0]) : undefined

  const violations: string[] = []

  const okOriginal   = originals.length === 1
  const okJudgment   = judgments.length === 1
  const okReflection = reflects.length >= 1
  const okReconstruct= reconstructs.length === 1

  if (!okOriginal) violations.push('require exactly one (savitarka, original)')
  if (!okJudgment) violations.push('require exactly one (nirvitarka, judgment)')
  if (!okReflection) violations.push('require at least one (savicara, reflection)')
  if (!okReconstruct) violations.push('require exactly one (nirvicara, reconstruction)')

  // Stage order: original < judgment; original < all reflections; all reflections < reconstruction; judgment < reconstruction
  let okStageOrder = true
  if (ordOriginal === undefined || ordJudgment === undefined || ordReconstruct === undefined || ordersRef.length === 0) {
    okStageOrder = false
    violations.push('missing order tags for one or more required steps')
  } else {
    if (!(ordOriginal < ordJudgment)) { okStageOrder = false; violations.push('order(original) must be < order(judgment)') }
    if (!(ordersRef.every(o => ordOriginal < o))) { okStageOrder = false; violations.push('all reflection orders must be > original') }
    if (!(ordersRef.every(o => o < ordReconstruct))) { okStageOrder = false; violations.push('all reflection orders must be < reconstruction') }
    if (!(ordJudgment < ordReconstruct)) { okStageOrder = false; violations.push('order(judgment) must be < order(reconstruction)') }
  }

  // Token placement constraint (optional but useful): equalsTruth only in reconstruction
  let okEqualsTruthPlacement = true
  const eqTok = 'equalsTruth'
  for (const id of ids) {
    const hasEq = tokensByHlo.get(id)?.has(eqTok) ?? false
    if (!hasEq) continue
    const role = tags.get(id)?.role
    if (role !== 'reconstruction') { okEqualsTruthPlacement = false; violations.push(`token ${eqTok} must appear only in reconstruction`) }
  }

  // Minimality: removing any of the three critical steps breaks invariants
  const breakIfRemoved = (removeId: string | undefined) => {
    if (!removeId) return false
    const rest = ids.filter(x => x !== removeId)
    const ro = rest.filter(id => has(id,'phase','savitarka') && has(id,'role','original')).length === 1
    const rj = rest.filter(id => has(id,'phase','nirvitarka') && has(id,'role','judgment')).length === 1
    const rr = rest.filter(id => has(id,'phase','nirvicara') && has(id,'role','reconstruction')).length === 1
    const rf = rest.filter(id => has(id,'phase','savicara') && has(id,'role','reflection')).length >= 1
    // Stage-order recompute (only if still present)
    let rok = true
    if (ro && rj && rf && rr) {
      const o2 = rest.find(id => has(id,'phase','savitarka') && has(id,'role','original'))!
      const j2 = rest.find(id => has(id,'phase','nirvitarka') && has(id,'role','judgment'))!
      const r2 = rest.find(id => has(id,'phase','nirvicara') && has(id,'role','reconstruction'))!
      const rs = rest.filter(id => has(id,'phase','savicara') && has(id,'role','reflection')).map(orderOf).filter(Number.isFinite) as number[]
      const oO = orderOf(o2)!, oJ = orderOf(j2)!, oR = orderOf(r2)!
      rok = oO < oJ && rs.every(o => oO < o && o < oR) && oJ < oR
    }
    // minimality is “true” if invariants fail after removal (i.e., not all ok)
    return !(ro && rj && rf && rr && rok)
  }

  const minimality = {
    removeOriginalBreaks: breakIfRemoved(originals[0]),
    removeJudgmentBreaks: breakIfRemoved(judgments[0]),
    removeReconstructBreaks: breakIfRemoved(reconstructs[0]),
  }

  const valid =
    okOriginal && okJudgment && okReflection && okReconstruct &&
    okStageOrder && okEqualsTruthPlacement &&
    minimality.removeOriginalBreaks && minimality.removeJudgmentBreaks && minimality.removeReconstructBreaks

  return {
    cycle,
    okOriginal, okJudgment, okReflection, okReconstruct,
    okStageOrder, okEqualsTruthPlacement,
    minimality,
    orders: { original: ordOriginal, judgment: ordJudgment, reflections: ordersRef, reconstruction: ordReconstruct },
    violations: valid ? [] : violations
  }
}

function main() {
  const file = process.argv[2]
  if (!file) {
    console.error('usage: tsx src/scripts/validate/method-proof.ts dist/datasets/dataset_science-of-logic-being.json')
    process.exit(2)
  }
  const art = load(file)
  const tags = collectTags(art)
  const tokensByHlo = collectTokensByHlo(art)

  // group HLOs by cycle
  const byCycle: Record<string, string[]> = {}
  for (const [hloId, t] of tags) {
    const cycle = t.cycle ?? 'UNSPECIFIED'
    ;(byCycle[cycle] ||= []).push(hloId)
  }

  const reports: CycleReport[] = []
  for (const [cycle, ids] of Object.entries(byCycle)) {
    reports.push(checkCycle(cycle, ids, tags, tokensByHlo))
  }

  const okAll = reports.every(r => r.violations.length === 0)
  console.log(JSON.stringify({ ok: okAll, reports }, null, 2))
  if (!okAll) process.exitCode = 1
}

main()
