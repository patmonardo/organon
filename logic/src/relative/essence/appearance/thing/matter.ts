/**
 * MATTER - Self-Subsistent Properties and Thing Constitution
 * =========================================================
 *
 * "The necessity of making the transition from properties to matters,
 * or of assuming that the properties are truly matters,
 * has resulted from the fact that they are what is the essential in things
 * and consequently their true self-subsistence."
 */

interface DialecticalMoment {
  getContradiction(): string;
  transcend(): DialecticalMoment | null;
}

export class Matter implements DialecticalMoment {
  private propertyToMatterTransition: PropertyToMatterTransition;
  private thingConstitutionFromMatters: ThingConstitutionFromMatters;
  private negativeThinghood: NegativeThinghood;
  private positiveStuffSubsistence: PositiveStuffSubsistence;

  constructor(property: Property) {
    this.propertyToMatterTransition = new PropertyToMatterTransition(property);
    this.thingConstitutionFromMatters = new ThingConstitutionFromMatters();
    this.negativeThinghood = new NegativeThinghood();
    this.positiveStuffSubsistence = new PositiveStuffSubsistence();
  }

  /**
   * CHEMISTRY'S INSTINCTIVE TRANSITION
   *
   * "The transition of property into a matter or into a self-subsistent stuff
   * is the familiar transition performed on sensible matter by chemistry"
   */
  getChemistryTransition(): ChemistryTransition {
    // "when it seeks to represent the properties of color, smell, etc.,
    // as luminous matter, coloring matter, odorific matter, sour, bitter matter and so on"
    const colorAsLuminousMatter = new ColorAsLuminousMatter();
    const smellAsOdorificMatter = colorAsLuminousMatter.smellAsOdorificMatter();
    const propertyAsMatter = smellAsOdorificMatter.propertyAsMatter();

    // "or when it simply assumes others, like calorific matter, electrical, magnetic matter,
    // in the conviction that it has thereby gotten hold of properties as they truly are"
    const calorificElectricalMatter = propertyAsMatter.calorificElectricalMatter();
    const propertiesAsTheyTrulyAre = calorificElectricalMatter.propertiesAsTheyTrulyAre();

    return new ChemistryTransition(propertiesAsTheyTrulyAre);
  }

  /**
   * THINGS CONSIST OF VARIOUS MATTERS
   *
   * "Equally current is the saying that things consist of various matters or stuffs."
   */
  getThingsConsistOfMatters(): ThingsConsistOfMatters {
    // "One is careful about calling these matters or stuffs 'things,'
    // even though one will readily admit that, for example, a pigment is a thing"
    const carefulAboutCallingThings = new CarefulAboutCallingThings();
    const pigmentIsAThing = carefulAboutCallingThings.pigmentIsAThing();

    // "but I do not know whether luminous matter, for instance, or calorific matter,
    // or electrical matter, etc., are called things"
    const luminousCalorificElectrical = pigmentIsAThing.luminousCalorificElectrical();
    const areCalledThings = luminousCalorificElectrical.areCalledThings();

    // "The distinction is made between things and their components
    // without any exact statement as to whether these components also,
    // and to what extent, are things or perhaps just half-things"
    const distinctionWithoutExactStatement = areCalledThings.distinctionWithoutExactStatement();
    const halfThings = distinctionWithoutExactStatement.halfThings();

    // "but they are at least concretes in general"
    const concretesInGeneral = halfThings.concretesInGeneral();

    return new ThingsConsistOfMatters(concretesInGeneral);
  }

  /**
   * NECESSITY OF PROPERTY-TO-MATTER TRANSITION
   *
   * "The necessity of making the transition from properties to matters...
   * has resulted from the fact that they are what is the essential in things
   * and consequently their true self-subsistence."
   */
  getNecessityOfTransition(): NecessityOfTransition {
    // "they are what is the essential in things and consequently their true self-subsistence"
    const essentialInThings = new EssentialInThings();
    const trueSelfSubsistence = essentialInThings.trueSelfSubsistence();

    return new NecessityOfTransition(trueSelfSubsistence);
  }

  /**
   * REFLECTION OF PROPERTY INTO ITSELF
   *
   * "At the same time, however, the reflection of the property into itself
   * constitutes only one side of the whole reflection"
   */
  getReflectionOfPropertyIntoItself(): ReflectionOfPropertyIntoItself {
    // "namely the sublation of the distinction and the continuity of the property
    // (which was supposed to be a concrete existence for an other) with itself"
    const sublationOfDistinction = new SublationOfDistinction();
    const continuityWithItself = sublationOfDistinction.continuityWithItself();
    const concreteExistenceForAnother = continuityWithItself.concreteExistenceForAnother();

    return new ReflectionOfPropertyIntoItself(concreteExistenceForAnother);
  }

  /**
   * THINGHOOD AS IMMANENT NEGATIVE REFLECTION
   *
   * "Thinghood, as immanent negative reflection and as a distinguishing
   * that repels itself from the other, has consequently been reduced to an unessential moment"
   */
  getThinghoodAsImmanentNegativeReflection(): ThinghoodAsImmanentNegativeReflection {
    const distinguishingRepelsItself = new DistinguishingRepelsItself();
    const reducedToUnessentialMoment = distinguishingRepelsItself.reducedToUnessentialMoment();

    // "at the same time, however, it has further determined itself"
    const furtherDeterminedItself = reducedToUnessentialMoment.furtherDeterminedItself();

    return new ThinghoodAsImmanentNegativeReflection(furtherDeterminedItself);
  }

  /**
   * FIRST: NEGATIVE MOMENT PRESERVED
   *
   * "First, this negative moment has preserved itself, for property has become
   * a matter continuous with itself and self-subsisting only inasmuch as
   * the difference of things has sublated itself"
   */
  getFirstNegativeMomentPreserved(): FirstNegativeMomentPreserved {
    // "property has become a matter continuous with itself and self-subsisting"
    const matterContinuousWithItself = new MatterContinuousWithItself();
    const selfSubsisting = matterContinuousWithItself.selfSubsisting();

    // "only inasmuch as the difference of things has sublated itself"
    const differenceOfThingsSublated = selfSubsisting.differenceOfThingsSublated();

    // "thus the continuity of the property in the otherness itself contains the moment of the negative"
    const continuityInOtherness = differenceOfThingsSublated.continuityInOtherness();
    const containsMomentOfNegative = continuityInOtherness.containsMomentOfNegative();

    return new FirstNegativeMomentPreserved(containsMomentOfNegative);
  }

  /**
   * NEGATIVE UNITY AND RESTORED THINGHOOD
   *
   * "and, as this negative unity, its self-subsistence is at the same time
   * the restored something of thinghood, negative self-subsistence versus
   * the positive self-subsistence of the stuff."
   */
  getNegativeUnityRestoredThinghood(): NegativeUnityRestoredThinghood {
    const negativeUnity = new NegativeUnity();
    const restoredSomethingOfThinghood = negativeUnity.restoredSomethingOfThinghood();
    const negativeVersusPositive = restoredSomethingOfThinghood.negativeVersusPositive();
    const positiveSubsistenceOfStuff = negativeVersusPositive.positiveSubsistenceOfStuff();

    return new NegativeUnityRestoredThinghood(positiveSubsistenceOfStuff);
  }

  /**
   * SECOND: THING PROGRESSED FROM INDETERMINACY TO FULL DETERMINATENESS
   *
   * "Second, the thing has thereby progressed from its indeterminacy to full determinateness."
   */
  getSecondThingProgressedToFullDeterminateness(): SecondThingProgressedToFullDeterminateness {
    // "As thing in itself, it is abstract identity, simple negative concrete existence,
    // or this concrete existence determined as the indeterminate"
    const abstractIdentity = new AbstractIdentity();
    const simpleNegativeConcrete = abstractIdentity.simpleNegativeConcrete();
    const determinedAsIndeterminate = simpleNegativeConcrete.determinedAsIndeterminate();

    // "it is then determined through its properties, by virtue of which
    // it is supposed to be distinguished from other things"
    const determinedThroughProperties = determinedAsIndeterminate.determinedThroughProperties();
    const distinguishedFromOthers = determinedThroughProperties.distinguishedFromOthers();

    return new SecondThingProgressedToFullDeterminateness(distinguishedFromOthers);
  }

  /**
   * THING CONTINUOUS WITH OTHER THINGS THROUGH PROPERTY
   *
   * "but, since through the property the thing is rather continuous with other things,
   * this imperfect distinction is sublated"
   */
  getThingContinuousWithOthers(): ThingContinuousWithOthers {
    // "the thing has thereby returned into itself and is now determined as determined;
    // it is determined in itself or is this thing"
    const returnedIntoItself = new ReturnedIntoItself();
    const determinedAsDetermined = returnedIntoItself.determinedAsDetermined();
    const determinedInItself = determinedAsDetermined.determinedInItself();
    const thisThing = determinedInItself.thisThing();

    return new ThingContinuousWithOthers(thisThing);
  }

  /**
   * THIRD: TURNING BACK INTO ITSELF AS UNESSENTIAL DETERMINATION
   *
   * "But, third, this turning back into itself, though a self-referring determination,
   * is at the same time an unessential determination"
   */
  getThirdTurningBackUnessentialDetermination(): ThirdTurningBackUnessentialDetermination {
    // "the self-continuous subsistence makes up the self-subsistent matter
    // in which the difference of things, their determinateness existing in and for itself, is sublated"
    const selfContinuousSubsistence = new SelfContinuousSubsistence();
    const selfSubsistentMatter = selfContinuousSubsistence.selfSubsistentMatter();
    const differenceOfThingsSublated = selfSubsistentMatter.differenceOfThingsSublated();
    const somethingExternal = differenceOfThingsSublated.somethingExternal();

    return new ThirdTurningBackUnessentialDetermination(somethingExternal);
  }

  /**
   * COMPLETE DETERMINATENESS IN ELEMENT OF INESSENTIALITY
   *
   * "Therefore, although the thing as this thing is complete determinateness,
   * this determinateness is such in the element of inessentiality."
   */
  getCompleteDeterminatenessInInessentiality(): CompleteDeterminatenessInInessentiality {
    const thisThing = this.getThingContinuousWithOthers().getThisThing();
    const completeDeterminateness = thisThing.completeDeterminateness();
    const elementOfInessentiality = completeDeterminateness.elementOfInessentiality();

    return new CompleteDeterminatenessInInessentiality(elementOfInessentiality);
  }

  /**
   * PROPERTY MOVEMENT RESULT
   *
   * "Considered from the side of the movement of the property, this result follows in this way."
   */
  getPropertyMovementResult(): PropertyMovementResult {
    // "The property is not only external determination but concrete existence immediately existing in itself"
    const notOnlyExternalDetermination = new NotOnlyExternalDetermination();
    const concreteExistenceImmediatelyExisting = notOnlyExternalDetermination.concreteExistenceImmediatelyExisting();

    // "This unity of externality and essentiality repels itself from itself"
    const unityOfExternalityEssentiality = concreteExistenceImmediatelyExisting.unityOfExternalityEssentiality();
    const repelsItselfFromItself = unityOfExternalityEssentiality.repelsItselfFromItself();

    return new PropertyMovementResult(repelsItselfFromItself);
  }

  /**
   * REFLECTION-INTO-ITSELF AND REFLECTION-INTO-OTHER
   *
   * "for it contains reflection-into-itself and reflection-into-other"
   */
  getReflectionIntoItselfAndOther(): ReflectionIntoItselfAndOther {
    // "and, on the one hand, it is determination as simple, self-identical and self-referring self-subsistent
    // in which the negative unity, the one of the thing, is sublated"
    const determinationSimpleSelfIdentical = new DeterminationSimpleSelfIdentical();
    const negativeUnitySublated = determinationSimpleSelfIdentical.negativeUnitySublated();

    // "on the other hand, it is this determination over against an other,
    // but likewise as a one which is reflected into itself and is determined in itself"
    const determinationOverAgainstOther = negativeUnitySublated.determinationOverAgainstOther();
    const reflectedIntoItselfDetermined = determinationOverAgainstOther.reflectedIntoItselfDetermined();

    // "it is, therefore, the matters and this thing"
    const mattersAndThisThing = reflectedIntoItselfDetermined.mattersAndThisThing();

    return new ReflectionIntoItselfAndOther(mattersAndThisThing);
  }

  /**
   * TWO MOMENTS OF SELF-IDENTICAL EXTERNALITY
   *
   * "These are the two moments of self-identical externality, or of property reflected into itself."
   */
  getTwoMomentsOfSelfIdenticalExternality(): TwoMomentsOfSelfIdenticalExternality {
    const reflection = this.getReflectionIntoItselfAndOther();
    const mattersAndThing = reflection.getMattersAndThisThing();
    const selfIdenticalExternality = mattersAndThing.asSelfIdenticalExternality();
    const propertyReflectedIntoItself = selfIdenticalExternality.asPropertyReflectedIntoItself();

    return new TwoMomentsOfSelfIdenticalExternality(propertyReflectedIntoItself);
  }

  /**
   * THING FREED FROM NEGATIVE SIDE
   *
   * "Since the thing has freed itself of its negative side of inhering in an other,
   * it has thereby also become free from its being determined by other things
   * and has returned into itself from the reference connecting it to the other."
   */
  getThingFreedFromNegativeSide(): ThingFreedFromNegativeSide {
    const freedFromInheringInOther = new FreedFromInheringInOther();
    const freeFromBeingDeterminedByOthers = freedFromInheringInOther.freeFromBeingDeterminedByOthers();
    const returnedFromReferenceToOther = freeFromBeingDeterminedByOthers.returnedFromReferenceToOther();

    return new ThingFreedFromNegativeSide(returnedFromReferenceToOther);
  }

  /**
   * THING-IN-ITSELF BECOME OTHER OF ITSELF
   *
   * "At the same time, however, it is only the thing-in-itself now become the other of itself,
   * for the manifold properties on their part have become self-subsistent"
   */
  getThingInItselfBecomeOtherOfItself(): ThingInItselfBecomeOtherOfItself {
    // "and their negative connection in the one of the thing is now only a sublated connection"
    const manifoldPropertiesSelfSubsistent = new ManifoldPropertiesSelfSubsistent();
    const negativeConnectionSublated = manifoldPropertiesSelfSubsistent.negativeConnectionSublated();

    // "Consequently, the thing is self-identical negation only as against the positive continuity of the material"
    const selfIdenticalNegation = negativeConnectionSublated.selfIdenticalNegation();
    const againstPositiveContinuity = selfIdenticalNegation.againstPositiveContinuity();

    return new ThingInItselfBecomeOtherOfItself(againstPositiveContinuity);
  }

  /**
   * THE "THIS" CONSTITUTES COMPLETE DETERMINATENESS
   *
   * "The 'this' thus constitutes the complete determinateness of the thing,
   * a determinateness which is at the same time an external determinateness."
   */
  getThisConstitutesCompleteDeterminateness(): ThisConstitutesCompleteDeterminateness {
    const completeDeterminateness = new CompleteDeterminateness();
    const externalDeterminateness = completeDeterminateness.asExternalDeterminateness();

    return new ThisConstitutesCompleteDeterminateness(externalDeterminateness);
  }

  /**
   * THING CONSISTS OF SELF-SUBSISTENT MATTERS INDIFFERENT TO CONNECTION
   *
   * "The thing consists of self-subsistent matters indifferent to the connection they have in the thing."
   */
  getThingConsistsOfIndifferentMatters(): ThingConsistsOfIndifferentMatters {
    // "This connection is therefore only an unessential linking of them"
    const unessentialLinking = new UnessentialLinking();

    // "the difference of one thing from another depending on whether there is in it
    // a more or less of particular matters and in what amount"
    const differenceMoreOrLess = unessentialLinking.differenceMoreOrLess();
    const particularMattersAmount = differenceMoreOrLess.particularMattersAmount();

    return new ThingConsistsOfIndifferentMatters(particularMattersAmount);
  }

  /**
   * MATTERS OVERRUN THIS THING
   *
   * "These matters overrun this thing, continue into others,
   * and that they belong to this thing is no restriction for them."
   */
  getMattersOverrunThisThing(): MattersOverrunThisThing {
    const overrunThisThing = new OverrunThisThing();
    const continueIntoOthers = overrunThisThing.continueIntoOthers();
    const belongingNoRestriction = continueIntoOthers.belongingNoRestriction();

    // "Just as little are they, moreover, a restriction for one another,
    // for their negative connection is only the impotent 'this.'"
    const notRestrictionForOneAnother = belongingNoRestriction.notRestrictionForOneAnother();
    const impotentThis = notRestrictionForOneAnother.impotentThis();

    return new MattersOverrunThisThing(impotentThis);
  }

  /**
   * MATTERS AS SELF-SUBSISTENT IMPENETRABLE
   *
   * "Hence, in being linked together in it, they do not sublate themselves;
   * they are as self-subsistent, impenetrable to each other"
   */
  getMattersImpenetrable(): MattersImpenetrable {
    // "in their determinateness they refer only to themselves
    // and are a mutually indifferent manifold of subsistence"
    const referOnlyToThemselves = new ReferOnlyToThemselves();
    const mutuallyIndifferentManifold = referOnlyToThemselves.mutuallyIndifferentManifold();

    // "the only limit of which they are capable is a quantitative one"
    const onlyQuantitativeLimit = mutuallyIndifferentManifold.onlyQuantitativeLimit();

    return new MattersImpenetrable(onlyQuantitativeLimit);
  }

  /**
   * THING AS MERE QUANTITATIVE CONNECTION
   *
   * "The thing as this is just their merely quantitative connection, a mere collection, their 'also.'"
   */
  getThingAsMereQuantitativeConnection(): ThingAsMereQuantitativeConnection {
    // "The thing consists of some quantum or other of a matter,
    // also of the quantum of another, and also of yet another"
    const someQuantumOrOther = new SomeQuantumOrOther();
    const alsoQuantumOfAnother = someQuantumOrOther.alsoQuantumOfAnother();
    const alsoOfYetAnother = alsoQuantumOfAnother.alsoOfYetAnother();

    // "this combination, of not having any combination alone constitutes the thing"
    const combinationOfNotHavingCombination = alsoOfYetAnother.combinationOfNotHavingCombination();
    const constitutesTheThing = combinationOfNotHavingCombination.constitutesTheThing();

    return new ThingAsMereQuantitativeConnection(constitutesTheThing);
  }

  getContradiction(): string {
    const chemistryTransition = this.getChemistryTransition();
    const thingConsistsIndifferent = this.getThingConsistsOfIndifferentMatters();
    const mattersOverrun = this.getMattersOverrunThisThing();
    const mereQuantitative = this.getThingAsMereQuantitativeConnection();

    return `Matter's essential contradiction - PROPERTIES BECOME SELF-SUBSISTENT:
    - Properties become matters (self-subsistent) yet remain properties
    - Thing consists of matters yet matters are indifferent to their connection in thing
    - Matters overrun this thing yet belong to this thing
    - Thing is complete determinateness yet in element of inessentiality
    - Connection is unessential linking yet constitutes the thing
    - Only quantitative limit yet matters are qualitatively distinct
    - ${chemistryTransition.getContradiction()}
    - ${thingConsistsIndifferent.getContradiction()}
    - ${mattersOverrun.getContradiction()}
    - ${mereQuantitative.getContradiction()}

    RESOLUTION: Thing becomes mere "also" - quantitative collection of self-subsistent matters.
    The combinaiton of not having any combination alone constitutes the thing.

    This is the BREAKDOWN of Essential Relations into mechanical aggregation! 💥`;
  }

  transcend(): DialecticalMoment | null {
    // Matter transcends into further determinations of Essential Relations
    const mereQuantitative = this.getThingAsMereQuantitativeConnection();
    return mereQuantitative.transcendToFurtherEssentialRelations();
  }
}

// Supporting classes for every logical movement

class PropertyToMatterTransition {
  constructor(private property: Property) {}
}

class ColorAsLuminousMatter {
  smellAsOdorificMatter(): SmellAsOdorificMatter { return new SmellAsOdorificMatter(); }
}

class SmellAsOdorificMatter {
  propertyAsMatter(): PropertyAsMatter { return new PropertyAsMatter(); }
}

class PropertyAsMatter {
  calorificElectricalMatter(): CalorificElectricalMatter { return new CalorificElectricalMatter(); }
}

class CalorificElectricalMatter {
  propertiesAsTheyTrulyAre(): PropertiesAsTheyTrulyAre { return new PropertiesAsTheyTrulyAre(); }
}

class PropertiesAsTheyTrulyAre {}

class ChemistryTransition {
  constructor(private properties: PropertiesAsTheyTrulyAre) {}
  getContradiction(): string {
    return "Chemistry seeks properties as they truly are by making them matters - but loses the essential relation";
  }
}

// [Many more supporting classes capturing every logical movement...]

class ThingAsMereQuantitativeConnection {
  constructor(private constitutes: ConstitutesTheThing) {}
  getContradiction(): string {
    return "Thing is mere quantitative connection - 'also' of matters - combination of not having combination";
  }
}

class ConstitutesTheThing {}

class SomeQuantumOrOther {
  alsoQuantumOfAnother(): AlsoQuantumOfAnother { return new AlsoQuantumOfAnother(); }
}

class AlsoQuantumOfAnother {
  alsoOfYetAnother(): AlsoOfYetAnother { return new AlsoOfYetAnother(); }
}

class AlsoOfYetAnother {
  combinationOfNotHavingCombination(): CombinationOfNotHavingCombination { return new CombinationOfNotHavingCombination(); }
}

class CombinationOfNotHavingCombination {
  constitutesTheThing(): ConstitutesTheThing { return new ConstitutesTheThing(); }
}

// Additional supporting classes would follow...

class ThingConstitutionFromMatters {}
class NegativeThinghood {}
class PositiveStuffSubsistence {}

// Placeholder for next dialectical development
class FurtherEssentialRelations {}

export { Matter };
