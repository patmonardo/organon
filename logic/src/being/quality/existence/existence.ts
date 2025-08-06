/**
 * EXISTENCE AS SUCH - The First Determinate Being
 * ===============================================
 *
 * Translation of Hegel's "Existence as Such" from existence-as-such.txt
 * This is where Qualitative Logic begins - not with empty forms,
 * but with Form:Quality Something as determinate being
 *
 * Structure: Existence as Such → (a) Existence in general, (b) Quality, (c) Something
 */

// Interface for determinate beings (no longer pure/empty like Being)
interface DeterminateBeing {
  name: string;
  description: string;
  getDeterminateness(): string;
  getQuality(): string;
  getSomething(): string;
}

/**
 * EXISTENCE AS SUCH - The determinate immediate
 *
 * "Existence proceeds from becoming. It is the simple oneness of being and nothing.
 * It is not mere being but existence, or Dasein - being in a certain place"
 */
export class ExistenceAsSuch implements DeterminateBeing {
  readonly name = "Existence as Such";
  readonly description = "The simple oneness of being and nothing, proceeding from becoming. Determinate being - Dasein.";

  private existenceInGeneral: ExistenceInGeneral;
  private quality: QualityDetermination;
  private something: Something;

  constructor() {
    this.existenceInGeneral = new ExistenceInGeneral();
    this.quality = new QualityDetermination();
    this.something = new Something();
  }

  /**
   * Existence's determinateness - what makes it concrete
   * "Existence is determinate being, something concrete"
   */
  getDeterminateness(): string {
    return `Existence as Such is determinate because:
    - It proceeds from becoming (mediation lies behind it)
    - It has the form of an immediate but contains mediation
    - It is being with a non-being taken up into simple unity
    - This non-being constitutes determinateness as such
    - It appears as a first from which forward move is made`;
  }

  /**
   * Quality as the determinateness of existence
   */
  getQuality(): string {
    return this.quality.getQualityStructure();
  }

  /**
   * Something as the result of existence's development
   */
  getSomething(): string {
    return this.something.getSomethingStructure();
  }

  /**
   * The three moments of Existence as Such
   */
  getMoments(): {
    existenceInGeneral: ExistenceInGeneral;
    quality: QualityDetermination;
    something: Something
  } {
    return {
      existenceInGeneral: this.existenceInGeneral,
      quality: this.quality,
      something: this.something
    };
  }

  /**
   * How Existence differs from pure Being
   * "Existence corresponds to being in the preceding sphere. But being is indeterminate"
   */
  getDifferenceFromBeing(): string {
    return `Existence vs Pure Being:
    - Being: Indeterminate, no determinations transpire in it
    - Existence: Determinate being, something concrete
    - Being: Pure immediacy without content
    - Existence: Immediacy that contains mediation (becoming)
    - Being: Abstract, empty
    - Existence: Several determinations and distinct relations emerge
    - Result: Logic begins with determinate Something, not empty forms`;
  }
}

/**
 * a. EXISTENCE IN GENERAL
 *
 * "Existence proceeds from becoming. It has the form of an immediate.
 * Its mediation, the becoming, lies behind it"
 */
export class ExistenceInGeneral {
  readonly name = "Existence in General";

  /**
   * How existence emerges from becoming
   * "Existence proceeds from becoming. It is the simple oneness of being and nothing"
   */
  getEmergenceFromBecoming(): string {
    return `Existence emerges from Becoming:
    - Simple oneness of being and nothing
    - Has the form of an immediate
    - Its mediation (becoming) lies behind it, sublated
    - Appears as a first from which forward move is made
    - At first in one-sided determination of being
    - But contains nothing which will come up in contrast`;
  }

  /**
   * Existence as Dasein - being-there
   * "It is not mere being but existence, or Dasein; being in a certain place"
   */
  getDasein(): string {
    return `Existence as Dasein (being-there):
    - Not mere being but existence
    - Dasein = being (Sein) in a certain place (da)
    - Being with a non-being taken up into simple unity
    - Non-being taken up into being constitutes determinateness
    - The 'da' (there) expresses the determinateness of existence`;
  }

  /**
   * The concrete nature of existence
   * "Existence is determinate being, something concrete"
   */
  getConcreteNature(): string {
    return `Existence as concrete:
    - Determinate being, not indeterminate like pure Being
    - Something concrete with several determinations
    - Several distinct relations of its moments emerge immediately
    - Contains being and nothing in immediate unity
    - Form of being/immediacy but with determinate content`;
  }
}

/**
 * b. QUALITY (as determinateness of existence)
 *
 * "Determinateness thus isolated by itself, as existent determinateness, is quality"
 */
export class QualityDetermination {
  readonly name = "Quality";

  private reality: Reality;
  private negation: Negation;

  constructor() {
    this.reality = new Reality();
    this.negation = new Negation();
  }

  /**
   * Quality as simple, immediate determinateness
   * "Quality: something totally simple, immediate"
   */
  getQualityStructure(): string {
    return `Quality as determinateness:
    - Determinateness isolated by itself as existent determinateness
    - Something totally simple, immediate
    - Not yet detached from being, immediate unity with being
    - On account of this simplicity, nothing further to say about quality as such
    - But must be posited in both determinations: reality and negation`;
  }

  /**
   * The two determinations of quality: reality and negation
   */
  getTwoDeterminations(): { reality: Reality; negation: Negation } {
    return {
      reality: this.reality,
      negation: this.negation
    };
  }

  /**
   * How quality measures the one-sidedness of existence
   * "Existence is itself the measure of the one-sidedness of quality"
   */
  getQualityMeasurement(): string {
    return `Quality's measurement of existence:
    - Existence contains nothing and being equally
    - This measures the one-sidedness of quality as only immediate determinateness
    - Quality must be posited in determination of nothing as well
    - Result: immediate determinateness becomes reflected, distinct
    - Nothing becomes determinate element - a reflected negation`;
  }
}

/**
 * Reality - Quality with accent on existing
 * "Quality, in the distinct value of existent, is reality"
 */
export class Reality {
  readonly name = "Reality";

  getReality(): string {
    return `Reality as quality:
    - Quality with the accent on being an existent
    - Determinateness and negation are concealed in it
    - Has only the value of something positive
    - Negating, restriction, lack are excluded from it
    - But reality itself contains negation - it is existence, not abstract being`;
  }
}

/**
 * Negation - Quality affected by negating
 * "When affected by a negating, it is negation in general"
 */
export class Negation {
  readonly name = "Negation";

  getNegation(): string {
    return `Negation as quality:
    - Quality affected by a negating
    - Still a quality but counts as a lack
    - Further determined as limit, restriction
    - Not mere lack (which would be nothing)
    - But an existence, a quality, only determined with a non-being
    - Equally existence, not supposed abstract nothing`;
  }
}

/**
 * c. SOMETHING - The first negation of negation
 *
 * "Something is the first negation of negation, as simple existent self-reference"
 */
export class Something {
  readonly name = "Something";

  /**
   * Something as negation of negation
   * "Something is the first negation of negation"
   */
  getSomethingStructure(): string {
    return `Something as first negation of negation:
    - Simple existent self-reference
    - Sublation of the distinction between reality and negation
    - Existence not void of distinctions but self-equal through sublation
    - Simplicity of existence mediated through this sublation
    - Being-in-itself, existent something`;
  }

  /**
   * Something as beginning of the subject
   * "As something, the negative of the negative is only the beginning of the subject"
   */
  getSubjectBeginning(): string {
    return `Something as beginning of subject:
    - First negation of negation
    - Only the beginning of the subject, in-itselfness still indeterminate
    - Determines itself further as existent-for-itself
    - Until it obtains in the concept the intensity of the subject
    - At base of all determinations lies negative unity with itself`;
  }

  /**
   * Something's mediation with itself
   * "Something is thereby equally the mediation of itself with itself"
   */
  getSelfMediation(): string {
    return `Something's self-mediation:
    - Mediation of itself with itself
    - Present in simplicity of something
    - Negation of negation as restoration of simple reference to itself
    - But when taken only as negation of negation, collapses into simple unity
    - Something is, and is therefore also an existent`;
  }

  /**
   * Something's becoming and alteration
   * "Something is a transition, the moments of which are themselves something"
   */
  getBecoming(): string {
    return `Something's becoming:
    - In itself also becoming, but no longer has only being and nothing
    - One moment: existence and further an existent
    - Other moment: equally existent but determined as negative - an other
    - As becoming, it is transition with moments that are themselves something
    - Therefore it is alteration - becoming that has become concrete
    - At first alters only in concept, maintains itself in self-reference`;
  }
}

/**
 * THE LOGIC BEGINS WITH DETERMINATE SOMETHING
 * ===========================================
 *
 * This demonstrates that Qualitative Logic doesn't begin with empty forms
 * but starts with Form:Quality Something as the determinate foundation
 */
class LogicalBeginningClass {
  private existenceAsSuch: ExistenceAsSuch;

  constructor() {
    this.existenceAsSuch = new ExistenceAsSuch();
  }

  /**
   * Why Logic begins with Something, not empty forms
   */
  getLogicalBeginning(): string {
    return `Logic begins with determinate Something:
    - Not empty Being but determinate Existence
    - Not abstract forms but Form:Quality Something
    - Something carries connotation of a real thing
    - Contains reality and negation as concrete determinations
    - First negation of negation as beginning of subject
    - Mediation with itself is posited, not abstract
    - This is the foundation for all further logical development`;
  }

  /**
   * The significance of starting with Something
   */
  getSignificance(): string {
    return `Significance of Something as beginning:
    - Qualitative Logic is conditioned by determinate content
    - Form:Quality structure is immediate given
    - Something is "real thing" not abstract category
    - Contains its own principle of development (self-mediation)
    - Negation of negation provides dialectical movement
    - Beginning of subject, leading to full subjectivity in Concept
    - Demonstrates that Logic has substantial, not formal, content`;
  }

  /**
   * Get the complete Existence as Such
   */
  getExistenceAsSuch(): ExistenceAsSuch {
    return this.existenceAsSuch;
  }
}

// Export the main classes
export { ExistenceAsSuch as default, LogicalBeginningClass as LogicalBeginning };
