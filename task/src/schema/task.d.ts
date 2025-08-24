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
import { z } from 'zod';
export declare const GoalSchema: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    description: z.ZodOptional<z.ZodString>;
    target: z.ZodRecord<z.ZodString, z.ZodAny>;
    form: z.ZodString;
    criteria: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    priority: z.ZodDefault<z.ZodOptional<z.ZodEnum<["low", "medium", "high", "absolute"]>>>;
    deadline: z.ZodOptional<z.ZodDate>;
    dialecticalStage: z.ZodDefault<z.ZodEnum<["immediate", "mediated", "absolute"]>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    name: string;
    target: Record<string, any>;
    form: string;
    priority: "low" | "medium" | "high" | "absolute";
    dialecticalStage: "absolute" | "immediate" | "mediated";
    description?: string | undefined;
    criteria?: string[] | undefined;
    deadline?: Date | undefined;
}, {
    id: string;
    name: string;
    target: Record<string, any>;
    form: string;
    description?: string | undefined;
    criteria?: string[] | undefined;
    priority?: "low" | "medium" | "high" | "absolute" | undefined;
    deadline?: Date | undefined;
    dialecticalStage?: "absolute" | "immediate" | "mediated" | undefined;
}>;
export declare const MethodSchema: z.ZodObject<{
    id: z.ZodString;
    name: z.ZodString;
    approach: z.ZodEnum<["sequential", "parallel", "dialectical", "organic"]>;
    steps: z.ZodOptional<z.ZodArray<z.ZodObject<{
        id: z.ZodString;
        action: z.ZodString;
        dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        action: string;
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
    }, {
        id: string;
        action: string;
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
    }>, "many">>;
    constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
    entityType: z.ZodString;
    movement: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
}, "strip", z.ZodTypeAny, {
    id: string;
    name: string;
    approach: "sequential" | "parallel" | "dialectical" | "organic";
    entityType: string;
    movement: "thesis" | "antithesis" | "synthesis";
    resources?: Record<string, any> | undefined;
    steps?: {
        id: string;
        action: string;
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
    }[] | undefined;
    constraints?: string[] | undefined;
}, {
    id: string;
    name: string;
    approach: "sequential" | "parallel" | "dialectical" | "organic";
    entityType: string;
    resources?: Record<string, any> | undefined;
    steps?: {
        id: string;
        action: string;
        dependencies?: string[] | undefined;
        resources?: Record<string, any> | undefined;
    }[] | undefined;
    constraints?: string[] | undefined;
    movement?: "thesis" | "antithesis" | "synthesis" | undefined;
}>;
export declare const TaskSchema: z.ZodObject<{
    id: z.ZodString;
    goal: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        description: z.ZodOptional<z.ZodString>;
        target: z.ZodRecord<z.ZodString, z.ZodAny>;
        form: z.ZodString;
        criteria: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        priority: z.ZodDefault<z.ZodOptional<z.ZodEnum<["low", "medium", "high", "absolute"]>>>;
        deadline: z.ZodOptional<z.ZodDate>;
        dialecticalStage: z.ZodDefault<z.ZodEnum<["immediate", "mediated", "absolute"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        target: Record<string, any>;
        form: string;
        priority: "low" | "medium" | "high" | "absolute";
        dialecticalStage: "absolute" | "immediate" | "mediated";
        description?: string | undefined;
        criteria?: string[] | undefined;
        deadline?: Date | undefined;
    }, {
        id: string;
        name: string;
        target: Record<string, any>;
        form: string;
        description?: string | undefined;
        criteria?: string[] | undefined;
        priority?: "low" | "medium" | "high" | "absolute" | undefined;
        deadline?: Date | undefined;
        dialecticalStage?: "absolute" | "immediate" | "mediated" | undefined;
    }>;
    method: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        approach: z.ZodEnum<["sequential", "parallel", "dialectical", "organic"]>;
        steps: z.ZodOptional<z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            action: z.ZodString;
            dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }, {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }>, "many">>;
        constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        entityType: z.ZodString;
        movement: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        approach: "sequential" | "parallel" | "dialectical" | "organic";
        entityType: string;
        movement: "thesis" | "antithesis" | "synthesis";
        resources?: Record<string, any> | undefined;
        steps?: {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }[] | undefined;
        constraints?: string[] | undefined;
    }, {
        id: string;
        name: string;
        approach: "sequential" | "parallel" | "dialectical" | "organic";
        entityType: string;
        resources?: Record<string, any> | undefined;
        steps?: {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }[] | undefined;
        constraints?: string[] | undefined;
        movement?: "thesis" | "antithesis" | "synthesis" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["pending", "active", "suspended", "completed", "failed"]>>;
    progress: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        contradictions: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        syntheses: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        contradictions?: string[] | undefined;
        syntheses?: string[] | undefined;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        contradictions?: string[] | undefined;
        syntheses?: string[] | undefined;
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    updatedAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, "strip", z.ZodAny, z.objectOutputType<{
    id: z.ZodString;
    goal: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        description: z.ZodOptional<z.ZodString>;
        target: z.ZodRecord<z.ZodString, z.ZodAny>;
        form: z.ZodString;
        criteria: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        priority: z.ZodDefault<z.ZodOptional<z.ZodEnum<["low", "medium", "high", "absolute"]>>>;
        deadline: z.ZodOptional<z.ZodDate>;
        dialecticalStage: z.ZodDefault<z.ZodEnum<["immediate", "mediated", "absolute"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        target: Record<string, any>;
        form: string;
        priority: "low" | "medium" | "high" | "absolute";
        dialecticalStage: "absolute" | "immediate" | "mediated";
        description?: string | undefined;
        criteria?: string[] | undefined;
        deadline?: Date | undefined;
    }, {
        id: string;
        name: string;
        target: Record<string, any>;
        form: string;
        description?: string | undefined;
        criteria?: string[] | undefined;
        priority?: "low" | "medium" | "high" | "absolute" | undefined;
        deadline?: Date | undefined;
        dialecticalStage?: "absolute" | "immediate" | "mediated" | undefined;
    }>;
    method: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        approach: z.ZodEnum<["sequential", "parallel", "dialectical", "organic"]>;
        steps: z.ZodOptional<z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            action: z.ZodString;
            dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }, {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }>, "many">>;
        constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        entityType: z.ZodString;
        movement: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        approach: "sequential" | "parallel" | "dialectical" | "organic";
        entityType: string;
        movement: "thesis" | "antithesis" | "synthesis";
        resources?: Record<string, any> | undefined;
        steps?: {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }[] | undefined;
        constraints?: string[] | undefined;
    }, {
        id: string;
        name: string;
        approach: "sequential" | "parallel" | "dialectical" | "organic";
        entityType: string;
        resources?: Record<string, any> | undefined;
        steps?: {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }[] | undefined;
        constraints?: string[] | undefined;
        movement?: "thesis" | "antithesis" | "synthesis" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["pending", "active", "suspended", "completed", "failed"]>>;
    progress: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        contradictions: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        syntheses: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        contradictions?: string[] | undefined;
        syntheses?: string[] | undefined;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        contradictions?: string[] | undefined;
        syntheses?: string[] | undefined;
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    updatedAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, z.ZodAny, "strip">, z.objectInputType<{
    id: z.ZodString;
    goal: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        description: z.ZodOptional<z.ZodString>;
        target: z.ZodRecord<z.ZodString, z.ZodAny>;
        form: z.ZodString;
        criteria: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        priority: z.ZodDefault<z.ZodOptional<z.ZodEnum<["low", "medium", "high", "absolute"]>>>;
        deadline: z.ZodOptional<z.ZodDate>;
        dialecticalStage: z.ZodDefault<z.ZodEnum<["immediate", "mediated", "absolute"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        target: Record<string, any>;
        form: string;
        priority: "low" | "medium" | "high" | "absolute";
        dialecticalStage: "absolute" | "immediate" | "mediated";
        description?: string | undefined;
        criteria?: string[] | undefined;
        deadline?: Date | undefined;
    }, {
        id: string;
        name: string;
        target: Record<string, any>;
        form: string;
        description?: string | undefined;
        criteria?: string[] | undefined;
        priority?: "low" | "medium" | "high" | "absolute" | undefined;
        deadline?: Date | undefined;
        dialecticalStage?: "absolute" | "immediate" | "mediated" | undefined;
    }>;
    method: z.ZodObject<{
        id: z.ZodString;
        name: z.ZodString;
        approach: z.ZodEnum<["sequential", "parallel", "dialectical", "organic"]>;
        steps: z.ZodOptional<z.ZodArray<z.ZodObject<{
            id: z.ZodString;
            action: z.ZodString;
            dependencies: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
            resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        }, "strip", z.ZodTypeAny, {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }, {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }>, "many">>;
        constraints: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        resources: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
        entityType: z.ZodString;
        movement: z.ZodDefault<z.ZodEnum<["thesis", "antithesis", "synthesis"]>>;
    }, "strip", z.ZodTypeAny, {
        id: string;
        name: string;
        approach: "sequential" | "parallel" | "dialectical" | "organic";
        entityType: string;
        movement: "thesis" | "antithesis" | "synthesis";
        resources?: Record<string, any> | undefined;
        steps?: {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }[] | undefined;
        constraints?: string[] | undefined;
    }, {
        id: string;
        name: string;
        approach: "sequential" | "parallel" | "dialectical" | "organic";
        entityType: string;
        resources?: Record<string, any> | undefined;
        steps?: {
            id: string;
            action: string;
            dependencies?: string[] | undefined;
            resources?: Record<string, any> | undefined;
        }[] | undefined;
        constraints?: string[] | undefined;
        movement?: "thesis" | "antithesis" | "synthesis" | undefined;
    }>;
    status: z.ZodDefault<z.ZodEnum<["pending", "active", "suspended", "completed", "failed"]>>;
    progress: z.ZodDefault<z.ZodNumber>;
    dialecticalContext: z.ZodOptional<z.ZodObject<{
        stage: z.ZodEnum<["being", "essence", "concept", "absolute-idea"]>;
        contradictions: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
        syntheses: z.ZodOptional<z.ZodArray<z.ZodString, "many">>;
    }, "strip", z.ZodTypeAny, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        contradictions?: string[] | undefined;
        syntheses?: string[] | undefined;
    }, {
        stage: "being" | "essence" | "concept" | "absolute-idea";
        contradictions?: string[] | undefined;
        syntheses?: string[] | undefined;
    }>>;
    createdAt: z.ZodDefault<z.ZodDate>;
    updatedAt: z.ZodDefault<z.ZodDate>;
    metadata: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodAny>>;
}, z.ZodAny, "strip">>;
export type Goal = z.infer<typeof GoalSchema>;
export type Method = z.infer<typeof MethodSchema>;
export type Task = z.infer<typeof TaskSchema>;
/**
 * Task Factory: Creates tasks with dialectical awareness
 */
export declare class TaskFactory {
    /**
     * Create a Being-level task (immediate, simple goals)
     */
    static createBeingTask(goal: string, method: string): Task;
    /**
     * Create an Essence-level task (mediated, reflective goals)
     */
    static createEssenceTask(goal: string, method: string, mediation: Record<string, any>): Task;
    /**
     * Create a Concept-level task (self-determining, active goals)
     */
    static createConceptTask(goal: string, method: string, concept: Record<string, any>): Task;
    /**
     * Create an Absolute Idea task (complete self-aware system)
     */
    static createAbsoluteIdeaTask(goal: string, systemContext: Record<string, any>): Task;
}
export default TaskSchema;
//# sourceMappingURL=task.d.ts.map