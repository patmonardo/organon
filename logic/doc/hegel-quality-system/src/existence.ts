/**
 * EXISTENCE: The Tamas Guna - Determinate Limited Being
 * ======================================================
 *
 * From Hegel's Greater Logic on Existence:
 * "Existence is Being with a determinateness"
 *
 * As Tamas Guna:
 * - Determinate, limited, bounded existence
 * - Being that has emerged from pure indeterminateness
 * - Quality as limitation and negation
 */

import { QualityGuna, TamasQuality, Guna } from './types/being';
import { Being } from './being';

export class Existence implements TamasQuality {
  readonly id: string;
  readonly name: string;
  readonly guna: 'tamas' = 'tamas';
  readonly existence_character: 'determinate_limited_existence' = 'determinate_limited_existence';

  // Qualitative Content - Tamas as Limitation
  readonly qualitative_content = {
    guna_character: 'tamas_as_determinate_limitation',
    substantial_determination: 'being_bounded_by_quality',
    qualitative_limitation: 'tamas_introduces_negation_and_finitude'
  };

  // BEC Structure with Tamas Integration
  readonly being = {
    guna_being: 'being_determined_by_tamas_limitation',
    qualitative_immediacy: false, // No longer pure immediacy
    guna_determination: 'tamas_determines_being_through_negation'
  };

  readonly essence = {
    guna_essence: 'essence_emerging_through_tamas_mediation',
    qualitative_reflection: true, // Reflection begins with limitation
    guna_mediation: 'tamas_mediates_through_boundary_and_limit'
  };

  readonly concept = {
    guna_concept: 'concept_as_tamas_particularity',
    qualitative_universality: 'universal_particularized_through_limits',
    guna_synthesis: 'tamas_enables_synthesis_through_negation'
  };

  // Dialectical Position
  readonly dialectical_moment: 'antithesis' = 'antithesis';
  readonly emerges_from = 'pure-being-sattva';
  readonly transitions_to = 'being-for-self-rajas';

  readonly description = 'Existence as Tamas Guna - determinate being that emerges through limitation and quality';

  private quality: QualitativeProperty;
  private limitation: Boundary;

  constructor(id: string = 'existence-tamas', quality?: QualitativeProperty) {
    this.id = id;
    this.name = 'Existence (Tamas)';
    this.quality = quality || new QualitativeProperty('default-quality');
    this.limitation = new Boundary(this.quality);
  }

  /**
   * Existence as Determinate Being
   * Unlike pure Being, Existence has qualities that limit it
   */
  getDeterminateness(): string {
    return `Existence as Tamas is determinate because:
    - It has quality (${this.quality.name}) that limits it
    - Tamas introduces boundaries and negation
    - It is no longer pure, unlimited being
    - Each quality excludes other qualities`;
  }

  /**
   * The Role of Quality in Existence
   */
  getQualitativeStructure(): string {
    return `Quality in Existence (Tamas):
    - Quality: ${this.quality.describe()}
    - Limitation: ${this.limitation.describe()}
    - Negation: Each quality negates what it is not
    - Finitude: Quality makes existence finite and bounded`;
  }

  /**
   * Existence's Transition to Being-for-Self
   */
  transitionToBeingForSelf(): string {
    return `Existence (Tamas) transitions to Being-for-Self (Rajas) because:
    - Determinate existence seeks to overcome its limitations
    - Tamas inertia generates Rajas activity
    - The bounded seeks to relate to itself as unbounded
    - Quality becomes self-relating quality`;
  }

  /**
   * The Contradiction in Existence
   */
  getInternalContradiction(): string {
    return `Existence's contradiction as Tamas:
    - Exists only through quality/limitation (Tamas)
    - But seeks to be unlimited like pure Being (Sattva)
    - Each quality both determines and negates existence
    - Tamas both enables and constrains existence`;
  }

  /**
   * How Existence Differs from Pure Being
   */
  differenceFromBeing(being: Being): string {
    return `Existence (Tamas) differs from Being (Sattva):
    - Being: Pure, unlimited, indeterminate (Sattva)
    - Existence: Qualified, limited, determinate (Tamas)
    - Being: No negation or boundaries
    - Existence: Constituted by negation and quality
    - Being: Pure immediacy
    - Existence: Mediated through quality`;
  }
}

/**
 * Qualitative Property - What gives Existence its determinateness
 */
export class QualitativeProperty {
  constructor(
    public readonly name: string,
    public readonly description: string = '',
    public readonly guna_aspect: Guna = 'tamas'
  ) {}

  describe(): string {
    return `Quality "${this.name}" (${this.guna_aspect}): ${this.description}`;
  }

  negates(other: QualitativeProperty): boolean {
    return this.name !== other.name;
  }
}

/**
 * Boundary - How quality limits existence
 */
export class Boundary {
  constructor(private quality: QualitativeProperty) {}

  describe(): string {
    return `Boundary established by quality "${this.quality.name}" - excludes all other qualities`;
  }

  getLimitation(): string {
    return `This existence is limited to being ${this.quality.name} and not being anything else`;
  }
}

// Factory functions
export const createExistence = (id?: string, quality?: QualitativeProperty): Existence =>
  new Existence(id, quality);

export const createQuality = (name: string, description?: string): QualitativeProperty =>
  new QualitativeProperty(name, description);

// Example existences with different qualities
export const RED_EXISTENCE = createExistence('red-existence', createQuality('red', 'The quality of redness'));
export const ROUND_EXISTENCE = createExistence('round-existence', createQuality('round', 'The quality of roundness'));
export const FINITE_EXISTENCE = createExistence('finite-existence', createQuality('finite', 'The quality of having limits'));
