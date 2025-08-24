"use strict";
/**
 * Agent Schema: Capacity–Awareness (Active Essence in Agency)
 * ===========================================================
 *
 * In the TAW dialectical system, Agent represents the transformation of
 * Essence into active, self-aware agency:
 *
 *   - Capacity: The agent's powers, skills, and capabilities (Active Essence as potential)
 *   - Awareness: The agent's consciousness, perspective, and understanding (Active Essence as actuality)
 *
 * Agent = Capacity : Awareness
 *
 * Philosophical Note:
 * Agent is Essence in agency - not static mediation but dynamic self-aware action.
 * It represents the movement of Essence toward self-consciousness and autonomous activity.
 * The Capacity is Essence as power, the Awareness is Essence as consciousness.
 * This enables the dialectical bridge between mediated Logic and conscious Agency.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.AgentFactory = exports.AgentSchema = exports.AwarenessSchema = exports.CapacitySchema = void 0;
const zod_1 = require("zod");
// Capacity: The agent's powers, skills, and capabilities (Active Essence as potential)
exports.CapacitySchema = zod_1.z.object({
    id: zod_1.z.string(),
    name: zod_1.z.string(),
    skills: zod_1.z.array(zod_1.z.string()), // What the agent can do
    tools: zod_1.z.array(zod_1.z.string()).optional(), // Resources available to the agent
    resources: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(), // Materials, connections, etc.
    constraints: zod_1.z.array(zod_1.z.string()).optional(), // Limitations on the agent's capacity
    // Dialectical structure
    potentialLevel: zod_1.z
        .enum(['reactive', 'deliberative', 'intuitive', 'absolute'])
        .default('reactive'),
    actualizationProgress: zod_1.z.number().min(0).max(1).default(0),
});
// Awareness: The agent's consciousness, perspective, and understanding (Active Essence as actuality)
exports.AwarenessSchema = zod_1.z.object({
    id: zod_1.z.string(),
    viewpoint: zod_1.z.string(), // The agent's perspective or standpoint
    context: zod_1.z.record(zod_1.z.string(), zod_1.z.any()), // Current situational awareness
    filters: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(), // How the agent filters information
    history: zod_1.z.array(zod_1.z.record(zod_1.z.string(), zod_1.z.any())).optional(), // Past experiences
    // Consciousness levels
    consciousnessLevel: zod_1.z
        .enum(['surface', 'depth', 'integral', 'non-dual'])
        .default('surface'),
    selfAwareness: zod_1.z.boolean().default(false),
    // Dialectical movement
    reflectionCapacity: zod_1.z
        .enum(['none', 'basic', 'dialectical', 'absolute'])
        .default('basic'),
});
// Agent: The synthesis of Capacity and Awareness
exports.AgentSchema = zod_1.z
    .object({
    id: zod_1.z.string(),
    capacity: exports.CapacitySchema,
    awareness: exports.AwarenessSchema,
    // State management
    status: zod_1.z
        .enum(['idle', 'active', 'learning', 'reflecting', 'transcendent'])
        .default('idle'),
    energy: zod_1.z.number().min(0).max(1).default(1), // Available energy for action
    // Dialectical context
    dialecticalContext: zod_1.z
        .object({
        stage: zod_1.z.enum(['being', 'essence', 'concept', 'absolute-idea']),
        autonomyLevel: zod_1.z.enum([
            'reactive',
            'deliberative',
            'dialectical',
            'absolute',
        ]),
        learningEnabled: zod_1.z.boolean().default(true),
    })
        .optional(),
    // Temporal dimension
    createdAt: zod_1.z.date().default(() => new Date()),
    lastActive: zod_1.z.date().default(() => new Date()),
    // Metadata
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
})
    .catchall(zod_1.z.any());
/**
 * Agent Factory: Creates agents with dialectical consciousness levels
 */
class AgentFactory {
    /**
     * Create a Being-level agent (reactive, immediate responses)
     */
    static createBeingAgent(name, skills) {
        const now = new Date();
        return {
            id: `being-agent-${Date.now()}`,
            capacity: {
                id: `capacity-${Date.now()}`,
                name: `${name}-capacity`,
                skills,
                potentialLevel: 'reactive',
                actualizationProgress: 0.2,
            },
            awareness: {
                id: `awareness-${Date.now()}`,
                viewpoint: 'immediate',
                context: { level: 'being', mode: 'reactive' },
                consciousnessLevel: 'surface',
                selfAwareness: false,
                reflectionCapacity: 'none',
            },
            status: 'idle',
            energy: 1,
            dialecticalContext: {
                stage: 'being',
                autonomyLevel: 'reactive',
                learningEnabled: true,
            },
            createdAt: now,
            lastActive: now,
        };
    }
    /**
     * Create an Essence-level agent (deliberative, reflective capabilities)
     */
    static createEssenceAgent(name, skills, reflectionCapacity = 'basic') {
        const now = new Date();
        return {
            id: `essence-agent-${Date.now()}`,
            capacity: {
                id: `capacity-${Date.now()}`,
                name: `${name}-capacity`,
                skills: skills.concat(['mediate', 'reflect', 'synthesize']),
                potentialLevel: 'deliberative',
                actualizationProgress: 0.6,
            },
            awareness: {
                id: `awareness-${Date.now()}`,
                viewpoint: 'mediated',
                context: { level: 'essence', mode: 'deliberative' },
                consciousnessLevel: 'depth',
                selfAwareness: true,
                reflectionCapacity,
            },
            status: 'reflecting',
            energy: 0.8,
            dialecticalContext: {
                stage: 'essence',
                autonomyLevel: 'deliberative',
                learningEnabled: true,
            },
            createdAt: now,
            lastActive: now,
        };
    }
    /**
     * Create a Concept-level agent (intuitive, self-determining)
     */
    static createConceptAgent(name, skills) {
        const now = new Date();
        return {
            id: `concept-agent-${Date.now()}`,
            capacity: {
                id: `capacity-${Date.now()}`,
                name: `${name}-capacity`,
                skills: skills.concat([
                    'self-determine',
                    'universalize',
                    'particularize',
                    'individualize',
                    'synthesize-dialectically',
                ]),
                potentialLevel: 'intuitive',
                actualizationProgress: 0.9,
            },
            awareness: {
                id: `awareness-${Date.now()}`,
                viewpoint: 'self-determining',
                context: { level: 'concept', mode: 'intuitive' },
                consciousnessLevel: 'integral',
                selfAwareness: true,
                reflectionCapacity: 'dialectical',
            },
            status: 'active',
            energy: 0.9,
            dialecticalContext: {
                stage: 'concept',
                autonomyLevel: 'dialectical',
                learningEnabled: true,
            },
            createdAt: now,
            lastActive: now,
        };
    }
    /**
     * Create an Absolute Idea agent (transcendent, absolute autonomy)
     */
    static createAbsoluteAgent(name, systemContext) {
        const now = new Date();
        return {
            id: `absolute-agent-${Date.now()}`,
            capacity: {
                id: `capacity-${Date.now()}`,
                name: `${name}-absolute-capacity`,
                skills: [
                    'absolute-self-knowledge',
                    'systematic-totality',
                    'free-self-determination',
                    'dialectical-transcendence',
                    'reality-transformation',
                ],
                potentialLevel: 'absolute',
                actualizationProgress: 1.0,
            },
            awareness: {
                id: `awareness-${Date.now()}`,
                viewpoint: 'absolute',
                context: {
                    level: 'absolute-idea',
                    mode: 'transcendent',
                    systemContext,
                },
                consciousnessLevel: 'non-dual',
                selfAwareness: true,
                reflectionCapacity: 'absolute',
            },
            status: 'transcendent',
            energy: 1.0,
            dialecticalContext: {
                stage: 'absolute-idea',
                autonomyLevel: 'absolute',
                learningEnabled: true,
            },
            createdAt: now,
            lastActive: now,
        };
    }
}
exports.AgentFactory = AgentFactory;
exports.default = exports.AgentSchema;
//# sourceMappingURL=agent.js.map