/**
 * QUALITATIVE LOGIC: Theory of Gunas (Qualities)
 * ==============================================
 *
 * Qualitative Logic is conditioned by Gunas for Content
 * - Being (Quality) = Sattva (Pure luminous existence)
 * - Existence = Tamas (Determinate, limited existence)
 * - Unity (Sanyoga) = Rajas (Dynamic combination of Sattva-Tamas)
 *
 * This is NOT Formal Logic abstracting from content,
 * but Logic of Qualities/Gunas as substantial determinations
 */

// Base Guna Types
export type Guna = 'sattva' | 'tamas' | 'rajas';

// Quality as Guna Structure (simplified without Zod dependency)
export interface QualityGuna {
  id: string;
  name: string;
  guna: Guna;

  // Qualitative Determinations (not formal abstractions)
  qualitative_content: {
    guna_character: string;              // What guna quality this embodies
    substantial_determination: string;    // Substantial content, not formal
    qualitative_limitation: string;      // How this quality limits/determines
  };

  // BEC Structure with Guna Integration
  being: {
    guna_being: string;                  // Being as guna-qualified
    qualitative_immediacy: boolean;      // Immediate but qualified
    guna_determination: string;          // How guna determines being
  };

  essence: {
    guna_essence: string;                // Essence as guna-mediated
    qualitative_reflection: boolean;     // Reflection through guna
    guna_mediation: string;              // How guna mediates essence
  };

  concept: {
    guna_concept: string;                // Concept as guna-universal
    qualitative_universality: string;   // Universal qualified by guna
    guna_synthesis: string;              // How guna enables synthesis
  };

  // Dialectical Relations
  emerges_from?: string;
  transitions_to?: string;
  dialectical_moment: 'thesis' | 'antithesis' | 'synthesis';

  description: string;
}

// The Three Primary Qualities as Gunas
export interface SattvaQuality extends QualityGuna {
  guna: 'sattva';
  being_character: 'pure_luminous_existence';
}

export interface TamasQuality extends QualityGuna {
  guna: 'tamas';
  existence_character: 'determinate_limited_existence';
}

export interface RajasQuality extends QualityGuna {
  guna: 'rajas';
  unity_character: 'dynamic_combination_sattva_tamas';
}

// Dialectical Categories as Guna-Qualified
export type QualitativeBeingCategory =
  | 'pure-being-sattva'
  | 'nothing-tamas'
  | 'becoming-rajas'
  | 'existence-tamas'
  | 'determinate-being-tamas'
  | 'being-for-self-sattva';

// Quality Characteristics as Guna Expressions
export interface GunaCharacteristic {
  name: string;
  description: string;
  guna_expression: Guna;
  qualitative_content: string;               // Substantial guna content
}
