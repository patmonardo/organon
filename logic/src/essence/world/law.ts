/**
 * LAW OF APPEARANCE - Essential Being as Law Itself
 * ================================================
 *
 * "The law is therefore the essential appearance;
 * it is the latter's reflection into itself in its positedness,
 * the identical content of itself and the unessential concrete existence."
 */

interface DialecticalMoment {
  getContradiction(): string;
  transcend(): DialecticalMoment | null;
}

export class Law implements DialecticalMoment {
  private appearanceAsEssentialConcrete: AppearanceAsEssentialConcrete;
  private reflectiveShineWithConcrete: ReflectiveShineWithConcrete;
  private lawAsPositiveEssentiality: LawAsPositiveEssentiality;
  private kingdomOfLaws: KingdomOfLaws;

  constructor(dissolution: Dissolution) {
    this.appearanceAsEssentialConcrete = new AppearanceAsEssentialConcrete();
    this.reflectiveShineWithConcrete = new ReflectiveShineWithConcrete();
    this.lawAsPositiveEssentiality = new LawAsPositiveEssentiality();
    this.kingdomOfLaws = new KingdomOfLaws();
  }

  /**
   * APPEARANCE AS ESSENCE IN CONCRETE EXISTENCE
   *
   * "At first, therefore, appearance is essence in its concrete existence;
   * essence is immediately present in it."
   */
  getAppearanceAsEssenceInConcreteExistence(): AppearanceAsEssenceInConcreteExistence {
    // "That it is not immediate, but rather reflected concrete existence,
    // constitutes the moment of essence in it"
    const notImmediate = new NotImmediate();
    const reflectedConcreteExistence = notImmediate.reflectedConcreteExistence();
    const momentOfEssence = reflectedConcreteExistence.momentOfEssence();

    // "or concrete existence, as essential concrete existence, is appearance"
    const essentialConcreteExistence = momentOfEssence.essentialConcreteExistence();
    const isAppearance = essentialConcreteExistence.isAppearance();

    return new AppearanceAsEssenceInConcreteExistence(isAppearance);
  }

  /**
   * SOMETHING IS ONLY APPEARANCE
   *
   * "Something is only appearance, in the sense that concrete existence is
   * as such only a posited being, not something that is in-and-for-itself."
   */
  getSomethingIsOnlyAppearance(): SomethingIsOnlyAppearance {
    // "This is what constitutes its essentiality, to have the negativity of reflection,
    // the nature of essence, within it"
    const negativityOfReflection = new NegativityOfReflection();
    const natureOfEssenceWithin = negativityOfReflection.natureOfEssenceWithin();
    const constitutesEssentiality = natureOfEssenceWithin.constitutesEssentiality();

    // "There is no question here of an alien, external reflection
    // to which essence would belong"
    const noAlienExternalReflection = constitutesEssentiality.noAlienExternalReflection();

    return new SomethingIsOnlyAppearance(noAlienExternalReflection);
  }

  /**
   * APPEARANCE'S OWN TRUTH
   *
   * "this essentiality of concrete existence, that it is appearance,
   * is concrete existence's own truth. The reflection by virtue of which
   * it is this is its own."
   */
  getAppearancesOwnTruth(): AppearancesOwnTruth {
    const essentialityOfConcreteExistence = new EssentialityOfConcreteExistence();
    const concreteExistencesOwnTruth = essentialityOfConcreteExistence.concreteExistencesOwnTruth();
    const reflectionIsItsOwn = concreteExistencesOwnTruth.reflectionIsItsOwn();

    return new AppearancesOwnTruth(reflectionIsItsOwn);
  }

  /**
   * APPEARANCE IS THE HIGHER TRUTH
   *
   * "But if it is said that something is only appearance,
   * meaning that as contrasted with it immediate concrete existence is the truth,
   * then the fact is that appearance is the higher truth"
   */
  getAppearanceIsHigherTruth(): AppearanceIsHigherTruth {
    // "for it is concrete existence as essential, whereas concrete existence is appearance
    // that is still void of essence"
    const concreteExistenceAsEssential = new ConcreteExistenceAsEssential();
    const appearanceVoidOfEssence = concreteExistenceAsEssential.appearanceVoidOfEssence();

    // "because it only contains in it the one moment of appearance,
    // namely that of concrete existence as immediate, not yet negative, reflection"
    const oneMomentOfAppearance = appearanceVoidOfEssence.oneMomentOfAppearance();
    const immediateNotYetNegative = oneMomentOfAppearance.immediateNotYetNegative();

    return new AppearanceIsHigherTruth(immediateNotYetNegative);
  }

  /**
   * ESSENCE APPEARS - REAL SHINE
   *
   * "Essence reflectively shines at first just within, in its simple identity;
   * as such, it is abstract reflection. Essence appears, and so it now is real shine"
   */
  getEssenceAppearsRealShine(): EssenceAppearsRealShine {
    // "since the moments of the shine have concrete existence"
    const momentsOfShineHaveConcreteExistence = new MomentsOfShineHaveConcreteExistence();

    // Abstract reflection -> Real shine
    const abstractReflection = new AbstractReflection();
    const realShine = abstractReflection.becomesRealShine(momentsOfShineHaveConcreteExistence);

    return new EssenceAppearsRealShine(realShine);
  }

  /**
   * APPEARANCE AS NEGATIVE MEDIATION OF ITSELF
   *
   * "Appearance, as we have seen, is the thing as the negative mediation of itself with itself"
   */
  getAppearanceAsNegativeMediationOfItself(): AppearanceAsNegativeMediationOfItself {
    // "the differences which it contains are self-subsisting matters
    // which are the contradiction of being an immediate subsistence,
    // yet of obtaining their subsistence only in an alien self-subsistence"
    const selfSubsistingMatters = new SelfSubsistingMatters();
    const contradictionImmediate = selfSubsistingMatters.contradictionImmediate();
    const subsistenceInAlienSubsistence = contradictionImmediate.subsistenceInAlienSubsistence();

    // "hence in the negation of their own, but then again, just because of that,
    // also in the negation of that alien self-subsistence"
    const negationOfTheirOwn = subsistenceInAlienSubsistence.negationOfTheirOwn();
    const negationOfAlien = negationOfTheirOwn.negationOfAlien();
    const negationOfOwnNegation = negationOfAlien.negationOfOwnNegation();

    return new AppearanceAsNegativeMediationOfItself(negationOfOwnNegation);
  }

  /**
   * UNITY OF REFLECTIVE SHINE AND CONCRETE EXISTENCE
   *
   * "Appearance is therefore the unity of reflective shine and concrete existence."
   */
  getUnityOfReflectiveShineAndConcreteExistence(): UnityOfReflectiveShineAndConcreteExistence {
    const reflectiveShine = new ReflectiveShine();
    const concreteExistence = new ConcreteExistence();

    // "Reflective shine is this same mediation, but its fleeting moments obtain in appearance
    // the shape of immediate self-subsistence"
    const fleetingMomentsObtainShape = reflectiveShine.fleetingMomentsObtainShape();
    const immediateShapeInAppearance = fleetingMomentsObtainShape.immediateShapeInAppearance();

    // "On the other hand, the immediate self-subsistence which pertains to concrete existence is reduced to a moment"
    const immediateReducedToMoment = concreteExistence.immediateReducedToMoment();

    const unity = new Unity(immediateShapeInAppearance, immediateReducedToMoment);
    return new UnityOfReflectiveShineAndConcreteExistence(unity);
  }

  /**
   * APPEARANCE DETERMINES ITSELF FURTHER
   *
   * "Appearance now determines itself further. It is concrete existence as essential;
   * as essential, concrete existence differs from the concrete existence which is unessential"
   */
  getAppearanceDeterminesItselfFurther(): AppearanceDeterminesItselfFurther {
    // "It is concrete existence as essential"
    const concreteExistenceAsEssential = new ConcreteExistenceAsEssential();

    // "differs from the concrete existence which is unessential"
    const differsFromUnessential = concreteExistenceAsEssential.differsFromUnessential();

    // "and these two sides refer to each other"
    const twoSidesReferToEachOther = differsFromUnessential.twoSidesReferToEachOther();

    return new AppearanceDeterminesItselfFurther(twoSidesReferToEachOther);
  }

  /**
   * 1. LAW AS SIMPLE SELF-IDENTITY IN DIVERSITY
   *
   * "Appearance is, therefore, first, simple self-identity which also contains
   * diverse content determinations"
   */
  getLawAsSimpleSelfIdentityInDiversity(): LawAsSimpleSelfIdentityInDiversity {
    // "and, both as identity and as the connecting reference of these determinations,
    // is that which remains self-equal in the flux of appearance"
    const connectingReference = new ConnectingReference();
    const remainsSelfEqualInFlux = connectingReference.remainsSelfEqualInFlux();

    // "this is the law of appearance"
    const lawOfAppearance = remainsSelfEqualInFlux.lawOfAppearance();

    return new LawAsSimpleSelfIdentityInDiversity(lawOfAppearance);
  }

  /**
   * A. THE LAW OF APPEARANCE - PART 1
   *
   * "1. Appearance is the concrete existent mediated through its negation,
   * which constitutes its subsistence."
   */
  getLawOfAppearancePart1(): LawOfAppearancePart1 {
    // "This, its negation, is indeed another self-subsistent; but the latter is just as essentially something sublated"
    const itsNegationAnotherSelfSubsistent = new ItsNegationAnotherSelfSubsistent();
    const essentiallySublated = itsNegationAnotherSelfSubsistent.essentiallySublated();

    // "The concrete existent is consequently the turning back of itself into itself
    // through its negation and through the negation of this negation"
    const turningBackIntoItself = essentiallySublated.turningBackIntoItself();
    const throughNegationOfNegation = turningBackIntoItself.throughNegationOfNegation();

    return new LawOfAppearancePart1(throughNegationOfNegation);
  }

  /**
   * ESSENTIAL SELF-SUBSISTENCE AND ABSOLUTE POSITEDNESS
   *
   * "it has, therefore, essential self-subsistence, just as it is equally
   * immediately an absolute positedness that has a ground and an other for its subsistence."
   */
  getEssentialSelfSubsistenceAndAbsolutePositedness(): EssentialSelfSubsistenceAndAbsolutePositedness {
    const essentialSelfSubsistence = new EssentialSelfSubsistence();
    const absolutePositedness = new AbsolutePositedness();
    const groundAndOtherForSubsistence = absolutePositedness.groundAndOtherForSubsistence();

    // "equally immediately" - the dialectical unity
    const equallyImmediately = new EquallyImmediately(essentialSelfSubsistence, groundAndOtherForSubsistence);

    return new EssentialSelfSubsistenceAndAbsolutePositedness(equallyImmediately);
  }

  /**
   * GROUND IS NEGATION AND OTHER IS POSITEDNESS
   *
   * "In the first place, therefore, appearance is concrete existence along with its essentiality,
   * the positedness along with its ground; but this ground is the negation"
   */
  getGroundIsNegationAndOtherIsPositedness(): GroundIsNegationAndOtherIsPositedness {
    // "and the other self-subsistent, the ground of the first, is equally only a positedness"
    const otherSelfSubsistent = new OtherSelfSubsistent();
    const groundOfFirst = otherSelfSubsistent.groundOfFirst();
    const equallyOnlyPositedness = groundOfFirst.equallyOnlyPositedness();

    return new GroundIsNegationAndOtherIsPositedness(equallyOnlyPositedness);
  }

  /**
   * REFLECTED INTO ANOTHER FOR GROUND
   *
   * "Or the concrete existent is, as an appearance, reflected into an other
   * and has this other for its ground, and this ground is itself only this,
   * to be reflected into another."
   */
  getReflectedIntoAnotherForGround(): ReflectedIntoAnotherForGround {
    const reflectedIntoAnOther = new ReflectedIntoAnOther();
    const hasThisOtherForGround = reflectedIntoAnOther.hasThisOtherForGround();
    const groundOnlyToBeReflectedIntoAnother = hasThisOtherForGround.groundOnlyToBeReflectedIntoAnother();

    return new ReflectedIntoAnotherForGround(groundOnlyToBeReflectedIntoAnother);
  }

  /**
   * RETURN OF NOTHING THROUGH NOTHING
   *
   * "The essential self-subsistence that belongs to it because it is a turning back into itself is,
   * for the sake of the negativity of the moments, the return of nothing through nothing back to itself"
   */
  getReturnOfNothingThroughNothing(): ReturnOfNothingThroughNothing {
    const negativityOfMoments = new NegativityOfMoments();
    const returnOfNothing = negativityOfMoments.returnOfNothing();
    const throughNothingBackToItself = returnOfNothing.throughNothingBackToItself();

    // "the self-subsistence of the concrete existent is therefore only the reflective shine of essence"
    const selfSubsistenceOnlyReflectiveShine = throughNothingBackToItself.selfSubsistenceOnlyReflectiveShine();

    return new ReturnOfNothingThroughNothing(selfSubsistenceOnlyReflectiveShine);
  }

  /**
   * RECIPROCAL NEGATION AS LINKAGE
   *
   * "The linkage of the reciprocally grounding concrete existents consists, therefore,
   * in this reciprocal negation"
   */
  getReciprocalNegationAsLinkage(): ReciprocalNegationAsLinkage {
    // "namely that the subsistence of the one is not the subsistence of the other but is its positedness,
    // where this connection of positedness alone constitutes their subsistence"
    const subsistenceOfOneNotOther = new SubsistenceOfOneNotOther();
    const isItsPositedness = subsistenceOfOneNotOther.isItsPositedness();
    const connectionOfPositednessConstitutes = isItsPositedness.connectionOfPositednessConstitutes();

    // "The ground is present as it is in truth, namely as being a first which is only a presupposed"
    const groundPresentInTruth = connectionOfPositednessConstitutes.groundPresentInTruth();
    const firstOnlyPresupposed = groundPresentInTruth.firstOnlyPresupposed();

    return new ReciprocalNegationAsLinkage(firstOnlyPresupposed);
  }

  /**
   * NEGATIVE SIDE AND POSITIVE IDENTITY
   *
   * "This now constitutes the negative side of appearance.
   * In this negative mediation, however, there is immediately contained
   * the positive identity of the concrete existent with itself."
   */
  getNegativeSideAndPositiveIdentity(): NegativeSideAndPositiveIdentity {
    const negativeSideOfAppearance = new NegativeSideOfAppearance();
    const negativeMediation = negativeSideOfAppearance.negativeMediation();
    const positiveIdentityContained = negativeMediation.positiveIdentityContained();
    const concreteExistentWithItself = positiveIdentityContained.concreteExistentWithItself();

    return new NegativeSideAndPositiveIdentity(concreteExistentWithItself);
  }

  /**
   * POSITEDNESS REFERS TO POSITEDNESS
   *
   * "For this concrete existent is not positedness vis-à-vis an essential ground,
   * or is not the reflective shine in a self-subsistent,
   * but is rather positedness that refers itself to a positedness"
   */
  getPositednessRefersToPositedness(): PositednessRefersToPositedness {
    // "or a reflective shine only in a reflective shine"
    const notVisAVisEssentialGround = new NotVisAVisEssentialGround();
    const notReflectiveShineInSelfSubsistent = notVisAVisEssentialGround.notReflectiveShineInSelfSubsistent();
    const positednessRefersToPositedness = notReflectiveShineInSelfSubsistent.positednessRefersToPositedness();
    const reflectiveShineOnlyInReflectiveShine = positednessRefersToPositedness.reflectiveShineOnlyInReflectiveShine();

    return new PositednessRefersToPositedness(reflectiveShineOnlyInReflectiveShine);
  }

  /**
   * SELF-IDENTICAL POSITIVE ESSENTIALITY
   *
   * "In this, its negation, or in its other which is itself something sublated,
   * it refers to itself and is thus self-identical or positive essentiality."
   */
  getSelfIdenticalPositiveEssentiality(): SelfIdenticalPositiveEssentiality {
    const inItsNegation = new InItsNegation();
    const otherItselfSublated = inItsNegation.otherItselfSublated();
    const refersToItself = otherItselfSublated.refersToItself();
    const selfIdenticalPositiveEssentiality = refersToItself.selfIdenticalPositiveEssentiality();

    return new SelfIdenticalPositiveEssentiality(selfIdenticalPositiveEssentiality);
  }

  /**
   * NOT IMMEDIACY OF CONCRETE EXISTENCE
   *
   * "This identity is not the immediacy that pertains to concrete existence as such
   * and only is its unessential moment of subsisting in an other."
   */
  getNotImmediacyOfConcreteExistence(): NotImmediacyOfConcreteExistence {
    const notImmediacy = new NotImmediacy();
    const pertainsToConcreteExistence = notImmediacy.pertainsToConcreteExistence();
    const unessentialMomentSubsisting = pertainsToConcreteExistence.unessentialMomentSubsisting();

    return new NotImmediacyOfConcreteExistence(unessentialMomentSubsisting);
  }

  /**
   * ESSENTIAL CONTENT WITH TWO SIDES
   *
   * "It is rather the essential content of appearance which has two sides:
   * first, to be in the form of positedness or external immediacy;
   * second, to be positedness as self-identical."
   */
  getEssentialContentWithTwoSides(): EssentialContentWithTwoSides {
    // First side: form of positedness or external immediacy
    const formOfPositedness = new FormOfPositedness();
    const externalImmediacy = formOfPositedness.externalImmediacy();

    // Second side: positedness as self-identical
    const positednessAsSelfIdentical = new PositednessAsSelfIdentical();

    const twoSides = new TwoSides(externalImmediacy, positednessAsSelfIdentical);
    const essentialContentOfAppearance = new EssentialContentOfAppearance(twoSides);

    return new EssentialContentWithTwoSides(essentialContentOfAppearance);
  }

  /**
   * FIRST SIDE: DETERMINATE BEING AS ACCIDENTAL
   *
   * "According to the first side, it is as a determinate being,
   * but one which in keeping with its immediacy is accidental, unessential,
   * and subject to transition, to coming-to-be and passing-away."
   */
  getFirstSideDeterminateBeingAccidental(): FirstSideDeterminateBeingAccidental {
    const determinateBeing = new DeterminateBeing();
    const accidentalUnessential = determinateBeing.accidentalUnessential();
    const subjectToTransition = accidentalUnessential.subjectToTransition();
    const comingToBePassingAway = subjectToTransition.comingToBePassingAway();

    return new FirstSideDeterminateBeingAccidental(comingToBePassingAway);
  }

  /**
   * SECOND SIDE: SIMPLE CONTENT DETERMINATION
   *
   * "According to the other side, it is the simple content determination
   * exempted from that flux, the permanent element in it."
   */
  getSecondSideSimpleContentDetermination(): SecondSideSimpleContentDetermination {
    const simpleContentDetermination = new SimpleContentDetermination();
    const exemptedFromFlux = simpleContentDetermination.exemptedFromFlux();
    const permanentElement = exemptedFromFlux.permanentElement();

    return new SecondSideSimpleContentDetermination(permanentElement);
  }

  /**
   * THIS UNITY IS THE LAW OF APPEARANCE
   *
   * The culmination - where the dialectical movement reaches LAW
   */
  getThisUnityIsTheLawOfAppearance(): ThisUnityIsTheLawOfAppearance {
    const firstSide = this.getFirstSideDeterminateBeingAccidental();
    const secondSide = this.getSecondSideSimpleContentDetermination();
    const essentialContent = this.getEssentialContentWithTwoSides();

    // The reflection of the two-sided subsistence
    const reflectionIntoItself = new ReflectionIntoItself();
    const identityOfTwoSidedSubsistence = reflectionIntoItself.identityOfTwoSidedSubsistence();
    const positednessOfOneIsPositednessOfOther = identityOfTwoSidedSubsistence.positednessOfOneIsPositednessOfOther();

    // "The two constitute one subsistence, each at the same time as a different content indifferent to the other"
    const twoConstituteOneSubsistence = positednessOfOneIsPositednessOfOther.twoConstituteOneSubsistence();
    const differentContentIndifferent = twoConstituteOneSubsistence.differentContentIndifferent();

    // "This unity is the law of appearance"
    const lawOfAppearance = differentContentIndifferent.asLawOfAppearance();

    return new ThisUnityIsTheLawOfAppearance(lawOfAppearance);
  }

  getContradiction(): string {
    const appearanceHigher = this.getAppearanceIsHigherTruth();
    const negativeMediationItself = this.getAppearanceAsNegativeMediationOfItself();
    const returnNothing = this.getReturnOfNothingThroughNothing();
    const unityLaw = this.getThisUnityIsTheLawOfAppearance();

    return `Law's essential contradiction - ESSENTIAL BEING AS LAW ITSELF:
    - Appearance is higher truth than immediate concrete existence
    - Essence appears as real shine - moments have concrete existence
    - Appearance is negative mediation of itself with itself through negation of negation
    - Self-subsistence is only reflective shine - return of nothing through nothing
    - Positedness refers to positedness - reflective shine in reflective shine
    - Essential content has two sides - accidental determinate being yet permanent element
    - ${appearanceHigher.getContradiction()}
    - ${negativeMediationItself.getContradiction()}
    - ${returnNothing.getContradiction()}
    - ${unityLaw.getContradiction()}

    RESOLUTION: This unity is the LAW of appearance - the identity of two-sided subsistence,
    where positedness of one is positedness of other, yet each different content indifferent.

    LAW IS NOT A MODEL - IT IS ESSENTIAL BEING ITSELF! 🔥⚡`;
  }

  transcend(): DialecticalMoment | null {
    // Law transcends into further determinations of Essential Relations
    const lawOfAppearance = this.getThisUnityIsTheLawOfAppearance();
    return lawOfAppearance.transcendToFurtherLawDeterminations();
  }
}

// Supporting classes for every logical movement - the complete dialectical structure

class AppearanceAsEssenceInConcreteExistence {
  constructor(private isAppearance: IsAppearance) {}
}

class NotImmediate {
  reflectedConcreteExistence(): ReflectedConcreteExistence { return new ReflectedConcreteExistence(); }
}

class ReflectedConcreteExistence {
  momentOfEssence(): MomentOfEssence { return new MomentOfEssence(); }
}

class MomentOfEssence {
  essentialConcreteExistence(): EssentialConcreteExistence { return new EssentialConcreteExistence(); }
}

class EssentialConcreteExistence {
  isAppearance(): IsAppearance { return new IsAppearance(); }
}

class IsAppearance {}

class SomethingIsOnlyAppearance {
  constructor(private noAlien: NoAlienExternalReflection) {}
}

class NegativityOfReflection {
  natureOfEssenceWithin(): NatureOfEssenceWithin { return new NatureOfEssenceWithin(); }
}

class NatureOfEssenceWithin {
  constitutesEssentiality(): ConstitutesEssentiality { return new ConstitutesEssentiality(); }
}

class ConstitutesEssentiality {
  noAlienExternalReflection(): NoAlienExternalReflection { return new NoAlienExternalReflection(); }
}

class NoAlienExternalReflection {}

class AppearanceIsHigherTruth {
  constructor(private immediateNotYet: ImmediateNotYetNegative) {}
  getContradiction(): string {
    return "Appearance is higher truth - concrete existence as essential vs appearance void of essence";
  }
}

class ConcreteExistenceAsEssential {
  appearanceVoidOfEssence(): AppearanceVoidOfEssence { return new AppearanceVoidOfEssence(); }
}

class AppearanceVoidOfEssence {
  oneMomentOfAppearance(): OneMomentOfAppearance { return new OneMomentOfAppearance(); }
}

class OneMomentOfAppearance {
  immediateNotYetNegative(): ImmediateNotYetNegative { return new ImmediateNotYetNegative(); }
}

class ImmediateNotYetNegative {}

class ThisUnityIsTheLawOfAppearance {
  constructor(private lawOfAppearance: LawOfAppearanceCore) {}
  getContradiction(): string {
    return "Unity is the law of appearance - identity of two-sided subsistence as different content indifferent";
  }

  transcendToFurtherLawDeterminations(): FurtherLawDeterminations | null {
    return new FurtherLawDeterminations();
  }
}

class LawOfAppearanceCore {}
class FurtherLawDeterminations {}

// Many more supporting classes capturing every dialectical movement...
class AbstractReflection {
  becomesRealShine(moments: MomentsOfShineHaveConcreteExistence): RealShine { return new RealShine(); }
}
class MomentsOfShineHaveConcreteExistence {}
class RealShine {}
class EssenceAppearsRealShine {
  constructor(private realShine: RealShine) {}
}

// [Hundreds more classes would follow for complete dialectical capture...]

export { Law };
