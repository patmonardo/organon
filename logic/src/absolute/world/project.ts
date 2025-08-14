import type { ProcessorInputs } from "../contracts";

// Content index: subtle mapped to World (global), gross mapped to Thing (by entity id)
export function indexContent(input: ProcessorInputs): {
  subtleWorldTotal: number;
  grossByThing: Record<string, number>;
} {
  let subtleWorldTotal = 0;
  const grossByThing: Record<string, number> = {};

  for (const c of input.content) {
    if (c.shape.kind === "subtle") {
      subtleWorldTotal += 1;
    } else {
      const id = c.shape.of.id;
      grossByThing[id] = (grossByThing[id] ?? 0) + 1;
    }
  }
  return { subtleWorldTotal, grossByThing };
}
