# Organon - Research Assistant Platform

A monorepo containing the complete Organon research assistant platform with dialectical architecture implementing the Science of Logic as computational processes.

## Architecture - The Four Pillars

### **CORE** - Being (Das Sein)
- **`core/`** - @organon/core - Graph processing kernel
- Pure computational existence and fundamental operations
- The substrate where all speculative ideas execute ultimately

### **TASK** - Essence (Das Wesen) 
- **`task/`** - @organon/task - Orchestration and workflow management
- The mediating reflection between core computation and application
- TAW (Task, Agent, Workflow) as essence in motion

### **LOGIC** - Concept (Der Begriff)
- **`logic/`** - @organon/logic - Pure reasoning and inference engine  
- The self-determining concept that unifies inference and reasoning
- Where absolute idea manifests in computational form

### **APP** - Realization (Die Verwirklichung)
- **`app/`** - @organon/app - Speculative ideas and UserLand interface
- The culmination where absolute spirit becomes objective and practical
- Contains FORM as the transcendental beginning

## The Dialectical Movement

```
APP (Speculative Ideas) → LOGIC (Pure Concept) → TASK (Mediating Essence) → CORE (Ultimate Execution)
                                                                              ↑
                                     [Return as realized knowledge] ←←←←←←←←←←←
```

## Development

### Prerequisites

- Node.js 18+
- pnpm 8+

### Setup

```bash
# Install dependencies for all packages
pnpm install

# Build all packages
pnpm build

# Run tests for all packages
pnpm test
```

### Working with Individual Packages

#### Core Library (Being)

```bash
# Build core kernel
pnpm core:build

# Run core tests  
pnpm core:test

# Watch mode for development
pnpm core:dev
```

#### Task Daemon (Essence)

```bash
# Build task orchestration
pnpm task:build

# Run task daemon in development mode
pnpm task:dev

# Start task daemon
pnpm task:start

# Run task daemon tests
pnpm task:test
```

#### Logic Engine (Concept)

```bash
# Build logic engine
pnpm logic:build

# Run logic tests
pnpm logic:test

# Watch mode for development  
pnpm logic:dev
```

#### App Interface (Realization)

```bash
# Build app interface
pnpm app:build

# Run app tests
pnpm app:test

# Watch mode for development
pnpm app:dev

# Start app interface
pnpm app:start
```

### Scripts

- `pnpm build` - Build all packages
- `pnpm test` - Run tests for all packages
- `pnpm dev` - Run all packages in development mode
- `pnpm core:*` - Core kernel (Being) commands
- `pnpm task:*` - Task orchestration (Essence) commands  
- `pnpm logic:*` - Logic engine (Concept) commands
- `pnpm app:*` - App interface (Realization) commands

## Philosophy

Organon implements **The Science of Logic** as a research assistant platform:

- **Being** (CORE) - Pure computational substrate
- **Essence** (TASK) - Mediating orchestration processes  
- **Concept** (LOGIC) - Self-determining reasoning engine
- **Idea** (APP) - Realized interface where speculative ideas manifest

The platform enables the complete dialectical movement from speculative conception in UserLand to ultimate execution in the computational kernel, with each iteration reaching higher levels of systematic knowledge.
