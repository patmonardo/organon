/**
 * NUMBER - The Complete Determinateness of Quantum
 * ==============================================
 *
 * "Thus completely posited in these determinations, quantum is number.
 * The complete positedness lies in the existence of the limit as a plurality
 * and so in its being distinguished from the unity."
 *
 * Number is quantum in complete determinateness - where the limit is
 * posited as determinate plurality having the One for its principle.
 */

// Base dialectical interface
interface DialecticalMoment {
  name: string;
  description: string;
  getContradiction(): string;
  transitionTo(): DialecticalMoment | null;
}

/**
 * NUMBER - Quantum in Complete Determinateness
 * ===========================================
 *
 * "Number appears for this reason as a discrete magnitude,
 * but in unity it has continuity as well."
 *
 * Number contains the complete contradiction:
 * - DISCRETE: as plurality of ones
 * - CONTINUOUS: as unity
 * - DETERMINATE: as specific limit
 */
export class Number implements DialecticalMoment {
  readonly name = "Number";
  readonly description: string;

  private amount: number;    // The "how many times" - the plurality
  private unit: number;      // The continuity, the unity
  private value: number;     // The complete determination

  /**
   * "Amount and unit constitute the moments of number"
   */
  constructor(amount: number, unit: number = 1) {
    this.amount = amount;
    this.unit = unit;
    this.value = amount * unit;
    this.description = `Number ${this.value} as ${amount} times unit ${unit}`;
  }

  /**
   * The intrinsic contradiction of number
   *
   * "This intrinsic contradiction of number or of quantum in general
   * is the quality of quantum"
   */
  getContradiction(): string {
    return `Number's intrinsic contradiction:
    - As DISCRETE magnitude: it is plurality of distinct ones (${this.amount} ones)
    - As CONTINUOUS magnitude: it is unity, simple immediacy (unit ${this.unit})
    - As DETERMINATE: it excludes other numbers yet refers to them externally
    - As ONE: it is absolutely determined yet has form of simple immediacy

    "This absolute exteriority is in the one itself" - the ones that constitute
    the number are both enclosed within its limit yet remain external to each other.`;
  }

  /**
   * Number transitions through its own internal development
   * The contradiction develops in further determinations
   */
  transitionTo(): DialecticalMoment | null {
    // Number's contradiction develops into extensive/intensive magnitude
    console.log(`🔢 Number ${this.value} developing its internal contradiction...`);
    return null; // Would transition to Extensive/Intensive Magnitude
  }

  /**
   * The One as principle of quantum
   *
   * "This one is therefore the principle of quantum, but as the one of quantity"
   */
  getOnePrinciple(): string {
    return `The One as principle of this number:
    First: CONTINUOUS - it is a unity (unit: ${this.unit})
    Second: DISCRETE - it is plurality of equal ones (amount: ${this.amount})
    Third: LIMITING - it negates other ones, excludes otherness

    The one is:
    (a) self-referring: each one refers to itself
    (b) enclosing: the limit encloses exactly ${this.amount} ones
    (c) other-excluding: excludes all other numbers`;
  }

  /**
   * How the limit pervades the number
   *
   * "But it was found... that the limit pervades existence,
   * that it extends so far as existence does"
   */
  getLimitPervision(): string {
    return `Limit pervades the entire number ${this.value}:

    Each of the ${this.amount} ones belongs equally to the limit.
    None has precedence - each is "just as much the ${this.getOrdinal()}th".
    The number cannot dispense with any of them for its determinateness.

    The ${this.amount} ones do not constitute existence different from the limit -
    they ARE the delimitation, the determinate quantum itself.`;
  }

  /**
   * Number as numerical one - absolutely determined yet externally related
   *
   * "Number is thus a numerical one that is absolutely determined
   * but which has at the same time the form of simple immediacy"
   */
  getNumericalOneCharacter(): string {
    return `Number ${this.value} as numerical one:

    ABSOLUTELY DETERMINED: it is precisely ${this.value}, no other
    SIMPLE IMMEDIACY: it appears as simple, immediate unity
    EXTERNAL REFERENCE: connection to other numbers remains external
    INDIFFERENT: indifferent to other numbers - this is its essential determination

    "This indifference of number to others is its essential determination;
    it constitutes its being-determined-in-itself, but at the same time also its own exteriority."`;
  }

  /**
   * The moments of number: Amount and Unit
   */
  getMoments(): { amount: number; unit: number; relationship: string } {
    return {
      amount: this.amount,
      unit: this.unit,
      relationship: `Amount (${this.amount}) is the "how many times" - the discreteness.
      Unit (${this.unit}) is the continuity of the amount.
      Together they constitute the complete determinateness of number ${this.value}.`
    };
  }

  /**
   * How the many ones exist in the limit
   *
   * "It is rightly said of amount that it consists of the many,
   * for the ones are not in it as sublated but are rather present in it"
   */
  getOnesInLimit(): string {
    const ones = Array.from({length: this.amount}, (_, i) => `one-${i+1}`);

    return `The ${this.amount} ones in number ${this.value}:

    PRESENT (not sublated): [${ones.join(', ')}]
    ENCLOSED: within the limiting boundary of ${this.value}
    INDIFFERENT: to the excluding limit individually
    EQUAL: each is just as much "the ${this.getOrdinal()}th"

    But the limit is NOT indifferent to them - it pervades all ${this.amount} ones.
    None can be dispensed with for the determinateness of ${this.value}.`;
  }

  /**
   * Number's indifference and exteriority
   *
   * "This indifference of number to others is its essential determination"
   */
  getIndifference(): string {
    return `Number ${this.value}'s essential indifference:

    QUANTITATIVE (not qualitative): distinguishing remains within comparing
    SELF-ENCLOSED: turned back onto itself, indifferent to others
    EXTERNALLY RELATED: reference to others remains completely external
    ABSOLUTELY DETERMINED: yet has form of simple immediacy

    This constitutes both its being-determined-in-itself AND its own exteriority.`;
  }

  // Helper method for ordinal description
  private getOrdinal(): string {
    const lastDigit = this.amount % 10;
    const lastTwoDigits = this.amount % 100;

    if (lastTwoDigits >= 11 && lastTwoDigits <= 13) {
      return `${this.amount}th`;
    }

    switch (lastDigit) {
      case 1: return `${this.amount}st`;
      case 2: return `${this.amount}nd`;
      case 3: return `${this.amount}rd`;
      default: return `${this.amount}th`;
    }
  }

  // Getters for the essential determinations
  getAmount(): number { return this.amount; }
  getUnit(): number { return this.unit; }
  getValue(): number { return this.value; }

  /**
   * Create a new number with different amount (keeping same unit)
   */
  withAmount(newAmount: number): Number {
    return new Number(newAmount, this.unit);
  }

  /**
   * Create a new number with different unit (keeping same amount)
   */
  withUnit(newUnit: number): Number {
    return new Number(this.amount, newUnit);
  }

  /**
   * The complete determinateness as string representation
   */
  toString(): string {
    if (this.unit === 1) {
      return `${this.value}`;
    } else {
      return `${this.amount} × ${this.unit} = ${this.value}`;
    }
  }
}

/**
 * FACTORY: Create numbers demonstrating different aspects
 */
export class NumberFactory {

  /**
   * Create a simple integer number
   */
  static createInteger(value: number): Number {
    return new Number(value, 1);
  }

  /**
   * Create a number showing amount × unit structure
   */
  static createComposite(amount: number, unit: number): Number {
    return new Number(amount, unit);
  }

  /**
   * Create the famous "hundred" from Hegel's example
   */
  static createHundred(): Number {
    const hundred = new Number(100, 1);
    console.log(`🎯 Created Hegel's hundred example:`);
    console.log(`   ${hundred.getLimitPervision()}`);
    return hundred;
  }

  /**
   * Demonstrate the complete determinateness
   */
  static demonstrateCompleteDeterminateness(): void {
    console.log(`🔢 DEMONSTRATING NUMBER'S COMPLETE DETERMINATENESS`);
    console.log(`================================================`);

    const seven = NumberFactory.createInteger(7);
    const dozen = NumberFactory.createComposite(12, 1);
    const score = NumberFactory.createComposite(20, 1);

    console.log(`\n📊 Numbers as Complete Determinations:`);
    console.log(`   Seven: ${seven.toString()}`);
    console.log(`   Dozen: ${dozen.toString()}`);
    console.log(`   Score: ${score.toString()}`);

    console.log(`\n🔍 Seven's Internal Structure:`);
    console.log(`   ${seven.getOnePrinciple()}`);

    console.log(`\n⚖️  Seven's Contradiction:`);
    console.log(`   ${seven.getContradiction()}`);

    console.log(`\n🎯 Seven's Indifference:`);
    console.log(`   ${seven.getIndifference()}`);
  }
}
