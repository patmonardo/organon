/**
 * QUANTUM - Extensive and Intensive Magnitude
 * ==========================================
 *
 * "Quantum has its determinateness as limit in amount.
 * In itself quantum is discrete, a plurality which does not have
 * a being different from its limit or its limit outside it."
 */

interface DialecticalMoment {
  getContradiction(): string;
  transcend(): DialecticalMoment | null;
}

export class Quantum implements DialecticalMoment {
  private extensiveMagnitude: ExtensiveMagnitude;
  private intensiveMagnitude: IntensiveMagnitude;

  constructor(limit: QuantitativeLimit, amount: Amount) {
    this.extensiveMagnitude = new ExtensiveMagnitude(limit, amount);
    this.intensiveMagnitude = new IntensiveMagnitude(limit);
  }

  /**
   * A. EXTENSIVE QUANTUM
   *
   * "Quantum, thus with its limit which as limit is a plurality,
   * is extensive magnitude."
   */
  getExtensiveMagnitude(): ExtensiveMagnitude {
    // "Extensive magnitude has the moment of continuity in it in its limit,
    // for its many is everywhere continuous"
    const continuousMany = this.extensiveMagnitude.getContinuousMany();

    // "the limit as negation appears, therefore, in this equality of the many
    // as a limiting of the unity"
    const limitingOfUnity = continuousMany.asLimitingOfUnity();

    return new ExtensiveMagnitude(
      new QuantitativeLimit(limitingOfUnity),
      this.extensiveMagnitude.getAmount()
    );
  }

  /**
   * B. INTENSIVE QUANTUM
   *
   * "In this simple determination of limit, quantum is intensive magnitude;
   * and the limit or the determinateness which is identical with quantum is now
   * also posited as simple: it is degree."
   */
  getIntensiveMagnitude(): IntensiveMagnitude {
    // "The limit of quantum which, as extensive, had its existent determinateness
    // as self-external number, thus passes over into simple determinateness"
    const simpleLimit = this.extensiveMagnitude.passOverToSimpleDeterminateness();

    // "Degree is thus a determinate magnitude, a quantum, but at the same time
    // it is not an aggregate or several within itself; it is only a plurality"
    const degree = new Degree(simpleLimit.asNotAggregate());

    return new IntensiveMagnitude(degree);
  }

  /**
   * DISTINCTION: Extensive vs Intensive
   *
   * "Extensive and intensive magnitudes are determinacies of the quantitative limit itself,
   * whereas quantum is identical with its limit"
   */
  getQuantitativeLimitDeterminacies(): QuantitativeLimitDeterminacies {
    // Extensive: determinateness as self-external plurality
    const extensiveDeterminacy = this.extensiveMagnitude.asSelfExternalPlurality();

    // Intensive: determinateness as simple self-reference
    const intensiveDeterminacy = this.intensiveMagnitude.asSimpleSelfReference();

    return new QuantitativeLimitDeterminacies(extensiveDeterminacy, intensiveDeterminacy);
  }

  /**
   * EXTENSIVE MAGNITUDE - Number with Explicit Plurality
   *
   * "extensive quantum is distinguished from number only because
   * in the latter the determinateness is explicitly posited as plurality"
   */
  asExtensiveQuantum(): ExtensiveQuantum {
    // "the simple determinateness which is amount essentially,
    // but the amount of one and the same unit"
    const amountOfSameUnit = this.extensiveMagnitude.getAmountOfSameUnit();

    // "extensive quantum is distinguished from number only because
    // in the latter the determinateness is explicitly posited as plurality"
    const explicitPlurality = amountOfSameUnit.asExplicitPlurality();

    return new ExtensiveQuantum(explicitPlurality);
  }

  /**
   * INTENSIVE MAGNITUDE - Degree as Simple Determinateness
   *
   * "When we speak of 10 or 20 degrees, the quantum which has that many degrees
   * is not the amount and sum of the degrees; it is rather only that one degree"
   */
  asIntensiveQuantum(): IntensiveQuantum {
    // "It does contain the determinateness found in the number ten or twenty,
    // but not as several ones: the number is there as a sublated amount"
    const sublatedAmount = this.getAmountAsSublated();

    // "as a simple determinateness"
    const simpleDeterminateness = sublatedAmount.asSimpleDeterminateness();

    return new IntensiveQuantum(new Degree(simpleDeterminateness));
  }

  /**
   * IDENTITY OF EXTENSIVE AND INTENSIVE
   *
   * "Extensive and intensive magnitude are, therefore, one and the same determinateness of quantum;
   * they are distinguished only inasmuch as the one has the amount within and the other has the same without"
   */
  getIdentityOfExtensiveAndIntensive(): QuantumIdentity {
    // "The twentieth degree contains the twenty within itself"
    const degreeContainsAmount = this.intensiveMagnitude.containsAmountWithin();

    // "But, inasmuch as the amount is its own, and the determinateness is at the same time
    // essentially as amount, the degree is extensive quantum"
    const degreeAsExtensive = degreeContainsAmount.asDegreeIsExtensive();

    // "indifferent to the otherwise determined intensities, it has the externality of amount in it;
    // thus intensive magnitude is just as essentially extensive magnitude"
    const intensiveAsExtensive = this.intensiveMagnitude.hasExternalityOfAmount();

    return new QuantumIdentity(degreeAsExtensive, intensiveAsExtensive);
  }

  /**
   * QUALITATIVE SOMETHING EMERGES
   *
   * "With this identity, the qualitative something comes on the scene"
   */
  getQualitativeSomething(): QualitativeSomething {
    const identity = this.getIdentityOfExtensiveAndIntensive();

    // "for the identity is the unity that refers back to itself through the negation of its distinct terms"
    const unityThroughNegation = identity.asUnityThroughNegation();

    // "The something is a quantum, but its qualitative existence is now posited as indifferent to it"
    const qualitativeExistence = unityThroughNegation.asQualitativeExistence();

    return new QualitativeSomething(qualitativeExistence);
  }

  /**
   * ALTERATION OF QUANTUM
   *
   * "A quantum, according to its quality, is therefore in absolute continuity with its externality,
   * with its otherness. Consequently, not only can every determinateness of magnitude be transcended,
   * not only can it be altered: that it must alter is now posited."
   */
  getAlterationNecessity(): AlterationNecessity {
    // "The determination of magnitude continues into its otherness in such a way that
    // it has its being only in this continuity with an other"
    const continuityWithOther = this.getContinuityWithOtherness();

    // "it is not just a limit that exists but one that becomes"
    const becomingLimit = continuityWithOther.asBecomingLimit();

    return new AlterationNecessity(becomingLimit);
  }

  /**
   * QUANTUM'S INFINITE PROGRESS
   *
   * "Quantum is equally infinite, posited as the self-referring negation;
   * it repels itself from itself."
   */
  getInfiniteProgress(): InfiniteProgress {
    // "But it is a determinate 'one,' the one which has passed over into existence and limit,
    // thus the repulsion of determinateness from itself"
    const repulsionFromItself = this.asRepulsionFromItself();

    // "It consists in this, that it increases or decreases;
    // it is within it the externality of determinateness"
    const increaseDecrease = repulsionFromItself.asIncreaseDecrease();

    // "Thus quantum sends itself beyond itself"
    const sendsBeyondItself = increaseDecrease.sendsBeyondItself();

    return new InfiniteProgress(sendsBeyondItself);
  }

  /**
   * THE BAD INFINITY
   *
   * "The limit which arises in this beyond is therefore only one that again sublates itself
   * and sends itself to a further limit, and so on to infinity."
   */
  getBadInfinity(): BadInfinity {
    const infiniteProgress = this.getInfiniteProgress();

    // "this other which it becomes is at first itself a quantum, but a quantum which is not a static limit
    // but one that impels itself beyond itself"
    const selfImpellingQuantum = infiniteProgress.asSelfImpellingQuantum();

    // "and so on to infinity"
    const endlessProgression = selfImpellingQuantum.asEndlessProgression();

    return new BadInfinity(endlessProgression);
  }

  /**
   * Quantum's Essential Contradiction
   */
  getContradiction(): string {
    const extensiveIntensive = this.getIdentityOfExtensiveAndIntensive();
    const alteration = this.getAlterationNecessity();

    return `The essential contradiction of quantum:
    - Extensive and intensive are one and the same determinateness yet distinguished
    - Has amount within (intensive) yet has same amount without (extensive)
    - Is determinate limit yet must alter beyond itself
    - Is self-referring determinateness yet refers essentially to otherness
    - ${extensiveIntensive.getContradiction()}
    - ${alteration.getContradiction()}

    This contradiction drives quantum into infinite progress - the bad infinity of endless becoming.`;
  }

  private getAmountAsSublated(): SublatedAmount {
    return new SublatedAmount(this.extensiveMagnitude.getAmount());
  }

  private getContinuityWithOtherness(): ContinuityWithOtherness {
    return new ContinuityWithOtherness();
  }

  private asRepulsionFromItself(): RepulsionFromItself {
    return new RepulsionFromItself();
  }

  transcend(): DialecticalMoment | null {
    // Quantum's bad infinity must be resolved in the transition to Measure
    return null;
  }
}

/**
 * EXTENSIVE MAGNITUDE - Quantum with Plurality as Limit
 */
export class ExtensiveMagnitude {
  constructor(
    private limit: QuantitativeLimit,
    private amount: Amount
  ) {}

  /**
   * "Extensive magnitude has the moment of continuity in it in its limit,
   * for its many is everywhere continuous"
   */
  getContinuousMany(): ContinuousMany {
    // "the limit as negation appears, therefore, in this equality of the many as a limiting of the unity"
    return new ContinuousMany(this.amount.asEqualityOfMany());
  }

  getAmount(): Amount {
    return this.amount;
  }

  passOverToSimpleDeterminateness(): SimpleDeterminateness {
    // "The limit of quantum which, as extensive, had its existent determinateness as self-external number,
    // thus passes over into simple determinateness"
    const selfExternalNumber = this.asSelfExternalNumber();
    return selfExternalNumber.passOverToSimple();
  }

  asSelfExternalPlurality(): SelfExternalPlurality {
    return new SelfExternalPlurality(this.amount);
  }

  getAmountOfSameUnit(): AmountOfSameUnit {
    return new AmountOfSameUnit(this.amount);
  }

  private asSelfExternalNumber(): SelfExternalNumber {
    return new SelfExternalNumber(this.amount);
  }
}

/**
 * INTENSIVE MAGNITUDE - Quantum as Degree
 */
export class IntensiveMagnitude {
  constructor(private degree: Degree) {}

  /**
   * "Degree is thus a determinate magnitude, a quantum, but at the same time
   * it is not an aggregate or several within itself; it is only a plurality"
   */
  getDegree(): Degree {
    return this.degree;
  }

  asSimpleSelfReference(): SimpleSelfReference {
    // "plurality is a severality that has gathered together into simple determination,
    // it is existence that has returned into being-for-itself"
    return new SimpleSelfReference(this.degree);
  }

  containsAmountWithin(): DegreeContainsAmount {
    // "The twentieth degree contains the twenty within itself"
    return new DegreeContainsAmount(this.degree);
  }

  hasExternalityOfAmount(): ExternalityOfAmount {
    // "indifferent to the otherwise determined intensities, it has the externality of amount in it"
    return new ExternalityOfAmount(this.degree);
  }
}

/**
 * DEGREE - Simple Determinateness of Intensive Quantum
 */
export class Degree {
  constructor(private simpleDeterminateness: SimpleDeterminateness | NotAggregate) {}

  /**
   * "When we speak of 10 or 20 degrees, the quantum which has that many degrees
   * [the tenth, the twentieth degree] is not the amount and sum of the degrees;
   * it is rather only that one degree, the tenth, the twentieth."
   */
  asOneOnly(): OneOnly {
    // "It does contain the determinateness found in the number ten or twenty,
    // but not as several ones: the number is there as a sublated amount"
    return new OneOnly(this.simpleDeterminateness);
  }

  /**
   * "As a self-referring quantitative determination, each degree is indifferent towards the others;
   * but, in itself, it equally refers to this externality"
   */
  getSelfReferenceToExternality(): SelfReferenceToExternality {
    // "it is what it is only through the intermediary of this externality"
    const throughExternality = new ThroughExternality();

    // "its reference to itself is not an indifferent reference to externality
    // but in this externality it possesses its quality"
    const possessesQualityInExternality = throughExternality.possessesQuality();

    return new SelfReferenceToExternality(possessesQualityInExternality);
  }
}

// Supporting classes for all logical movements

class QuantitativeLimit {
  constructor(private limiting?: LimitingOfUnity) {}
}

class Amount {
  asEqualityOfMany(): EqualityOfMany { return new EqualityOfMany(); }
  asSelfExternalPlurality(): SelfExternalPlurality { return new SelfExternalPlurality(this); }
}

class ContinuousMany {
  constructor(private equality: EqualityOfMany) {}
  asLimitingOfUnity(): LimitingOfUnity { return new LimitingOfUnity(); }
}

class LimitingOfUnity {}
class EqualityOfMany {}

class SimpleDeterminateness {}
class NotAggregate {}
class SelfExternalNumber {
  constructor(private amount: Amount) {}
  passOverToSimple(): SimpleDeterminateness { return new SimpleDeterminateness(); }
}

class SublatedAmount {
  constructor(private amount: Amount) {}
  asSimpleDeterminateness(): SimpleDeterminateness { return new SimpleDeterminateness(); }
}

class QuantitativeLimitDeterminacies {
  constructor(
    private extensive: SelfExternalPlurality,
    private intensive: SimpleSelfReference
  ) {}
}

class SelfExternalPlurality {
  constructor(private amount: Amount) {}
}

class SimpleSelfReference {
  constructor(private degree: Degree) {}
}

class ExtensiveQuantum {
  constructor(private plurality: ExplicitPlurality) {}
}

class IntensiveQuantum {
  constructor(private degree: Degree) {}
}

class AmountOfSameUnit {
  constructor(private amount: Amount) {}
  asExplicitPlurality(): ExplicitPlurality { return new ExplicitPlurality(); }
}

class ExplicitPlurality {}

class DegreeContainsAmount {
  constructor(private degree: Degree) {}
  asDegreeIsExtensive(): DegreeAsExtensive { return new DegreeAsExtensive(); }
}

class ExternalityOfAmount {
  constructor(private degree: Degree) {}
}

class DegreeAsExtensive {}

class QuantumIdentity {
  constructor(
    private degreeExtensive: DegreeAsExtensive,
    private intensiveExtensive: ExternalityOfAmount
  ) {}

  asUnityThroughNegation(): UnityThroughNegation { return new UnityThroughNegation(); }
  getContradiction(): string {
    return "Extensive and intensive are identical yet distinct - same determinateness in different forms";
  }
}

class UnityThroughNegation {
  asQualitativeExistence(): QualitativeExistence { return new QualitativeExistence(); }
}

class QualitativeExistence {}

class QualitativeSomething {
  constructor(private existence: QualitativeExistence) {}
}

class ContinuityWithOtherness {
  asBecomingLimit(): BecomingLimit { return new BecomingLimit(); }
}

class BecomingLimit {}

class AlterationNecessity {
  constructor(private becoming: BecomingLimit) {}
  getContradiction(): string {
    return "Quantum must alter - has being only in continuity with its other, not static limit but becoming";
  }
}

class RepulsionFromItself {
  asIncreaseDecrease(): IncreaseDecrease { return new IncreaseDecrease(); }
}

class IncreaseDecrease {
  sendsBeyondItself(): SendsBeyondItself { return new SendsBeyondItself(); }
}

class SendsBeyondItself {}

class InfiniteProgress {
  constructor(private beyondItself: SendsBeyondItself) {}
  asSelfImpellingQuantum(): SelfImpellingQuantum { return new SelfImpellingQuantum(); }
}

class SelfImpellingQuantum {
  asEndlessProgression(): EndlessProgression { return new EndlessProgression(); }
}

class EndlessProgression {}

class BadInfinity {
  constructor(private endless: EndlessProgression) {}
}

class OneOnly {
  constructor(private simple: SimpleDeterminateness | NotAggregate) {}
}

class ThroughExternality {
  possessesQuality(): PossessesQualityInExternality { return new PossessesQualityInExternality(); }
}

class PossessesQualityInExternality {}

class SelfReferenceToExternality {
  constructor(private quality: PossessesQualityInExternality) {}
}

export {
  Quantum,
  ExtensiveMagnitude,
  IntensiveMagnitude,
  Degree
};
