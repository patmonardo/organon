import { createContent, type Content, type ContentKind } from "../../schema/content";
import type { ProcessorInputs } from "../core/contracts";

/**
 * Deterministic Content projection from Contexts:
 * - Classify each Context to Content.kind ("subtle" | "gross").
 * - Sort contexts by kind (subtle first), then by ctx id for stable order.
 * - Within a context, sort entities by id; optional per-context dedupe.
 * - Deterministic ids: content:{kind}:{ctxId}:{entityId}
 */
export function projectContentFromContexts(
  input: ProcessorInputs,
  opts?: {
    classify?: (ctx: ProcessorInputs["contexts"][number]) => ContentKind;
    contentType?: string;
    dedupeEntities?: boolean;
  }
): Content[] {
  const classify =
    opts?.classify ??
    ((ctx: ProcessorInputs["contexts"][number]): ContentKind => {
      const t = ctx.shape.core.type?.toLowerCase?.() ?? "";
      const n = (ctx.shape.core.name ?? "").toLowerCase();
      return t.includes("world") || n.includes("world") ? "subtle" : "gross";
    });

  const contentType = opts?.contentType ?? "system.Content";
  const dedupeEntities = opts?.dedupeEntities ?? true;

  // Precompute classification, then sort by kind (subtle first) and ctx id
  const contexts = input.contexts
    .map((ctx) => ({ ctx, kind: classify(ctx) }))
    .sort((a, b) => {
      const rank = (k: ContentKind) => (k === "subtle" ? 0 : 1);
      const r = rank(a.kind) - rank(b.kind);
      return r !== 0
        ? r
        : a.ctx.shape.core.id.localeCompare(b.ctx.shape.core.id);
    });

  const out: Content[] = [];

  for (const { ctx, kind } of contexts) {
    const entities = [...ctx.shape.entities].sort((a, b) =>
      a.id.localeCompare(b.id)
    );
    const seen = new Set<string>();

    for (const ref of entities) {
      const key = `${ref.type}:${ref.id}`;
      if (dedupeEntities && seen.has(key)) continue;
      if (dedupeEntities) seen.add(key);

      const id = deterministicContentId(ctx.shape.core.id, ref.id, kind);
      out.push(
        createContent({
          id,
          type: contentType,
          name: `${kind}:${ctx.shape.core.name ?? ctx.shape.core.id}#${ref.id}`,
          kind,
          of: { id: ref.id, type: ref.type },
        })
      );
    }
  }

  return out;
}

function deterministicContentId(
  ctxId: string,
  entityId: string,
  kind: ContentKind
): string {
  return `content:${kind}:${ctxId}:${entityId}`;
}
