
# Task/Agent/Workflow: UML & Engineering Overview

## System UML Overview

```uml
@startuml
class Task {
  +id: string
  +name: string
  +state: TaskState
  +config: TaskConfig
  +execute(): TaskResult
}

class Agent {
  +id: string
  +name: string
  +capabilities: Capability[]
  +health: AgentHealth
  +assignTask(task: Task): void
}

class Workflow {
  +id: string
  +name: string
  +steps: WorkflowStep[]
  +run(): WorkflowResult
}

Task "1" -- "*" Agent : assignedTo
Workflow "1" -- "*" Task : contains
Agent "1" -- "*" Workflow : participatesIn
@enduml
```

## Engineering Schema Structure

- **Task**: Represents a computational work unit. Tracks execution state, configuration, and results. Can be assigned to Agents and included in Workflows.
- **Agent**: Represents an active computational entity. Manages capabilities, health, and can be assigned Tasks. Participates in Workflows.
- **Workflow**: Orchestrates Tasks and Agents in a directed acyclic graph (DAG) or sequence. Handles execution, scheduling, and monitoring.

### Relationships

- A **Workflow** contains multiple **Tasks**.
- An **Agent** can be assigned to multiple **Tasks** and participate in multiple **Workflows**.
- **Tasks** are executed by **Agents** within the context of a **Workflow**.

## Schema Files

- `src/schema/task.ts`: Task Zod schema (engineering, execution, state)
- `src/schema/agent.ts`: Agent Zod schema (capabilities, health, assignment)
- `src/schema/workflow.ts`: Workflow Zod schema (orchestration, steps, monitoring)
- `src/schema/definition.ts`: Philosophical foundation (reference only)
- `src/schema/index.ts`: Module exports

## Design Principles

- **API-First**: REST/GraphQL ready, strong validation
- **Framework Integration**: Native NestJS & Genkit compatibility
- **Production-Ready**: Security, monitoring, audit, error handling
- **Extensible**: Plugin/configuration architecture
- **Observable**: Metrics, logging, tracing
- **Scalable**: Distributed execution, resource management
- **Type-Safe**: Full TypeScript type inference from Zod schemas

## Practical Implementation

1. **Service Classes**: Business logic for Task, Agent, Workflow
2. **Controller Classes**: REST API endpoints (NestJS)
3. **Repository Classes**: Data persistence
4. **Executor Classes**: Runtime execution engines
5. **Integration Adapters**: Genkit, external tools

## Conceptual Context (Summary)

- The Task/Agent/Workflow system is grounded in a triadic, dialectical logic (see `src/schema/definition.ts`).
- Tasks = Being (work units), Agents = Entity (actors), Workflows = Container (orchestration logic).
- The system is designed for both practical engineering and as a living demonstration of knowledge creation and orchestration.
