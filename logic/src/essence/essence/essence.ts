/**
 * ESSENCE: The Truth of Being and the Idea of Shine
 * =================================================
 *
 * The foundational module of Essence showing how Being's truth is revealed
 * through the movement of recollection into itself, and how this generates
 * the fundamental distinction of Essential and Unessential (Shine).
 *
 * Based on Hegel's Logic Book Two, showing how:
 * - "The truth of being is essence"
 * - Essence is "past being" (gewesen) - timeless recollection
 * - Shine is essence's own positing, not external otherness
 * - The movement from immediate Being to mediated Essence
 *
 * "Essence is past [but timelessly past] being"
 */

/**
 * Base interface for essence determinations
 */
interface EssentialDetermination {
  dialecticalMovement(): string;
  getEssentialNature(): string;
  getReflectiveStructure(): string;
}

/**
 * Interface for the essence-shine relationship
 */
interface EssenceShineRelation extends EssentialDetermination {
  getEssentialMoment(): string;
  getUnessentialMoment(): string;
  getShineAsOwnPositing(): string;
}

/**
 * THE IDEA OF ESSENCE
 * ===================
 *
 * Essence as the truth of being - the movement of being's self-recollection
 */
class IdeaOfEssence implements EssentialDetermination {
  private truthOfBeing: string;
  private pastBeing: string;
  private infiniteMovement: string;
  private absoluteInItselfness: string;

  constructor() {
    this.truthOfBeing = "the truth of being is essence";
    this.pastBeing = "essence is past (gewesen) but timelessly past being";
    this.infiniteMovement = "being's own movement of self-recollection";
    this.absoluteInItselfness = "absolute in-itselfness through sublation of otherness";
  }

  getEssentialNature(): string {
    return `Essence as the truth of being:
    - Being is the immediate - knowledge cannot stop there
    - Knowledge penetrates beyond immediate being
    - Presupposes something other than being itself behind being
    - This background constitutes the truth of being
    - Cognition is mediated knowledge - not found immediately
    - Starts from being and makes a way that penetrates into it
    - Knowledge recollects itself out of immediate being
    - Finds essence through this mediation

    The German Insight:
    - "Essence" (Wesen) kept in past participle (gewesen) of "to be" (sein)
    - For essence IS past being - but timelessly past
    - Not temporal past but logical past - sublated immediacy`;
  }

  getReflectiveStructure(): string {
    return `The movement is being's own self-development:
    - Not external activity of cognition indifferent to being's nature
    - This course IS the movement of being itself
    - Being's nature is to recollect itself into itself
    - Becomes essence by virtue of this interiorizing
    - This has been displayed in being itself

    From Absolute as Being to Absolute as Essence:
    - If absolute was first determined as being
    - Now it is determined as essence
    - Pure being forces reflection - presupposes recollection
    - Movement that distilled immediate existence into pure being
    - Being thus determined as essence - being in which finite is negated`;
  }

  /**
   * The danger of abstract essence
   */
  getAbstractEssenceCritique(): string {
    return `Against Abstract Essence as Mere Product:
    - If pure essence defined as sum total of all realities
    - These realities subject to abstractive reflection
    - Sum total reduced to empty simplicity - only a product, artifact
    - External reflection only lifts determinacies out of being
    - Deposits them elsewhere, letting them exist as before
    - Thus essence neither in itself nor for itself
    - By virtue of another, through external abstractive reflection
    - For another (abstraction and existent remaining opposite)
    - In its determination: dead and empty absence of determinateness`;
  }

  /**
   * True essence as infinite movement
   */
  getTrueEssence(): string {
    return `True Essence as Self-Development:
    - As it has come to be here, essence is what it is
    - Not through negativity foreign to it, but through its own
    - The infinite movement of being
    - Being-in-and-for-itself, absolute in-itselfness
    - Indifferent to every determinateness of being
    - Otherness and reference to other sublated
    - But not only in-itselfness (would be mere abstraction)
    - Being-for-itself just as essentially
    - Itself this negativity - self-sublation of otherness and determinateness`;
  }

  dialecticalMovement(): string {
    return `${this.getEssentialNature()}

    Self-Movement: ${this.getReflectiveStructure()}

    Against Abstraction: ${this.getAbstractEssenceCritique()}

    True Self-Development: ${this.getTrueEssence()}`;
  }
}

/**
 * ESSENCE AS COMPLETE TURNING BACK
 * ================================
 *
 * Essence as indeterminate essence passing over into existence
 */
class EssenceAsTurningBack implements EssentialDetermination {
  private completeTurningBack: string;
  private indeterminateEssence: string;
  private passageToExistence: string;
  private differentDetermining: string;

  constructor() {
    this.completeTurningBack = "complete turning back of being into itself";
    this.indeterminateEssence = "at first indeterminate essence";
    this.passageToExistence = "must pass over into existence";
    this.differentDetermining = "determining of another nature than in being";
  }

  getEssentialNature(): string {
    return `Essence as Complete Turning Back:
    - Complete turning back of being into itself
    - Thus at first the indeterminate essence
    - Determinacies of being sublated in it
    - Holds them in itself but without their being posited in it
    - Absolute essence in simple unity with itself has no existence

    But Must Pass Over into Existence:
    - For it is being-in-and-for-itself
    - It differentiates the determinations which it holds
    - Repelling of itself from itself or indifference towards itself
    - Negative self-reference - posits itself against itself
    - Infinite being-for-itself only through differentiating from itself
    - While remaining in unity with itself`;
  }

  getReflectiveStructure(): string {
    return `The Different Nature of Essential Determining:
    - This determining is of another nature than determining in being
    - Determinations of essence have another character than of being
    - Essence = absolute unity of being-in-itself and being-for-itself
    - Its determining remains inside this unity
    - Neither a becoming nor a passing over
    - Determinations neither other as other nor references to other
    - They are self-subsisting but conjoined in unity of essence

    Positing Within Its Own Sphere:
    - Since essence at first simple negativity
    - To give itself existence and then being-for-itself
    - Must posit in its sphere the determinateness
    - Which it contains in principle only in itself`;
  }

  /**
   * Essence vs Quality and Quantity
   */
  getEssenceVsBeingDeterminations(): string {
    return `Essence vs Quality and Quantity:
    - Essence is in the whole what quality was in sphere of being
    - Absolute indifference with respect to limit
    - Quantity: indifference in immediate determination
    - External limit necessary to it and exists in it
    - In essence: determinateness does not exist
    - Posited only by essence itself, not free
    - Only with reference to unity of essence
    - Negativity of essence IS reflection
    - Determinations are reflected, posited by essence itself
    - In which they remain as sublated`;
  }

  /**
   * Essence as middle between Being and Concept
   */
  getEssenceAsMiddle(): string {
    return `Essence as Middle Term:
    - Essence stands between being and concept
    - Makes up their middle
    - Its movement constitutes transition of being into concept
    - Being-in-and-for-itself in determination of being-in-itself
    - General determination: emerges from being - first negation of being
    - Movement consists in positing negation or determination in being
    - Thereby giving itself existence
    - Becoming as infinite being-for-itself what it is in itself
    - Thus gives itself existence equal to its being-in-itself
    - Becomes concept - absolute as it is absolutely in and for itself`;
  }

  dialecticalMovement(): string {
    return `${this.getEssentialNature()}

    Different Determining: ${this.getReflectiveStructure()}

    Vs Being Categories: ${this.getEssenceVsBeingDeterminations()}

    Middle Term: ${this.getEssenceAsMiddle()}`;
  }
}

/**
 * THE IDEA OF SHINE
 * =================
 *
 * How essence's own positing creates the distinction of essential and unessential
 */
class IdeaOfShine implements EssenceShineRelation {
  private essenceOwnPositing: string;
  private notExternalOther: string;
  private essenceShining: string;
  private reflectionNature: string;

  constructor() {
    this.essenceOwnPositing = "shine is essence's own positing";
    this.notExternalOther = "not something external, other than essence";
    this.essenceShining = "essence's own shining within itself";
    this.reflectionNature = "this shining of essence within it is reflection";
  }

  getEssentialMoment(): string {
    return `The Essential Side:
    - Essence issues from being - not immediately in and for itself
    - At first taken as something immediate
    - Determinate existence to which another stands opposed
    - Only essential existence, as against the unessential
    - But essence is being sublated in and for itself
    - What stands over against it is only shine`;
  }

  getUnessentialMoment(): string {
    return `The Unessential as Shine:
    - As essence issues from being, seems to stand over against it
    - This immediate being is, first, the unessential
    - But second, more than just unessential
    - Being void of essence - it is shine
    - Third, shine not something external, other than essence
    - But is essence's own shining
    - This shining of essence within it is reflection`;
  }

  getShineAsOwnPositing(): string {
    return `Shine as Essence's Own Positing:
    - The shine is essence's own positing
    - Not external otherness but internal self-development
    - Essence must shine within itself to be essence
    - Reflection as the process of this shining
    - Essential and unessential as moments of essence's self-development
    - Not fixed opposites but dialectical moments`;
  }

  getEssentialNature(): string {
    return `Essence and Shine as Unity:
    - Essence sublated being - simple equality with itself
    - As negation of sphere of being in general
    - Has immediacy over against it as something from which it came
    - But preserved and maintained itself in this sublating
    - Essence in this determination: existent immediate essence
    - Being is only something negative, nothing in and for itself
    - Essence therefore is determined negation`;
  }

  getReflectiveStructure(): string {
    return `The Essential-Unessential Dialectic:
    - Being and essence relate as others mutually indifferent
    - Each has being, immediacy - stand in equal value
    - But contrasted with essence, being is unessential
    - Has determination of something sublated
    - As it relates to essence as other only in general
    - Essence itself not essence proper but another existence
    - The distinction makes essence relapse into existence sphere

    External vs Internal Distinction:
    - External positing - taking apart leaving existence untouched
    - Separation falling on side of third
    - Dependent on external standpoint or consideration
    - Same content sometimes essential, sometimes unessential
    - But this is not the true essential-unessential relation`;
  }

  /**
   * The true nature of shine as non-essence
   */
  getTrueShine(): string {
    return `True Nature of Shine as Non-Essence:
    - Essence becomes essential only as contrasted with unessential
    - Because essence taken as sublated being or existence
    - Only first negation or negation which is determinateness
    - But essence is absolute negativity of being
    - Being itself, not determined only as other
    - Being that has sublated itself as immediate being
    - And as immediate negation affected by otherness

    The Result:
    - Being or existence does not persist except as what essence is
    - Immediate differing from essence not just unessential existence
    - But immediate which is null in and for itself
    - It only IS a non-essence, shine
    - Shine is not external but essence's own internal development`;
  }

  dialecticalMovement(): string {
    return `${this.getEssentialMoment()}

    Unessential Moment: ${this.getUnessentialMoment()}

    Own Positing: ${this.getShineAsOwnPositing()}

    Reflective Structure: ${this.getReflectiveStructure()}

    True Shine: ${this.getTrueShine()}`;
  }
}

/**
 * ESSENCE AND SHINE SYSTEM
 * ========================
 *
 * The complete foundational development of Essence including the Idea of Shine
 */
class EssenceAndShineSystem {
  private ideaOfEssence: IdeaOfEssence;
  private turningBack: EssenceAsTurningBack;
  private ideaOfShine: IdeaOfShine;

  constructor() {
    this.ideaOfEssence = new IdeaOfEssence();
    this.turningBack = new EssenceAsTurningBack();
    this.ideaOfShine = new IdeaOfShine();
  }

  /**
   * Complete development of Essence and Shine
   */
  getCompleteEssenceAndShineDevelopment(): string {
    return `ESSENCE: The Truth of Being and the Idea of Shine

    THE IDEA OF ESSENCE:
    ${this.ideaOfEssence.dialecticalMovement()}

    ESSENCE AS COMPLETE TURNING BACK:
    ${this.turningBack.dialecticalMovement()}

    THE IDEA OF SHINE:
    ${this.ideaOfShine.dialecticalMovement()}`;
  }

  /**
   * The fundamental transition from Being to Essence
   */
  getBeingToEssenceTransition(): string {
    return `The Fundamental Transition - Being to Essence:

    FROM BEING:
    - Being is immediate, but knowledge cannot stop there
    - Forces penetration beyond to find the truth
    - Being's own movement of self-recollection
    - "Past being" (gewesen) - essence as timeless sublation

    TO ESSENCE:
    - Truth of being is essence
    - Complete turning back of being into itself
    - Absolute unity of being-in-itself and being-for-itself
    - Determining of different nature than in being sphere

    THE SHINE EMERGENCE:
    - Essence must posit determinations within itself
    - Creates essential-unessential distinction
    - But shine is essence's own positing, not external
    - Foundation for reflection as essence's self-development

    This prepares the way for Reflection as the Truth of Being!`;
  }

  /**
   * Preparation for Reflection proper
   */
  getReflectionPreparation(): string {
    return `Preparation for Reflection as Truth of Being:

    ESSENCE STRUCTURE ESTABLISHED:
    - Essence as past being but not abstract essence
    - Self-development through infinite movement
    - Different determining nature from being sphere
    - Shine as essence's own positing within itself

    READY FOR REFLECTION DEVELOPMENT:
    - First: Essence as reflection within (Determinations of Reflection)
    - Second: Essence emerging into existence (Appearance)
    - Third: Essence one with appearance (Actuality)

    The stage is set for translating Identity, Difference, Contradiction
    as the fundamental Determinations of Reflection!`;
  }

  /**
   * Access to individual moments
   */
  getIdeaOfEssence(): IdeaOfEssence {
    return this.ideaOfEssence;
  }

  getTurningBack(): EssenceAsTurningBack {
    return this.turningBack;
  }

  getIdeaOfShine(): IdeaOfShine {
    return this.ideaOfShine;
  }
}

// Export the main classes
export {
  EssenceAndShineSystem as default,
  IdeaOfEssence,
  EssenceAsTurningBack,
  IdeaOfShine,
  EssentialDetermination,
  EssenceShineRelation
};
