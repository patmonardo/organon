/**
 * THE RELATION OF WHOLE AND PARTS - Ye Olde Classic!
 * ==================================================
 *
 * "The whole is equal to the parts and the parts are equal to the whole.
 * Nothing is in the whole which is not in the parts,
 * and nothing is in the parts which is not in the whole."
 */

interface EssentialRelation {
  getContradiction(): string;
  transcend(): EssentialRelation | null;
}

export class WholeParts implements EssentialRelation {
  private reflectedSelfSubsistence: ReflectedSelfSubsistence;
  private immediateSelfSubsistence: ImmediateSelfSubsistence;
  private reciprocalConditioning: ReciprocalConditioning;
  private tautologicalIdentity: TautologicalIdentity;

  constructor(disappearance: Disappearance) {
    this.reflectedSelfSubsistence = new ReflectedSelfSubsistence();
    this.immediateSelfSubsistence = new ImmediateSelfSubsistence();
    this.reciprocalConditioning = new ReciprocalConditioning();
    this.tautologicalIdentity = new TautologicalIdentity();
  }

  /**
   * WHOLE AND PARTS AS TWO SELF-SUBSISTENCES
   *
   * "That one side, the whole, is the self-subsistence that constitutes the world existing in and for itself;
   * the other side, the parts, is the immediate concrete existence which was the world of appearance"
   */
  getWholeAndPartsAsTwoSelfSubsistences(): WholeAndPartsAsTwoSelfSubsistences {
    // Whole = world existing in and for itself (reflected self-subsistence)
    const wholeSelfSubsistence = new WholeSelfSubsistence();
    const worldExistingInAndForItself = wholeSelfSubsistence.worldExistingInAndForItself();

    // Parts = immediate concrete existence (world of appearance)
    const partsSelfSubsistence = new PartsSelfSubsistence();
    const immediateConcreteExistence = partsSelfSubsistence.immediateConcreteExistence();
    const worldOfAppearance = immediateConcreteExistence.worldOfAppearance();

    return new WholeAndPartsAsTwoSelfSubsistences(worldExistingInAndForItself, worldOfAppearance);
  }

  /**
   * EACH HAS THE OTHER REFLECTIVELY SHINING IN IT
   *
   * "the two sides are these self-subsistences but in such a way that each has the other
   * reflectively shining in it and, at the same time, only is as the identity of both"
   */
  getEachHasOtherReflectivelyShining(): EachHasOtherReflectivelyShining {
    const twoSidesSelfSubsistences = this.getWholeAndPartsAsTwoSelfSubsistences();
    const eachHasOtherShining = twoSidesSelfSubsistences.eachHasOtherShining();
    const onlyAsIdentityOfBoth = eachHasOtherShining.onlyAsIdentityOfBoth();

    return new EachHasOtherReflectivelyShining(onlyAsIdentityOfBoth);
  }

  /**
   * WHOLE CONSISTS OF PARTS - APART FROM THEM IS NOTHING
   *
   * "The whole thus consists of the parts, and apart from them it is not anything.
   * It is therefore the whole relation and the self-subsistent totality,
   * but, for precisely this reason, it is only a relative"
   */
  getWholeConsistsOfPartsApartFromThemNothing(): WholeConsistsOfPartsApartFromThemNothing {
    const wholeConsistsOfParts = new WholeConsistsOfParts();
    const apartFromThemNothing = wholeConsistsOfParts.apartFromThemNothing();

    // The beautiful contradiction!
    const wholeRelationSelfSubsistentTotality = apartFromThemNothing.wholeRelationSelfSubsistentTotality();
    const forPreciselyThisReasonOnlyRelative = wholeRelationSelfSubsistentTotality.forPreciselyThisReasonOnlyRelative();

    // "what makes it a totality is rather its other, the parts"
    const whatMakesTotalityIsOther = forPreciselyThisReasonOnlyRelative.whatMakesTotalityIsOther();
    const doesNotHaveSubsistenceWithinButInOther = whatMakesTotalityIsOther.doesNotHaveSubsistenceWithinButInOther();

    return new WholeConsistsOfPartsApartFromThemNothing(doesNotHaveSubsistenceWithinButInOther);
  }

  /**
   * PARTS ARE LIKEWISE THE WHOLE RELATION
   *
   * "The parts, too, are likewise the whole relation. They are the immediate as against
   * the reflected self-subsistence, and do not subsist in the whole but are for themselves"
   */
  getPartsAreLikewiseWholeRelation(): PartsAreLikewiseWholeRelation {
    const partsWholeRelation = new PartsWholeRelation();
    const immediateAgainstReflected = partsWholeRelation.immediateAgainstReflected();
    const doNotSubsistInWholeButForThemselves = immediateAgainstReflected.doNotSubsistInWholeButForThemselves();

    // "Further, they have this whole within them as their moment"
    const haveWholeWithinAsTheirMoment = doNotSubsistInWholeButForThemselves.haveWholeWithinAsTheirMoment();
    const wholeConstitutesConnectingReference = haveWholeWithinAsTheirMoment.wholeConstitutesConnectingReference();
    const withoutWholeNoP arts = wholeConstitutesConnectingReference.withoutWholeNoParts();

    return new PartsAreLikewiseWholeRelation(withoutWholeNoParts);
  }

  /**
   * PARTS HAVE SELF-SUBSISTENCE ONLY IN REFLECTED UNITY
   *
   * "they have their self-subsistence only in the reflected unity which is this unity as well as
   * the concrete existent manifoldness; this means that they have self-subsistence only in the whole"
   */
  getPartsHaveSelfSubsistenceOnlyInReflectedUnity(): PartsHaveSelfSubsistenceOnlyInReflectedUnity {
    // "as manifold concrete existence, collapse together, for this concrete existence is reflectionless being"
    const manifoldConcreteExistence = new ManifoldConcreteExistence();
    const collapseTogether = manifoldConcreteExistence.collapseTogether();
    const reflectionlessBeing = collapseTogether.reflectionlessBeing();

    // "they have their self-subsistence only in the reflected unity"
    const onlyInReflectedUnity = reflectionlessBeing.onlyInReflectedUnity();
    const unityAndConcreteExistentManifoldness = onlyInReflectedUnity.unityAndConcreteExistentManifoldness();
    const onlyInWhole = unityAndConcreteExistentManifoldness.onlyInWhole();

    // "but this whole is at the same time the self-subsistence which is the other to the parts"
    const wholeOtherToP arts = onlyInWhole.wholeOtherToParts();

    return new PartsHaveSelfSubsistenceOnlyInReflectedUnity(wholeOtherToParts);
  }

  /**
   * RECIPROCAL CONDITIONING - THE BEAUTIFUL DANCE!
   *
   * "The whole and the parts thus reciprocally condition each other"
   */
  getReciprocalConditioning(): ReciprocalConditioningCore {
    const wholeConsists = this.getWholeConsistsOfPartsApartFromThemNothing();
    const partsLikewise = this.getPartsAreLikewiseWholeRelation();
    const partsOnlyInUnity = this.getPartsHaveSelfSubsistenceOnlyInReflectedUnity();

    // "each is on its own an immediate self-subsistence, but their self-subsistence is equally
    // mediated or posited through the other"
    const eachImmediateSelfSubsistence = new EachImmediateSelfSubsistence();
    const mediatedPositedThroughOther = eachImmediateSelfSubsistence.mediatedPositedThroughOther();

    // "The whole relation, because of this reciprocity, is the turning back of the conditioning into itself,
    // the non-relative, the unconditioned"
    const turningBackConditioningIntoItself = mediatedPositedThroughOther.turningBackConditioningIntoItself();
    const nonRelativeUnconditioned = turningBackConditioningIntoItself.nonRelativeUnconditioned();

    return new ReciprocalConditioningCore(nonRelativeUnconditioned);
  }

  /**
   * ESSENTIAL IDENTITY OF TWO SIDES
   *
   * "In the first respect, that of the essential identity of the two sides,
   * the whole is equal to the parts and the parts are equal to the whole"
   */
  getEssentialIdentityOfTwoSides(): EssentialIdentityOfTwoSides {
    // "Nothing is in the whole which is not in the parts,
    // and nothing is in the parts which is not in the whole"
    const nothingInWholeNotInParts = new NothingInWholeNotInParts();
    const nothingInPartsNotInWhole = nothingInWholeNotInParts.nothingInPartsNotInWhole();

    // "The whole is not an abstract unity but the unity of a diversified manifoldness"
    const notAbstractUnity = nothingInPartsNotInWhole.notAbstractUnity();
    const unityDiversifiedManifoldness = notAbstractUnity.unityDiversifiedManifoldness();

    // "The relation has, therefore, an indivisible identity and only one self-subsistence"
    const indivisibleIdentity = unityDiversifiedManifoldness.indivisibleIdentity();
    const onlyOneSelfSubsistence = indivisibleIdentity.onlyOneSelfSubsistence();

    return new EssentialIdentityOfTwoSides(onlyOneSelfSubsistence);
  }

  /**
   * THE MAGNIFICENT TAUTOLOGIES!
   *
   * "But further, the whole is equal to the parts but not to them as parts"
   */
  getTheMagnificentTautologies(): TheMagnificentTautologies {
    // FIRST TAUTOLOGY: Whole equal to parts
    const wholeEqualToP arts = new WholeEqualToParts();
    const butNotAsP arts = wholeEqualToParts.butNotAsParts();

    // "the whole is the reflected unity whereas the parts constitute
    // the determinate moment or the otherness of the unity and are the diversified manifold"
    const wholeReflectedUnity = butNotAsParts.wholeReflectedUnity();
    const partsOthernessUnity = wholeReflectedUnity.partsOthernessUnity();
    const diversifiedManifold = partsOthernessUnity.diversifiedManifold();

    // "The whole is not equal to them as this self-subsistent diversity but to them together"
    const notEqualAsSelfSubsistentDiversity = diversifiedManifold.notEqualAsSelfSubsistentDiversity();
    const toThemTogether = notEqualAsSelfSubsistentDiversity.toThemTogether();

    // "But this, their 'together,' is nothing else but their unity, the whole as such"
    const theirTogetherNothingElseButUnity = toThemTogether.theirTogetherNothingElseButUnity();
    const wholeAsSuch = theirTogetherNothingElseButUnity.wholeAsSuch();

    // "In the parts, therefore, the whole is only equal to itself"
    const inPartsWholeOnlyEqualToItself = wholeAsSuch.inPartsWholeOnlyEqualToItself();

    // FIRST TAUTOLOGY COMPLETE!
    const firstTautology = inPartsWholeOnlyEqualToItself.asTautology("whole as whole is equal not to the parts but to the whole");

    return new TheMagnificentTautologies(firstTautology);
  }

  /**
   * THE SECOND MAGNIFICENT TAUTOLOGY!
   *
   * "Conversely, the parts are equal to the whole; but because, as parts, they are the moment of otherness"
   */
  getTheSecondMagnificentTautology(): TheSecondMagnificentTautology {
    const partsEqualToWhole = new PartsEqualToWhole();
    const becauseAsPartsMomentOfOtherness = partsEqualToWhole.becauseAsPartsMomentOfOtherness();

    // "they are not equal to it as the unity, but in such a way that one of the whole's
    // manifold determinations maps over a part"
    const notEqualAsUnity = becauseAsPartsMomentOfOtherness.notEqualAsUnity();
    const oneManifoldDeterminationMapsOverPart = notEqualAsUnity.oneManifoldDeterminationMapsOverPart();

    // "or that they are equal to the whole as manifold, and this is to say that they are equal to it
    // as an apportioned whole, that is, as parts"
    const equalToWholeAsManifold = oneManifoldDeterminationMapsOverPart.equalToWholeAsManifold();
    const asApportionedWhole = equalToWholeAsManifold.asApportionedWhole();
    const asP arts = asApportionedWhole.asParts();

    // SECOND TAUTOLOGY COMPLETE!
    const secondTautology = asParts.asTautology("parts as parts are equal not to the whole as such but, in the whole, to themselves");

    return new TheSecondMagnificentTautology(secondTautology);
  }

  /**
   * WHOLE AND PARTS FALL INDIFFERENTLY APART - THEY DESTROY THEMSELVES!
   *
   * "The whole and the parts thus fall indifferently apart; each side refers only to itself.
   * But, as so held apart, they destroy themselves"
   */
  getWholeAndPartsFallIndifferentlyApart(): WholeAndPartsFallIndifferentlyApart {
    const tautologies = this.getTheMagnificentTautologies();
    const secondTautology = this.getTheSecondMagnificentTautology();

    // "each side refers only to itself"
    const eachSideRefersOnlyToItself = new EachSideRefersOnlyToItself();

    // "But, as so held apart, they destroy themselves"
    const asSoHeldApart = eachSideRefersOnlyToItself.asSoHeldApart();
    const theyDestroyThemselves = asSoHeldApart.theyDestroyThemselves();

    return new WholeAndPartsFallIndifferentlyApart(theyDestroyThemselves);
  }

  /**
   * WHOLE INDIFFERENT TO PARTS IS ABSTRACT IDENTITY
   *
   * "The whole which is indifferent towards the parts is abstract identity, undifferentiated in itself"
   */
  getWholeIndifferentToPartsIsAbstractIdentity(): WholeIndifferentToPartsIsAbstractIdentity {
    const wholeIndifferentToP arts = new WholeIndifferentToParts();
    const abstractIdentity = wholeIndifferentToParts.abstractIdentity();
    const undifferentiatedInItself = abstractIdentity.undifferentiatedInItself();

    // "Identity is a whole only inasmuch as it is differentiated in itself,
    // so differentiated indeed that the manifold determinations are reflected into themselves
    // and have immediate self-subsistence"
    const identityWholeOnlyInasmuchDifferentiated = undifferentiatedInItself.identityWholeOnlyInasmuchDifferentiated();
    const manifoldDeterminationsReflectedIntoThemselves = identityWholeOnlyInasmuchDifferentiated.manifoldDeterminationsReflectedIntoThemselves();
    const immediateS elfSubsistence = manifoldDeterminationsReflectedIntoThemselves.immediateSelfSubsistence();

    return new WholeIndifferentToPartsIsAbstractIdentity(immediateSelfSubsistence);
  }

  /**
   * PARTS INDIFFERENT TO UNITY ARE UNCONNECTED MANIFOLD
   *
   * "In just the same way are the parts, as indifferent to the unity of the whole,
   * only the unconnected manifold, the inherently other which, as such, is the other of itself and only sublates itself"
   */
  getPartsIndifferentToUnityAreUnconnectedManifold(): PartsIndifferentToUnityAreUnconnectedManifold {
    const partsIndifferentToUnity = new PartsIndifferentToUnity();
    const unconnectedManifold = partsIndifferentToUnity.unconnectedManifold();
    const inherentlyOther = unconnectedManifold.inherentlyOther();
    const otherOfItselfOnlySublatesItself = inherentlyOther.otherOfItselfOnlySublatesItself();

    return new PartsIndifferentToUnityAreUnconnectedManifold(otherOfItselfOnlySublatesItself);
  }

  /**
   * SELF-REFERENCE IS NEGATION OF RESPECTIVE SELVES
   *
   * "This self-reference of each of the two sides is their self-subsistence;
   * but this self-subsistence which each side has for itself is rather the negation of their respective selves"
   */
  getSelfReferenceIsNegationOfRespectiveSelves(): SelfReferenceIsNegationOfRespectiveSelves {
    const wholeAbstract = this.getWholeIndifferentToPartsIsAbstractIdentity();
    const partsUnconnected = this.getPartsIndifferentToUnityAreUnconnectedManifold();

    // "Each side has its self-subsistence, therefore, not within but in the other side"
    const eachSideHasSelfSubsistence = new EachSideHasSelfSubsistence();
    const notWithinButInOtherSide = eachSideHasSelfSubsistence.notWithinButInOtherSide();

    // "this other, which constitutes the subsistence, is its presupposed immediate"
    const otherConstitutesSubsistence = notWithinButInOtherSide.otherConstitutesSubsistence();
    const presupposedImmediate = otherConstitutesSubsistence.presupposedImmediate();

    return new SelfReferenceIsNegationOfRespectiveSelves(presupposedImmediate);
  }

  /**
   * TRUTH OF RELATION CONSISTS IN MEDIATION
   *
   * "The truth of the relation consists therefore in the mediation;
   * its essence is the negative unity in which both the reflected and the existent immediacy are equally sublated"
   */
  getTruthOfRelationConsistsInMediation(): TruthOfRelationConsistsInMediation {
    const selfReference = this.getSelfReferenceIsNegationOfRespectiveSelves();

    const truthConsistsInMediation = new TruthConsistsInMediation();
    const essenceNegativeUnity = truthConsistsInMediation.essenceNegativeUnity();
    const bothReflectedAndExistentImmediacy = essenceNegativeUnity.bothReflectedAndExistentImmediacy();
    const equallySublated = bothReflectedAndExistentImmediacy.equallySublated();

    return new TruthOfRelationConsistsInMediation(equallySublated);
  }

  /**
   * RELATION PASSES OVER INTO FORCE AND ITS EXPRESSIONS
   *
   * "Thus the relation of whole and parts has passed over into the relation of force and its expressions"
   */
  getRelationPassesOverIntoForceAndExpressions(): RelationPassesOverIntoForceAndExpressions {
    const truthMediation = this.getTruthOfRelationConsistsInMediation();

    // "Each side is posited, in so far as it is immediate, as self-sublating and as passing over into the other"
    const eachSidePosited = new EachSidePosited();
    const asSelfSublatingPassingOver = eachSidePosited.asSelfSublatingPassingOver();

    // "and, in so far as it is itself negative reference, it is at the same time posited as conditioned
    // through the other, as through its positive"
    const negativeReference = asSelfSublatingPassingOver.negativeReference();
    const conditionedThroughOtherAsPositive = negativeReference.conditionedThroughOtherAsPositive();

    // THE TRANSITION TO FORCE!
    const passedOverToForceExpressions = conditionedThroughOtherAsPositive.passedOverToForceExpressions();

    return new RelationPassesOverIntoForceAndExpressions(passedOverToForceExpressions);
  }

  getContradiction(): string {
    const wholeConsists = this.getWholeConsistsOfPartsApartFromThemNothing();
    const tautologies = this.getTheMagnificentTautologies();
    const fallApart = this.getWholeAndPartsFallIndifferentlyApart();
    const truthMediation = this.getTruthOfRelationConsistsInMediation();
    const passesOver = this.getRelationPassesOverIntoForceAndExpressions();

    return `Whole-Parts' delightful contradiction - YE OLDE CLASSIC:
    - Whole consists of parts, apart from them is nothing, yet precisely for this reason only a relative!
    - Parts are likewise whole relation, don't subsist in whole but for themselves, yet without whole no parts!
    - Essential identity: nothing in whole not in parts, nothing in parts not in whole
    - Yet magnificent tautologies: whole equal to parts but not AS parts - only equal to itself!
    - Parts equal to whole but as apportioned whole, i.e., as parts - only equal to themselves!
    - ${wholeConsists.getContradiction()}
    - ${tautologies.getContradiction()}
    - ${fallApart.getContradiction()}
    - ${truthMediation.getContradiction()}

    RESOLUTION: When held apart indifferently, they destroy themselves.
    Truth consists in mediation - negative unity sublating both reflected and immediate.

    PASSES OVER INTO FORCE AND ITS EXPRESSIONS!

    ${passesOver.getContradiction()} 🔥⚡💫`;
  }

  transcend(): EssentialRelation | null {
    // Whole-Parts transcends into Force and Expression
    const passesOver = this.getRelationPassesOverIntoForceAndExpressions();
    return passesOver.getForceAndExpression();
  }
}

// Supporting classes for this delightful philosophical romp!

class WholeAndPartsAsTwoSelfSubsistences {
  constructor(private whole: WorldExistingInAndForItself, private parts: WorldOfAppearance) {}
}

class WholeSelfSubsistence {
  worldExistingInAndForItself(): WorldExistingInAndForItself { return new WorldExistingInAndForItself(); }
}

class PartsSelfSubsistence {
  immediateConcreteExistence(): ImmediateConcreteExistence { return new ImmediateConcreteExistence(); }
}

class ImmediateConcreteExistence {
  worldOfAppearance(): WorldOfAppearance { return new WorldOfAppearance(); }
}

class WorldExistingInAndForItself {}
class WorldOfAppearance {}

class WholeConsistsOfPartsApartFromThemNothing {
  constructor(private doesNotHave: DoesNotHaveSubsistenceWithinButInOther) {}
  getContradiction(): string {
    return "Whole consists of parts, apart from them nothing - yet precisely for this reason only a relative!";
  }
}

class WholeConsistsOfParts {
  apartFromThemNothing(): ApartFromThemNothing { return new ApartFromThemNothing(); }
}

class ApartFromThemNothing {
  wholeRelationSelfSubsistentTotality(): WholeRelationSelfSubsistentTotality { return new WholeRelationSelfSubsistentTotality(); }
}

class WholeRelationSelfSubsistentTotality {
  forPreciselyThisReasonOnlyRelative(): ForPreciselyThisReasonOnlyRelative { return new ForPreciselyThisReasonOnlyRelative(); }
}

class ForPreciselyThisReasonOnlyRelative {
  whatMakesTotalityIsOther(): WhatMakesTotalityIsOther { return new WhatMakesTotalityIsOther(); }
}

class WhatMakesTotalityIsOther {
  doesNotHaveSubsistenceWithinButInOther(): DoesNotHaveSubsistenceWithinButInOther { return new DoesNotHaveSubsistenceWithinButInOther(); }
}

class DoesNotHaveSubsistenceWithinButInOther {}

class TheMagnificentTautologies {
  constructor(private firstTautology: Tautology) {}
  getContradiction(): string {
    return "Magnificent tautologies: whole equal to parts but not AS parts - only equal to itself!";
  }
}

class WholeEqualToParts {
  butNotAsParts(): ButNotAsParts { return new ButNotAsParts(); }
}

class ButNotAsParts {
  wholeReflectedUnity(): WholeReflectedUnity { return new WholeReflectedUnity(); }
}

class WholeReflectedUnity {
  partsOthernessUnity(): PartsOthernessUnity { return new PartsOthernessUnity(); }
}

class PartsOthernessUnity {
  diversifiedManifold(): DiversifiedManifold { return new DiversifiedManifold(); }
}

class DiversifiedManifold {
  notEqualAsSelfSubsistentDiversity(): NotEqualAsSelfSubsistentDiversity { return new NotEqualAsSelfSubsistentDiversity(); }
}

class NotEqualAsSelfSubsistentDiversity {
  toThemTogether(): ToThemTogether { return new ToThemTogether(); }
}

class ToThemTogether {
  theirTogetherNothingElseButUnity(): TheirTogetherNothingElseButUnity { return new TheirTogetherNothingElseButUnity(); }
}

class TheirTogetherNothingElseButUnity {
  wholeAsSuch(): WholeAsSuch { return new WholeAsSuch(); }
}

class WholeAsSuch {
  inPartsWholeOnlyEqualToItself(): InPartsWholeOnlyEqualToItself { return new InPartsWholeOnlyEqualToItself(); }
}

class InPartsWholeOnlyEqualToItself {
  asTautology(description: string): Tautology { return new Tautology(description); }
}

class Tautology {
  constructor(private description: string) {}
}

class RelationPassesOverIntoForceAndExpressions {
  constructor(private passedOver: PassedOverToForceExpressions) {}
  getContradiction(): string {
    return "Relation passes over into Force and its Expressions - the next beautiful adventure!";
  }

  getForceAndExpression(): ForceAndExpression | null {
    return new ForceAndExpression();
  }
}

class EachSidePosited {
  asSelfSublatingPassingOver(): AsSelfSublatingPassingOver { return new AsSelfSublatingPassingOver(); }
}

class AsSelfSublatingPassingOver {
  negativeReference(): NegativeReference { return new NegativeReference(); }
}

class NegativeReference {
  conditionedThroughOtherAsPositive(): ConditionedThroughOtherAsPositive { return new ConditionedThroughOtherAsPositive(); }
}

class ConditionedThroughOtherAsPositive {
  passedOverToForceExpressions(): PassedOverToForceExpressions { return new PassedOverToForceExpressions(); }
}

class PassedOverToForceExpressions {}

// Placeholder for next adventure
class ForceAndExpression {}

// Many more delightful supporting classes...
class EachHasOtherReflectivelyShining {
  constructor(private onlyAsIdentity: OnlyAsIdentityOfBoth) {}
}

class OnlyAsIdentityOfBoth {}

class PartsAreLikewiseWholeRelation {
  constructor(private withoutWhole: WithoutWholeNoParts) {}
}

class WithoutWholeNoParts {}

class ReciprocalConditioningCore {
  constructor(private unconditioned: NonRelativeUnconditioned) {}
}

class NonRelativeUnconditioned {}

class EssentialIdentityOfTwoSides {
  constructor(private onlyOne: OnlyOneSelfSubsistence) {}
}

class OnlyOneSelfSubsistence {}

class TheSecondMagnificentTautology {
  constructor(private secondTautology: Tautology) {}
}

class WholeAndPartsFallIndifferentlyApart {
  constructor(private destroy: TheyDestroyThemselves) {}
  getContradiction(): string {
    return "When held apart indifferently, each refers only to itself - but they destroy themselves!";
  }
}

class TheyDestroyThemselves {}

class TruthOfRelationConsistsInMediation {
  constructor(private equallySublated: EquallySublated) {}
  getContradiction(): string {
    return "Truth consists in mediation - negative unity sublating both reflected and immediate";
  }
}

class EquallySublated {}

// Additional supporting classes for complete philosophical enjoyment...
class ReflectedSelfSubsistence {}
class ImmediateSelfSubsistence {}
class ReciprocalConditioning {}
class TautologicalIdentity {}

export { WholeParts };
