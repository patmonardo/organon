"use strict";
/**
 * Task Schema: Goal–Method (Active Being in Motion)
 * =================================================
 *
 * In the TAW dialectical system, Task represents the transformation of
 * Being into active, goal-directed motion:
 *
 *   - Goal: The telos or intended end-state (Active Being as destination)
 *   - Method: The means or approach for achieving the goal (Active Being as process)
 *
 * Task = Goal : Method
 *
 * Philosophical Note:
 * Task is Being in motion - not static existence but purposeful becoming.
 * It represents the first movement of Being toward agency and transformation.
 * The Goal is Being as telos, the Method is Being as kinesis.
 * This enables the dialectical bridge between static Logic and dynamic Agency.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.TaskFactory = exports.TaskSchema = exports.MethodSchema = exports.GoalSchema = void 0;
const zod_1 = require("zod");
// Goal: The telos or intended end-state (Active Being as destination)
exports.GoalSchema = zod_1.z.object({
    id: zod_1.z.string(),
    name: zod_1.z.string(),
    description: zod_1.z.string().optional(),
    target: zod_1.z.record(zod_1.z.string(), zod_1.z.any()), // The desired end-state
    form: zod_1.z.string(), // The ontological type or form of the goal
    criteria: zod_1.z.array(zod_1.z.string()).optional(), // Success criteria
    priority: zod_1.z
        .enum(['low', 'medium', 'high', 'absolute'])
        .optional()
        .default('medium'),
    deadline: zod_1.z.date().optional(),
    // Dialectical metadata
    dialecticalStage: zod_1.z
        .enum(['immediate', 'mediated', 'absolute'])
        .default('immediate'),
});
// Method: The means or approach for achieving the goal (Active Being as process)
exports.MethodSchema = zod_1.z.object({
    id: zod_1.z.string(),
    name: zod_1.z.string(),
    approach: zod_1.z.enum(['sequential', 'parallel', 'dialectical', 'organic']),
    steps: zod_1.z
        .array(zod_1.z.object({
        id: zod_1.z.string(),
        action: zod_1.z.string(),
        dependencies: zod_1.z.array(zod_1.z.string()).optional(),
        resources: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
    }))
        .optional(),
    constraints: zod_1.z.array(zod_1.z.string()).optional(),
    resources: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
    // Ontological structure
    entityType: zod_1.z.string(),
    // Dialectical movement
    movement: zod_1.z.enum(['thesis', 'antithesis', 'synthesis']).default('thesis'),
});
// Task: The synthesis of Goal and Method
exports.TaskSchema = zod_1.z
    .object({
    id: zod_1.z.string(),
    goal: exports.GoalSchema,
    method: exports.MethodSchema,
    // State management
    status: zod_1.z
        .enum(['pending', 'active', 'suspended', 'completed', 'failed'])
        .default('pending'),
    progress: zod_1.z.number().min(0).max(1).default(0),
    // Dialectical context
    dialecticalContext: zod_1.z
        .object({
        stage: zod_1.z.enum(['being', 'essence', 'concept', 'absolute-idea']),
        contradictions: zod_1.z.array(zod_1.z.string()).optional(),
        syntheses: zod_1.z.array(zod_1.z.string()).optional(),
    })
        .optional(),
    // Temporal dimension
    createdAt: zod_1.z.date().default(() => new Date()),
    updatedAt: zod_1.z.date().default(() => new Date()),
    // Metadata
    metadata: zod_1.z.record(zod_1.z.string(), zod_1.z.any()).optional(),
})
    .catchall(zod_1.z.any());
/**
 * Task Factory: Creates tasks with dialectical awareness
 */
class TaskFactory {
    /**
     * Create a Being-level task (immediate, simple goals)
     */
    static createBeingTask(goal, method) {
        const now = new Date();
        return {
            id: `being-task-${Date.now()}`,
            goal: {
                id: `goal-${Date.now()}`,
                name: goal,
                target: { beingState: 'immediate' },
                form: 'being-form',
                priority: 'medium',
                dialecticalStage: 'immediate',
            },
            method: {
                id: `method-${Date.now()}`,
                name: method,
                approach: 'sequential',
                entityType: 'being-entity',
                movement: 'thesis',
            },
            status: 'pending',
            progress: 0,
            dialecticalContext: {
                stage: 'being',
            },
            createdAt: now,
            updatedAt: now,
        };
    }
    /**
     * Create an Essence-level task (mediated, reflective goals)
     */
    static createEssenceTask(goal, method, mediation) {
        const now = new Date();
        return {
            id: `essence-task-${Date.now()}`,
            goal: {
                id: `goal-${Date.now()}`,
                name: goal,
                target: { essenceState: 'mediated', mediation },
                form: 'essence-form',
                priority: 'high',
                dialecticalStage: 'mediated',
            },
            method: {
                id: `method-${Date.now()}`,
                name: method,
                approach: 'dialectical',
                entityType: 'essence-entity',
                movement: 'antithesis',
                constraints: ['requires-mediation'],
            },
            status: 'pending',
            progress: 0,
            dialecticalContext: {
                stage: 'essence',
                contradictions: ['immediate-vs-mediated'],
            },
            createdAt: now,
            updatedAt: now,
        };
    }
    /**
     * Create a Concept-level task (self-determining, active goals)
     */
    static createConceptTask(goal, method, concept) {
        const now = new Date();
        return {
            id: `concept-task-${Date.now()}`,
            goal: {
                id: `goal-${Date.now()}`,
                name: goal,
                target: { conceptState: 'self-determining', concept },
                form: 'concept-form',
                criteria: ['self-determination', 'universality', 'particularity'],
                priority: 'high',
                dialecticalStage: 'absolute',
            },
            method: {
                id: `method-${Date.now()}`,
                name: method,
                approach: 'organic',
                entityType: 'concept-entity',
                movement: 'synthesis',
                steps: [
                    {
                        id: 'universalize',
                        action: 'abstract-to-universal',
                    },
                    {
                        id: 'particularize',
                        action: 'determine-particular',
                        dependencies: ['universalize'],
                    },
                    {
                        id: 'individualize',
                        action: 'concrete-individual',
                        dependencies: ['universalize', 'particularize'],
                    },
                ],
            },
            status: 'pending',
            progress: 0,
            dialecticalContext: {
                stage: 'concept',
                syntheses: ['universal-particular-individual'],
            },
            createdAt: now,
            updatedAt: now,
        };
    }
    /**
     * Create an Absolute Idea task (complete self-aware system)
     */
    static createAbsoluteIdeaTask(goal, systemContext) {
        const now = new Date();
        return {
            id: `absolute-idea-task-${Date.now()}`,
            goal: {
                id: `goal-${Date.now()}`,
                name: goal,
                target: {
                    absoluteIdeaState: 'self-aware-system',
                    systemContext,
                    isAbsolute: true,
                },
                form: 'absolute-idea-form',
                criteria: [
                    'complete-self-knowledge',
                    'systematic-totality',
                    'free-self-determination',
                    'absolute-objectivity',
                ],
                priority: 'absolute',
                dialecticalStage: 'absolute',
            },
            method: {
                id: `method-${Date.now()}`,
                name: 'absolute-method',
                approach: 'organic',
                entityType: 'absolute-entity',
                movement: 'synthesis',
                steps: [
                    {
                        id: 'self-knowledge',
                        action: 'achieve-complete-self-knowledge',
                    },
                    {
                        id: 'systematic-totality',
                        action: 'organize-systematic-totality',
                        dependencies: ['self-knowledge'],
                    },
                    {
                        id: 'free-determination',
                        action: 'exercise-free-self-determination',
                        dependencies: ['self-knowledge', 'systematic-totality'],
                    },
                ],
            },
            status: 'pending',
            progress: 0,
            dialecticalContext: {
                stage: 'absolute-idea',
                syntheses: [
                    'being-essence-concept',
                    'logic-nature-spirit',
                    'subjective-objective-absolute',
                ],
            },
            createdAt: now,
            updatedAt: now,
        };
    }
}
exports.TaskFactory = TaskFactory;
exports.default = exports.TaskSchema;
//# sourceMappingURL=task.js.map