# Dataset Tree DSL Design (Rust Way, Polars Style)

Date: 2026-02-08

## Goal

Define a Tree-centric Dataset API that is Rust-native, expression-driven, and scalable. The Tree data type is a first-class collection, with a dedicated Namespace (public DSL), an Expr layer (composable AST), and a Functional layer (pure transforms). This mirrors Polars: user-facing namespace builders that create expressions, and runtime evaluation on concrete data.

This note focuses on Tree. The same structure will apply to Feature (type-dict) and to Text/Tokens/Annotations.

## Core Principles

- Tree is a dataset-native collection, not a derived view.
- Namespace API is thin and only constructs expressions.
- Expr is lazy and composable; it does not mutate data.
- Functions are pure, deterministic, and operate on concrete trees.
- Storage is stable and supports IDs, spans, and positional addressing.

## Structure Overview

- Namespace: tree::ns
- Expr: tree::expr
- Functions: tree::fn
- Entry point: tree.rs (re-exports and macro integration only)

## Tree Data Model

Minimal, stable model that supports NLTK-style functionality while mapping cleanly to GDS datasets.

- Tree
  - label: any label type (string or symbol)
  - children: ordered list of leaf or subtree
  - optional metadata: span, id

- TreePos
  - path of indices for descendant addressing
  - used by Expr and for alignment

- TreeSpan
  - token span (start, end)
  - optional byte span for raw text alignment

- Variants
  - ParentedTree: single parent pointer
  - MultiParentedTree: shared subtrees (DAG)
  - ImmutableTree: hashable and cache-friendly

## Namespace Layer (tree::ns)

Purpose: public API surface, small and expressive. It only builds TreeExpr.

Examples of intended API:

- tree::ns::TreeNs::node(label, children)
- tree::ns::TreeNs::leaf(value)
- tree::ns::TreeNs::pos(path)
- tree::ns::TreeNs::span(start, end)
- tree::ns::TreeNs::from_brackets(str)
- tree::ns::TreeNs::transform(expr, op)

This is equivalent to Polars Expr builders and is always lazy.

## Expr Layer (tree::expr)

Purpose: composable AST for tree operations. Used for query planning and lazy evaluation.

Proposed core enum:

- TreeExpr::Node(label, children_exprs)
- TreeExpr::Leaf(value)
- TreeExpr::Path(tree_ref, TreePos)
- TreeExpr::Span(tree_ref, TreeSpan)
- TreeExpr::Transform(tree_ref, TreeOp)
- TreeExpr::Map(tree_ref, func)
- TreeExpr::Filter(tree_ref, predicate)

TreeRef is a reference to a dataset tree column or a temporary tree value in an expression pipeline.

## Functional Layer (tree::fn)

Purpose: pure transforms on concrete trees. These are reusable by Expr evaluation and direct calls.

- map_nodes(tree, f)
- filter_nodes(tree, p)
- fold(tree, acc, f)
- normalize(tree)
- collapse_unary(tree, opts)
- cnf(tree, opts)
- align_tokens(tree, tokens)
- project_graph(tree, opts)

These functions do not depend on dataset storage or runtime.

## Macro DSL (Scheme-Python Client)

Macros allow concise tree specifications and are a top-level UX feature. They expand into TreeExpr constructors.

Suggested macros:

- tree! { label => [child1, child2, ...] }
- tleaf!("Mary")
- tpos!(0, 1, 0)
- tspan!(3, 7)

Macro output is always TreeExpr, not a concrete Tree, preserving lazy semantics.

## Dataset Integration

TreeCollection is a dataset column type with stable IDs and optional indices.

- TreeCollection
  - storage: tree nodes in a compact arena or adjacency table
  - ids: stable TreeId for linking to features and graph projection
  - optional indices: by TreePos or TreeSpan

Dataset semantics:

- select / filter / transform build TreeExpr pipelines
- materialize evaluates TreeExpr to concrete TreeCollection

## Alignment With NLTK

- Tree positions map to TreePos addressing
- Parented and MultiParented variants map to dataset-level constraints
- Immutable trees support hashing and caching
- Bracketed string parsing is supported as a Namespace builder
- Tree transforms (CNF, unary collapse) live in tree::fn and are used by Expr

## Next Steps

1. Draft the module layout and type signatures in Rust.
2. Implement the TreeExpr enum and TreeNs builders.
3. Implement minimal TreeCollection storage with TreeId and TreePos.
4. Add macro expansion tests for tree!, tpos!, tspan!.
5. Apply this same pattern to Feature, then Text/Tokens/Annotations.
