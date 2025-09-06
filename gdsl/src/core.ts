export interface Num {
  kind: 'num';
  value: number;
}
export interface Str {
  kind: 'str';
  value: string;
}
export interface Sym {
  kind: 'sym';
  name: string;
}
export interface Var {
  kind: 'var';
  name: string;
}

export type Term = Num | Str | Sym | Var;

export interface Atom {
  pred: string;
  args: Term[];
}
export interface Neg {
  kind: 'neg';
  atom: Atom;
}
export type BuiltinOp =
  | 'eq'
  | 'neq'
  | 'lt'
  | 'le'
  | 'gt'
  | 'ge'
  | 'add'
  | 'sub'
  | 'mul'
  | 'div';
export interface Builtin {
  kind: 'builtin';
  op: BuiltinOp;
  args: Term[];
}
export type AggFun = 'count' | 'sum' | 'min' | 'max' | 'avg';
export interface Aggregate {
  kind: 'agg';
  fun: AggFun;
  over: Atom;
  into: Var;
  by?: Var[];
  value?: Var;
}
export type Literal = Atom | Neg | Builtin | Aggregate;

export interface Rule {
  head: Atom;
  body: Literal[];
  // allow metadata like { tags: [...] }
  tags?: string[];
  [k: string]: unknown;
}
export interface Fact {
  atom: Atom;
  tags?: string[];
  [k: string]: unknown;
}
export interface Program {
  facts: Fact[];
  rules: Rule[];
  meta?: { tags?: string[]; [k: string]: unknown };
}

// Helpers
export const num = (value: number): Num => ({ kind: 'num', value });
export const str = (value: string): Str => ({ kind: 'str', value });
export const sym = (name: string): Sym => ({ kind: 'sym', name });
export const v = (name: string): Var => ({ kind: 'var', name });
export const atom = (pred: string, args: Term[]): Atom => ({ pred, args });
export const neg = (a: Atom): Neg => ({ kind: 'neg', atom: a });
export const bi = (op: BuiltinOp, args: Term[]): Builtin => ({
  kind: 'builtin',
  op,
  args,
});

// Match current call sites: agg('count', N, quality(X,Q), undefined, [X])
export const agg = (
  fun: AggFun,
  into: Var,
  over: Atom,
  value?: Var,
  by?: Var[],
): Aggregate => ({
  kind: 'agg',
  fun,
  over,
  into,
  by,
  value,
});

// Accept optional options (e.g., { tags: [...] })
export const rule = (
  head: Atom,
  body: Literal[],
  opts?: Partial<Rule>,
): Rule => ({ head, body, ...(opts ?? {}) });
// New: fact helper (accepts optional metadata like tags)
export const fact = (a: Atom, opts?: Partial<Fact>): Fact => ({
  atom: a,
  ...(opts ?? {}),
});

// Validation
export interface ValidationIssue {
  kind: 'error' | 'warn';
  msg: string;
  where?: string;
}

function arityOf(a: Atom): number {
  return a.args.length;
}

export function validate(p: Program): ValidationIssue[] {
  const issues: ValidationIssue[] = [];
  const sig = new Map<string, number>();
  const noteSig = (pred: string, arity: number, where: string) => {
    const prev = sig.get(pred);
    if (prev === undefined) sig.set(pred, arity);
    else if (prev !== arity) {
      issues.push({
        kind: 'error',
        msg: `Arity mismatch for ${pred}: saw ${arity}, expected ${prev}`,
        where,
      });
    }
  };

  // collect signatures from facts
  p.facts?.forEach((f, i) => {
    noteSig(f.atom.pred, arityOf(f.atom), `fact[${i}]`);
  });

  const varsInTerm = (t: Term, out: Set<string>) => {
    if ((t as any).kind === 'var') out.add((t as Var).name);
  };
  const varsInAtom = (a: Atom, out: Set<string>) =>
    a.args.forEach((t) => varsInTerm(t, out));

  // scan rules
  p.rules?.forEach((r, i) => {
    const where = `rule[${i}]`;
    // head signature
    noteSig(r.head.pred, arityOf(r.head), `${where}.head`);

    // collect vars
    const headVars = new Set<string>();
    const bodyVars = new Set<string>();
    varsInAtom(r.head, headVars);

    r.body.forEach((lit, j) => {
      const lwhere = `${where}.body[${j}]`;
      // atoms
      if (!('kind' in (lit as any))) {
        // treat as Atom
        const a = lit as Atom;
        noteSig(a.pred, arityOf(a), lwhere);
        varsInAtom(a, bodyVars);
        return;
      }
      switch ((lit as any).kind) {
        case 'builtin': {
          const b = lit as Builtin;
          // simple arity checks
          const k2 = new Set<BuiltinOp>(['eq', 'neq', 'lt', 'le', 'gt', 'ge']);
          const k3 = new Set<BuiltinOp>(['add', 'sub', 'mul', 'div']);
          if (k2.has(b.op) && b.args.length !== 2) {
            issues.push({
              kind: 'error',
              msg: `Builtin ${b.op} expects 2 args, got ${b.args.length}`,
              where: lwhere,
            });
          }
          if (k3.has(b.op) && b.args.length !== 3) {
            issues.push({
              kind: 'error',
              msg: `Builtin ${b.op} expects 3 args, got ${b.args.length}`,
              where: lwhere,
            });
          }
          b.args.forEach((t) => varsInTerm(t, bodyVars));
          break;
        }
        case 'neg': {
          const n = lit as Neg;
          noteSig(n.atom.pred, arityOf(n.atom), `${lwhere}.neg`);
          varsInAtom(n.atom, bodyVars);
          break;
        }
        case 'agg': {
          const a = lit as Aggregate;
          noteSig(a.over.pred, arityOf(a.over), `${lwhere}.agg.over`);
          varsInAtom(a.over, bodyVars);
          // into/by/value must be vars if present
          if ((a.into as any).kind !== 'var') {
            issues.push({
              kind: 'error',
              msg: `Aggregate.into must be a Var`,
              where: lwhere,
            });
          } else {
            bodyVars.add((a.into as Var).name);
          }
          a.by?.forEach((v) => bodyVars.add(v.name));
          if (a.value && (a.value as any).kind === 'var')
            bodyVars.add((a.value as Var).name);
          break;
        }
        default: {
          // treat unknown as Atom for robustness
          const a = lit as unknown as Atom;
          if (a && typeof a.pred === 'string' && Array.isArray(a.args)) {
            noteSig(a.pred, arityOf(a), lwhere);
            varsInAtom(a, bodyVars);
          }
        }
      }
    });

    // range restriction: head vars must appear in body
    headVars.forEach((vn) => {
      if (!bodyVars.has(vn)) {
        issues.push({
          kind: 'error',
          msg: `Head variable ${vn} does not appear in body (range restriction)`,
          where,
        });
      }
    });
  });

  return issues;
}
