"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.KnowledgeUnitSchema = exports.JustificationSchema = exports.ClaimSchema = exports.EpistemicStatusSchema = void 0;
const zod_1 = require("zod");
const provenance_1 = require("./provenance");
exports.EpistemicStatusSchema = zod_1.z.enum([
    'hypothesis', // proposed, minimally justified
    'supported', // has evidence and coherent fit
    'contested', // conflicting evidence/arguments
    'established', // robust, widely coherent
    'axiomatic', // taken as foundational within a frame
]);
exports.ClaimSchema = zod_1.z.object({
    id: zod_1.z.string(),
    statement: zod_1.z.string(),
    language: zod_1.z.string().default('en'),
    context: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
exports.JustificationSchema = zod_1.z.object({
    id: zod_1.z.string(),
    rationale: zod_1.z.string().optional(),
    arguments: zod_1.z.array(zod_1.z.string()).optional(),
    modelRefs: zod_1.z.array(zod_1.z.string()).optional(),
});
exports.KnowledgeUnitSchema = zod_1.z.object({
    id: zod_1.z.string(),
    claim: exports.ClaimSchema,
    justification: exports.JustificationSchema.optional(),
    provenance: provenance_1.ProvenanceSchema,
    status: exports.EpistemicStatusSchema.default('hypothesis'),
    confidence: zod_1.z.number().min(0).max(1).default(0.5),
    tags: zod_1.z.array(zod_1.z.string()).optional(),
    createdAt: zod_1.z.date().default(() => new Date()),
    updatedAt: zod_1.z.date().default(() => new Date()),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
//# sourceMappingURL=knowledge.js.map