/**
 * FINITUDE: The Self-Contradictory Nature of Finite Being
 * ======================================================
 *
 * The third and final step of FINITUDE - where finite being reveals itself
 * as essentially self-negating, containing the germ of its own transgression.
 *
 * Based on Hegel's Logic, showing how:
 * - Finitude is qualitative negation driven to the extreme
 * - The finite contains restriction and ought as contradictory moments
 * - Through self-contradiction, the finite transitions into the infinite
 *
 * "The hour of their birth is the hour of their death"
 */

/**
 * Base interface for dialectical moments
 */
interface DialecticalMoment {
  dialecticalMovement(): string;
}

/**
 * Interface for finite determinations that contain their own contradiction
 */
interface FiniteDetermination extends DialecticalMoment {
  getFiniteNature(): string;
  getContradiction(): string;
  getTransitionToInfinite(): string;
}

/**
 * THE ESSENCE OF FINITUDE
 * =======================
 *
 * Non-being constitutes the nature and being of finite things
 */
class FinitudeEssence implements FiniteDetermination {
  private qualitativeNegation: string;
  private immanentLimit: string;
  private perishingNature: string;

  constructor() {
    this.qualitativeNegation = "negation driven to the extreme";
    this.immanentLimit = "limit immanent to existence as in-itselfness";
    this.perishingNature = "germ of transgression in their in-itselfness";
  }

  getFiniteNature(): string {
    return `The essence of finitude:
    - Existence is determinate - something has quality and is delimited
    - Quality is its limit, affected by it remains affirmative existence
    - But developed so opposition of existence and negation as limit
    - IS the very in-itselfness of something - thus only becoming in it
    - Non-being constitutes their nature, their being
    - Finite things refer to themselves negatively in self-reference
    - They propel themselves beyond themselves, beyond their being
    - Truth of their being is their finis, their end
    - The hour of their birth is the hour of their death`;
  }

  getContradiction(): string {
    return `The fundamental contradiction:
    - Finite does not just alter but perishes
    - Perishing not mere possibility but essential nature
    - Being as such of finite things: germ of transgression in in-itselfness
    - They are, yet refer to themselves as non-being
    - Self-reference becomes self-propulsion beyond themselves`;
  }

  getTransitionToInfinite(): string {
    return "Through self-contradiction, finitude reveals its infinite nature";
  }

  dialecticalMovement(): string {
    return `${this.getFiniteNature()}

    Contradiction: ${this.getContradiction()}`;
  }
}

/**
 * (a) THE IMMEDIACY OF FINITUDE
 * =============================
 *
 * Finitude as the most obstinate category of understanding
 */
class ImmediateFinitude implements FiniteDetermination {
  private mournfulNote: string;
  private obstinateCategory: string;
  private understandingsPersistence: string;

  constructor() {
    this.mournfulNote = "qualitative negation driven to extreme";
    this.obstinateCategory = "negation fixed in itself, stark contrast to affirmative";
    this.understandingsPersistence = "makes non-being imperishable and absolute";
  }

  getFiniteNature(): string {
    return `The immediacy of finitude - mournful note:
    - Qualitative negation driven to the extreme
    - No longer affirmative being distinct from determination
    - Things destined to ruin through qualitative simplicity of negation
    - Abstract opposition of nothing and perishing to being
    - Most obstinate of categories of understanding
    - Negation fixed in itself, stark contrast to its affirmative`;
  }

  getContradiction(): string {
    return `Understanding's obstinate contradiction:
    - Finite lets itself be submitted to flux - comes to an end
    - But refuses affirmative connection to infinite
    - Inseparably posited with its nothing
    - Cut off from reconciliation with affirmative
    - Understanding makes non-being imperishable and absolute
    - Finitude becomes eternal through this fixation
    - "The finite is absolutely opposed to the infinite"`;
  }

  getTransitionToInfinite(): string {
    return `The critical insight:
    - But no philosophy endorses finite as absolute
    - Question: does transitoriness persist or does perishing perish?
    - If finite not to persist but perish into affirmative
    - Then perishing would be perishing of perishing
    - Contradiction must be brought to consciousness
    - Development will show finite collapses internally
    - In collapse, actually resolves the contradiction
    - Perishing is not the last - perishing rather perishes`;
  }

  dialecticalMovement(): string {
    return `${this.getFiniteNature()}

    Contradiction: ${this.getContradiction()}

    Resolution: ${this.getTransitionToInfinite()}`;
  }
}

/**
 * (b) RESTRICTION AND THE OUGHT
 * =============================
 *
 * The finite as connecting determination and limit through ought and restriction
 */
class RestrictionAndOught implements FiniteDetermination {
  private restriction: string;
  private ought: string;
  private doubleDetermination: string;
  private indivisibleConnection: string;

  constructor() {
    this.restriction = "something's own limit posited as negative which is essential";
    this.ought = "negative reference to limit, reference to itself as restriction";
    this.doubleDetermination = "in-itselfness over against negation, and non-being as restriction";
    this.indivisibleConnection = "ought elevated above restriction, has restriction only as ought";
  }

  getFiniteNature(): string {
    return `Restriction and ought as finite structure:
    - Something reflected into itself with limit within it
    - Limit as immanent to something, quality of its being-in-itself, is finitude
    - Determination already contained otherness in in-itself
    - Externality of otherness within something's own inwardness
    - Otherness as limit = negation of negation
    - Unity of something with itself through negating immanent limit
    - Self-identical in-itself refers to itself as own non-being`;
  }

  getContradiction(): string {
    return `The ought-restriction dialectic:

    RESTRICTION:
    - Something's own limit posited as essential negative
    - Not only limit as such, but restriction
    - For limit to be restriction, something must transcend it in itself
    - Must refer to it from within as non-existent
    - Something transcends limit as sublatedness of limit

    OUGHT:
    - Negative reference to limit distinguished from itself
    - In-itself as negative reference to itself as restriction
    - Contains double determination:
      * Determination with in-itselfness over against negation
      * Non-being as restriction, itself determination existing in itself

    FINITE CONNECTION:
    - Finite determined as connecting determination and limit
    - Determination is ought, limit is restriction
    - Both moments finite - ought as well as restriction
    - Only restriction posited as finite
    - Ought restricted only in itself, only for us`;
  }

  getTransitionToInfinite(): string {
    return `The contradictory unity:
    - "What ought to be is, and at the same time is not"
    - If it were, would not be what merely ought to be
    - Ought has restriction essentially - not anything alien
    - Finite's own determination is also its restriction
    - As ought, finite transcends restriction
    - Same determinateness which is negation is also sublated
    - Ought elevated above restriction, but has restriction only as ought
    - The two are indivisible - contradiction in unity`;
  }

  dialecticalMovement(): string {
    return `${this.getFiniteNature()}

    Dialectic: ${this.getContradiction()}

    Unity: ${this.getTransitionToInfinite()}`;
  }
}

/**
 * (c) TRANSITION OF THE FINITE INTO THE INFINITE
 * ==============================================
 *
 * The self-sublation of finite contradiction into infinite affirmation
 */
class TransitionToInfinite implements FiniteDetermination {
  private selfContradiction: string;
  private negationOfNegation: string;
  private infiniteResult: string;

  constructor() {
    this.selfContradiction = "finite in itself contradiction of itself";
    this.negationOfNegation = "identity with itself as affirmative being";
    this.infiniteResult = "other of finite as negation of the negation";
  }

  getFiniteNature(): string {
    return `The finite as self-contradiction:
    - Ought contains restriction explicitly, for itself
    - Restriction contains the ought
    - Their mutual connection IS the finite itself
    - Contains both in its in-itself
    - Moments qualitatively opposed:
      * Restriction = negative of ought
      * Ought = negative of restriction
    - Finite thus in itself contradiction of itself`;
  }

  getContradiction(): string {
    return `The self-sublating movement:
    - Finite sublates itself, goes away and ceases to be
    - But this result, negative as such, IS its very determination
    - For it is negative of the negative
    - In going away, finite has not ceased
    - Only become momentarily another finite
    - Going-away as going-over into another finite
    - And so forth to infinity (bad infinite)`;
  }

  getTransitionToInfinite(): string {
    return `The true result - transition to infinite:
    - Consider result more closely:
    - In going-away and ceasing-to-be, in negation of itself
    - Finite has attained its being-in-itself
    - In it, has rejoined itself

    Each moment contains this result:
    - Ought transcends restriction = transcends itself
    - But its beyond/other is only restriction itself
    - Restriction points beyond itself to ought
    - But ought is same diremption as restriction
    - In going beyond itself, restriction rejoins itself

    THE INFINITE:
    - This identity with itself = negation of negation
    - Is affirmative being = other of finite
    - Finite supposed to have first negation for determinateness
    - This other is the INFINITE
    - True result of finite's self-contradiction`;
  }

  dialecticalMovement(): string {
    return `${this.getFiniteNature()}

    Self-Sublation: ${this.getContradiction()}

    True Result: ${this.getTransitionToInfinite()}`;
  }
}

/**
 * THE COMPLETE FINITUDE SYSTEM
 * ============================
 *
 * The full development of finite being as self-contradictory
 * and self-sublating into infinite
 */
class FinitudeSystem {
  private essence: FinitudeEssence;
  private immediacy: ImmediateFinitude;
  private restrictionOught: RestrictionAndOught;
  private transition: TransitionToInfinite;

  constructor() {
    this.essence = new FinitudeEssence();
    this.immediacy = new ImmediateFinitude();
    this.restrictionOught = new RestrictionAndOught();
    this.transition = new TransitionToInfinite();
  }

  /**
   * The complete dialectical development
   */
  getCompleteFinitudeDevelopment(): string {
    return `FINITUDE - The Self-Contradictory Nature of Finite Being:

    ESSENCE OF FINITUDE:
    ${this.essence.dialecticalMovement()}

    (a) IMMEDIACY OF FINITUDE:
    ${this.immediacy.dialecticalMovement()}

    (b) RESTRICTION AND THE OUGHT:
    ${this.restrictionOught.dialecticalMovement()}

    (c) TRANSITION TO INFINITE:
    ${this.transition.dialecticalMovement()}`;
  }

  /**
   * The profound insight about finite and infinite
   */
  getFiniteInfiniteInsight(): string {
    return `The Finite-Infinite Dialectical Truth:

    - Finite is not simply opposed to infinite
    - Finite contains infinite as its own truth
    - Through self-contradiction, finite becomes infinite
    - "Perishing of the perishing" reveals true infinite
    - Negation of negation = affirmative being
    - The infinite is not beyond finite but its own result
    - Hour of birth = hour of death, but death dies into life
    - This prepares the logic of true infinite vs spurious infinite`;
  }

  /**
   * Hegel's famous formulation
   */
  getHegelianFormula(): string {
    return `Hegel's Essential Formulation:
    "The hour of their birth is the hour of their death"

    But dialectically understood:
    - Death is not the end but the beginning
    - Finite perishes into its truth - the infinite
    - Self-negation becomes self-affirmation
    - The finite IS the infinite in its truth

    This is the foundation of all dialectical logic:
    Contradiction is not destruction but self-development`;
  }

  /**
   * Access to individual moments
   */
  getEssence(): FinitudeEssence {
    return this.essence;
  }

  getImmediacy(): ImmediateFinitude {
    return this.immediacy;
  }

  getRestrictionOught(): RestrictionAndOught {
    return this.restrictionOught;
  }

  getTransition(): TransitionToInfinite {
    return this.transition;
  }
}

// Export the main classes
export {
  FinitudeSystem as default,
  FinitudeEssence,
  ImmediateFinitude,
  RestrictionAndOught,
  TransitionToInfinite,
  FiniteDetermination
};
