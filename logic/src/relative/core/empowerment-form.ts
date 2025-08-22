import {
  Empowerment,
  EmpowermentSchema,
  createRootEmpowerment,
} from '../../schema/empowerment';

export class FormEmpowerment {
  constructor(private readonly doc: Empowerment) {}

  static fromSchema(doc: Empowerment): FormEmpowerment {
    return new FormEmpowerment(EmpowermentSchema.parse(doc));
  }

  static createRoot(): FormEmpowerment {
    return new FormEmpowerment(createRootEmpowerment());
  }

  get id(): string {
    return this.doc.id;
  }

  get subject(): string {
    return this.doc.subject;
  }

  // returns numeric score for this token (weight * jnana or confidence)
  score(): number {
    const j = this.doc.jnana ?? this.doc.confidence ?? 1;
    return (this.doc.weight ?? 0) * j;
  }

  // Check whether this empowerment allows the given action in the given scope
  check(
    action: string,
    scope?: string,
  ): { allowed: boolean; score: number; reason?: string } {
    if (!this.doc.actions || this.doc.actions.length === 0) {
      return { allowed: false, score: 0, reason: 'no-actions' };
    }

    const allowsAction =
      this.doc.actions.includes('*') || this.doc.actions.includes(action);
    if (!allowsAction)
      return { allowed: false, score: 0, reason: 'action-not-allowed' };

    if (scope && this.doc.scope && this.doc.scope.length > 0) {
      const allowsScope =
        this.doc.scope.includes('*') || this.doc.scope.includes(scope);
      if (!allowsScope)
        return { allowed: false, score: 0, reason: 'scope-mismatch' };
    }

    return { allowed: true, score: this.score() };
  }

  toSchema(): Empowerment {
    return this.doc;
  }

  // Combine multiple FormEmpowerment into a single aggregated score and trace
  static combine(list: FormEmpowerment[], root?: FormEmpowerment) {
    const tokens = [...(list || [])];
    if (root) tokens.push(root);

    const scores = tokens.map((t) => ({
      id: t.id,
      subject: t.subject,
      score: t.score(),
      provenance: (t.toSchema() as any).provenance,
    }));
    const total = scores.reduce((s, v) => s + v.score, 0);
    return { total, scores };
  }
}

export default FormEmpowerment;
