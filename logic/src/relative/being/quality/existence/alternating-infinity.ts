/**
 * ALTERNATING INFINITY: The Bad Infinite and Its Contradictions
 * ============================================================
 *
 * The second moment of infinity - how the infinite falls back into
 * external opposition with the finite, creating the spurious progression
 * "and so forth to infinity" that traps the understanding.
 *
 * Based on Hegel's Logic, showing how:
 * - Infinite falls back into category of "something with a limit"
 * - Bad infinite as "indeterminate emptiness, the beyond"
 * - Spurious progression as "repetitious monotony"
 * - Understanding's unresolved contradictions
 */

/**
 * Base interface for dialectical moments
 */
interface DialecticalMoment {
  dialecticalMovement(): string;
}

/**
 * Interface for alternating determinations
 */
interface AlternatingDetermination extends DialecticalMoment {
  getContradiction(): string;
  getAlternation(): string;
  getSpuriousProgression(): string;
}

/**
 * THE INFINITE'S FALL INTO DETERMINATENESS
 * ========================================
 *
 * How the infinite becomes "something determinate in general"
 */
class InfiniteFallback implements AlternatingDetermination {
  private categoryFallback: string;
  private limitReappearance: string;
  private qualitativeOpposition: string;

  constructor() {
    this.categoryFallback = "fallen back into category of something with a limit";
    this.limitReappearance = "immediate being resurrects the finite again";
    this.qualitativeOpposition = "standing in qualitative mutual reference";
  }

  getContradiction(): string {
    return `The infinite's contradictory position:
    - The infinite IS (in immediacy)
    - At same time negation of other (the finite)
    - As existent and non-being of other simultaneously
    - Has fallen back into category of something determinate
    - More precisely: existence reflected into itself
    - Results from mediating sublation of determinateness
    - But posited as existence distinct from its determinateness
    - Therefore fallen back into category of something with limit`;
  }

  getAlternation(): string {
    return `How finite and infinite oppose each other:
    - Finite stands over against infinite as real existence
    - They remain outside each other
    - Standing in qualitative mutual reference
    - Immediate being of infinite resurrects finite again
    - Finite seemed to have vanished into infinite
    - But now returns as real existence confronting infinite`;
  }

  getSpuriousProgression(): string {
    return "This creates the foundation for spurious progression";
  }

  dialecticalMovement(): string {
    return `${this.getContradiction()}

    Opposition: ${this.getAlternation()}`;
  }
}

/**
 * THE BAD INFINITE PROPER
 * =======================
 *
 * The infinite of the understanding as "indeterminate emptiness"
 */
class BadInfinite implements AlternatingDetermination {
  private restrictionDetermination: string;
  private infinityAsNothing: string;
  private indeterminateEmptiness: string;
  private understandingsSatisfaction: string;

  constructor() {
    this.restrictionDetermination = "finite is restriction posited as restriction";
    this.infinityAsNothing = "infinity is nothing of finite, the in-itself it ought to be";
    this.indeterminateEmptiness = "indeterminate emptiness, the beyond of finite";
    this.understandingsSatisfaction = "believes it has attained satisfaction in reconciliation";
  }

  getContradiction(): string {
    return `The bad infinite's contradictory structure:
    - Finite: restriction posited as restriction
    - Passes over into its in-itself and becomes infinite
    - Infinity: nothing of finite, the in-itself finite ought to be
    - But as reflected within itself, as realized ought
    - Only affirmative self-referring being
    - In infinity: all determinateness, alteration, restriction sublated
    - Nothing of finite is posited

    BUT the contradiction:
    - This affirmation is qualitatively immediate self-reference
    - Led back to category of being with finite confronting it
    - Negative nature posited as existent negation
    - Hence as first and immediate negation
    - Infinite burdened with opposition to finite
    - Finite remains real existence even when posited as sublated`;
  }

  getAlternation(): string {
    return `The bad infinite as indeterminate emptiness:
    - Infinite is "that which is not finite"
    - Being in determinateness of negation
    - Contrasted with finite, with series of realities
    - Infinite is indeterminate emptiness
    - The beyond of finite
    - Whose being-in-itself is not in its existence
    - (which is something determinate)`;
  }

  getSpuriousProgression(): string {
    return `The understanding's delusion:
    - Infinite posited over against finite
    - Two connected by qualitative mutual reference of others
    - This infinite = BAD INFINITE
    - Infinite of the understanding
    - For which it counts as highest, absolute truth
    - Understanding believes it has attained satisfaction
    - While entangled in unreconciled, absolute contradictions
    - These contradictions must make it conscious of the fact`;
  }

  /**
   * The two worlds problem
   */
  getTwoWorldsProblem(): string {
    return `The two worlds contradiction:
    - Infinite remains over against finite
    - Result: there are two determinacies
    - Two worlds: one infinite, one finite
    - In their connection, infinite is only limit of finite
    - Thus only a determinate, itself finite infinite
    - This is the fundamental contradiction`;
  }

  dialecticalMovement(): string {
    return `${this.getContradiction()}

    Bad Infinite: ${this.getAlternation()}

    Understanding's Error: ${this.getSpuriousProgression()}

    Two Worlds: ${this.getTwoWorldsProblem()}`;
  }
}

/**
 * THE SEPARATION AND CONNECTION
 * =============================
 *
 * How finite and infinite are separated yet essentially connected
 */
class SeparationConnection implements AlternatingDetermination {
  private spatialSeparation: string;
  private essentialConnection: string;
  private mutualPositing: string;

  constructor() {
    this.spatialSeparation = "finite here, infinite there as nebulous beyond";
    this.essentialConnection = "connected through very negation that divides them";
    this.mutualPositing = "each posits its non-being as the other";
  }

  getContradiction(): string {
    return `The separation that connects:
    - Understanding elevates itself above finite world
    - Rises to what is highest for it: the infinite
    - Finite world remains as something on this side
    - Infinite posited only above finite, separated from it
    - Each placed in different location:
      * Finite as existence here
      * Infinite as beyond, nebulous, inaccessible distance
    - Outside infinite stands enduring finite

    YET they are essentially connected:
    - Through very negation that divides them
    - Negation is common limit of each against other
    - Each has this limit not merely in it but as its in-itselfness
    - Each has limit in it in separation from other`;
  }

  getAlternation(): string {
    return `The mutual positing process:
    - Limit is first negation - both are limited, finite in themselves
    - Yet each affirmatively refers itself to itself
    - Each is also negation of its limit
    - Each repels negation from itself as its non-being
    - Qualitatively severed, posits it as other being outside it:
      * Finite posits its non-being as this infinite
      * Infinite likewise posits the finite
    - Each is through its determination the positing of its other
    - The two are inseparable`;
  }

  getSpuriousProgression(): string {
    return `The hidden unity that drives progression:
    - Unity rests hidden in their qualitative otherness
    - Inner unity that lies only at their base
    - Unity appears in existence as turning over/transition
    - Finite into infinite and vice versa
    - Each arises in the other independently and immediately
    - Their connection appears only external`;
  }

  dialecticalMovement(): string {
    return `${this.getContradiction()}

    Mutual Positing: ${this.getAlternation()}

    Hidden Unity: ${this.getSpuriousProgression()}`;
  }
}

/**
 * THE SPURIOUS PROGRESSION TO INFINITY
 * ====================================
 *
 * The "and so forth to infinity" as repetitious monotony
 */
class SpuriousProgression implements AlternatingDetermination {
  private transitionProcess: string;
  private alternatingDetermination: string;
  private progressionStructure: string;
  private abstractTranscending: string;

  constructor() {
    this.transitionProcess = "finite passes over into infinite, limit arises again";
    this.alternatingDetermination = "finite only with reference to infinite, vice versa";
    this.progressionStructure = "repetitious monotony, tedious alternation";
    this.abstractTranscending = "incomplete because transcending itself not transcended";
  }

  getContradiction(): string {
    return `The spurious progression's structure:
    - Finite passes over into infinite (appears external doing)
    - In emptiness beyond finite, what arises? What positive?
    - On account of inseparability (infinite itself restricted):
      * The limit arises
      * Infinite has vanished, finite stepped in
    - Stepping in appears external to infinite
    - New limit doesn't arise from infinite itself but found given
    - Back at previous determination, sublated in vain
    - New limit only something to be sublated/transcended
    - Arises again emptiness, nothing with same determination
    - AND SO FORTH TO INFINITY`;
  }

  getAlternation(): string {
    return `The alternating determination:
    - Finite is finite only with reference to ought/infinite
    - Infinite is only infinite with reference to finite
    - Two are inseparable and absolutely other
    - Each has in it the other of itself
    - Each is unity of itself and its other
    - In determinateness not to be what itself and other is
    - IT IS EXISTENCE`;
  }

  getSpuriousProgression(): string {
    return `The progress to infinity exposed:
    - Alternating determination of self-negating and negating the negating
    - Passes as "progress to infinity"
    - Accepted in many shapes as unsurpassable ultimate
    - Thought having reached "and so on to infinity" achieves its end
    - Breaks out wherever relative determinations pressed to opposition
    - Though in inseparable unity, each attributed independent existence
    - Therefore the contradiction not resolved but always pronounced present

    ABSTRACT TRANSCENDING:
    - Remains incomplete because transcending itself not transcended
    - Infinite is transcended (another limit posited)
    - But only return made back to finite
    - Bad infinite same as perpetual ought
    - Negation of finite but unable to free itself from it
    - Finite constantly resurfaces as its other
    - Progress to infinity: repetitious monotony
    - One and same tedious alternation`;
  }

  /**
   * The rigid beyond
   */
  getRigidBeyond(): string {
    return `The infinite as rigid beyond:
    - Infinity of infinite progress remains burdened by finite
    - Thereby restricted, itself finite
    - Posited as unity of finite and infinite (but not reflected upon)
    - Unity alone rouses finite in infinite, infinite in finite
    - Impulse driving infinite progress
    - Progress is outside of this unity where representation fixated
    - Perennial repetition of same alternation
    - Empty unrest of progression across limit toward infinite
    - In infinity finds new limit, unable to halt at either
    - Infinite has rigid determination of beyond that cannot be attained
    - For very reason it ought not be attained
    - Determinateness of beyond, existent negation, not let go
    - Finite unable to raise itself to infinite
    - Existence perennially regenerates itself in beyond by being different`;
  }

  dialecticalMovement(): string {
    return `${this.getContradiction()}

    Alternation: ${this.getAlternation()}

    False Progress: ${this.getSpuriousProgression()}

    Rigid Beyond: ${this.getRigidBeyond()}`;
  }
}

/**
 * THE ALTERNATING INFINITY SYSTEM
 * ===============================
 *
 * Complete analysis of the bad infinite and its contradictions
 */
class AlternatingInfinitySystem {
  private fallback: InfiniteFallback;
  private badInfinite: BadInfinite;
  private separation: SeparationConnection;
  private progression: SpuriousProgression;

  constructor() {
    this.fallback = new InfiniteFallback();
    this.badInfinite = new BadInfinite();
    this.separation = new SeparationConnection();
    this.progression = new SpuriousProgression();
  }

  /**
   * The complete analysis of alternating infinity
   */
  getCompleteAnalysis(): string {
    return `ALTERNATING INFINITY - The Bad Infinite's Contradictions:

    1. INFINITE'S FALLBACK:
    ${this.fallback.dialecticalMovement()}

    2. BAD INFINITE PROPER:
    ${this.badInfinite.dialecticalMovement()}

    3. SEPARATION AND CONNECTION:
    ${this.separation.dialecticalMovement()}

    4. SPURIOUS PROGRESSION:
    ${this.progression.dialecticalMovement()}`;
  }

  /**
   * The key insights about the bad infinite
   */
  getBadInfiniteInsights(): string {
    return `Critical insights about the bad infinite:

    1. UNDERSTANDING'S TRAP:
    - Believes it has found absolute in bad infinite
    - Actually entangled in unresolved contradictions
    - Two worlds problem: finite here, infinite beyond

    2. SPURIOUS PROGRESSION:
    - "And so forth to infinity" as repetitious monotony
    - Abstract transcending that never transcends itself
    - Finite constantly resurfaces in the infinite

    3. RIGID BEYOND:
    - Infinite as unattainable beyond that ought not be attained
    - Empty unrest across limit toward infinite
    - Perennial regeneration of same contradiction

    4. HIDDEN UNITY:
    - Unity of finite and infinite present but not reflected upon
    - This unity is the impulse driving the progression
    - Points toward true infinite as resolution`;
  }

  /**
   * The preparation for affirmative infinity
   */
  getPreparationForResolution(): string {
    return `Preparation for affirmative infinity:

    The alternating infinity has shown:
    - Why the bad infinite fails (external opposition)
    - How spurious progression traps understanding
    - That unity of finite and infinite is already present
    - But hidden and not reflected upon

    This sets up the need for:
    - True infinite that reflects this unity
    - Affirmative infinity as self-returning movement
    - Resolution of finite-infinite opposition
    - Concrete negation of negation as true being`;
  }

  /**
   * Access to individual moments
   */
  getFallback(): InfiniteFallback {
    return this.fallback;
  }

  getBadInfinite(): BadInfinite {
    return this.badInfinite;
  }

  getSeparation(): SeparationConnection {
    return this.separation;
  }

  getProgression(): SpuriousProgression {
    return this.progression;
  }
}

// Export the main classes
export {
  AlternatingInfinitySystem as default,
  InfiniteFallback,
  BadInfinite,
  SeparationConnection,
  SpuriousProgression,
  AlternatingDetermination
};
