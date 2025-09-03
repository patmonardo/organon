import type {
  Program,
  Fact,
  Rule,
  Atom,
  Literal,
  Builtin,
  Aggregate,
  Term,
  Num,
  Str,
  Sym,
  Var,
} from '../core';
import { stratify } from './stratify';
import type { Neg } from '../core' // optional, for completeness

type Tup = (string | number)[];
type Rel = { schema: number; rows: Set<string> }; // serialized tuples as JSON
export interface Database {
  rels: Map<string, Rel>;
  trace: { rule: string; newTuples: number }[];
}

function key(t: Tup): string {
  return JSON.stringify(t);
}
function add(db: Database, pred: string, t: Tup): boolean {
  if (!db.rels.has(pred))
    db.rels.set(pred, { schema: t.length, rows: new Set() });
  const rel = db.rels.get(pred)!;
  if (rel.schema !== t.length) throw new Error(`Arity mismatch for ${pred}`);
  const k = key(t);
  const had = rel.rows.size;
  rel.rows.add(k);
  return rel.rows.size > had;
}

function isVar(t: Term): t is Var {
  return (t as any).kind === 'var';
}
function groundNonVar(t: Exclude<Term, Var>): string | number {
  switch (t.kind) {
    case 'num': return (t as Num).value
    case 'str': return (t as Str).value
    case 'sym': return (t as Sym).name
  }
}
// Keep for builtins (may receive vars)
function groundTerm(
  t: Term,
  env: Record<string, string | number>,
): string | number {
  if (isVar(t)) return env[t.name] as any
  return groundNonVar(t as Exclude<Term, Var>)
}

// New: matchAtom implementation (used throughout)
function matchAtom(
  db: Database,
  a: Atom,
  env: Record<string, string | number> = {},
): Record<string, string | number>[] {
  const rel = db.rels.get(a.pred);
  if (!rel) return [];
  const out: Record<string, string | number>[] = [];
  for (const row of rel.rows) {
    const tup = JSON.parse(row) as Tup;
    const trial = { ...env };
    let ok = true;
    for (let i = 0; i < a.args.length; i++) {
      const arg = a.args[i];
      if (isVar(arg)) {
        const name = arg.name;
        if (trial[name] === undefined) trial[name] = tup[i];
        else if (trial[name] !== tup[i]) { ok = false; break; }
      } else {
        const val = groundNonVar(arg as Exclude<Term, Var>);
        if (val !== tup[i]) { ok = false; break; }
      }
    }
    if (ok) out.push(trial);
  }
  return out;
}

function evalBuiltin(b: Builtin, env: Record<string, string | number>) {
  const vals = b.args.map((a) => groundTerm(a, env));
  const [x, y] = vals as [any, any];
  switch (b.op) {
    case 'eq':
      return x === y;
    case 'neq':
      return x !== y;
    case 'lt':
      return x < y;
    case 'le':
      return x <= y;
    case 'gt':
      return x > y;
    case 'ge':
      return x >= y;
    case 'add':
      env[(b.args[2] as Var).name] = (x as number) + (y as number);
      return true;
    case 'sub':
      env[(b.args[2] as Var).name] = (x as number) - (y as number);
      return true;
    case 'mul':
      env[(b.args[2] as Var).name] = (x as number) * (y as number);
      return true;
    case 'div':
      env[(b.args[2] as Var).name] = (x as number) / (y as number);
      return true;
    default:
      return false;
  }
}

function evalAgg(
  db: Database,
  a: Aggregate,
  seeds: Record<string, string | number>[],
) {
  // Group by vars in a.by; iterate over matching tuples of a.over
  const over = a.over;
  const rel = db.rels.get(over.pred);
  if (!rel) return [];
  const out: Record<string, string | number>[] = [];
  const gb = (a.by ?? []).map((v) => (v as Var).name);
  const groups = new Map<
    string,
    { keyEnv: Record<string, any>; rows: Tup[] }
  >();
  for (const row of rel.rows) {
    const tup = JSON.parse(row) as Tup;
    // project env for grouping
    const env: Record<string, any> = {};
    for (let i = 0; i < over.args.length; i++) {
      const arg = over.args[i];
      if (arg.kind === 'var') env[(arg as Var).name] = tup[i];
    }
    const gkey = JSON.stringify(gb.map((n) => env[n]));
    if (!groups.has(gkey))
      groups.set(gkey, {
        keyEnv: Object.fromEntries(gb.map((n) => [n, env[n]])),
        rows: [],
      });
    groups.get(gkey)!.rows.push(tup);
  }
  for (const g of groups.values()) {
    const env = { ...g.keyEnv };
    const values: number[] = [];
    if (a.value) {
      const name = (a.value as Var).name;
      for (const row of g.rows) {
        const envRow: Record<string, any> = {};
        for (let i = 0; i < over.args.length; i++) {
          const arg = over.args[i];
          if (arg.kind === 'var') envRow[(arg as Var).name] = row[i];
        }
        values.push(envRow[name]);
      }
    } else {
      values.push(...g.rows.map(() => 1));
    }
    let agg: number;
    switch (a.fun) {
      case 'count':
        agg = values.length;
        break;
      case 'sum':
        agg = values.reduce((s, v) => s + v, 0);
        break;
      case 'min':
        agg = Math.min(...values);
        break;
      case 'max':
        agg = Math.max(...values);
        break;
      case 'avg':
        agg = values.reduce((s, v) => s + v, 0) / (values.length || 1);
        break;
    }
    env[(a.into as Var).name] = agg!;
    out.push(env);
  }
  return out;
}

export function evaluate(p: Program): Database {
  const db: Database = { rels: new Map(), trace: [] };
  // seed facts
  for (const f of p.facts) {
    const tup = f.atom.args.map((a) =>
      (a as any).kind === 'var'
        ? `_${(a as Var).name}`
        : (a as any).name ?? (a as any).value,
    );
    add(db, f.atom.pred, tup);
  }
  const { strata } = stratify(p);
  // naive per-stratum saturation
  for (const s of strata) {
    let added = 0;
    do {
      added = 0;
      for (const r of p.rules) {
        if (!s.includes(r.head.pred)) continue;
        // enumerate environments
        let envs: Record<string, any>[] = [{}];
        for (const lit of r.body) {
          if ((lit as any).pred) {
            const a = lit as Atom;
            const next: Record<string, any>[] = [];
            for (const e of envs) next.push(...matchAtom(db, a, e));
            envs = next;
          } else if ((lit as any).kind === 'neg') {
            const a = (lit as any).atom as Atom;
            envs = envs.filter((e) => matchAtom(db, a, e).length === 0);
          } else if ((lit as any).kind === 'builtin') {
            envs = envs.filter((e) => evalBuiltin(lit as Builtin, e));
          } else if ((lit as any).kind === 'agg') {
            const rows = evalAgg(db, lit as Aggregate, envs);
            envs = rows;
          }
        }
        for (const e of envs) {
          const tup = r.head.args.map((a) => {
            if (a.kind === 'var') return e[(a as Var).name];
            return (a as any).name ?? (a as any).value;
          }) as Tup;
          if (add(db, r.head.pred, tup)) added++;
        }
        if (added) db.trace.push({ rule: r.head.pred, newTuples: added });
      }
    } while (added > 0);
  }
  return db;
}
