/**
 * BEING-FOR-SELF: The Rajas Guna - Dynamic Self-Relating Unity
 * ============================================================
 *
 * From Hegel's Greater Logic on Being-for-Self:
 * "Being-for-self is the unity of being and nothing as becoming"
 *
 * As Rajas Guna:
 * - Dynamic, active, combining principle
 * - Unity that relates Sattva (Being) and Tamas (Nothing/Existence)
 * - Self-relating quality that transcends mere limitation
 */

import { QualityGuna, RajasQuality, Guna } from './types/being';
import { Being } from './being';
import { Existence, QualitativeProperty } from './existence';

export class BeingForSelf implements RajasQuality {
  readonly id: string;
  readonly name: string;
  readonly guna: 'rajas' = 'rajas';
  readonly unity_character: 'dynamic_combination_sattva_tamas' = 'dynamic_combination_sattva_tamas';

  // Qualitative Content - Rajas as Dynamic Unity
  readonly qualitative_content = {
    guna_character: 'rajas_as_dynamic_self_relating_activity',
    substantial_determination: 'self_determining_through_relation_to_self',
    qualitative_limitation: 'rajas_transcends_limitation_through_self_relation'
  };

  // BEC Structure with Rajas Integration
  readonly being = {
    guna_being: 'being_as_rajas_self_activity',
    qualitative_immediacy: false, // Mediated self-relation
    guna_determination: 'rajas_determines_being_as_self_relating_process'
  };

  readonly essence = {
    guna_essence: 'essence_actualized_through_rajas_self_relation',
    qualitative_reflection: true, // Full self-reflection achieved
    guna_mediation: 'rajas_mediates_through_dynamic_self_relation'
  };

  readonly concept = {
    guna_concept: 'concept_as_rajas_concrete_universality',
    qualitative_universality: 'universal_that_particularizes_itself',
    guna_synthesis: 'rajas_achieves_synthesis_of_sattva_and_tamas'
  };

  // Dialectical Position
  readonly dialectical_moment: 'synthesis' = 'synthesis';
  readonly emerges_from = 'existence-tamas';
  readonly transitions_to = 'quantity-rajas'; // Moves beyond quality to quantity

  readonly description = 'Being-for-Self as Rajas Guna - the dynamic self-relating unity that synthesizes Being and Existence';

  private being_moment: Being;
  private existence_moment: Existence;
  private self_relation: SelfRelation;

  constructor(
    id: string = 'being-for-self-rajas',
    being?: Being,
    existence?: Existence
  ) {
    this.id = id;
    this.name = 'Being-for-Self (Rajas)';
    this.being_moment = being || new Being('being-in-bfs');
    this.existence_moment = existence || new Existence('existence-in-bfs');
    this.self_relation = new SelfRelation(this.being_moment, this.existence_moment);
  }

  /**
   * Being-for-Self as Unity of Being and Nothing/Existence
   */
  getDialecticalUnity(): string {
    return `Being-for-Self (Rajas) unifies:
    - Being (Sattva): ${this.being_moment.description}
    - Existence (Tamas): ${this.existence_moment.description}
    - Unity (Rajas): ${this.self_relation.describe()}

    This is not mere combination but dynamic self-relating process`;
  }

  /**
   * Self-Relation as the Key to Being-for-Self
   */
  getSelfRelationalStructure(): string {
    return `Being-for-Self relates to itself as:
    - Subject relating to itself as object
    - Quality that qualifies itself
    - Being that determines its own existence
    - Rajas activity that combines Sattva and Tamas in itself`;
  }

  /**
   * How Being-for-Self Transcends Mere Existence
   */
  transcendenceOfExistence(): string {
    return `Being-for-Self (Rajas) transcends Existence (Tamas):
    - Existence: Limited by external quality
    - Being-for-Self: Self-limiting, self-determining
    - Existence: Passive recipient of determination
    - Being-for-Self: Active self-determination
    - Existence: Bounded by other
    - Being-for-Self: Self-bounded, therefore infinite`;
  }

  /**
   * The Infinity of Being-for-Self
   */
  getInfinityStructure(): string {
    return `Being-for-Self achieves infinity through:
    - Self-relation: No external boundary
    - Rajas activity: Constantly overcoming limits
    - Dynamic process: Never static or fixed
    - Concrete universality: Universal that particularizes itself
    - True synthesis: Sattva-Tamas unity in constant motion`;
  }

  /**
   * Transition to Quantity
   */
  transitionToQuantity(): string {
    return `Being-for-Self (Rajas) transitions to Quantity because:
    - Self-relation becomes indifferent to specific qualities
    - Rajas activity seeks pure multiplicity
    - Quality becomes external, indifferent determination
    - The focus shifts from what something is to how much`;
  }

  /**
   * The Guna Synthesis Achieved
   */
  getGunasSynthesis(): string {
    return `Being-for-Self achieves Guna synthesis:
    - Sattva (Being): Pure luminous presence preserved
    - Tamas (Existence): Determinate limitation preserved
    - Rajas (Unity): Dynamic combination that is neither and both
    - Result: Concrete universal that contains its own determinateness`;
  }
}

/**
 * Self-Relation - The core structure of Being-for-Self
 */
export class SelfRelation {
  constructor(
    private being_aspect: Being,
    private existence_aspect: Existence
  ) {}

  describe(): string {
    return `Self-relation that unifies Being (${this.being_aspect.name}) and Existence (${this.existence_aspect.name}) in dynamic Rajas activity`;
  }

  /**
   * How self-relation works
   */
  getRelationalStructure(): string {
    return `Self-relation structure:
    - Self as subject: Being (Sattva) - pure presence
    - Self as object: Existence (Tamas) - determinate content
    - Relating process: Rajas - dynamic unification
    - Result: Subject-object identity in difference`;
  }

  /**
   * The infinity achieved through self-relation
   */
  achievesInfinity(): boolean {
    return true; // Self-relation overcomes external limitation
  }
}

/**
 * The One - Being-for-Self as absolute unity
 */
export class TheOne extends BeingForSelf {
  constructor() {
    super('the-one-absolute',
          new Being('being-in-one'),
          new Existence('existence-in-one'));
  }

  /**
   * The One as absolute Being-for-Self
   */
  getAbsoluteUnity(): string {
    return `The One as absolute Being-for-Self:
    - Completely self-related
    - Excludes all otherness (pure Rajas activity)
    - Contains Being and Existence as its moments
    - Is the perfect synthesis of all three Gunas`;
  }

  /**
   * How The One relates to multiplicity
   */
  relationToMany(): string {
    return `The One's relation to the Many:
    - Repels otherness (Rajas repulsion)
    - But this repulsion creates the Many
    - Each Many is itself a One
    - Leading to infinite regress and dialectical development`;
  }
}

// Factory functions
export const createBeingForSelf = (
  id?: string,
  being?: Being,
  existence?: Existence
): BeingForSelf => new BeingForSelf(id, being, existence);

export const createTheOne = (): TheOne => new TheOne();

// The absolute synthesis
export const THE_ONE_ABSOLUTE = createTheOne();
