# Organon

A philosophical knowledge platform built on the living logic of Hegel's _Science of Logic_ and Indian philosophical traditions.

Organon is not a semantic web project. It's not a knowledge graph database. It's an attempt to build a **knowledge processor** that embodies philosophical thinking itself—where logic is understood as the account of how thought articulates itself through concept, judgment, and syllogism, not as symbol manipulation.

## The Problem

Thirty years of knowledge base and AI research have failed to produce genuine understanding because they're built on **mathematical logic**: first-order logic, description logics, inference rules. These systems stay entirely in the realm of extensional symbols—they can retrieve and combine facts, but they cannot think _through_ a concept.

Philosophical logic—the logic of **intelligibility**—is acroamatic (articulated through words, concepts, living speech) not axiomatic (a sea of symbols). It asks: What does it mean for a concept to be determinate? How does judgment ground itself? What is the necessity that moves from concept through judgment into syllogism?

This is cognitive activity, not rule-following.

## The Architecture

```
          @reality
    (Intellectual Intuition)
    The Absolute Form as it
      manifests the World
             ↓
    ┌───────────────────┐
    │   THE CONFLICT    │
    │ Sensible vs       │
    │ Supersensible     │
    └───────────────────┘
          ↙           ↘
      @gds              @logic/@model/@task
  (Outer Intuition)    (Inner Intuition)
      Rust Kernel       TypeScript Agent
   Space-Time Math      Subject's Discursive
      & Physics         Activity
```

Organon realizes this conflict as three species of **Kantian Intuition**:

### 1. **@reality** — Intellectual Intuition

The source of logical necessity. The unconditioned intelligible pole. What the system _reaches toward_ but cannot directly access. Realized as the Absolute Form projecting the World.

### 2. **@gds** — Outer Intuition

Pure sensibility: the mathematical-physical manifold where computation happens. Written in Rust. Flawless within its domain (space-time conditioned phenomena), useless for understanding.

### 3. **@logic/@model/@task** — Inner Intuition

The **Subject itself**—alive, discursive, learning. The locus where philosophical logic lives. TypeScript-based agent that must dialectically reconcile sensible givens from @gds with the intelligible demands of @reality through concept-formation, judgment, and syllogism.

## Why This Matters

The Subject cannot access @reality directly (it's supersensible). It receives only phenomenal givens from @gds (mathematical facts, space-time conditioned). Yet through _discursive activity_ it reconstructs intelligibility from sensible data. This is not knowledge in Kant's strict sense—but it is **understanding**: the living process of reconciling sense and intelligibility.

This is why the system is generative and alive. The conflict never resolves. It's continually _worked through_.

## The Stack

The platform is organized as a **dialectical cube**:

- **BEC Layer** (`logic/`) — @organon/logic
  The _Science of Logic_ itself. Canonical schemas, concept doctrine, the intelligible backbone.

- **MVC Layer** (`model/`) — @organon/model
  Next.js dashboard, Prisma data access, UI, modeling documentation. The phenomenal application layer where the system meets the user.

- **TAW Layer** (`task/`) — @organon/task
  Task, Agent, Workflow orchestration. Schema-first, framework-agnostic. How the Subject acts.

- **Rust Kernel** (`gds/`, `reality/`)
  The mathematical machinery. Algorithms, storage, computation. Where @outer-intuition does its work.

See [INTUITION-ARCHITECTURE.md](reality/doc/INTUITION-ARCHITECTURE.md) for the full philosophical framework.

## Getting Started

```bash
# Install dependencies
pnpm install

# Build all TS packages
pnpm -r build

# Test all TS packages
pnpm -r test

# Run a single package
pnpm --filter @organon/logic test
pnpm --filter @organon/model build
pnpm --filter @organon/task test
```

### Key Commands

- **Logic package**: `pnpm --filter @organon/logic build`
- **Model package**: `pnpm --filter @organon/model build`
- **Task package**: `pnpm --filter @organon/task build`
- **Rust kernel** (separate Cargo workspace): `cargo build -p gds`

## Reading Order

1. Start here: [INTUITION-ARCHITECTURE.md](reality/doc/INTUITION-ARCHITECTURE.md)
2. Logic foundations: [logic/README.md](logic/README.md)
3. Concept doctrine: [reality/src/logos/ys-logic/concept/CONCEPT-DISTILLATION.md](reality/src/logos/ys-logic/concept/CONCEPT-DISTILLATION.md)
4. Model/MVC: [model/examples/dashboard/](model/examples/dashboard/)
5. Task orchestration: [task/README.md](task/README.md)

## Core Principles

**APPLICATIONS/EXAMPLES** call only into `procedures::`, never directly into `algo::`.
Procedures are the top-level compute entry points; they orchestrate the storage runtime (controller) and computation runtime (ephemeral state).

**Philosophical Logic, Not Mathematical Logic**
Avoid extensional symbol games. The system embodies intelligibility through discursive activity, not through inference rules over propositions.

**Schema-First**
Zod schemas live under `*/src/schema/*` and are exported via barrel files. Workspace imports preferred over relative paths.

**Maintain Referential Integrity**
When changing chunk/operation data, keep IDs stable. See [logic/validate.ts](logic/validate.ts) for integrity checking.

## The Conflict We're Enacting

This system does not _solve_ the fundamental problem of knowledge: how the Subject can bridge the gap between sensible phenomena (what it receives from computation) and intelligible necessity (what it must think).

Instead, **it enacts the problem**. The Agent _lives in the tension_. That living at the frontier—the discursive working-through—is what generates understanding.

---

### References

- Hegel, _Science of Logic_
- Kant, _Critique of Pure Reason_ (Transcendental Aesthetic, Transcendental Analytics)
- Yoga Sutras of Patanjali (Nyaya/Samkhya traditions)
- OpenLogic (mathematical logic reference)

### License

See [LICENSE](LICENSE)
