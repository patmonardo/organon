"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ProvenanceSchema = exports.EvidenceSchema = exports.KnowledgeOriginSchema = void 0;
const zod_1 = require("zod");
// Source and provenance of knowledge claims
exports.KnowledgeOriginSchema = zod_1.z.enum([
    'empirical', // sensible cognition / data-derived
    'transcendental', // higher cognition / a priori structure
    'synthetic', // result of synthesis across sources
    'reflective', // agent judgment / meta-cognition
]);
exports.EvidenceSchema = zod_1.z.object({
    id: zod_1.z.string(),
    kind: zod_1.z
        .enum([
        'observation',
        'experiment',
        'derivation',
        'citation',
        'model',
        'testimonial',
        'other',
    ])
        .default('other'),
    description: zod_1.z.string().optional(),
    dataRef: zod_1.z.string().optional(), // pointer into @organon/model, left opaque here
    strength: zod_1.z.number().min(0).max(1).optional(),
    timestamp: zod_1.z.date().optional(),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
exports.ProvenanceSchema = zod_1.z.object({
    id: zod_1.z.string(),
    origin: exports.KnowledgeOriginSchema,
    sources: zod_1.z.array(zod_1.z.string()).optional(),
    evidence: zod_1.z.array(exports.EvidenceSchema).optional(),
    agentId: zod_1.z.string().optional(),
    createdAt: zod_1.z.date().default(() => new Date()),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
//# sourceMappingURL=provenance.js.map