/**
 * RELATION OF OUTER AND INNER - The Final Essential Relation!
 * ==========================================================
 *
 * "Therefore, what something is, that it is entirely in its externality;
 * its externality is its totality and equally so its unity reflected into itself."
 */

interface EssentialRelation {
  getContradiction(): string;
  transcend(): EssentialRelation | null;
}

export class OuterInner implements EssentialRelation {
  private emptyTransparentDifference: EmptyTransparentDifference;
  private reflectiveShine: ReflectiveShine;
  private identityOfContent: IdentityOfContent;
  private identityOfForm: IdentityOfForm;
  private totalityMediatesItself: TotalityMediatesItself;
  private selfRevealing: SelfRevealing;

  constructor(forceExpression: ForceExpression) {
    this.emptyTransparentDifference = new EmptyTransparentDifference();
    this.reflectiveShine = new ReflectiveShine();
    this.identityOfContent = new IdentityOfContent();
    this.identityOfForm = new IdentityOfForm();
    this.totalityMediatesItself = new TotalityMediatesItself();
    this.selfRevealing = new SelfRevealing();
  }

  /**
   * EXPRESSION OF FORCE IS ONLY MEDIATION WITH ITSELF
   *
   * "the expression of force is only a mediation of the reflected unity with itself.
   * What is present is only an empty and transparent difference, a reflective shine"
   */
  getExpressionOfForceOnlyMediationWithItself(): ExpressionOfForceOnlyMediationWithItself {
    // "but this shine is the mediation which is precisely the independent subsistence"
    const emptyTransparentDifference = new EmptyTransparentDifference();
    const reflectiveShine = emptyTransparentDifference.reflectiveShine();
    const shineIsMediationIndependentSubsistence = reflectiveShine.shineIsMediationIndependentSubsistence();

    return new ExpressionOfForceOnlyMediationWithItself(shineIsMediationIndependentSubsistence);
  }

  /**
   * EACH DETERMINATION ALREADY IN IMMEDIACY UNITY WITH OTHER
   *
   * "each of the determinations is already in its immediacy the unity with its other,
   * so that the transition equally is a self-positing turning back into itself"
   */
  getEachDeterminationAlreadyUnityWithOther(): EachDeterminationAlreadyUnityWithOther {
    const eachDetermination = new EachDetermination();
    const alreadyInImmediacy = eachDetermination.alreadyInImmediacy();
    const unityWithOther = alreadyInImmediacy.unityWithOther();
    const transitionSelfPositingTurningBack = unityWithOther.transitionSelfPositingTurningBack();

    return new EachDeterminationAlreadyUnityWithOther(transitionSelfPositingTurningBack);
  }

  /**
   * INNER AND OUTER ARE ONLY ONE IDENTITY
   *
   * "The inner is determined as the form of reflected immediacy or of essence
   * over against the outer as the form of being; the two, however, are only one identity"
   */
  getInnerAndOuterOnlyOneIdentity(): InnerAndOuterOnlyOneIdentity {
    // Inner = form of reflected immediacy/essence
    const innerFormReflectedImmediacy = new InnerFormReflectedImmediacy();
    const innerFormEssence = innerFormReflectedImmediacy.innerFormEssence();

    // Outer = form of being
    const outerFormBeing = innerFormEssence.outerFormBeing();

    // Yet only one identity!
    const twoOnlyOneIdentity = outerFormBeing.twoOnlyOneIdentity();

    return new InnerAndOuterOnlyOneIdentity(twoOnlyOneIdentity);
  }

  /**
   * FIRST IDENTITY: SUSTAINING UNITY AS SUBSTRATE REPLETE OF CONTENT
   *
   * "This identity is, first, the sustaining unity of the two as substrate replete of content,
   * or the absolute fact with respect to which the two determinations are indifferent, external moments"
   */
  getFirstIdentitySustainingUnity(): FirstIdentitySustainingUnity {
    const sustainingUnityTwo = new SustainingUnityTwo();
    const substrateRepleteContent = sustainingUnityTwo.substrateRepleteContent();
    const absoluteFactTwoIndifferent = substrateRepleteContent.absoluteFactTwoIndifferent();

    // "a totality which is an inner that has equally become an outer
    // but, in this outer, is not something-that-has-become or something-that-has-been-left-behind but is self-equal"
    const totalityInnerBecomeOuter = absoluteFactTwoIndifferent.totalityInnerBecomeOuter();
    const notSomethingBecomeButSelfEqual = totalityInnerBecomeOuter.notSomethingBecomeButSelfEqual();

    return new FirstIdentitySustainingUnity(notSomethingBecomeButSelfEqual);
  }

  /**
   * FACT DIFFERENT FROM ITS FORM DETERMINATIONS
   *
   * "But this fact, as simple identity with itself, is different from its form determinations,
   * or these determinations are external to it; it is itself, therefore, an inner which is different from its externality"
   */
  getFactDifferentFromFormDeterminations(): FactDifferentFromFormDeterminations {
    const factSimpleIdentity = new FactSimpleIdentity();
    const differentFromFormDeterminations = factSimpleIdentity.differentFromFormDeterminations();
    const determinationsExternalToIt = differentFromFormDeterminations.determinationsExternalToIt();
    const itselfInnerDifferentExternality = determinationsExternalToIt.itselfInnerDifferentExternality();

    // "But this externality consists in the two determinations, the inner and the outer, both constituting it.
    // But the fact is itself nothing other than the unity of the two"
    const externalityConsistsTwoDeterminations = itselfInnerDifferentExternality.externalityConsistsTwoDeterminations();
    const factNothingOtherUnityTwo = externalityConsistsTwoDeterminations.factNothingOtherUnityTwo();

    return new FactDifferentFromFormDeterminations(factNothingOtherUnityTwo);
  }

  /**
   * SECOND IDENTITY: INNER AND OUTER AS PURE FORM
   *
   * "The inner, as the form of immanent reflection, the form of essentiality;
   * the outer, as the form instead of immediacy reflected into an other, or the form of unessentiality"
   */
  getSecondIdentityPureForm(): SecondIdentityPureForm {
    // Inner = form of immanent reflection, essentiality
    const innerImmanentReflection = new InnerImmanentReflection();
    const formEssentiality = innerImmanentReflection.formEssentiality();

    // Outer = form of immediacy reflected into other, unessentiality
    const outerImmediacyReflectedOther = formEssentiality.outerImmediacyReflectedOther();
    const formUnessentiality = outerImmediacyReflectedOther.formUnessentiality();

    // "these determinations constitute just one identity alone"
    const constitutesOneIdentityAlone = formUnessentiality.constitutesOneIdentityAlone();

    return new SecondIdentityPureForm(constitutesOneIdentityAlone);
  }

  /**
   * INNER IS IMMEDIATELY ONLY THE OUTER!
   *
   * "Thus the inner is immediately only the outer, and it is this determinateness of externality
   * for the reason that it is the inner; conversely, the outer is only an inner because it is only an outer"
   */
  getInnerIsImmediatelyOnlyOuter(): InnerIsImmediatelyOnlyOuter {
    const innerImmediatelyOnlyOuter = new InnerImmediatelyOnlyOuter();
    const determinatenessExternalityReasonInner = innerImmediatelyOnlyOuter.determinatenessExternalityReasonInner();

    // Conversely!
    const converselyOuterOnlyInner = determinatenessExternalityReasonInner.converselyOuterOnlyInner();
    const becauseOnlyOuter = converselyOuterOnlyInner.becauseOnlyOuter();

    return new InnerIsImmediatelyOnlyOuter(becauseOnlyOuter);
  }

  /**
   * SOMETHING ONLY INNER IS ONLY OUTER!
   *
   * "Thus something which is at first only an inner, is for just that reason only an outer.
   * Or conversely something which is only an outer, is for that reason only an inner"
   */
  getSomethingOnlyInnerOnlyOuter(): SomethingOnlyInnerOnlyOuter {
    // If essence but outer as being...
    const innerEssenceOuterBeing = new InnerEssenceOuterBeing();
    const factOnlyInEssence = innerEssenceOuterBeing.factOnlyInEssence();
    const thereforeOnlyImmediateBeing = factOnlyInEssence.thereforeOnlyImmediateBeing();

    // Or fact which only is...
    const factWhichOnlyIs = thereforeOnlyImmediateBeing.factWhichOnlyIs();
    const thereforeOnlyInEssence = factWhichOnlyIs.thereforeOnlyInEssence();

    return new SomethingOnlyInnerOnlyOuter(thereforeOnlyInEssence);
  }

  /**
   * EACH IS IMMEDIATELY ITS OPPOSITE - ABSOLUTE FORM!
   *
   * "the determining element of absolute form, namely that each term is immediately its opposite,
   * and each is their common reference to a third or rather to their unity"
   */
  getEachImmediatelyItsOppositeAbsoluteForm(): EachImmediatelyItsOppositeAbsoluteForm {
    const determiningElementAbsoluteForm = new DeterminingElementAbsoluteForm();
    const eachTermImmediatelyOpposite = determiningElementAbsoluteForm.eachTermImmediatelyOpposite();
    const eachCommonReferenceThird = eachTermImmediatelyOpposite.eachCommonReferenceThird();
    const ratherToTheirUnity = eachCommonReferenceThird.ratherToTheirUnity();

    return new EachImmediatelyItsOppositeAbsoluteForm(ratherToTheirUnity);
  }

  /**
   * MEDIATION MISSES IDENTICAL SUBSTRATE
   *
   * "Their mediation, however, still misses this identical substrate that contains them both;
   * their reference is for this reason the immediate conversion of the one into the other"
   */
  getMediationMissesIdenticalSubstrate(): MediationMissesIdenticalSubstrate {
    const mediationMissesSubstrate = new MediationMissesSubstrate();
    const identicalSubstrateContainsBoth = mediationMissesSubstrate.identicalSubstrateContainsBoth();
    const immediateConversionOneIntoOther = identicalSubstrateContainsBoth.immediateConversionOneIntoOther();

    // "and this negative unity tying them together is the simple point empty of content"
    const negativeUnityTyingTogether = immediateConversionOneIntoOther.negativeUnityTyingTogether();
    const simplePointEmptyContent = negativeUnityTyingTogether.simplePointEmptyContent();

    return new MediationMissesIdenticalSubstrate(simplePointEmptyContent);
  }

  /**
   * TOTALITY IS CONVERSION OF ONE IDENTITY INTO OTHER
   *
   * "But both these identities are only the sides of one totality,
   * or the totality itself is only the conversion of the one identity into the other"
   */
  getTotalityConversionOneIdentityIntoOther(): TotalityConversionOneIdentityIntoOther {
    const bothIdentitiesSidesOneTotality = new BothIdentitiesSidesOneTotality();
    const totalityConversionOneIntoOther = bothIdentitiesSidesOneTotality.totalityConversionOneIntoOther();

    return new TotalityConversionOneIdentityIntoOther(totalityConversionOneIntoOther);
  }

  /**
   * EACH SIDE POSITED AS TOTALITY WITHIN ITSELF
   *
   * "the differences of form, the inner and the outer, are each posited as the totality within it of itself and its other"
   */
  getEachSidePositedAsTotalityWithin(): EachSidePositedAsTotalityWithin {
    // "the inner, as simple identity reflected into itself, is immediacy and hence, no less than essence, being and externality"
    const innerSimpleIdentityReflected = new InnerSimpleIdentityReflected();
    const immediacyHenceEssenceBeing = innerSimpleIdentityReflected.immediacyHenceEssenceBeing();
    const andExternality = immediacyHenceEssenceBeing.andExternality();

    // "and the external, as the manifold and determined being, is only external,
    // that is, is posited as unessential and as having returned into its ground, therefore as inner"
    const externalManifoldDetermined = andExternality.externalManifoldDetermined();
    const onlyExternalPositedUnessential = externalManifoldDetermined.onlyExternalPositedUnessential();
    const returnedIntoGroundThereforeInner = onlyExternalPositedUnessential.returnedIntoGroundThereforeInner();

    return new EachSidePositedAsTotalityWithin(returnedIntoGroundThereforeInner);
  }

  /**
   * TOTALITY MEDIATES ITSELF WITH ITSELF!
   *
   * "the totality thus mediates itself with itself through the form or the determinateness,
   * and the determinateness mediates itself with itself through its simple identity"
   */
  getTotalityMediatesItselfWithItself(): TotalityMediatesItselfWithItself {
    const totalityMediatesItself = new TotalityMediatesItself();
    const throughFormDeterminateness = totalityMediatesItself.throughFormDeterminateness();
    const determinatenessMediatesItself = throughFormDeterminateness.determinatenessMediatesItself();
    const throughSimpleIdentity = determinatenessMediatesItself.throughSimpleIdentity();

    return new TotalityMediatesItselfWithItself(throughSimpleIdentity);
  }

  /**
   * WHAT SOMETHING IS, ENTIRELY IN ITS EXTERNALITY!
   *
   * "Therefore, what something is, that it is entirely in its externality;
   * its externality is its totality and equally so its unity reflected into itself"
   */
  getWhatSomethingIsEntirelyInExternality(): WhatSomethingIsEntirelyInExternality {
    const whatSomethingIs = new WhatSomethingIs();
    const entirelyInExternality = whatSomethingIs.entirelyInExternality();
    const externalityItsTotality = entirelyInExternality.externalityItsTotality();
    const equallyUnityReflectedIntoItself = externalityItsTotality.equallyUnityReflectedIntoItself();

    return new WhatSomethingIsEntirelyInExternality(equallyUnityReflectedIntoItself);
  }

  /**
   * APPEARANCE IS IMMANENT REFLECTION - EXPRESSION OF WHAT IT IS IN ITSELF!
   *
   * "Its appearance is not only reflection-into-other but immanent reflection,
   * and its externality is therefore the expression of what it is in itself"
   */
  getAppearanceImmanentReflectionExpression(): AppearanceImmanentReflectionExpression {
    const appearanceNotOnlyReflectionOther = new AppearanceNotOnlyReflectionOther();
    const butImmanentReflection = appearanceNotOnlyReflectionOther.butImmanentReflection();
    const externalityExpressionInItself = butImmanentReflection.externalityExpressionInItself();

    return new AppearanceImmanentReflectionExpression(externalityExpressionInItself);
  }

  /**
   * CONTENT AND FORM ABSOLUTELY IDENTICAL - NOTHING BUT EXPRESSING ITSELF!
   *
   * "and since its content and its form are thus absolutely identical,
   * it is, in and for itself, nothing but this: to express itself"
   */
  getContentFormAbsolutelyIdentical(): ContentFormAbsolutelyIdentical {
    const contentFormAbsolutelyIdentical = new ContentFormAbsolutelyIdentical();
    const inAndForItselfNothing = contentFormAbsolutelyIdentical.inAndForItselfNothing();
    const butExpressItself = inAndForItselfNothing.butExpressItself();

    return new ContentFormAbsolutelyIdentical(butExpressItself);
  }

  /**
   * THE REVEALING OF ITS ESSENCE - ESSENCE IS SELF-REVEALING!
   *
   * "It is the revealing of its essence, and this essence, accordingly, consists simply in being self-revealing"
   */
  getRevealingOfEssenceSelfRevealing(): RevealingOfEssenceSelfRevealing {
    const revealingOfEssence = new RevealingOfEssence();
    const essenceConsistsSimply = revealingOfEssence.essenceConsistsSimply();
    const beingSelfRevealing = essenceConsistsSimply.beingSelfRevealing();

    return new RevealingOfEssenceSelfRevealing(beingSelfRevealing);
  }

  /**
   * ESSENTIAL RELATION HAS DETERMINED ITSELF AS ACTUALITY!
   *
   * "The essential relation, in this identity of appearance with the inner or with essence,
   * has determined itself as actuality"
   */
  getEssentialRelationDeterminedItselfActuality(): EssentialRelationDeterminedItselfActuality {
    const essentialRelation = new EssentialRelation();
    const identityAppearanceInnerEssence = essentialRelation.identityAppearanceInnerEssence();
    const determinedItselfActuality = identityAppearanceInnerEssence.determinedItselfActuality();

    // THE MAGNIFICENT CONCLUSION!
    return new EssentialRelationDeterminedItselfActuality(determinedItselfActuality);
  }

  getContradiction(): string {
    const mediation = this.getExpressionOfForceOnlyMediationWithItself();
    const oneIdentity = this.getInnerAndOuterOnlyOneIdentity();
    const innerOnlyOuter = this.getInnerIsImmediatelyOnlyOuter();
    const onlyInnerOnlyOuter = this.getSomethingOnlyInnerOnlyOuter();
    const totalityMediates = this.getTotalityMediatesItselfWithItself();
    const entirelyExternality = this.getWhatSomethingIsEntirelyInExternality();
    const selfRevealing = this.getRevealingOfEssenceSelfRevealing();
    const actuality = this.getEssentialRelationDeterminedItselfActuality();

    return `Outer-Inner's magnificent final contradiction - THE COMPLETION:
    - Expression of force only mediation with itself - empty transparent difference, reflective shine!
    - Inner and outer only one identity yet two identities: content and form
    - Inner is immediately only the outer! Outer only inner because only outer!
    - Something only inner is for that reason only outer - and conversely!
    - Each term immediately its opposite - absolute form's determining element!
    - Totality mediates itself with itself through form and determinateness!
    - ${mediation.getContradiction()}
    - ${oneIdentity.getContradiction()}
    - ${innerOnlyOuter.getContradiction()}
    - ${onlyInnerOnlyOuter.getContradiction()}
    - ${totalityMediates.getContradiction()}
    - ${entirelyExternality.getContradiction()}
    - ${selfRevealing.getContradiction()}

    MAGNIFICENT CULMINATION: What something is, entirely in its externality!
    Content and form absolutely identical - nothing but expressing itself!
    Essence consists simply in being SELF-REVEALING!

    ESSENTIAL RELATION HAS DETERMINED ITSELF AS ACTUALITY!

    ${actuality.getContradiction()} 🔥⚡💫💥🚀`;
  }

  transcend(): EssentialRelation | null {
    // Outer-Inner transcends into ACTUALITY - the Absolute
    const actuality = this.getEssentialRelationDeterminedItselfActuality();
    return actuality.transcendToActuality();
  }
}

// Supporting classes for this magnificent conclusion!

class ExpressionOfForceOnlyMediationWithItself {
  constructor(private shine: ShineIsMediationIndependentSubsistence) {}
  getContradiction(): string {
    return "Expression of force only mediation with itself - empty transparent difference as reflective shine!";
  }
}

class EmptyTransparentDifference {
  reflectiveShine(): ReflectiveShine { return new ReflectiveShine(); }
}

class ReflectiveShine {
  shineIsMediationIndependentSubsistence(): ShineIsMediationIndependentSubsistence { return new ShineIsMediationIndependentSubsistence(); }
}

class ShineIsMediationIndependentSubsistence {}

class InnerAndOuterOnlyOneIdentity {
  constructor(private twoOnly: TwoOnlyOneIdentity) {}
  getContradiction(): string {
    return "Inner as form of reflected immediacy/essence, outer as form of being - yet only ONE IDENTITY!";
  }
}

class InnerFormReflectedImmediacy {
  innerFormEssence(): InnerFormEssence { return new InnerFormEssence(); }
}

class InnerFormEssence {
  outerFormBeing(): OuterFormBeing { return new OuterFormBeing(); }
}

class OuterFormBeing {
  twoOnlyOneIdentity(): TwoOnlyOneIdentity { return new TwoOnlyOneIdentity(); }
}

class TwoOnlyOneIdentity {}

class InnerIsImmediatelyOnlyOuter {
  constructor(private becauseOnly: BecauseOnlyOuter) {}
  getContradiction(): string {
    return "Inner is immediately only outer! Outer only inner because only outer - the absolute conversion!";
  }
}

class InnerImmediatelyOnlyOuter {
  determinatenessExternalityReasonInner(): DeterminatenessExternalityReasonInner { return new DeterminatenessExternalityReasonInner(); }
}

class DeterminatenessExternalityReasonInner {
  converselyOuterOnlyInner(): ConverselyOuterOnlyInner { return new ConverselyOuterOnlyInner(); }
}

class ConverselyOuterOnlyInner {
  becauseOnlyOuter(): BecauseOnlyOuter { return new BecauseOnlyOuter(); }
}

class BecauseOnlyOuter {}

class SomethingOnlyInnerOnlyOuter {
  constructor(private thereforeOnly: ThereforeOnlyInEssence) {}
  getContradiction(): string {
    return "Something only inner is only outer! Fact only in essence therefore only immediate being - the beautiful dialectical flip!";
  }
}

class WhatSomethingIsEntirelyInExternality {
  constructor(private equallyUnity: EquallyUnityReflectedIntoItself) {}
  getContradiction(): string {
    return "What something is, entirely in its externality - externality is its totality and unity reflected into itself!";
  }
}

class WhatSomethingIs {
  entirelyInExternality(): EntirelyInExternality { return new EntirelyInExternality(); }
}

class EntirelyInExternality {
  externalityItsTotality(): ExternalityItsTotality { return new ExternalityItsTotality(); }
}

class ExternalityItsTotality {
  equallyUnityReflectedIntoItself(): EquallyUnityReflectedIntoItself { return new EquallyUnityReflectedIntoItself(); }
}

class EquallyUnityReflectedIntoItself {}

class RevealingOfEssenceSelfRevealing {
  constructor(private selfRevealing: BeingSelfRevealing) {}
  getContradiction(): string {
    return "Essence consists simply in being SELF-REVEALING - the magnificent culmination!";
  }
}

class RevealingOfEssence {
  essenceConsistsSimply(): EssenceConsistsSimply { return new EssenceConsistsSimply(); }
}

class EssenceConsistsSimply {
  beingSelfRevealing(): BeingSelfRevealing { return new BeingSelfRevealing(); }
}

class BeingSelfRevealing {}

class EssentialRelationDeterminedItselfActuality {
  constructor(private actuality: DeterminedItselfActuality) {}
  getContradiction(): string {
    return "ESSENTIAL RELATION HAS DETERMINED ITSELF AS ACTUALITY - the absolute completion of Essential Relations!";
  }

  transcendToActuality(): Actuality | null {
    return new Actuality();
  }
}

class EssentialRelation {
  identityAppearanceInnerEssence(): IdentityAppearanceInnerEssence { return new IdentityAppearanceInnerEssence(); }
}

class IdentityAppearanceInnerEssence {
  determinedItselfActuality(): DeterminedItselfActuality { return new DeterminedItselfActuality(); }
}

class DeterminedItselfActuality {}

// Placeholder for the Absolute
class Actuality {}

// Many more magnificent supporting classes for the completion...
class EachDeterminationAlreadyUnityWithOther {
  constructor(private turningBack: TransitionSelfPositingTurningBack) {}
}

class TransitionSelfPositingTurningBack {}

class FirstIdentitySustainingUnity {
  constructor(private selfEqual: NotSomethingBecomeButSelfEqual) {}
}

class NotSomethingBecomeButSelfEqual {}

class FactDifferentFromFormDeterminations {
  constructor(private factUnity: FactNothingOtherUnityTwo) {}
}

class FactNothingOtherUnityTwo {}

class SecondIdentityPureForm {
  constructor(private oneIdentity: ConstitutesOneIdentityAlone) {}
}

class ConstitutesOneIdentityAlone {}

class EachImmediatelyItsOppositeAbsoluteForm {
  constructor(private unity: RatherToTheirUnity) {}
}

class RatherToTheirUnity {}

class MediationMissesIdenticalSubstrate {
  constructor(private simplePoint: SimplePointEmptyContent) {}
}

class SimplePointEmptyContent {}

class TotalityConversionOneIdentityIntoOther {
  constructor(private conversion: TotalityConversionOneIntoOther) {}
}

class TotalityConversionOneIntoOther {}

class EachSidePositedAsTotalityWithin {
  constructor(private returnedGround: ReturnedIntoGroundThereforeInner) {}
}

class ReturnedIntoGroundThereforeInner {}

class TotalityMediatesItselfWithItself {
  constructor(private throughSimple: ThroughSimpleIdentity) {}
  throughFormDeterminateness(): ThroughFormDeterminateness { return new ThroughFormDeterminateness(); }
}

class ThroughFormDeterminateness {
  determinatenessMediatesItself(): DeterminatenessMediatesItself { return new DeterminatenessMediatesItself(); }
}

class DeterminatenessMediatesItself {
  throughSimpleIdentity(): ThroughSimpleIdentity { return new ThroughSimpleIdentity(); }
}

class ThroughSimpleIdentity {}

class AppearanceImmanentReflectionExpression {
  constructor(private expression: ExternalityExpressionInItself) {}
}

class ExternalityExpressionInItself {}

class ContentFormAbsolutelyIdentical {
  constructor(private express: ButExpressItself) {}
}

class ButExpressItself {}

// Additional core classes
class EmptyTransparentDifference {}
class ReflectiveShine {}
class IdentityOfContent {}
class IdentityOfForm {}
class TotalityMediatesItself {}
class SelfRevealing {}

export { OuterInner };
