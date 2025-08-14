import type { ProcessorInputs } from "../contracts";
import { assembleWorld } from "../world/assemble";
import { projectContentFromContexts } from "../form/project";
import { deriveSyllogisticEdges } from "../judgment/syllogism";
import { indexContent } from "../world/project";
import type { Content } from "../../schema/content";
import type { World } from "../../schema/world";

export type KriyaOptions = {
  // If true, project content from contexts (default: true)
  projectContent?: boolean;
  // Which content to index (default: "inputs")
  // - "inputs": index input.content only
  // - "projected": index content projected from contexts
  // - "both": index union of input.content and projected
  contentIndexSource?: "inputs" | "projected" | "both";
  // If true, derive syllogistic edges (computed, not merged into world yet) (default: false)
  deriveSyllogistic?: boolean;
};

export type KriyaResult = {
  world: World;
  projectedContent: Content[];
  // Derived syllogistic edges are returned but not merged into world (non-destructive)
  derivedEdges: ReturnType<typeof deriveSyllogisticEdges>;
  indexes: {
    content: ReturnType<typeof indexContent>;
  };
};

export async function runKriya(
  input: ProcessorInputs,
  opts: KriyaOptions = {}
): Promise<KriyaResult> {
  const options: Required<KriyaOptions> = {
    projectContent: opts.projectContent ?? true,
    contentIndexSource: opts.contentIndexSource ?? "inputs",
    deriveSyllogistic: opts.deriveSyllogistic ?? false
  };

  // 1) Assemble world (deterministic)
  const world = assembleWorld(input);

  // 2) Project content from contexts (deterministic ids)
  const projectedContent = options.projectContent
    ? projectContentFromContexts(input)
    : [];

  // 3) Derive syllogistic edges (layer only, not merged)
  const derivedEdges = options.deriveSyllogistic
    ? deriveSyllogisticEdges(input)
    : [];

  // 4) Index content with stable policy
  const contentForIndex =
    options.contentIndexSource === "projected"
      ? projectedContent
      : options.contentIndexSource === "both"
      ? // Combine deterministically by id (inputs first, then projected uniques)
        dedupeById([...input.content, ...projectedContent])
      : input.content;

  const indexes = {
    content: indexContent({ ...input, content: contentForIndex })
  };

  return { world, projectedContent, derivedEdges, indexes };
}

// Helper: deterministic dedupe by shape.core.id (or shape.of/id fallback)
function dedupeById(list: Content[]): Content[] {
  const key = (c: Content) =>
    c.shape.core.id ??
    `content:${c.shape.kind}:${c.shape.of.id}:${c.shape.core.name ?? ""}`;
  const seen = new Set<string>();
  const out: Content[] = [];
  for (const c of list) {
    const k = key(c);
    if (seen.has(k)) continue;
    seen.add(k);
    out.push(c);
  }
  // stable order by id
  out.sort((a, b) => (a.shape.core.id ?? "").localeCompare(b.shape.core.id ?? ""));
  return out;
}
