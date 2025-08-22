/**
 * RATIO OF POWERS - Quantum as Being-For-Itself
 * =============================================
 *
 * "Quantum, in positing itself as self-identical in its otherness
 * and in determining its own movement of self-surpassing,
 * has come to be a being-for-itself."
 */

interface DialecticalMoment {
  getContradiction(): string;
  transcend(): DialecticalMoment | null;
}

export class RatioOfPowers implements DialecticalMoment {
  private quantumAsBeingForItself: QuantumAsBeingForItself;
  private qualitativeTotality: QualitativeTotality;
  private conceptualRealization: ConceptualRealization;

  constructor(inverseRatio: InverseRatio) {
    this.quantumAsBeingForItself = new QuantumAsBeingForItself();
    this.qualitativeTotality = new QualitativeTotality();
    this.conceptualRealization = new ConceptualRealization();
  }

  /**
   * 1. QUANTUM AS BEING-FOR-ITSELF
   *
   * "As such a qualitative totality, in positing itself as developed,
   * it has for its moments the conceptual determinations of number:
   * the unit and the amount."
   */
  getQuantumAsBeingForItself(): QuantumAsBeingForItself {
    // "This last, amount, is in the inverse ratio still an aggregate
    // which is not determined as amount by the unit itself but from elsewhere"
    const amountFromElsewhere = new AmountFromElsewhere();

    // "but now it is posited as determined only by the unit"
    const determinedOnlyByUnit = amountFromElsewhere.determinedOnlyByUnit();

    // "This is the case in the ratio of powers where the unit,
    // which in it is amount, is at the same time the amount as against itself as unit"
    const unitIsAmount = determinedOnlyByUnit.unitIsAmount();
    const amountAgainstItself = unitIsAmount.amountAgainstItself();

    return new QuantumAsBeingForItself(amountAgainstItself);
  }

  /**
   * THE OTHERNESS IS THE UNIT ITSELF
   *
   * "The otherness, the amount of units, is the unit itself.
   * The power is an aggregate of units, each of which is this aggregate itself."
   */
  getOthernessIsUnitItself(): OthernessIsUnitItself {
    // "The power is an aggregate of units, each of which is this aggregate itself"
    const aggregateOfUnits = new AggregateOfUnits();
    const eachIsThisAggregate = aggregateOfUnits.eachIsThisAggregate();

    return new OthernessIsUnitItself(eachIsThisAggregate);
  }

  /**
   * ALTERATION AS RAISING TO POWER
   *
   * "The quantum, as indifferent determinateness, changes;
   * but inasmuch as the alteration is the raising to a power,
   * the otherness of the quantum is determined purely by itself."
   */
  getAlterationAsRaisingToPower(): AlterationAsRaisingToPower {
    // "The quantum is thus posited in the power as having returned into itself"
    const returnedIntoItself = new ReturnedIntoItself();

    // "it is immediately itself and also its otherness"
    const itselfAndItsOtherness = returnedIntoItself.itselfAndItsOtherness();

    return new AlterationAsRaisingToPower(itselfAndItsOtherness);
  }

  /**
   * EXPONENT OF ENTIRELY QUALITATIVE NATURE
   *
   * "The exponent of this ratio is no longer an immediate quantum,
   * as in the direct ratio and also in the inverse ratio.
   * In the ratio of powers, the exponent is of an entirely qualitative nature"
   */
  getExponentQualitativeNature(): ExponentQualitativeNature {
    // "it is this simple determinateness: that the amount is the unit itself,
    // that the quantum is self-identical in its otherness"
    const amountIsUnitItself = new AmountIsUnitItself();
    const selfIdenticalInOtherness = amountIsUnitItself.selfIdenticalInOtherness();

    // "And the side of its quantitative nature is to be found in this:
    // that the limit or negation is not an immediate existent"
    const limitNotImmediateExistent = selfIdenticalInOtherness.limitNotImmediateExistent();

    // "but that existence is posited rather as continuing in its otherness"
    const existenceContinuingInOtherness = limitNotImmediateExistent.existenceContinuingInOtherness();

    return new ExponentQualitativeNature(existenceContinuingInOtherness);
  }

  /**
   * TRUTH OF QUALITY IS QUANTITY
   *
   * "For the truth of quality is precisely to be quantity,
   * or immediate determinateness as sublated."
   */
  getTruthOfQualityIsQuantity(): TruthOfQualityIsQuantity {
    const immediateAssublated = new ImmediateAsSublated();
    return new TruthOfQualityIsQuantity(immediateAssublated);
  }

  /**
   * 2. QUANTUM ATTAINS ITS CONCEPT
   *
   * "The ratio of powers appears at first as an external alteration
   * to which a given quantum is subjected; but it has a closer connection
   * with the concept of quantum"
   */
  getQuantumAttainsConcept(): QuantumAttainsConcept {
    // "that in the existence into which the quantum has developed in the ratio of powers,
    // quantum has attained that concept, has realized its concept to the fullest"
    const existenceDeveloped = new ExistenceDeveloped();
    const attainedConcept = existenceDeveloped.attainedConcept();
    const realizedConceptFullest = attainedConcept.realizedConceptFullest();

    return new QuantumAttainsConcept(realizedConceptFullest);
  }

  /**
   * DISPLAY OF WHAT QUANTUM IS IMPLICITLY
   *
   * "The ratio of powers is the display of what the quantum is implicitly in itself;
   * it expresses its determinateness of quantum or the quality by which
   * it is distinguished from another."
   */
  getDisplayOfImplicit(): DisplayOfImplicit {
    // "Quantum is indifferent determinateness posited as sublated"
    const indifferentAsSublated = new IndifferentAsSublated();

    // "that is to say, determinateness as limit, one which is just as much no determinateness"
    const limitNoEterminateness = indifferentAsSublated.limitNoEterminateness();

    // "which continues in its otherness and in it, therefore, remains identical with itself"
    const continuesInOtherness = limitNoEterminateness.continuesInOtherness();
    const remainsIdenticalWithItself = continuesInOtherness.remainsIdenticalWithItself();

    return new DisplayOfImplicit(remainsIdenticalWithItself);
  }

  /**
   * OTHERNESS DETERMINED THROUGH QUANTUM ITSELF
   *
   * "Thus is quantum posited in the ratio of powers:
   * its otherness, the surpassing of itself in another quantum,
   * as determined through the quantum itself."
   */
  getOthernessDeterminedThroughQuantum(): OthernessDeterminedThroughQuantum {
    const surpassingInAnother = new SurpassingInAnother();
    const determinedThroughQuantumItself = surpassingInAnother.determinedThroughQuantumItself();

    return new OthernessDeterminedThroughQuantum(determinedThroughQuantumItself);
  }

  /**
   * PROGRESSIVE REALIZATION COMPARISON
   *
   * "If we compare the progressive realization of quantum in the preceding ratios,
   * we find that quantum's quality of being the difference of itself from itself is simply this:
   * that it is a ratio."
   */
  getProgressiveRealization(): ProgressiveRealization {
    // Direct ratio: "quantum is this posited difference only in the first instance or immediately"
    const directRatioImmediate = new DirectRatioImmediate();

    // Inverse ratio: "as negatively determined, quantum is a relating of itself to itself"
    const inverseRatioNegative = new InverseRatioNegative();

    // Powers ratio: "quantum is present in the difference as a difference of itself from itself"
    const powersRatioDifference = new PowersRatioDifference();

    return new ProgressiveRealization(directRatioImmediate, inverseRatioNegative, powersRatioDifference);
  }

  /**
   * EXTERNALITY AS QUANTUM'S OWN DETERMINING
   *
   * "This externality of determinateness is the quality of quantum and is thus posited,
   * in conformity to the concept of quantum, as quantum's own determining,
   * as its reference to itself, its quality."
   */
  getExternalityAsOwnDetermining(): ExternalityAsOwnDetermining {
    const externalityOfDeterminateness = new ExternalityOfDeterminateness();
    const qualityOfQuantum = externalityOfDeterminateness.asQualityOfQuantum();
    const ownDetermining = qualityOfQuantum.asOwnDetermining();
    const referenceToItself = ownDetermining.asReferenceToItself();

    return new ExternalityAsOwnDetermining(referenceToItself);
  }

  /**
   * 3. QUANTUM PASSED OVER INTO ANOTHER DETERMINATION
   *
   * "By being thus posited as it is in conformity to its concept,
   * quantum has passed over into another determination"
   */
  getQuantumPassedOver(): QuantumPassedOver {
    // "or, as we can also say, its determination is now also as the determinateness,
    // the in-itself also as existence"
    const determinationAsDeterminateness = new DeterminationAsDeterminateness();
    const inItselfAsExistence = determinationAsDeterminateness.inItselfAsExistence();

    return new QuantumPassedOver(inItselfAsExistence);
  }

  /**
   * QUANTUM BECOMES QUALITY
   *
   * "it has become the other of itself, namely quality,
   * in so far as that same externality is now posited as mediated by quantum itself"
   */
  getQuantumBecomesQuality(): QuantumBecomesQuality {
    // "and hence as a moment of quantum, so that in that very externality
    // quantum refers itself to itself, is being as quality"
    const externalityMediated = new ExternalityMediated();
    const momentOfQuantum = externalityMediated.asMomentOfQuantum();
    const refersToItselfInExternality = momentOfQuantum.refersToItselfInExternality();
    const beingAsQuality = refersToItselfInExternality.beingAsQuality();

    return new QuantumBecomesQuality(beingAsQuality);
  }

  /**
   * QUANTITY AS TRUTH OF QUALITY
   *
   * "At first quantity as such thus appears in opposition to quality;
   * but quantity is itself a quality, self-referring determinateness as such"
   */
  getQuantityAsTruthOfQuality(): QuantityAsTruthOfQuality {
    // "Except that quantity is not only a quality, but the truth of quality itself is quantity"
    const quantityNotOnlyQuality = new QuantityNotOnlyQuality();
    const truthOfQualityIsQuantity = quantityNotOnlyQuality.truthOfQualityIsQuantity();

    // "Quantity, in its truth, is instead the externality which has returned into itself"
    const externalityReturnedIntoItself = truthOfQualityIsQuantity.externalityReturnedIntoItself();

    // "which is no longer indifferent. Thus is quantity quality itself"
    const noLongerIndifferent = externalityReturnedIntoItself.noLongerIndifferent();
    const quantityIsQuality = noLongerIndifferent.quantityIsQuality();

    return new QuantityAsTruthOfQuality(quantityIsQuality);
  }

  /**
   * DOUBLE TRANSITION NECESSITY
   *
   * "For the totality to be posited, a double transition is required,
   * not only the transition of one determinateness into the other,
   * but equally the transition of this other into the first, its going back into it."
   */
  getDoubleTransition(): DoubleTransition {
    // "Through the first transition, the identity of the two is present at first only in itself:
    // quality is contained in quantity, but the latter still is only a one-sided determinateness"
    const firstTransition = new FirstTransition();
    const identityInItself = firstTransition.identityInItself();
    const qualityContainedInQuantity = identityInItself.qualityContainedInQuantity();

    // "Conversely, that quantity is equally contained in quality,
    // that it is equally also only as sublated, this results in the second transition"
    const secondTransition = new SecondTransition();
    const quantityContainedInQuality = secondTransition.quantityContainedInQuality();
    const goingBackIntoFirst = quantityContainedInQuality.goingBackIntoFirst();

    return new DoubleTransition(qualityContainedInQuantity, goingBackIntoFirst);
  }

  /**
   * SCIENTIFIC METHOD NECESSITY
   *
   * "This remark regarding the necessity of the double transition
   * is everywhere of great importance for scientific method."
   */
  getScientificMethodNecessity(): ScientificMethodNecessity {
    const doubleTransition = this.getDoubleTransition();
    const greatImportance = doubleTransition.asGreatImportance();

    return new ScientificMethodNecessity(greatImportance);
  }

  /**
   * QUANTUM SUBLATED AS INDIFFERENT DETERMINATION
   *
   * "Quantum is henceforth no longer an indifferent or external determination
   * but is sublated as such, and it is a quality and that by virtue of which anything is what it is"
   */
  getQuantumSublatedAsIndifferent(): QuantumSublatedAsIndifferent {
    // "and it is a quality and that by virtue of which anything is what it is"
    const qualityByVirtueOfWhich = new QualityByVirtueOfWhich();
    const anythingIsWhatItIs = qualityByVirtueOfWhich.anythingIsWhatItIs();

    return new QuantumSublatedAsIndifferent(anythingIsWhatItIs);
  }

  /**
   * TRUTH OF QUANTUM IS MEASURE
   *
   * "the truth of quantum is to be measure."
   */
  getTruthOfQuantumIsMeasure(): TruthOfQuantumIsMeasure {
    const quantum = this.getQuantumSublatedAsIndifferent();
    const truthIsMeasure = quantum.truthIsMeasure();

    return new TruthOfQuantumIsMeasure(truthIsMeasure);
  }

  /**
   * TRANSITION TO MEASURE
   *
   * The complete dialectical movement that leads to Measure
   */
  getTransitionToMeasure(): TransitionToMeasure {
    const truthOfQuantum = this.getTruthOfQuantumIsMeasure();
    const doubleTransition = this.getDoubleTransition();
    const qualityQuantityUnity = this.getQuantityAsTruthOfQuality();

    // "quantum is henceforth no longer an indifferent or external determination
    // but is sublated as such, and it is a quality"
    const quantumAsQuality = new QuantumAsQuality();

    // "that by virtue of which anything is what it is; the truth of quantum is to be measure"
    const measure = quantumAsQuality.asMeasure();

    return new TransitionToMeasure(measure);
  }

  getContradiction(): string {
    const beingForItself = this.getQuantumAsBeingForItself();
    const conceptRealized = this.getQuantumAttainsConcept();
    const becomeQuality = this.getQuantumBecomesQuality();

    return `Ratio of powers' essential contradiction - the CULMINATION:
    - Quantum is being-for-itself yet still quantum (quantitative)
    - Amount is unit itself yet distinct from unit
    - Otherness is unit itself yet otherness to unit
    - Externality is quantum's own determining yet externality
    - Quantum becomes quality yet remains quantum
    - Quality's truth is quantity yet quantity becomes quality
    - ${beingForItself.getContradiction()}
    - ${conceptRealized.getContradiction()}
    - ${becomeQuality.getContradiction()}

    RESOLUTION: Double transition - quantum's truth is MEASURE.
    Quantum is sublated as indifferent determination, becomes quality
    by virtue of which anything is what it is.

    THE LEAP TO QUANTITATIVE ESSENCE AWAITS! 🔥`;
  }

  transcend(): DialecticalMoment | null {
    // Ratio of Powers transcends into MEASURE - the unity of quality and quantity
    const transition = this.getTransitionToMeasure();
    return transition.getMeasure();
  }
}

// Supporting classes capturing every logical movement from Hegel's text

class AmountFromElsewhere {
  determinedOnlyByUnit(): DeterminedOnlyByUnit { return new DeterminedOnlyByUnit(); }
}

class DeterminedOnlyByUnit {
  unitIsAmount(): UnitIsAmount { return new UnitIsAmount(); }
}

class UnitIsAmount {
  amountAgainstItself(): AmountAgainstItself { return new AmountAgainstItself(); }
}

class AmountAgainstItself {}

class QuantumAsBeingForItself {
  constructor(private amount: AmountAgainstItself) {}
  getContradiction(): string {
    return "Quantum is being-for-itself - unit is amount yet amount against itself as unit";
  }
}

class AggregateOfUnits {
  eachIsThisAggregate(): EachIsThisAggregate { return new EachIsThisAggregate(); }
}

class EachIsThisAggregate {}

class OthernessIsUnitItself {
  constructor(private each: EachIsThisAggregate) {}
}

class ReturnedIntoItself {
  itselfAndItsOtherness(): ItselfAndItsOtherness { return new ItselfAndItsOtherness(); }
}

class ItselfAndItsOtherness {}

class AlterationAsRaisingToPower {
  constructor(private itself: ItselfAndItsOtherness) {}
}

class AmountIsUnitItself {
  selfIdenticalInOtherness(): SelfIdenticalInOtherness { return new SelfIdenticalInOtherness(); }
}

class SelfIdenticalInOtherness {
  limitNotImmediateExistent(): LimitNotImmediateExistent { return new LimitNotImmediateExistent(); }
}

class LimitNotImmediateExistent {
  existenceContinuingInOtherness(): ExistenceContinuingInOtherness { return new ExistenceContinuingInOtherness(); }
}

class ExistenceContinuingInOtherness {}

class ExponentQualitativeNature {
  constructor(private existence: ExistenceContinuingInOtherness) {}
}

class ImmediateAsSublated {}

class TruthOfQualityIsQuantity {
  constructor(private immediate: ImmediateAsSublated) {}
}

class ExistenceDeveloped {
  attainedConcept(): AttainedConcept { return new AttainedConcept(); }
}

class AttainedConcept {
  realizedConceptFullest(): RealizedConceptFullest { return new RealizedConceptFullest(); }
}

class RealizedConceptFullest {}

class QuantumAttainsConcept {
  constructor(private realized: RealizedConceptFullest) {}
  getContradiction(): string {
    return "Quantum has realized its concept to fullest - yet concept was always implicit in quantum";
  }
}

class IndifferentAsSublated {
  limitNoEterminateness(): LimitNoDeterminateness { return new LimitNoDeterminateness(); }
}

class LimitNoDeterminateness {
  continuesInOtherness(): ContinuesInOtherness { return new ContinuesInOtherness(); }
}

class ContinuesInOtherness {
  remainsIdenticalWithItself(): RemainsIdenticalWithItself { return new RemainsIdenticalWithItself(); }
}

class RemainsIdenticalWithItself {}

class DisplayOfImplicit {
  constructor(private remains: RemainsIdenticalWithItself) {}
}

class SurpassingInAnother {
  determinedThroughQuantumItself(): DeterminedThroughQuantumItself { return new DeterminedThroughQuantumItself(); }
}

class DeterminedThroughQuantumItself {}

class OthernessDeterminedThroughQuantum {
  constructor(private determined: DeterminedThroughQuantumItself) {}
}

class DirectRatioImmediate {}
class InverseRatioNegative {}
class PowersRatioDifference {}

class ProgressiveRealization {
  constructor(
    private direct: DirectRatioImmediate,
    private inverse: InverseRatioNegative,
    private powers: PowersRatioDifference
  ) {}
}

class ExternalityOfDeterminateness {
  asQualityOfQuantum(): QualityOfQuantum { return new QualityOfQuantum(); }
}

class QualityOfQuantum {
  asOwnDetermining(): AsOwnDetermining { return new AsOwnDetermining(); }
}

class AsOwnDetermining {
  asReferenceToItself(): AsReferenceToItself { return new AsReferenceToItself(); }
}

class AsReferenceToItself {}

class ExternalityAsOwnDetermining {
  constructor(private reference: AsReferenceToItself) {}
}

class DeterminationAsDeterminateness {
  inItselfAsExistence(): InItselfAsExistence { return new InItselfAsExistence(); }
}

class InItselfAsExistence {}

class QuantumPassedOver {
  constructor(private existence: InItselfAsExistence) {}
}

class ExternalityMediated {
  asMomentOfQuantum(): AsMomentOfQuantum { return new AsMomentOfQuantum(); }
}

class AsMomentOfQuantum {
  refersToItselfInExternality(): RefersToItselfInExternality { return new RefersToItselfInExternality(); }
}

class RefersToItselfInExternality {
  beingAsQuality(): BeingAsQuality { return new BeingAsQuality(); }
}

class BeingAsQuality {}

class QuantumBecomesQuality {
  constructor(private being: BeingAsQuality) {}
  getContradiction(): string {
    return "Quantum becomes quality - externality mediated by quantum itself, refers to itself in externality";
  }
}

class QuantityNotOnlyQuality {
  truthOfQualityIsQuantity(): TruthOfQualityIsQuantityResult { return new TruthOfQualityIsQuantityResult(); }
}

class TruthOfQualityIsQuantityResult {
  externalityReturnedIntoItself(): ExternalityReturnedIntoItself { return new ExternalityReturnedIntoItself(); }
}

class ExternalityReturnedIntoItself {
  noLongerIndifferent(): NoLongerIndifferent { return new NoLongerIndifferent(); }
}

class NoLongerIndifferent {
  quantityIsQuality(): QuantityIsQuality { return new QuantityIsQuality(); }
}

class QuantityIsQuality {}

class QuantityAsTruthOfQuality {
  constructor(private quantity: QuantityIsQuality) {}
}

class FirstTransition {
  identityInItself(): IdentityInItself { return new IdentityInItself(); }
}

class IdentityInItself {
  qualityContainedInQuantity(): QualityContainedInQuantity { return new QualityContainedInQuantity(); }
}

class QualityContainedInQuantity {}

class SecondTransition {
  quantityContainedInQuality(): QuantityContainedInQuality { return new QuantityContainedInQuality(); }
}

class QuantityContainedInQuality {
  goingBackIntoFirst(): GoingBackIntoFirst { return new GoingBackIntoFirst(); }
}

class GoingBackIntoFirst {}

class DoubleTransition {
  constructor(
    private qualityInQuantity: QualityContainedInQuantity,
    private goingBack: GoingBackIntoFirst
  ) {}

  asGreatImportance(): GreatImportance { return new GreatImportance(); }
}

class GreatImportance {}

class ScientificMethodNecessity {
  constructor(private importance: GreatImportance) {}
}

class QualityByVirtueOfWhich {
  anythingIsWhatItIs(): AnythingIsWhatItIs { return new AnythingIsWhatItIs(); }
}

class AnythingIsWhatItIs {}

class QuantumSublatedAsIndifferent {
  constructor(private anything: AnythingIsWhatItIs) {}
  truthIsMeasure(): TruthIsMeasure { return new TruthIsMeasure(); }
}

class TruthIsMeasure {}

class TruthOfQuantumIsMeasure {
  constructor(private truth: TruthIsMeasure) {}
}

class QuantumAsQuality {
  asMeasure(): Measure { return new Measure(); }
}

class Measure {}

class TransitionToMeasure {
  constructor(private measure: Measure) {}
  getMeasure(): Measure { return this.measure; }
}

// Supporting types from previous translations
class QualitativeTotality {}
class ConceptualRealization {}
class InverseRatio {}

export { RatioOfPowers };
