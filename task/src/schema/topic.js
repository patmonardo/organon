"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TopicModelSchema = exports.TopicIdentitySchema = void 0;
const zod_1 = require("zod");
const graph_1 = require("./graph");
const knowledge_1 = require("./knowledge");
// Topic model as reflective encyclopedia of science for an Agent/Task context
exports.TopicIdentitySchema = zod_1.z.object({
    id: zod_1.z.string(),
    name: zod_1.z.string(),
    description: zod_1.z.string().optional(),
});
exports.TopicModelSchema = zod_1.z.object({
    identity: exports.TopicIdentitySchema,
    corpusRef: zod_1.z.string().optional(), // pointer to dataset(s), opaque here
    vocabulary: zod_1.z.array(zod_1.z.string()).optional(),
    topics: zod_1.z
        .array(zod_1.z.object({
        id: zod_1.z.string(),
        label: zod_1.z.string(),
        terms: zod_1.z
            .array(zod_1.z.object({ term: zod_1.z.string(), weight: zod_1.z.number() }))
            .optional(),
        knowledgeUnits: zod_1.z.array(knowledge_1.KnowledgeUnitSchema).optional(),
        metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
    }))
        .optional(),
    graph: graph_1.KnowledgeGraphSchema.optional(),
    createdAt: zod_1.z.date().default(() => new Date()),
    updatedAt: zod_1.z.date().default(() => new Date()),
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
});
//# sourceMappingURL=topic.js.map