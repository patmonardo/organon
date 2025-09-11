export interface GdslOperatorSpec {
  name: string
  arity: number | 'var'
  doc: string
  category: 'structural' | 'process' | 'meta' | 'annotation'
}

export const GDSL_OPERATORS: GdslOperatorSpec[] = [
  { name: 'tag', arity: 1, doc: "tag('k','v') or tag(pair) — annotate.", category: 'annotation' },
  { name: 'stance', arity: 1, doc: "stance(subject := relation) — positional assertion.", category: 'structural' },
  { name: 'principle', arity: 1, doc: "principle(X := ground(...)) — generative source.", category: 'structural' },
  { name: 'mix', arity: 'var', doc: 'mix(a,b,...) — entangled composite.', category: 'process' },
  { name: 'organic', arity: 1, doc: 'organic(X) — internal dual generation.', category: 'meta' },
  { name: 'simultaneous', arity: 'var', doc: 'simultaneous(a,b,...) — co-origination.', category: 'structural' },
  { name: 'internalGeneration', arity: 1, doc: 'internalGeneration(expr) — produced from within.', category: 'process' },
  { name: 'construct', arity: 1, doc: 'construct(X := expr) — built composite.', category: 'process' },
  { name: 'present', arity: 1, doc: 'present(given → faculty)', category: 'process' },
  { name: 'respond', arity: 1, doc: 'respond(faculty(given) ⇒ result)', category: 'process' },
  { name: 'purify', arity: 1, doc: 'purify(expr) — removal trajectory.', category: 'process' },
  { name: 'disentangle', arity: 1, doc: 'disentangle(set(...)) — factor roles.', category: 'process' },
  { name: 'path', arity: 1, doc: 'path(a → b) — directed transformation.', category: 'structural' },
  { name: 'warn', arity: 1, doc: 'warn(condition) — caution marker.', category: 'meta' },
  { name: 'require', arity: 1, doc: 'require(condition) — necessity.', category: 'meta' },
  { name: 'negate', arity: 1, doc: 'negate(expr) — logical reversal.', category: 'process' },
  { name: 'elevation', arity: 1, doc: 'elevation(from → higher)', category: 'process' },
  { name: 'ladder', arity: 'var', doc: 'ladder(a1,a2,...) — ordered principle chain.', category: 'structural' },
  { name: 'closure', arity: 1, doc: 'closure(H) — terminal principle.', category: 'meta' },
  { name: 'reciprocity', arity: 1, doc: 'reciprocity(a ↔ b)', category: 'structural' },
  { name: 'mode', arity: 1, doc: 'mode(label := spec)', category: 'meta' },
  { name: 'chain', arity: 1, doc: 'chain(appearance^n)', category: 'process' },
  { name: 'reIntegration', arity: 1, doc: 'reIntegration(expr)', category: 'process' },
]
export const GDSL_OPERATOR_SET = new Set(GDSL_OPERATORS.map(o => o.name))
