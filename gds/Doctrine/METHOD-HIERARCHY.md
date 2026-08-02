# Doctrine Method Hierarchy

Purpose: define one canonical hierarchy for Doctrine-driven learning and platform design.

## Canonical Hierarchy

```text
Doctrine
  -> References
  -> Exemplars
  -> Examples
  -> Fixtures
  -> Workbench
```

This is the authoritative order of explanation and execution.

Read as a software-design topic map, this hierarchy also defines the developer knowledge base of the platform.

It also defines the rationale for a Doctrine First Methodology, whose operational name is the Doctrinal Method.

## Layer Meanings

1. Doctrine

- Location: `gds/Doctrine/`
- Role: constitutional claims, method law, curriculum authority.

2. References

- Location: `gds/Doctrine/REFERENCES/`
- Role: stable definitions and concept boundaries.
- Rule: define, do not narrate.

3. Exemplars

- Location: `gds/Doctrine/EXEMPLARS/`
- Role: canonical teaching sequence by doctrinal moment.
- Rule: one main principle per exemplar.

4. Examples

- Location: `gds/examples/`
- Role: executable realization of exemplar claims.
- Rule: runtime evidence over rhetorical claims.

5. Fixtures

- Location: `gds/fixtures/collections/**`
- Role: persisted evidence and replay artifacts.
- Rule: fixtures are evidence, not replacement doctrine.

6. Workbench

- Locations: `gds/Doctrine/workbench.sh`, `gds/Doctrine/WORKBENCH.md`
- Role: notebook-like runnable curriculum for researchers.
- Rule: every workbench run should produce inspectable fixture evidence.

## Knowledge Base Reading

Doctrine is not merely documentation.
It is a developer reference knowledge base with executable capabilities.

That means each layer answers a different developer question:

- Doctrine: what is the law of the system?
- References: what does this term mean?
- Exemplars: how is the law taught in sequence?
- Examples: what actually runs?
- Fixtures: what evidence was produced?
- Workbench: how do I study or replay the path as a coherent session?

The method becomes more powerful as these answers remain joined under one root.

## Doctrine First Methodology

The Doctrine First Methodology means:

- the system states its law before it merely exposes APIs
- the system teaches its sequence before it merely offers features
- the system binds runnable evidence to principled explanation

The Doctrinal Method is this methodology in action through the hierarchy:

```text
Doctrine -> Reference -> Exemplar -> Example -> Fixture -> Workbench
```

The method is "first" not because execution is secondary in value, but because execution should be intelligible through doctrine rather than opaque to it.

## Closed Specification Mode

When the platform enters closed specification mode, the Doctrinal Method becomes stricter rather than weaker.

Closed specification mode means:

- begin from the end-state Principle rather than from open option generation
- constrain work to named contracts and evidence-bearing surfaces
- refuse speculative widening when Doctrine already fixes the direction
- demand that implementation changes remain legible through examples, fixtures, and workbench outputs

This is especially important for the GDS Kernel, where cost pressure and architectural maturity require convergence more than exploration.

## IDE Direction

In the longer arc, this hierarchy is the conceptual spine of a general IDE for building rational agents.

The preferred name for that surface is simply **Doctrine**: the Rational Agent IDE app.
`OG Script` is the shorthand name for the internal DSL that this IDE should foreground.

OG Script begins from Shell, but it exceeds Shell in scope. Its priority order should be read as:

- Dataset Builder: authoring surface for DataFrame -> Dataset -> TaskFrame construction
- Domain Model Design Surface: explicit language for principled model design, fit discipline, and rationale tracking
- Shell Script Builder: readable control surface for mediated execution
- ToolChain Coordinator: orchestration of examples, procedures, evaluation, and runtime assets

All four roles should be materially grounded in Corpus, LM, and Logic artifacts. OG Script should therefore be readable not merely as command syntax, but as the coordination language of those artifacts.

The intended progression is:

```text
developer knowledge base -> executable workbench -> server-side learning app -> Doctrine
```

The IDE idea should not break the hierarchy. It should render the same hierarchy more directly, with OG Script functioning as the readable authoring surface over Doctrine, Corpus/LM/Logic artifacts, Examples, Fixtures, and Workbench sessions.

## Mediation Law

For current kernel practice, the canonical runtime mediation is:

```text
DataFrame -> TaskFrame through shell_ -> proc_ -> eval_
```

This mediation is expected to be visible at artifact level in the workbench and fixture roots.

## Invariant Checks

All hierarchy levels must preserve:

- empirical adequacy
- reflexive consistency
- dialectical mediation

## Promotion Rule (How Something Becomes Canonical)

1. Draft explanation starts in exemplar work.
2. Runnable example proves claim.
3. Fixture captures evidence.
4. Workbench integrates into researcher path.
5. Reference updates stabilize vocabulary.
6. Doctrine index/README links become canonical entry.

## Anti-Drift Rules

- Do not let examples evolve without exemplar/reference updates.
- Do not let references drift into speculative notebook prose.
- Do not let fixture manifests replace principled explanation.
- Do not add alternate hierarchies in parallel docs.
