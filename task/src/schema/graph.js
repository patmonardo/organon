"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.KnowledgeGraphSchema = exports.KnowledgeEdgeSchema = exports.KnowledgeNodeSchema = void 0;
const zod_1 = require("zod");
exports.KnowledgeNodeSchema = zod_1.z.object({
    id: zod_1.z.string(),
    type: zod_1.z.enum(['claim', 'concept', 'entity', 'topic']),
    label: zod_1.z.string(),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
exports.KnowledgeEdgeSchema = zod_1.z.object({
    id: zod_1.z.string(),
    source: zod_1.z.string(),
    target: zod_1.z.string(),
    relation: zod_1.z.string(),
    weight: zod_1.z.number().min(0).max(1).optional(),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
exports.KnowledgeGraphSchema = zod_1.z.object({
    id: zod_1.z.string(),
    nodes: zod_1.z.array(exports.KnowledgeNodeSchema),
    edges: zod_1.z.array(exports.KnowledgeEdgeSchema),
    createdAt: zod_1.z.date().default(() => new Date()),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
//# sourceMappingURL=graph.js.map