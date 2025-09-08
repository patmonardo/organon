# Repository Guidelines

## Project Structure & Module Organization
- Monorepo managed by pnpm workspaces: `reality/`, `core/`, `logic/`, `model/`, `task/`.
- Source lives under each package’s `src/`; package-local tests under `test/`. Root `test/` holds examples/demos.
- TypeScript path aliases are provided (see `tsconfig.json`). Example: `import { X } from "@organon/gds"`.

## Build, Test, and Development Commands
- Install: `pnpm install` (Node >= 18, pnpm >= 8).
- Build all: `pnpm build` (runs `tsc` or `nest build` per package).
- Dev (watch): `pnpm dev` or package-scoped: `pnpm --filter=@organon/gds dev`.
- Test all: `pnpm test`; package-scoped: `pnpm --filter=@organon/model test`.
- Run Task service (NestJS): `pnpm --filter=@organon/task start` (or `start:debug`).

## Coding Style & Naming Conventions
- Indentation: 2 spaces, LF line endings (`.editorconfig`).
- Language: TypeScript strict mode; favor explicit types at public boundaries.
- Naming: PascalCase for classes/types, camelCase for functions/vars, kebab-case for filenames.
- Imports: use workspace aliases (e.g., `@organon/logic/*`) instead of deep relative paths.
- Lint/format: `pnpm lint` runs recursively; full ESLint/Prettier configured in `@organon/task` (`pnpm --filter=@organon/task format`).

## Testing Guidelines
- Framework: Vitest across packages.
- Test files: co-locate as `*.spec.ts` or in `test/`. Build excludes `*.spec.ts`/`*.test.ts`.
- Run: `pnpm test` or `pnpm --filter=@organon/gds test`; coverage (task): `pnpm --filter=@organon/task test:cov`.
- Expectations: add/extend unit tests for new modules; keep tests deterministic and fast.

## Commit & Pull Request Guidelines
- Commits: imperative mood, concise summary; include scope when helpful (e.g., `core: add TypedEdge parser`, `logic: refine form morphisms`).
- PRs: clear description, linked issues, rationale, and usage notes; include API examples when changing interfaces. Ensure `pnpm build` and `pnpm test` pass.

## Security & Configuration Tips
- Do not commit secrets. Use environment variables (Nest `@organon/task` uses `@nestjs/config`).
- Prefer path aliases and public package entry points; avoid importing from `dist/`.
- When adding packages, update `pnpm-workspace.yaml` and respect `onlyBuiltDependencies`.

## Agent-Specific Notes
- Agent code and schemas live in `task/src/agent` and `task/src/schema`. Follow existing naming and module boundaries; keep schemas in `schema/` and execution code in `agent/`.

