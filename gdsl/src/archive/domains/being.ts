import { atom, rule, agg, v, type Atom, type Var } from '../core';
import { being as qualBeing } from './quality';

// Base predicates for Being
export const pure = (X: Var): Atom => atom('being:pure', [X]);
export const nothing = (X: Var): Atom => atom('being:nothing', [X]);
export const becoming = (X: Var): Atom => atom('being:becoming', [X]);

// Minimal rules scaffold (safe to compile; fill later)
export function beingRules() {
  const X = v('X');
  return [
    // Example: if X is in the quality domain at all, it “exists” there;
    // derive nothing/becoming as needed later. Keeping empty to avoid logic coupling.
    // rule(pure(X), [qualBeing(X)], { tags: ['being:pure:stub'] }),
  ];
}
