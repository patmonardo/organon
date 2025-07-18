# Model Package (`@/model`): System Instruction & Architecture

## Purpose

The `@/model` package implements the Model-View-Controller (MVC) pattern as executable, empirical sciences. Each MVC is a Knowledge Graph, representing an applied science or domain of inference and action.

## Architectural Role

- **Driven by `@/task`**: All model execution (inference, action) is orchestrated by the Task/Agent/Workflow system in `@/task`, using Genkit and NestJS.
- **Constructed by `@/logic`**: Model construction, schema generation, and EPR (Entity-Property-Relation) logic are provided by the Hegelian engine in `@/logic`.
- **Integrated with `@/core`**: Agents in `@/task` may utilize local LLMs or LLM caches in `@/core` for advanced reasoning, caching, or native language model execution.

## Key Principles

- Each Model is an executable Knowledge Graph (MVC).
- Models are not static; they are dynamically constructed and orchestrated.
- Inference is treated as a type of action—model execution is always active.
- The system is designed for extensibility, composability, and orchestration by higher-level controllers.
- Models will evolve to use NestJS Decorators for seamless orchestration by `@/task`.

## System Flow

1. **Model Construction**: `@/logic` generates EPR schemas and Knowledge Graphs.
2. **Model Execution**: `@/task` orchestrates the execution of Models (MVCs) as part of workflows.
3. **Agent Integration**: Agents may leverage `@/core` for LLM-based reasoning or caching.

## File Structure

- `model/` — Model definitions and schemas
- `view/` — View logic and presentation
- `controller/` — Controller logic for model orchestration
- `form/` — Input/output schemas and validation
- `schema/` — Model schema definitions
- `index.ts` — Main exports

## Evolution Roadmap

- Refactor models to use NestJS Decorators for better integration and control by `@/task`.
- Enhance composability and orchestration of models as Knowledge Graphs.
- Expand schema and controller logic for richer, more dynamic model execution.

---

This README serves as the system instruction and architectural guide for the `@/model` package. Update as the package evolves.
