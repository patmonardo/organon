/**
 * CONSTITUTION: Determination, Constitution, and Limit
 * ===================================================
 *
 * The second step of FINITUDE - how determination passes into constitution
 * and develops into limit as the contradiction driving something beyond itself.
 *
 * Based on Hegel's Logic, showing how:
 * - Determination is affirmative determinateness preserving something
 * - Constitution is external determinateness something is given over to
 * - Limit is the contradiction that makes something finite
 */

/**
 * Base interface for dialectical moments
 */
interface DialecticalMoment {
  dialecticalMovement(): string;
}

/**
 * Interface for moments that develop through determination-constitution-limit
 */
interface ConstitutionalDetermination extends DialecticalMoment {
  getDetermination(): string;
  getConstitution(): string;
  getLimit(): string;
  getTransitionToFinitude(): string;
}

/**
 * 1. DETERMINATION
 * ================
 *
 * Affirmative determinateness - the in-itself by which something abides
 * in its existence while involved with another that would determine it
 */
class Determination implements ConstitutionalDetermination {
  private inItself: string;
  private beingForOther: string;
  private filling: string;

  constructor() {
    this.inItself = "mediated through being-for-other, no longer abstract";
    this.beingForOther = "present in the in-itself as its moment";
    this.filling = "determinateness that accrues as something relates to other";
  }

  getDetermination(): string {
    return `Determination as affirmative determinateness:
    - The in-itself by which something abides in existence
    - While involved with other that would determine it
    - Preserves itself in self-equality through being-for-other
    - Something fulfills determination through filling
    - What something is in itself is also present in it`;
  }

  getConstitution(): string {
    return "Determination develops into constitution through externality";
  }

  getLimit(): string {
    return "Determination encounters limit in its relation to other";
  }

  getTransitionToFinitude(): string {
    return "Determination as foundation for finite development";
  }

  /**
   * Hegel's example: Human determination as rational thought
   */
  getHumanDetermination(): string {
    return `Human determination example:
    - Vocation is rational thought - simple determinateness
    - Distinguished from brute by thinking in himself
    - Thinking distinguished from being-for-other (sensuous nature)
    - But thinking is also in him - exists as thinking
    - Thinking is concrete existence and actuality
    - Must have content and filling - rational thought
    - Yet remains in-itself as ought against immediate existence`;
  }

  /**
   * The dialectical movement within determination
   */
  dialecticalMovement(): string {
    return `${this.getDetermination()}

    Human example: ${this.getHumanDetermination()}`;
  }
}

/**
 * 2. CONSTITUTION
 * ===============
 *
 * The filling of being-in-itself with determinateness that is distinct
 * from determination - external existence not belonging to being-in-itself
 */
class Constitution implements ConstitutionalDetermination {
  private externalInfluences: string;
  private alteration: string;
  private middleTerm: string;

  constructor() {
    this.externalInfluences = "external connection and being determined through other";
    this.alteration = "falls on side of constitution, not determination";
    this.middleTerm = "that which something has in it connects determination and constitution";
  }

  getDetermination(): string {
    return "Constitution emerges from determination's openness to relation with other";
  }

  getConstitution(): string {
    return `Constitution as external determinateness:
    - Filling of being-in-itself distinct from determination
    - External existence not belonging to being-in-itself
    - Something caught up in external influences and relationships
    - Appears accidental but is quality of something to have constitution
    - Alteration falls on constitution, not determination
    - Something preserves itself while constitution alters`;
  }

  getLimit(): string {
    return "Constitution develops toward limit through its instability";
  }

  getTransitionToFinitude(): string {
    return "Constitution shows something's finite nature through external determination";
  }

  /**
   * The relationship between determination and constitution
   */
  getDeterminationConstitutionRelation(): string {
    return `Determination and constitution relationship:
    - Distinct from each other yet connected
    - Something indifferent to constitution according to determination
    - Middle term: that which something has in it
    - Being-in-something falls apart into these extremes
    - Simple middle term is determinateness as such
    - Determination passes into constitution and vice versa`;
  }

  /**
   * How they pass into each other
   */
  getTransitionLogic(): string {
    return `Transition of determination into constitution:
    - Determination open to relation with other (being-for-other)
    - Contains qualitative distinction from being-in-itself
    - Holds other in itself united with being-in-itself
    - Introduces otherness into determination
    - Constitution depends on determination
    - External determining determined by immanent determination
    - Something alters along with its constitution`;
  }

  dialecticalMovement(): string {
    return `${this.getConstitution()}

    Relation: ${this.getDeterminationConstitutionRelation()}

    Transition: ${this.getTransitionLogic()}`;
  }
}

/**
 * 3. LIMIT
 * ========
 *
 * The determinateness that joins and separates two somethings
 * The contradiction that drives something beyond itself into finitude
 */
class Limit implements ConstitutionalDetermination {
  private negationOfNegation: string;
  private contradiction: string;
  private finitude: string;

  constructor() {
    this.negationOfNegation = "in-itselfness as negation of negation";
    this.contradiction = "limit holds moments of something and other as distinct";
    this.finitude = "something posited with immanent limit as contradiction";
  }

  getDetermination(): string {
    return "Limit emerges from determination's development through constitution";
  }

  getConstitution(): string {
    return "Limit as result of constitution's instability and externality";
  }

  getLimit(): string {
    return `Limit as determinate negation:
    - Non-being-for-other emphasized
    - Qualitative negation of other
    - Keeps other out of something reflected into itself
    - Internally reflected negation of something
    - Holds moments of something and other as distinct
    - Joins somethings together and separates them
    - One determinateness of two somethings`;
  }

  getTransitionToFinitude(): string {
    return `Transition to finitude:
    - Something posited with immanent limit
    - Contradiction of itself driving it beyond itself
    - Directed and driven out and beyond itself
    - The finite as result of this development`;
  }

  /**
   * The three moments of limit development
   */
  getLimitMoments(): string {
    return `Three moments of limit:

    (a) Limit as non-being of other:
    - Something has limit with respect to other
    - Limit is non-being of other, not of something itself
    - But other is also something - has same limit
    - Limit is non-being of both somethings
    - Yet through limit something IS
    - Limit is mediation whereby something and other both is and is not

    (b) Existence outside limit:
    - Something has existence outside its limit
    - Limit as middle point where they leave off
    - Non-existence and existence fall outside each other
    - Like line outside point, plane outside line
    - First occurs to figurative representation

    (c) Something only in its limit:
    - Unlimited something is only existence in general
    - Not distinguished from its other
    - Both have same determination as existence
    - Limit is their common distinguishedness
    - Something has existence only in limit
    - Points beyond itself to its non-being
    - Passes over into its other`;
  }

  /**
   * The spatial examples from Hegel
   */
  getSpatialExamples(): string {
    return `Spatial limit examples:
    - Point is limit of line but also its beginning
    - Line is limit of plane but also its element
    - Plane is limit of solid but also its element
    - Limits are principles of what they delimit
    - Point is dialectic of becoming line
    - Line is dialectic of becoming plane
    - Plane is dialectic of becoming total space
    - Self-contradictory beginnings repel themselves
    - No point, just as no line or plane (as fixed)`;
  }

  dialecticalMovement(): string {
    return `${this.getLimit()}

    Moments: ${this.getLimitMoments()}

    Spatial Examples: ${this.getSpatialExamples()}

    Result: ${this.getTransitionToFinitude()}`;
  }
}

/**
 * THE CONSTITUTION SYSTEM
 * =======================
 *
 * The complete development from determination through constitution to limit
 * Second step of FINITUDE showing how external determinateness develops
 */
class ConstitutionSystem {
  private determination: Determination;
  private constitution: Constitution;
  private limit: Limit;

  constructor() {
    this.determination = new Determination();
    this.constitution = new Constitution();
    this.limit = new Limit();
  }

  /**
   * The overall development
   */
  getOverallDevelopment(): string {
    return `Constitution System Development:

    Introduction: The in-itself no longer abstract but mediated
    - Identity by virtue of which something has present what it is in itself
    - Being-for-other present because in-itself is sublation of it
    - Determinateness existent-in-itself, immanently reflected

    1. DETERMINATION: ${this.determination.getDetermination()}

    2. CONSTITUTION: ${this.constitution.getConstitution()}

    3. LIMIT: ${this.limit.getLimit()}

    RESULT: ${this.limit.getTransitionToFinitude()}`;
  }

  /**
   * The dialectical progression
   */
  getDialecticalProgression(): string {
    return `Dialectical progression through constitution:

    Stage 1 - Determination:
    ${this.determination.dialecticalMovement()}

    Stage 2 - Constitution:
    ${this.constitution.dialecticalMovement()}

    Stage 3 - Limit:
    ${this.limit.dialecticalMovement()}`;
  }

  /**
   * The key insight about alteration
   */
  getAlterationInsight(): string {
    return `Key insight about alteration:
    - First alteration was only implicit (inner concept)
    - Now alteration is posited in the something
    - Something itself is further determined
    - Negation posited as immanent to it
    - As its developed being-in-itself
    - This prepares transition to finitude proper`;
  }

  /**
   * Access to individual moments
   */
  getDetermination(): Determination {
    return this.determination;
  }

  getConstitution(): Constitution {
    return this.constitution;
  }

  getLimit(): Limit {
    return this.limit;
  }
}

// Export the main classes
export {
  ConstitutionSystem as default,
  Determination,
  Constitution,
  Limit,
  ConstitutionalDetermination
};
