# Concept (minimal draft)

A Concept is the realized Plan: the synthesized artifact that combines a Morph (planned transformation) and the Aspect (manifestation) observed under a Relation. Concept artifacts capture intent, realization, provenance, and an auditable predicate that validates the realization (truth-of-empowerment).

Fields (minimal)

- id: string
- morphRef?: { id: string; shapeId?: string }
- aspectRef?: { id: string; entity?: string; properties?: Record<string, any> }
- groundRef?: { id: string; repository?: string }
- relationRef?: { id: string; type?: string }
- intent?: string // human/machine-readable summary of the Plan
- truthOfEmpowerment?: boolean // deterministic predicate result
- score?: number // combined score used to validate
- provenance?: any // free-form provenance object with source/method/modality/confidence/timestamp/trace
- agentRef?: { id: string; type?: string } // optional knowledge-agent that adopts the Concept
- createdAt?: string
- meta?: Record<string, any>

Example (JSON)

{
  "id": "concept:c1",
  "morphRef": { "id": "m1", "shapeId": "shape:vehicle" },
  "aspectRef": { "id": "a1", "entity": "vehicle:123", "properties": { "color": "red" } },
  "groundRef": { "id": "ground:reg" },
  "relationRef": { "id": "relation:display" },
  "intent": "Register vehicle:123 as red",
  "truthOfEmpowerment": true,
  "score": 1.0,
  "provenance": { "source": "system.bootstrap", "confidence": 1 },
  "createdAt": "2025-08-22T00:00:00Z"
}

Minimal Zod sketch (for discussion, not codegen)

```ts
import { z } from 'zod';

export const ConceptSchema = z.object({
  id: z.string(),
  morphRef: z.object({ id: z.string(), shapeId: z.string() }).optional(),
  aspectRef: z.object({ id: z.string(), entity: z.string(), properties: z.record(z.any()) }).optional(),
  groundRef: z.object({ id: z.string(), repository: z.string() }).optional(),
  relationRef: z.object({ id: z.string(), type: z.string() }).optional(),
  intent: z.string().optional(),
  truthOfEmpowerment: z.boolean().optional(),
  score: z.number().optional(),
  provenance: z.any().optional(),
  agentRef: z.object({ id: z.string(), type: z.string() }).optional(),
  createdAt: z.string().optional(),
  meta: z.record(z.any()).optional(),
});
```

Notes

- `Concept` is intentionally light-weight and extensible. Implementations may tighten provenance and schema rules over time.
- Concepts are primary artifacts for knowledge agents: when adopted they become the actionable "Idea" that informs future Morphs and decisions.
