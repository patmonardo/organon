# ORGANON Introduction

**ORGANON** is a computational platform for the Science of Action—implementing the complete BEC-MVC-TAW dialectical cube as a research assistant for transformative knowledge work. It unifies Hegelian dialectics, Action Yoga, and Vedic logic into a living, extensible system.

---

## The Dialectical Cube: Three Layers, Three Triads

ORGANON is architected as a **3×3×3 dialectical cube**—three layers, each with its own triad, each realized as a TypeScript package:

| Layer | Package | Triad | Domain | Core Object | Repository | Schema |
|-------|---------|-------|--------|-------------|------------|--------|
| BEC   | `@organon/logic` | Being, Essence, Concept | Pure Logic | **Form** | FormRepository | FormSchema |
| MVC   | `@organon/model` | Model, View, Controller | Logic of Experience | **DataModel** | ModelRepository | ModelSchema |
| TAW   | `@organon/task`  | Task, Agent, Workflow | Absolute Synthesis | **TopicModel** | TaskRepository | TaskSchema |

Each package implements a **parallel structure**:

- **<object>**: The unique, theory-bearing entity of the layer (Form, DataModel, TopicModel)
- **repository/**: The runtime store and interface for managing objects of that layer
- **schema/**: Canonical Zod schemas defining invariants, types, and creation/update helpers

---

## Layer Details

### 1. BEC Layer (`@organon/logic`) — Pure Noumenal Logic

- **Triad**: Being (Form-Entity), Essence (Context-Property), Concept (Morph-Relation)
- **Core Object**: `Form` — the canonical unit of pure logic
- **Repository**: `FormRepository` — manages forms and their relations
- **Schema**: `FormSchema`, `EntitySchema`, etc. — Zod models for all logic primitives

**Purpose**:  
Defines the theoretical ground for all transformation. No empirical data—only pure, invariant logical structures. The Form layer wraps schemas for engine ergonomics; the Processor integrates both into runtime.

---

### 2. MVC Layer (`@organon/model`) — Logic of Experience

- **Triad**: Model (State-Structure), View (Representation-Perspective), Controller (Action-Rule)
- **Core Object**: `DataModel` — the empirical, actionable substrate
- **Repository**: `ModelRepository` — manages data models, views, and controllers
- **Schema**: `ModelSchema`, `ViewSchema`, `ControllerSchema` — Zod models for experiential logic

**Purpose**:  
Transforms pure logic into experiential actuality. Here, objects are “given for us” (Kant)—the same logical structure, now as data, state, and action. The repository and schema folders mirror the logic layer, but for empirical forms.

---

### 3. TAW Layer (`@organon/task`) — Absolute Practical Synthesis

- **Triad**: Task (Goal-Method), Agent (Capacity-Awareness), Workflow (Process-Coordination)
- **Core Object**: `TopicModel` — the reflective encyclopedia, uniting science and synthesis
- **Repository**: `TaskRepository` — manages tasks, agents, and workflows
- **Schema**: `TaskSchema`, `AgentSchema`, `WorkflowSchema` — Zod models for practical synthesis

**Purpose**:  
Completes the dialectic: the Absolute Synthesis where logic and experience are unified in practical, systematic knowledge. The `TopicModel` is envisioned as a reflective, encyclopedic structure—an evolving map of science and transformation.

---

## Parallel Structure in Every Package

Every major package (`logic`, `model`, `task`) implements:

- **<object>/**: The core domain object(s) for the layer (e.g., `Form`, `DataModel`, `TopicModel`)
- **repository/**: The runtime store, manager, and interface for those objects
- **schema/**: Canonical Zod schemas, helpers, and invariants

This parallelism ensures:

- **Consistency**: Every layer is architecturally similar, making the system easy to extend and reason about.
- **Extensibility**: New dialectical layers or domain objects can be added with minimal friction.
- **Clarity**: Contributors can quickly locate and understand the role of each file and folder.

---

## The Role of @organon/reality

While not yet fully defined, `@organon/reality` is reserved for expressing the **fundamental breakdown of reality**—the dialectical substrate that mediates between pure logic and empirical experience. It will eventually encode the five-fold dialectical process, serving as the bridge between intellectual and sensible intuition.

---

## The Science of Action in Code

- **Pure Logic (BEC)**: Theoretical invariants, canonical forms, and logical relations.
- **Logic of Experience (MVC)**: Data models, state, and actionable structures.
- **Absolute Synthesis (TAW)**: Practical knowledge, workflow, and systematic transformation.

Each layer is both **philosophically grounded** and **technically rigorous**, with Zod schemas as the source of truth, ergonomic wrappers for runtime, and processors/repositories for dynamic operation.

---

## Getting Started

- See the [README.md](./README.md) for dialectical philosophy, setup, and development scripts.
- Explore each package’s `schema/`, `<object>/`, and `repository/` folders for canonical models and runtime logic.
- Contribute by following the architectural and naming guidelines—every addition should fit the dialectical cube and parallel structure.

---

**ORGANON: Where pure logic becomes living system through
