import FormEmpowerment from './empowerment-form';

export type EmpowermentCombineResult = {
  total: number;
  scores: Array<{ id: string; subject: string; score: number; provenance?: string }>;
};

export function combineEmpowerments(tokens: FormEmpowerment[], root?: FormEmpowerment): EmpowermentCombineResult {
  // simple algorithm: sum the token scores; include root as fallback
  return FormEmpowerment.combine(tokens, root);
}

export default { combineEmpowerments };
