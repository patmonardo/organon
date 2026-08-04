# Graph Feature Grammar Formal (v1)

Date: 2026-08-04
Status: Speculative Formal Reference

This document is the formal layer for graph feature grammar in the Dataset doctrine.
It upgrades the working specification into explicit syntax, typing, derivation,
normalization, and validation obligations.

---

## Task Framing

Layer:

- Kernel: enforceable static and runtime checks.
- Agent: compilation from declarations to execution commitments.
- Logic: proof obligations for validity and admissibility.

Target:

- empirical adequacy
- conceptual validity
- rationale coherence

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

---

## 1. Formal Objects

Base sets:

- Stratum S = {Graph, Node, Edge}
- ValueType T = {Scalar, Vector, Symbolic, Distribution}
- Cardinality C = {One, Many}
- DerivationKind D = {Aggregate, Propagate, Compose, Infer}
- ViolationClass V = {ScopeCollapse, TypeCollapse, ProvenanceBreak}

Identifiers:

- feature names f in FeatureId
- grammar names g in GrammarId

Typed feature address:

feature address a := (s, f)
where s in S and f in FeatureId

---

## 2. Concrete Grammar Syntax

EBNF:

```text
GrammarDecl      ::= "grammar" Name Version? ":" ScopeDecl StrataDecl DeriveDecl? ProhibitDecl? NormDecl?
Version          ::= "@" Number

ScopeDecl        ::= "scope" ":" ScopeBody
ScopeBody        ::= "graph_id" "=" String "," "relations" "=" "[" RelKindList "]" "," "boundary" "=" BoundaryKind

StrataDecl       ::= "strata" ":" GraphBlock NodeBlock EdgeBlock
GraphBlock       ::= "Graph" ":" FeatureRule*
NodeBlock        ::= "Node" ":" FeatureRule*
EdgeBlock        ::= "Edge" ":" FeatureRule*

FeatureRule      ::= "-" FeatureName ":" TypeSpec RequirementSpec CardinalitySpec ConstraintSpec?
TypeSpec         ::= "type" "=" ValueType
RequirementSpec  ::= "required" "=" Bool
CardinalitySpec  ::= "cardinality" "=" Cardinality
ConstraintSpec   ::= "constraints" "=" "[" ConstraintList "]"

DeriveDecl       ::= "derive" ":" DeriveRule+
DeriveRule       ::= "-" Target "<-" DeriveOp "(" SourceList ")" TraceSpec?
Target           ::= Stratum "." FeatureName
SourceList       ::= Source ("," Source)*
Source           ::= Stratum "." FeatureName
DeriveOp         ::= DerivationKind
TraceSpec        ::= "trace" "=" Bool

ProhibitDecl     ::= "prohibit" ":" ProhibitRule+
ProhibitRule     ::= "-" Code ":" ViolationClass ":" String

NormDecl         ::= "norms" ":" NormRule+
NormRule         ::= "-" NormName ":" NormExpr
```

This syntax remains doctrine-level and is designed to be lowered into Dataset pipeline declarations.

---

## 3. Static Typing Contexts

Typing environments:

- Delta: declared feature addresses to (T, C)
- Sigma: model scope constraints
- Pi: provenance policy obligations

Judgments:

1. Feature rule well-formedness

Sigma ; Delta |- feature(s,f,t,c,constraints) ok

2. Derivation rule well-formedness

Sigma ; Delta ; Pi |- derive(srcs -> dst, kind, trace) ok

3. Grammar well-formedness

Sigma ; Delta ; Pi |- grammar g ok

---

## 4. Core Typing Rules

Rule T-Addr:

- If s in S and f is unique in stratum s, then (s,f) is a valid address.

Rule T-Feature:

- If address (s,f) is valid and t in T and c in C, then declaration of (s,f,t,c)
  extends Delta.

Rule T-NoDuplicate:

- For any fixed stratum s, no duplicate feature name f is allowed.

Rule T-Derive-Source:

- Every source address in a derivation must already exist in Delta.

Rule T-Derive-Target:

- Target address must exist in Delta or be declared in the same grammar block with unique identity.

Rule T-Derive-TypeCompat:

- Derivation operator kind constrains source and target types:
  - Aggregate: source may be Node or Edge; target must be Graph.
  - Propagate: source may be Graph or Edge; target must be Node or Edge.
  - Compose: source strata may mix; target type must be declared composition-compatible.
  - Infer: source/target compatibility delegated to logic constraints, but explicit norm rule is required.

Rule T-TraceRequired:

- If derivation crosses strata, trace=true is mandatory.

Rule T-GraphIrreducibility:

- No rule may define Graph target exclusively as untracked summary columns in Node/Edge artifacts.
  A Graph-stratum address must remain declared at Graph stratum.

Rule T-ProhibitionSound:

- Each prohibition code must correspond to a syntactically detectable violation pattern.

---

## 5. Dynamic Semantics (Compilation-Oriented)

Let compile(g) produce a rule graph R and commitments K.

R contains:

- typed feature nodes
- derivation edges
- prohibition guards
- norm obligations

K contains:

- FeatureMap commitments
- ProgramPlan commitments
- ProgramImage checks

Operational intent:

1. parse(g) -> AST
2. typecheck(AST, Sigma, Delta, Pi) -> TypedAST | Error
3. normalize(TypedAST) -> NormalForm
4. lower(NormalForm) -> (R, K)
5. emit(K) -> artifact commitments

---

## 6. Normal Form

A grammar is in normal form iff:

1. All feature declarations are explicitly stratum-qualified.
2. All derivations are explicit and acyclic in declaration order.
3. Cross-strata derivations carry trace=true.
4. All prohibition rules reference extant declaration symbols.
5. All norm rules reference extant declaration symbols.

Normalization rewrite obligations:

- implicit stratum references are expanded
- shorthand type aliases are resolved
- default trace policy is materialized as explicit booleans

---

## 7. Safety Properties

P1. Strata Separation Safety

- Well-typed grammars preserve Graph/Node/Edge separation at declaration level.

P2. Trace Continuity Safety

- Any accepted cross-strata derivation yields explicit trace obligations in commitments.

P3. Prohibition Detectability Safety

- Every prohibition can be checked by syntactic or typed rule graph inspection.

P4. Deterministic Replay Safety

- For fixed input grammar text and fixed normalization version, compile(g) is deterministic.

---

## 8. Minimal Soundness Obligations

For promotion beyond speculative status, implementations must provide evidence for:

1. Preservation

- If grammar g is well-typed, lowering preserves typing invariants in produced commitments.

2. Progress

- If grammar g is well-typed and normalizable, lowering either emits commitments or returns classified errors.

3. Irreducibility

- Graph-stratum declarations are not erased or demoted during lowering.

4. Provenance binding

- Cross-strata derivations imply provenance edges in emitted commitments.

---

## 9. Dataset Artifact Mapping Contract

Lowering contract:

- Delta -> FeatureMap artifact rows
- derivation edges -> ProgramPlan derivation rows
- prohibition and norm checks -> ProgramImage validation rows
- trace obligations -> provenance fields in SemanticSubgraph/Table outputs

No alternative opaque output container is permitted in v1.

---

## 10. Conformance Test Matrix (Doctrine-Level)

T1. Duplicate feature rejection

- same stratum, same feature name -> fail with TypeCollapse class

T2. Cross-strata trace enforcement

- Graph <- Aggregate(Node.x) with trace=false -> fail with ProvenanceBreak class

T3. Graph irreducibility enforcement

- declaring Graph density only as Node summary alias -> fail with ScopeCollapse class

T4. Deterministic replay

- compile same grammar twice -> identical rule graph digest

T5. Valid baseline acceptance

- well-typed mini grammar -> accepted with commitments emitted

---

## 11. Relation to Current Naming

This formal spec is compatible with both names:

- current: GraphFeatureGrammarForm
- generalized family name: GraphGrammarForm

Rename is lexical and does not change the formal content.

---

## 12. Practical Next Move

Implement one parser/checker slice that covers:

1. strata declarations
2. typed feature declarations
3. one Aggregate derivation with trace policy
4. one prohibition check

Then run focused tests before extending operator coverage.
