/**
 * QUALITY - THE LOGIC OF BEING
 * ============================
 *
 * Translation of Hegel's Logic of Being from being.txt
 * Quality as Sattva - where PureBeing is Sat
 *
 * Structure: Quality → Being (PureBeing, Nothing, Becoming)
 */

// Base interface for dialectical moments
interface DialecticalMoment {
  name: string;
  description: string;
  getContradiction(): string;
  transitionTo(): DialecticalMoment | null;
}

/**
 * A. BEING (Pure Being) - SAT (Sattva)
 *
 * "Being, pure being, without further determination.
 * In its indeterminate immediacy it is equal only to itself"
 */
export class PureBeing implements DialecticalMoment {
  readonly name = "Pure Being";
  readonly description = "Pure being, without further determination. Indeterminate immediacy equal only to itself.";

  /**
   * The internal contradiction of Pure Being
   * "Being, the indeterminate immediate is in fact nothing"
   */
  getContradiction(): string {
    return `Pure Being's contradiction:
    - It is pure indeterminateness and emptiness
    - There is nothing to be intuited in it
    - It is only pure empty intuiting itself
    - Being, the indeterminate immediate is in fact nothing
    - Neither more nor less than nothing`;
  }

  /**
   * Pure Being immediately transitions to Nothing
   * Due to its complete indeterminateness
   */
  transitionTo(): Nothing {
    return new Nothing();
  }

  /**
   * Pure Being as Sat (existence principle of Sattva)
   */
  asSat(): string {
    return `Pure Being as SAT (Sattva):
    - Pure existence without qualification
    - Indeterminate immediacy
    - Equal only to itself
    - Free of all determinateness
    - The pure light of being itself`;
  }

  /**
   * The purity of Being - what it excludes
   */
  getPurity(): string {
    return `Pure Being maintains its purity by:
    - Having no difference within it, nor any outwardly
    - No determination or content posited in it as distinct
    - Not being posited as distinct from another
    - Pure indeterminateness and emptiness
    - Reflectionless being as it immediately is`;
  }
}

/**
 * B. NOTHING (Pure Nothingness)
 *
 * "Nothing, pure nothingness; it is simple equality with itself,
 * complete emptiness, complete absence of determination and content"
 */
export class Nothing implements DialecticalMoment {
  readonly name = "Nothing";
  readonly description = "Pure nothingness - simple equality with itself, complete emptiness, absence of determination.";

  /**
   * Nothing's identity with Being
   * "Nothing is therefore the same determination, and thus altogether the same as what pure being is"
   */
  getContradiction(): string {
    return `Nothing's contradiction with Being:
    - Nothing is simple equality with itself (like Being)
    - Complete emptiness (like Being's indeterminateness)
    - To think nothing has meaning - so nothing exists in thinking
    - Nothing is the empty thinking itself (like pure being)
    - Nothing is the same determination as pure being
    - Yet they must be distinguished`;
  }

  /**
   * Nothing transitions to Being (the same transition as Being to Nothing)
   */
  transitionTo(): PureBeing {
    return new PureBeing();
  }

  /**
   * Nothing as the practical negating principle
   */
  asNegation(): string {
    return `Nothing as pure negation:
    - Complete absence of determination and content
    - Lack of all distinction within
    - The empty intuiting and thinking itself
    - Makes a difference when something or nothing is thought
    - The practical principle that determines through negation`;
  }

  /**
   * The meaning of thinking Nothing
   */
  getMeaning(): string {
    return `To think Nothing has meaning because:
    - It makes a difference whether something or nothing is thought
    - Nothing exists in our intuiting or thinking
    - It is the empty intuiting and thinking itself
    - This gives Nothing concrete existence in consciousness`;
  }
}

/**
 * C. BECOMING (Unity of Being and Nothing)
 *
 * "Pure being and pure nothing are therefore the same.
 * The truth is neither being nor nothing, but rather that being has passed over into nothing"
 */
export class Becoming implements DialecticalMoment {
  readonly name = "Becoming";
  readonly description = "The unity of being and nothing - the movement of immediate vanishing of one into the other.";

  private comingToBe: ComingToBe;
  private ceasingToBe: CeasingToBe;

  constructor() {
    this.comingToBe = new ComingToBe();
    this.ceasingToBe = new CeasingToBe();
  }

  /**
   * Becoming contains its own contradiction
   * "It contradicts itself in itself, because what it unites within itself is self-opposed"
   */
  getContradiction(): string {
    return `Becoming's self-contradiction:
    - Unites being and nothing which are self-opposed
    - Such a union destroys itself
    - It is ceaseless unrest that collapses into quiescent result
    - Contains coming-to-be and ceasing-to-be which paralyze each other
    - Each moment sublates itself and becomes its opposite`;
  }

  /**
   * Becoming transitions to Existence (Determinate Being)
   * "This result is the unity of being and nothing that has become quiescent simplicity"
   */
  transitionTo(): null {
    // This would transition to Existence/Determinate Being
    // But that's the next level of descent
    return null;
  }

  /**
   * The two moments of Becoming
   */
  getMoments(): { comingToBe: ComingToBe; ceasingToBe: CeasingToBe } {
    return {
      comingToBe: this.comingToBe,
      ceasingToBe: this.ceasingToBe
    };
  }

  /**
   * The unity that is not abstract but determinate
   */
  getUnity(): string {
    return `Becoming as determinate unity:
    - Not the unity that abstracts from being and nothing
    - But the unity in which being and nothing equally are
    - They are unseparated but each is not (as vanishing)
    - They exist only as sublated moments
    - A movement where distinctions immediately dissolve`;
  }

  /**
   * The sublation of Becoming
   * "The equilibrium in which coming-to-be and ceasing-to-be are poised"
   */
  getSublation(): string {
    return `Becoming's sublation:
    - The equilibrium of coming-to-be and ceasing-to-be
    - Collects itself in quiescent unity
    - Being and nothing are in it only as vanishing
    - The vanishing of becoming itself
    - Results in unity of being and nothing as quiescent simplicity
    - This becomes Existence`;
  }
}

/**
 * Coming-to-Be (one direction of Becoming)
 * "Nothing is the immediate, refers to being, passes over into it"
 */
export class ComingToBe {
  readonly name = "Coming-to-Be";

  getDirection(): string {
    return `Coming-to-Be direction:
    - Nothing is the immediate
    - The determination begins with nothing
    - This refers to being and passes over into it
    - Nothing goes over into being
    - The movement from non-being to being`;
  }
}

/**
 * Ceasing-to-Be (other direction of Becoming)
 * "Being is the immediate, passes over into nothing"
 */
export class CeasingToBe {
  readonly name = "Ceasing-to-Be";

  getDirection(): string {
    return `Ceasing-to-Be direction:
    - Being is the immediate
    - The determination begins with being
    - This passes over into nothing
    - Being sublates itself and becomes passing-over into nothing
    - The movement from being to non-being`;
  }
}

/**
 * QUALITY - The overarching category containing Being
 *
 * "Being is the indeterminate immediate; it is free of determinateness"
 */
export class Quality {
  private being: PureBeing;
  private nothing: Nothing;
  private becoming: Becoming;

  constructor() {
    this.being = new PureBeing();
    this.nothing = new Nothing();
    this.becoming = new Becoming();
  }

  /**
   * Quality as determinateness itself
   * "The character of indeterminateness attaches to it only in opposition to what is determinate"
   */
  getQualityStructure(): string {
    return `Quality (Determinateness):
    - Being is indeterminate immediate
    - Free of determinateness with respect to essence
    - Reflectionless being as it immediately is
    - But indeterminateness constitutes its quality
    - Quality emerges in opposition to the determinate`;
  }

  /**
   * The dialectical development within Quality
   */
  getDialecticalMovement(): string {
    return `Quality's dialectical development:
    1. Pure Being (indeterminate immediate) - SAT principle
    2. Nothing (pure negation) - practical determining principle
    3. Becoming (unity of being and nothing) - movement
    4. → Transitions to Existence (determinate being)

    This shows that first being is in itself determinate
    and passes over into existence`;
  }

  /**
   * Get the three moments of Quality
   */
  getMoments(): { being: PureBeing; nothing: Nothing; becoming: Becoming } {
    return {
      being: this.being,
      nothing: this.nothing,
      becoming: this.becoming
    };
  }
}

// Export the main Quality class and its moments
export { Quality as default };
