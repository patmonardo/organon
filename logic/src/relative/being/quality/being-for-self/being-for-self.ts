/**
 * BEING-FOR-ITSELF: The Completion of Qualitative Being
 * =====================================================
 *
 * Chapter 3 - The mystical culmination of Quality where qualitative being
 * is brought to completion as infinite being through self-reference.
 *
 * Based on Hegel's Logic, capturing the profound movement from:
 * - Existence (sphere of differentiation, dualism, finitude)
 * - To Being-for-Self (equalization of being and negation)
 * - The mystical utterances about I, Spirit, God as idealizations
 *
 * "In being-for-itself, qualitative being is brought to completion;
 *  it is infinite being; absolutely determinate being."
 */

/**
 * Base interface for dialectical moments
 */
interface DialecticalMoment {
  dialecticalMovement(): string;
}

/**
 * Interface for being-for-self determinations
 */
interface BeingForSelfDetermination extends DialecticalMoment {
  getInfiniteNature(): string;
  getSelfReference(): string;
  getIdealization(): string;
}

/**
 * THE COMPLETION OF QUALITATIVE BEING
 * ===================================
 *
 * Introduction to Being-for-Self as infinite being
 */
class QualitativeCompletion implements BeingForSelfDetermination {
  private infiniteBeing: string;
  private existenceVsBeingForSelf: string;
  private absolutelyDeterminate: string;

  constructor() {
    this.infiniteBeing = "qualitative being brought to completion as infinite being";
    this.existenceVsBeingForSelf = "existence as sphere of differentiation vs being-for-self as equalization";
    this.absolutelyDeterminate = "negation as simple self-reference, equalization with being";
  }

  getInfiniteNature(): string {
    return `Being-for-itself as infinite being:
    - Qualitative being brought to completion
    - It IS infinite being (not merely related to infinite)
    - Being of beginning was void of determination
    - Existence = sublated but only immediately sublated being
    - Contains only first negation, itself immediate
    - Being retained, united in existence in simple unity
    - But each still unlike other, unity not posited`;
  }

  getSelfReference(): string {
    return `From differentiation to equalization:
    - Existence = sphere of differentiation, dualism, domain of finitude
    - Determinateness as determinateness as such
    - Being which is relatively, not absolutely, determined
    - In being-for-itself: distinction between being and negation POSITED AND EQUALIZED
    - Quality, otherness, limit, reality, ought = incomplete configurations
    - Based on differentiation of the two
    - But negation passed over into infinity as negation of negation
    - Negation becomes simple self-reference
    - Therefore equalization with being: ABSOLUTELY DETERMINATE BEING`;
  }

  getIdealization(): string {
    return `The three-fold structure:
    1. Being-for-itself immediately as existent-for-itself: THE ONE
    2. One passes into multiplicity of ones: REPULSION → ATTRACTION
    3. Alternating determination of repulsion and attraction
    - Sink into equilibrium
    - Quality driven to head in being-for-itself
    - Passes over into QUANTITY`;
  }

  dialecticalMovement(): string {
    return `${this.getInfiniteNature()}

    Equalization: ${this.getSelfReference()}

    Structure: ${this.getIdealization()}`;
  }
}

/**
 * BEING-FOR-ITSELF AS SUCH
 * ========================
 *
 * The general concept and its correspondence to representation
 */
class BeingForItselfAsSuch implements BeingForSelfDetermination {
  private conceptRepresentation: string;
  private sublatingSelfMovement: string;
  private infiniteTurningBack: string;

  constructor() {
    this.conceptRepresentation = "expression corresponds to the concept";
    this.sublatingSelfMovement = "sublates otherness, connection and community with other";
    this.infiniteTurningBack = "infinite turning back into itself";
  }

  getInfiniteNature(): string {
    return `Being-for-itself as sublating otherness:
    - Something is for itself inasmuch as it sublates otherness
    - Sublates its connection and community with other
    - Has rejected them by abstracting from them
    - Other is in it only as something sublated, as its moment
    - Consists in having transcended limitation, its otherness
    - Consists in being, as this negation, the infinite turning back into itself`;
  }

  getSelfReference(): string {
    return `The polemical, negative relating:
    - Being-for-itself is polemical, negative relating to limiting other
    - Through this negation of other, is being-reflected-within-itself
    - Even while becoming involved in negative of itself, in other
    - It abides with itself
    - Self-reference achieved through negation of otherness`;
  }

  getIdealization(): string {
    return `Consciousness as example of being-for-itself:
    - In representing intended object (feeling, intuiting, etc.)
    - Consciousness contains determination of being-for-itself
    - Has content of object in it - thus an idealization
    - Even involved in other, abides with itself
    - But consciousness is phenomenal, dualistic:
      * Knows external object as other than it
      * Is for-itself, has object as idealized, abides with itself`;
  }

  dialecticalMovement(): string {
    return `${this.getInfiniteNature()}

    Negative Relating: ${this.getSelfReference()}

    Consciousness Example: ${this.getIdealization()}`;
  }
}

/**
 * THE MYSTICAL UTTERANCE: SELF-CONSCIOUSNESS
 * ==========================================
 *
 * Self-consciousness as being-for-itself brought to completion
 */
class SelfConsciousnessCompletion implements BeingForSelfDetermination {
  private completedBeingForSelf: string;
  private infinityPresence: string;
  private qualitativeInfinity: string;

  constructor() {
    this.completedBeingForSelf = "being-for-itself brought to completion and posited";
    this.infinityPresence = "nearest example of presence of infinity";
    this.qualitativeInfinity = "infinity with qualitative determinateness";
  }

  getInfiniteNature(): string {
    return `Self-consciousness as completed being-for-itself:
    - Being-for-itself brought to completion and posited
    - Side of reference to another, to external object, is REMOVED
    - No longer dualistic like consciousness
    - Pure self-reference without external mediation`;
  }

  getSelfReference(): string {
    return `Nearest example of presence of infinity:
    - Self-consciousness = nearest example of presence of infinity
    - Granted, still abstract infinity
    - But totally different, concrete determination
    - Than being-for-itself in general
    - Whose infinity still has only qualitative determinateness`;
  }

  getIdealization(): string {
    return `The mystical significance:
    - Self-consciousness shows pure self-reference
    - Infinite that has achieved concrete presence
    - Not abstract beyond but present self-relation
    - Foundation for all spiritual determinations`;
  }

  dialecticalMovement(): string {
    return `${this.getInfiniteNature()}

    Infinity Present: ${this.getSelfReference()}

    Mystical Foundation: ${this.getIdealization()}`;
  }
}

/**
 * EXISTENCE AND BEING-FOR-ITSELF
 * ==============================
 *
 * How infinity sinks into simple being
 */
class ExistenceBeingForItself implements BeingForSelfDetermination {
  private infinitySinking: string;
  private negationOfNegation: string;
  private beingForOne: string;

  constructor() {
    this.infinitySinking = "infinity that has sunk into simple being";
    this.negationOfNegation = "negative nature of infinity as negation of negation";
    this.beingForOne = "moment of existence present as being-for-one";
  }

  getInfiniteNature(): string {
    return `Infinity sunk into simple being:
    - Being-for-itself = infinity that has sunk into simple being
    - It is existence insofar as negative nature of infinity
    - Which is negation of negation
    - Is only as negation in general
    - As infinite qualitative determinateness`;
  }

  getSelfReference(): string {
    return `Being distinguished from being-for-itself:
    - In this determinateness wherein it is existence
    - Being at once distinguished from this being-for-itself
    - Which is such only as infinite qualitative determinateness
    - Nevertheless existence is moment of being-for-itself
    - Latter contains being affected by negation`;
  }

  getIdealization(): string {
    return `Bent back into infinite unity:
    - Determinateness which in existence is other, being-for-other
    - Is bent back into infinite unity of being-for-itself
    - Moment of existence present in being-for-itself as BEING-FOR-ONE
    - The first emergence of "the one" as determination`;
  }

  dialecticalMovement(): string {
    return `${this.getInfiniteNature()}

    Distinction: ${this.getSelfReference()}

    Unity: ${this.getIdealization()}`;
  }
}

/**
 * THE MYSTICAL UTTERANCE: BEING-FOR-ONE
 * =====================================
 *
 * The profound analysis of idealization and the mystical examples
 */
class BeingForOne implements BeingForSelfDetermination {
  private finiteInInfinite: string;
  private idealityNature: string;
  private mysticalExamples: string;

  constructor() {
    this.finiteInInfinite = "how finite is in unity with infinite as idealization";
    this.idealityNature = "undistinguishedness of two sides in being-for-one";
    this.mysticalExamples = "I, spirit in general, or God as idealizations";
  }

  getInfiniteNature(): string {
    return `Being-for-one as idealization moment:
    - Gives expression to how finite is in unity with infinite
    - Or as an idealization
    - Being-for-itself does not have negation as determinateness or limit
    - Consequently not as reference to existence other than it
    - Yet designated as being-for-one
    - But nothing at hand for which it would be
    - Not yet the one of which it would be moment`;
  }

  getSelfReference(): string {
    return `The undistinguishedness revealed:
    - Nothing yet fixed in being-for-itself
    - That for which something would be is likewise a moment
    - Itself only being-for-one, not yet a one
    - Undistinguishedness of two sides in being-for-one
    - Only one being-for-another = only being-for-one
    - Only one ideality of that for which and that which should be moment
    - Being-for-one and being-for-itself not two genuine determinacies`;
  }

  getIdealization(): string {
    return `THE MYSTICAL UTTERANCE - I, Spirit, God:
    - Being-for-itself refers itself to itself as to sublated other
    - Therefore for-one; in its other refers itself only to itself
    - An idealization is necessarily for-one, but not for an other
    - The one for which it is, is only itself

    THE PROFOUND EXAMPLES:
    - The "I," therefore, spirit in general, or God, are idealizations
    - Because they are infinite
    - As existents which are for-themselves
    - Not ideationally different from that which is for-one
    - If they were different, would be only immediate existence
    - Being-for-another rather than for-themselves

    THE ULTIMATE MYSTICAL STATEMENT:
    - God is therefore for himself, insofar as he is himself that which is for him
    - Being-for-itself and being-for-one = essential, inseparable moments of ideality`;
  }

  dialecticalMovement(): string {
    return `${this.getInfiniteNature()}

    Undistinguishedness: ${this.getSelfReference()}

    MYSTICAL REVELATION: ${this.getIdealization()}`;
  }
}

/**
 * THE ONE
 * =======
 *
 * Being-for-itself as simple unity and the contradictory moments
 */
class TheOne implements BeingForSelfDetermination {
  private simpleUnity: string;
  private abstractLimit: string;
  private contradictoryMoments: string;

  constructor() {
    this.simpleUnity = "simple unity of itself and its moments";
    this.abstractLimit = "totally abstract limit of itself";
    this.contradictoryMoments = "six moments occurring one outside the other";
  }

  getInfiniteNature(): string {
    return `The One as simple unity:
    - Being-for-itself = simple unity of itself and its moments
    - Of the being-for-one
    - Only one determination present: self-reference itself of sublating
    - Moments have sunk into indifferentiation which is immediacy or being
    - But immediacy based on negating posited as its determination`;
  }

  getSelfReference(): string {
    return `Being-for-itself as existent-for-itself:
    - Thus an existent-for-itself
    - Since in this immediacy its inner meaning vanishes
    - It is the totally abstract limit of itself: THE ONE
    - The culmination of being-for-itself in pure self-reference`;
  }

  getIdealization(): string {
    return `THE CONTRADICTORY MOMENTS OF THE ONE:
    - Attention drawn to difficulties in exposition of development of one
    - Moments constituting concept of one as being-for-itself
    - Occur in it one outside the other:

    (1) Negation in general
    (2) Two negations that are, therefore,
    (3) The same,
    (4) Absolutely opposed
    (5) Self-reference, identity as such
    (6) Negative reference which is nonetheless self-reference

    THE SOURCE OF DIFFICULTY:
    - Form of immediacy, of being, enters being-for-itself as existent-for-itself
    - Each moment posited as determination existent on its own
    - Yet just as inseparable
    - Of each determination the opposite must equally be said
    - This contradiction causes difficulty with abstract nature of moments`;
  }

  dialecticalMovement(): string {
    return `${this.getInfiniteNature()}

    Abstract Limit: ${this.getSelfReference()}

    CONTRADICTORY STRUCTURE: ${this.getIdealization()}`;
  }
}

/**
 * THE BEING-FOR-ITSELF SYSTEM
 * ===========================
 *
 * Complete first part of Being-for-Self with mystical culmination
 */
class BeingForItselfSystem {
  private completion: QualitativeCompletion;
  private asSuch: BeingForItselfAsSuch;
  private selfConsciousness: SelfConsciousnessCompletion;
  private existence: ExistenceBeingForItself;
  private beingForOne: BeingForOne;
  private theOne: TheOne;

  constructor() {
    this.completion = new QualitativeCompletion();
    this.asSuch = new BeingForItselfAsSuch();
    this.selfConsciousness = new SelfConsciousnessCompletion();
    this.existence = new ExistenceBeingForItself();
    this.beingForOne = new BeingForOne();
    this.theOne = new TheOne();
  }

  /**
   * The complete first part of Being-for-Self
   */
  getCompleteBeingForItselfDevelopment(): string {
    return `BEING-FOR-ITSELF - The Mystical Completion of Quality:

    INTRODUCTION - QUALITATIVE COMPLETION:
    ${this.completion.dialecticalMovement()}

    A. BEING-FOR-ITSELF AS SUCH:
    ${this.asSuch.dialecticalMovement()}

    MYSTICAL MOMENT - SELF-CONSCIOUSNESS:
    ${this.selfConsciousness.dialecticalMovement()}

    a. EXISTENCE AND BEING-FOR-ITSELF:
    ${this.existence.dialecticalMovement()}

    b. BEING-FOR-ONE - THE MYSTICAL UTTERANCE:
    ${this.beingForOne.dialecticalMovement()}

    c. THE ONE - CONTRADICTORY MOMENTS:
    ${this.theOne.dialecticalMovement()}`;
  }

  /**
   * The mystical insights of Being-for-Self
   */
  getMysticalInsights(): string {
    return `THE MYSTICAL DIMENSION OF BEING-FOR-ITSELF:

    1. CONSCIOUSNESS AS BEING-FOR-ITSELF:
    - Already contains idealization of its object
    - Abides with itself even in otherness
    - Polemical, negative relating to limiting other

    2. SELF-CONSCIOUSNESS AS COMPLETION:
    - Being-for-itself brought to completion and posited
    - Nearest example of presence of infinity
    - Reference to external object removed

    3. THE PROFOUND UTTERANCE - I, SPIRIT, GOD:
    - "The I, therefore, spirit in general, or God, are idealizations"
    - "Because they are infinite"
    - "God is for himself, insofar as he is himself that which is for him"
    - Being-for-itself and being-for-one as essential moments of ideality

    4. THE ONE AS CONTRADICTORY UNITY:
    - Simple unity containing contradictory moments
    - Six moments occurring "one outside the other"
    - Abstract limit of itself through immediate form
    - Source of all dialectical difficulties in development`;
  }

  /**
   * Preparation for the development of One and Many
   */
  getPreparationForMultiplicity(): string {
    return `Preparation for One and Many, Repulsion and Attraction:

    The One has been established as:
    - Simple unity of being-for-itself and being-for-one
    - Totally abstract limit of itself
    - Containing contradictory moments in immediate form

    This sets up the necessity for:
    - Development of multiplicity of ones
    - Repulsion as otherness of the one
    - Attraction as sublation into ideality
    - Alternating determination leading to quantity

    The mystical foundation is complete - now comes the dynamic development!`;
  }

  /**
   * Access to individual moments
   */
  getCompletion(): QualitativeCompletion {
    return this.completion;
  }

  getAsSuch(): BeingForItselfAsSuch {
    return this.asSuch;
  }

  getSelfConsciousness(): SelfConsciousnessCompletion {
    return this.selfConsciousness;
  }

  getExistence(): ExistenceBeingForItself {
    return this.existence;
  }

  getBeingForOne(): BeingForOne {
    return this.beingForOne;
  }

  getTheOne(): TheOne {
    return this.theOne;
  }
}

// Export the main classes
export {
  BeingForItselfSystem as default,
  QualitativeCompletion,
  BeingForItselfAsSuch,
  SelfConsciousnessCompletion,
  ExistenceBeingForItself,
  BeingForOne,
  TheOne,
  BeingForSelfDetermination
};
