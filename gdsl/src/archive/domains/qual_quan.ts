import { atom, v, type Atom, type Var } from '../core'

// Quality (Being → Determinate Being → Being-for-self)
export const isa = (x: Var, T: Var | string): Atom => atom('isa', [x, typeof T === 'string' ? { kind:'sym', name:T } : T])
export const sub = (A: Var | string, B: Var | string): Atom =>
  atom('sub', [typeof A==='string'?{kind:'sym',name:A}:A, typeof B==='string'?{kind:'sym',name:B}:B])
export const hasProp = (x: Var, p: Var | string): Atom =>
  atom('hasProp', [x, typeof p==='string'?{kind:'sym',name:p}:p])
export const excludes = (p: Var | string, q: Var | string): Atom =>
  atom('excludes', [typeof p==='string'?{kind:'sym',name:p}:p, typeof q==='string'?{kind:'sym',name:q}:q])
export const opposes = (p: Var | string, q: Var | string): Atom =>
  atom('opposes', [typeof p==='string'?{kind:'sym',name:p}:p, typeof q==='string'?{kind:'sym',name:q}:q])
export const requires = (p: Var | string, q: Var | string): Atom =>
  atom('requires', [typeof p==='string'?{kind:'sym',name:p}:p, typeof q==='string'?{kind:'sym',name:q}:q])

// Quantity (magnitude, comparison, ratio, measure)
export const magnitude = (x: Var, m: Var): Atom => atom('magnitude', [x, m])
export const equal = (a: Var, b: Var): Atom => atom('equal', [a, b])
export const lessThan = (a: Var, b: Var): Atom => atom('lessThan', [a, b])
export const ratio = (a: Var, b: Var, r: Var): Atom => atom('ratio', [a, b, r])
export const measure = (x: Var, dim: Var | string, val: Var): Atom =>
  atom('measure', [x, typeof dim==='string'?{kind:'sym',name:dim}:dim, val])
