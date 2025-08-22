/**
 * THE ONE AND THE MANY: The First Problem of Metaphysics
 * ======================================================
 *
 * The dialectical solution to the eternal metaphysical problem of how
 * the One relates to the Many. Shows how The One necessarily generates
 * multiplicity through repulsion while maintaining self-reference.
 *
 * Based on Hegel's Logic, showing how:
 * - The One turns ideality into reality (most fixed and abstract)
 * - The One contains nothing as the void (its own quality)
 * - Repulsion as the One's self-generation of many ones
 * - Infinity as contradiction that unconstrainedly produces itself
 *
 * "The plurality of the ones is infinity as a contradiction
 *  that unconstrainedly produces itself."
 */

/**
 * Base interface for dialectical moments
 */
interface DialecticalMoment {
  dialecticalMovement(): string;
}

/**
 * Interface for One-Many determinations
 */
interface OneManyDetermination extends DialecticalMoment {
  getUnityStructure(): string;
  getMultiplicityGeneration(): string;
  getRepulsionNature(): string;
}

/**
 * THE ONE AS IMMEDIATE REALITY
 * ============================
 *
 * How ideality turns into the most fixed and abstract reality
 */
class OneAsImmediateReality implements OneManyDetermination {
  private idealityToReality: string;
  private infiniteSelfDetermining: string;
  private excludingOther: string;

  constructor() {
    this.idealityToReality = "ideality of being-for-itself turns into reality";
    this.infiniteSelfDetermining = "self-reference as infinite self-determining";
    this.excludingOther = "excluding of the one as other from itself";
  }

  getUnityStructure(): string {
    return `The One as simple reference to itself:
    - Simple reference of being-for-itself to itself
    - Its moments have fallen together
    - Being-for-itself has form of immediacy
    - Moments are now there as existents
    - As self-reference of negative, the one is determining
    - As self-reference, it is infinite self-determining`;
  }

  getMultiplicityGeneration(): string {
    return `From ideality to reality - the metaphysical transformation:
    - Because of present immediacy, distinctions no longer only moments
    - But at same time posited as existents
    - Ideality of being-for-itself as totality turns into reality
    - A reality of most fixed and abstract kind: THE ONE
    - The One = posited unity of being and existence
    - Absolute union of reference to another and reference to itself`;
  }

  getRepulsionNature(): string {
    return `The contradiction that generates otherness:
    - Determinateness of being enters opposition to infinite negation
    - What the one is in itself, it is now only IN it
    - The negative consequently is an other distinct from it
    - What shows itself distinct from one is one's own self-determining
    - Unity with itself, as distinct from itself, demoted to reference
    - As negative unity: negation of itself as other
    - The EXCLUDING of the one as other from itself`;
  }

  dialecticalMovement(): string {
    return `${this.getUnityStructure()}

    Ideality→Reality: ${this.getMultiplicityGeneration()}

    Self-Exclusion: ${this.getRepulsionNature()}`;
  }
}

/**
 * THE ONE WITHIN
 * ==============
 *
 * The One's pure self-containment and its internal nothing
 */
class OneWithin implements OneManyDetermination {
  private pureWithinness: string;
  private unalterableNature: string;
  private internalNothing: string;

  constructor() {
    this.pureWithinness = "within it, the one just is";
    this.unalterableNature = "not capable of becoming any other, unalterable";
    this.internalNothing = "in the one there is nothing - posited as the void";
  }

  getUnityStructure(): string {
    return `The One's pure being within itself:
    - Within it, the one just IS
    - This being is not an existence
    - Not determination as reference to other
    - Not a constitution
    - Rather its having negated this circle of categories
    - The one not capable of becoming any other - UNALTERABLE`;
  }

  getMultiplicityGeneration(): string {
    return `The One's determinate indeterminateness:
    - It is indeterminate, yet no longer like being
    - Its indeterminateness = determinateness of self-reference
    - Absolutely determined being, posited in-itselfness
    - As self-referring negation, has distinction in it
    - Directs away from itself toward another
    - But direction immediately reversed - no other to address
    - Directing reverts back to itself`;
  }

  getRepulsionNature(): string {
    return `The internal nothing as void:
    - In simple immediacy, mediation of existence and ideality vanished
    - All diversity and manifoldness have vanished
    - IN THE ONE THERE IS NOTHING
    - This nothing = abstraction of self-reference
    - Distinguished from in-itselfness of the one
    - A POSITED nothing (in-itselfness now concrete through mediation)
    - This nothing, posited as in the one, is nothing as THE VOID
    - The void is thus the QUALITY of the one in its immediacy`;
  }

  dialecticalMovement(): string {
    return `${this.getUnityStructure()}

    Self-Reference: ${this.getMultiplicityGeneration()}

    Internal Void: ${this.getRepulsionNature()}`;
  }
}

/**
 * THE ONE AND THE VOID
 * ====================
 *
 * The external relationship between One and Void as existence
 */
class OneAndVoid implements OneManyDetermination {
  private abstractSelfReference: string;
  private absoluteDiversity: string;
  private externalExistence: string;

  constructor() {
    this.abstractSelfReference = "one is void as abstract self-reference of negation";
    this.absoluteDiversity = "void absolutely diverse from simple immediacy of one";
    this.externalExistence = "being-for-itself has again acquired an existence";
  }

  getUnityStructure(): string {
    return `The One as void in abstract self-reference:
    - The one IS the void as abstract self-reference of negation
    - But void, as nothing, absolutely diverse from simple immediacy of one
    - From being of latter which is also affirmative
    - Two stand in one single reference (namely to the one)
    - Their diversity is posited`;
  }

  getMultiplicityGeneration(): string {
    return `Void outside the One as existent:
    - As distinct from affirmative being
    - Nothing stands as void OUTSIDE the one as existent
    - Being-for-itself, determined as one and void
    - Has again acquired an EXISTENCE
    - Unity withdraws to one side, lowered to existence
    - Confronted by its other determination: negation as void`;
  }

  getRepulsionNature(): string {
    return `Common terrain of negative self-reference:
    - One and void have negative self-reference as common terrain
    - Moments of being-for-itself come out of this unity
    - Become external to themselves
    - Through simple unity, determination of being comes into play
    - Unity thus confronted by other determination
    - Standing over against it as existence of nothing: THE VOID`;
  }

  dialecticalMovement(): string {
    return `${this.getUnityStructure()}

    External Void: ${this.getMultiplicityGeneration()}

    Negative Terrain: ${this.getRepulsionNature()}`;
  }
}

/**
 * MANY ONES - REPULSION
 * =====================
 *
 * The generation of multiplicity through the One's self-repulsion
 */
class ManyOnesRepulsion implements OneManyDetermination {
  private becomingManyOnes: string;
  private repulsionProper: string;
  private infiniteContradiction: string;

  constructor() {
    this.becomingManyOnes = "one is consequently a becoming of many ones";
    this.repulsionProper = "one repels itself from itself - repulsion";
    this.infiniteContradiction = "plurality as infinity that unconstrainedly produces itself";
  }

  getUnityStructure(): string {
    return `From One and Void to Many Ones:
    - One and void constitute first existence of being-for-itself
    - Each has negation for determination, posited as existence
    - One = negation in determination of being
    - Void = negation in determination of non-being
    - Essentially, one is only self-reference as referring negation
    - It is itself same as void outside it is supposed to be`;
  }

  getMultiplicityGeneration(): string {
    return `The One becomes Many Ones:
    - Being-for-itself of one = ideality of existence and other
    - Does not refer to other but only to itself
    - But inasmuch as being-for-itself fixed as the one
    - As existent for itself, immediately present
    - Its negative reference to itself = reference to an existent
    - Since reference is negative, what it refers to remains other
    - As essentially self-reference, other is not void but likewise A ONE
    - THE ONE IS CONSEQUENTLY A BECOMING OF MANY ONES`;
  }

  getRepulsionNature(): string {
    return `Repulsion as self-generation:
    - Strictly speaking, not just becoming (being→nothing transition)
    - The one becomes only a one
    - One contains negative as reference, has this reference in it
    - Instead of becoming: one's own immanent reference present
    - Since reference negative and one is existent
    - THE ONE REPELS ITSELF FROM ITSELF
    - This negative reference of one to itself IS REPULSION

    Repulsion according to concept:
    - Positing of many ones through the one itself
    - One's own coming-forth-from-itself
    - To such outside it as are themselves only ones
    - Distinguished from external repulsion (mutual holding off)`;
  }

  /**
   * The determinations of the many ones
   */
  getManyOnesDeterminations(): string {
    return `The Many Ones as presupposed-posited:
    - Becoming of many immediately vanishes as product of positing
    - What is produced are ones, not for another
    - But as infinitely referring to themselves
    - One repels only itself from itself - it already IS
    - Repelled is equally a one, an existent
    - Repelling and being repelled applies equally - no difference

    PRESUPPOSED-POSITED STRUCTURE:
    - Ones presupposed with respect to each other
    - Posited through repulsion of one from itself
    - But presupposed = posited as non-posited
    - Their being-posited is sublated
    - They are existents referring only to themselves`;
  }

  /**
   * Plurality as external determination
   */
  getPluralityExternal(): string {
    return `Plurality completely external to the One:
    - Plurality appears not as otherness
    - But as determination completely external to one
    - One in repelling itself remains reference to itself
    - That ones are other to one another does not concern the one
    - Their connecting reference determined as NONE
    - It is again the previously posited VOID
    - Void is their limit but external limit
    - They are not supposed to be for one another in it`;
  }

  /**
   * The infinite contradiction
   */
  getInfiniteContradiction(): string {
    return `Infinity as self-producing contradiction:
    - Repulsion of one from itself = making explicit what one is implicitly
    - But laid out as one-outside-other
    - Infinity here = infinity that has externalized itself
    - Through immediacy of the infinite, of the one

    THE CONTRADICTION:
    - Infinity is simple reference of one to one
    - AND EQUALLY one's absolute lack of reference
    - Former according to affirmative reference to itself
    - Latter according to same reference as negative

    FINAL RESULT:
    - Plurality of ones = one's own positing of the one
    - One is nothing but negative reference of one to itself
    - This reference IS the plural one
    - But equally, plurality utterly external to one
    - For one is precisely sublating of otherness
    - THE PLURALITY OF THE ONES IS INFINITY AS A CONTRADICTION
    - THAT UNCONSTRAINEDLY PRODUCES ITSELF`;
  }

  dialecticalMovement(): string {
    return `${this.getUnityStructure()}

    Many Ones Generation: ${this.getMultiplicityGeneration()}

    Repulsion Proper: ${this.getRepulsionNature()}

    Many Ones Structure: ${this.getManyOnesDeterminations()}

    External Plurality: ${this.getPluralityExternal()}

    INFINITE CONTRADICTION: ${this.getInfiniteContradiction()}`;
  }
}

/**
 * THE ONE AND MANY SYSTEM
 * =======================
 *
 * Complete dialectical solution to the first problem of metaphysics
 */
class OneAndManySystem {
  private immediateReality: OneAsImmediateReality;
  private oneWithin: OneWithin;
  private oneVoid: OneAndVoid;
  private manyOnes: ManyOnesRepulsion;

  constructor() {
    this.immediateReality = new OneAsImmediateReality();
    this.oneWithin = new OneWithin();
    this.oneVoid = new OneAndVoid();
    this.manyOnes = new ManyOnesRepulsion();
  }

  /**
   * Complete dialectical development of One and Many
   */
  getCompleteOneAndManyDevelopment(): string {
    return `THE ONE AND THE MANY - Solution to First Problem of Metaphysics:

    INTRODUCTION - THE ONE AS IMMEDIATE REALITY:
    ${this.immediateReality.dialecticalMovement()}

    a. THE ONE WITHIN:
    ${this.oneWithin.dialecticalMovement()}

    b. THE ONE AND THE VOID:
    ${this.oneVoid.dialecticalMovement()}

    c. MANY ONES - REPULSION:
    ${this.manyOnes.dialecticalMovement()}`;
  }

  /**
   * The metaphysical revolution
   */
  getMetaphysicalRevolution(): string {
    return `HEGEL'S SOLUTION TO THE ONE-MANY PROBLEM:

    TRADITIONAL METAPHYSICS:
    - One and Many as external, opposed principles
    - How can unity generate multiplicity?
    - How can many relate to the one?
    - Endless debates about participation, emanation, etc.

    HEGELIAN DIALECTICAL SOLUTION:
    - The One necessarily generates the Many through self-repulsion
    - Repulsion is the One's own self-determining activity
    - Many Ones are the One's own self-externalization
    - Plurality is both internal to and external to the One
    - "Infinity as contradiction that unconstrainedly produces itself"

    KEY INSIGHTS:
    1. IDEALITY→REALITY: Being-for-itself becomes fixed as The One
    2. INTERNAL VOID: The One contains nothing as its own quality
    3. SELF-REPULSION: One generates Many through negative self-reference
    4. INFINITE CONTRADICTION: Unity and plurality in perpetual tension

    This solves the problem by showing it as necessary self-development!`;
  }

  /**
   * Preparation for Attraction
   */
  getPreparationForAttraction(): string {
    return `Preparation for Attraction:

    Repulsion has established:
    - Many Ones as presupposed-posited
    - Each referring only to itself
    - Plurality as external determination
    - Void as their connecting non-reference
    - Infinite contradiction of unity and multiplicity

    This sets up the necessity for:
    - Attraction as the sublation of repulsion
    - Unity reasserting itself over multiplicity
    - The dialectical interplay of repulsion and attraction
    - Leading to equilibrium and transition to Quantity

    The eternal metaphysical problem finds its dynamic resolution!`;
  }

  /**
   * Access to individual moments
   */
  getImmediateReality(): OneAsImmediateReality {
    return this.immediateReality;
  }

  getOneWithin(): OneWithin {
    return this.oneWithin;
  }

  getOneVoid(): OneAndVoid {
    return this.oneVoid;
  }

  getManyOnes(): ManyOnesRepulsion {
    return this.manyOnes;
  }
}

// Export the main classes
export {
  OneAndManySystem as default,
  OneAsImmediateReality,
  OneWithin,
  OneAndVoid,
  ManyOnesRepulsion,
  OneManyDetermination
};
