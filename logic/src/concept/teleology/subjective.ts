/**
 * THE SUBJECTIVE PURPOSE - Rational Self-Determining Activity
 * =========================================================
 *
 * "Purpose, therefore, is the subjective concept as an essential striving and impulse
 * to posit itself externally. In this, it is exempt from transition."
 *
 * KEY STRUCTURE:
 * 1. Unity that repels itself from itself
 * 2. Essential striving to posit itself externally
 * 3. Exempt from transition (unlike force/cause)
 * 4. Rational in its concrete existence
 * 5. Syllogism structure: Universal-Particular-Singular
 * 6. Finite purpose with infinite form
 * 7. Impulse to realization through sublating presupposition
 */

interface RationalSelfDeterminingActivity {
  getUnityRepelsItself(): UnityRepelsItself;
  getEssentialStrivingExternal(): EssentialStrivingExternal;
  getExemptFromTransition(): ExemptFromTransition;
  getRationalConcreteExistence(): RationalConcreteExistence;
  getSyllogismStructure(): SyllogismStructure;
  getFinitePurposeInfiniteForm(): FinitePurposeInfiniteForm;
  getImpulseRealization(): ImpulseRealization;
  dialecticalMovement(): string;
}

export class SubjectivePurpose implements RationalSelfDeterminingActivity {
  private unityRepelsItself: UnityRepelsItself;
  private essentialStrivingExternal: EssentialStrivingExternal;
  private exemptFromTransition: ExemptFromTransition;
  private rationalConcreteExistence: RationalConcreteExistence;
  private syllogismStructure: SyllogismStructure;
  private finitePurposeInfiniteForm: FinitePurposeInfiniteForm;
  private impulseRealization: ImpulseRealization;

  constructor() {
    this.unityRepelsItself = new UnityRepelsItself();
    this.essentialStrivingExternal = new EssentialStrivingExternal();
    this.exemptFromTransition = new ExemptFromTransition();
    this.rationalConcreteExistence = new RationalConcreteExistence();
    this.syllogismStructure = new SyllogismStructure();
    this.finitePurposeInfiniteForm = new FinitePurposeInfiniteForm();
    this.impulseRealization = new ImpulseRealization();
  }

  /**
   * UNITY REPELS ITSELF FROM ITSELF
   *
   * "Its simple unity is therefore the unity that repels itself from itself
   * and in this repelling maintains itself."
   */
  getUnityRepelsItself(): UnityRepelsItself {
    // "unity that repels itself from itself and in this repelling maintains itself"
    const unityRepelsMaintains = new UnityRepelsMaintains();

    // "Its determinateness...has the determinateness of externality within it"
    const determinatenessExternalityWithin = unityRepelsMaintains.determinatenessExternalityWithin();

    return new UnityRepelsItself(determinatenessExternalityWithin);
  }

  /**
   * ESSENTIAL STRIVING EXTERNAL
   *
   * "Purpose, therefore, is the subjective concept as an essential striving and impulse
   * to posit itself externally."
   */
  getEssentialStrivingExternal(): EssentialStrivingExternal {
    // "essential striving and impulse to posit itself externally"
    const essentialStrivingImpulsePositExternal = new EssentialStrivingImpulsePositExternal();

    return new EssentialStrivingExternal(essentialStrivingImpulsePositExternal);
  }

  /**
   * EXEMPT FROM TRANSITION
   *
   * "In this, it is exempt from transition. It is neither a force expressing itself,
   * nor a substance or a cause manifesting itself in its accidents or effects."
   */
  getExemptFromTransition(): ExemptFromTransition {
    const notForceSubstanceCause = this.getNotForceSubstanceCause();
    const causeOfItself = this.getCauseOfItself();

    return new ExemptFromTransition(notForceSubstanceCause, causeOfItself);
  }

  /**
   * NOT FORCE SUBSTANCE CAUSE
   *
   * "It is neither a force expressing itself, nor a substance or a cause
   * manifesting itself in its accidents or effects."
   */
  getNotForceSubstanceCause(): NotForceSubstanceCause {
    // Force, substance, cause have actuality only in externalization
    const forceSubstanceCauseActualityExternalization = new ForceSubstanceCauseActualityExternalization();

    // "their activity is a transition against which they do not maintain themselves in freedom"
    const activityTransitionNotMaintainFreedom = forceSubstanceCauseActualityExternalization.activityTransitionNotMaintainFreedom();

    return new NotForceSubstanceCause(activityTransitionNotMaintainFreedom);
  }

  /**
   * CAUSE OF ITSELF
   *
   * "a cause that solicits itself to expression, or a cause that is a cause of itself
   * or whose effect is immediately the cause."
   */
  getCauseOfItself(): CauseOfItself {
    // "cause that solicits itself to expression"
    const causeSolicitsItselfExpression = new CauseSolicitsItselfExpression();

    // "cause that is a cause of itself"
    const causeCauseItself = causeSolicitsItselfExpression.causeCauseItself();

    // "whose effect is immediately the cause"
    const effectImmediatelyCause = causeCauseItself.effectImmediatelyCause();

    return new CauseOfItself(effectImmediatelyCause);
  }

  /**
   * RATIONAL CONCRETE EXISTENCE
   *
   * "purpose is to be taken as the rational in its concrete existence.
   * It manifests rationality by being the concrete concept that holds
   * the objective difference in its absolute unity."
   */
  getRationalConcreteExistence(): RationalConcreteExistence {
    // "rational in its concrete existence"
    const rationalConcreteExistence = new RationalConcreteExistenceStructure();

    // "concrete concept that holds the objective difference in its absolute unity"
    const concreteConceptObjectiveDifferenceAbsoluteUnity = rationalConcreteExistence.concreteConceptObjectiveDifferenceAbsoluteUnity();

    return new RationalConcreteExistence(concreteConceptObjectiveDifferenceAbsoluteUnity);
  }

  /**
   * SYLLOGISM STRUCTURE
   *
   * "Within, therefore, it is essentially syllogism. It is the self-equal universal...
   * it determines itself immediately and gives itself the moment of particularity...
   * is just as immediately...singularity."
   */
  getSyllogismStructure(): SyllogismStructure {
    const selfEqualUniversal = this.getSelfEqualUniversal();
    const momentParticularity = this.getMomentParticularity();
    const reflectionSingularity = this.getReflectionSingularity();

    return new SyllogismStructure(selfEqualUniversal, momentParticularity, reflectionSingularity);
  }

  /**
   * SELF-EQUAL UNIVERSAL
   *
   * "It is the self-equal universal; more precisely, inasmuch as it contains
   * self-repelling negativity, it is universal though at first still indeterminate activity."
   */
  getSelfEqualUniversal(): SelfEqualUniversal {
    // "self-equal universal"
    const selfEqualUniversal = new SelfEqualUniversalStructure();

    // "contains self-repelling negativity"
    const containsSelfRepellingNegativity = selfEqualUniversal.containsSelfRepellingNegativity();

    // "universal though at first still indeterminate activity"
    const universalIndeterminateActivity = containsSelfRepellingNegativity.universalIndeterminateActivity();

    return new SelfEqualUniversal(universalIndeterminateActivity);
  }

  /**
   * MOMENT PARTICULARITY
   *
   * "But since this activity is negative self-reference, it determines itself immediately
   * and gives itself the moment of particularity"
   */
  getMomentParticularity(): MomentParticularity {
    // "negative self-reference...determines itself immediately"
    const negativeSelfReferenceDeterminesImmediately = new NegativeSelfReferenceDeterminesImmediately();

    // "gives itself the moment of particularity"
    const givesMomentParticularity = negativeSelfReferenceDeterminesImmediately.givesMomentParticularity();

    // "totality of the form reflected into itself, is content"
    const totalityFormReflectedContent = givesMomentParticularity.totalityFormReflectedContent();

    return new MomentParticularity(totalityFormReflectedContent);
  }

  /**
   * REFLECTION SINGULARITY
   *
   * "The same negativity, through its self-reference, is just as immediately
   * the reflection of the form into itself and singularity."
   */
  getReflectionSingularity(): ReflectionSingularity {
    const reflectionIntoItself = this.getReflectionIntoItself();
    const outwardsReflection = this.getOutwardsReflection();

    return new ReflectionSingularity(reflectionIntoItself, outwardsReflection);
  }

  /**
   * REFLECTION INTO ITSELF
   *
   * "From the one side, this reflection is the inner universality of the subject"
   */
  getReflectionIntoItself(): ReflectionIntoItself {
    // "inner universality of the subject"
    const innerUniversalitySubject = new InnerUniversalitySubject();

    return new ReflectionIntoItself(innerUniversalitySubject);
  }

  /**
   * OUTWARDS REFLECTION
   *
   * "from the other side, however, it is outwards reflection;
   * and to this extent purpose is still something subjective"
   */
  getOutwardsReflection(): OutwardsReflection {
    // "outwards reflection"
    const outwardsReflection = new OutwardsReflectionStructure();

    // "purpose is still something subjective, its activity still directed to an external objectivity"
    const purposeSubjectiveActivityExternalObjectivity = outwardsReflection.purposeSubjectiveActivityExternalObjectivity();

    return new OutwardsReflection(purposeSubjectiveActivityExternalObjectivity);
  }

  /**
   * FINITE PURPOSE INFINITE FORM
   *
   * "Accordingly, purpose is finite, even though according to form
   * it is equally infinite subjectivity."
   */
  getFinitePurposeInfiniteForm(): FinitePurposeInfiniteForm {
    const determinateContent = this.getDeterminateContent();
    const objectiveIndifference = this.getObjectiveIndifference();
    const presupposition = this.getPresupposition();
    const extraMundaneExistence = this.getExtraMundaneExistence();

    return new FinitePurposeInfiniteForm(determinateContent, objectiveIndifference, presupposition, extraMundaneExistence);
  }

  /**
   * DETERMINATE CONTENT
   *
   * "the self-determination or the particularity as simple reflection into itself
   * is distinguished from the concrete form, and is a determinate content."
   */
  getDeterminateContent(): DeterminateContent {
    // "self-determination...distinguished from concrete form"
    const selfDeterminationDistinguishedConcreteForm = new SelfDeterminationDistinguishedConcreteForm();

    // "is a determinate content"
    const determinateContent = selfDeterminationDistinguishedConcreteForm.determinateContent();

    return new DeterminateContent(determinateContent);
  }

  /**
   * OBJECTIVE INDIFFERENCE
   *
   * "since its determinateness has the form of objective indifference,
   * it has the shape of a presupposition"
   */
  getObjectiveIndifference(): ObjectiveIndifference {
    // "determinateness has the form of objective indifference"
    const determinatenessFormObjectiveIndifference = new DeterminatenessFormObjectiveIndifference();

    return new ObjectiveIndifference(determinatenessFormObjectiveIndifference);
  }

  /**
   * PRESUPPOSITION
   *
   * "its finitude consists in its having before it an objective,
   * mechanical and chemical world to which its activity is directed"
   */
  getPresupposition(): Presupposition {
    // "having before it an objective, mechanical and chemical world"
    const objectiveMechanicalChemicalWorld = new ObjectiveMechanicalChemicalWorld();

    // "to which its activity is directed as to something already there"
    const activityDirectedSomethingAlreadyThere = objectiveMechanicalChemicalWorld.activityDirectedSomethingAlreadyThere();

    return new Presupposition(activityDirectedSomethingAlreadyThere);
  }

  /**
   * EXTRA-MUNDANE EXISTENCE
   *
   * "To this extent purpose still has a truly extra-mundane concrete existence"
   */
  getExtraMundaneExistence(): ExtraMundaneExistence {
    // "truly extra-mundane concrete existence"
    const extraMundaneConcreteExistence = new ExtraMundaneConcreteExistenceStructure();

    // "this objectivity stands opposed to it"
    const objectivityStandsOpposed = extraMundaneConcreteExistence.objectivityStandsOpposed();

    return new ExtraMundaneExistence(objectivityStandsOpposed);
  }

  /**
   * IMPULSE REALIZATION
   *
   * "the movement of purpose can now be expressed as being directed at sublating its presupposition,
   * that is, the immediacy of the object, and at positing it as determined by the concept."
   */
  getImpulseRealization(): ImpulseRealization {
    const sublatingPresupposition = this.getSublatingPresupposition();
    const positiveRealization = this.getPositiveRealization();
    const repulsionResolution = this.getRepulsionResolution();

    return new ImpulseRealization(sublatingPresupposition, positiveRealization, repulsionResolution);
  }

  /**
   * SUBLATING PRESUPPOSITION
   *
   * "directed at sublating its presupposition, that is, the immediacy of the object,
   * and at positing it as determined by the concept."
   */
  getSublatingPresupposition(): SublatingPresupposition {
    // "sublating its presupposition...the immediacy of the object"
    const sublatingPresuppositionImmediacyObject = new SublatingPresuppositionImmediacyObject();

    // "positing it as determined by the concept"
    const positingDeterminedConcept = sublatingPresuppositionImmediacyObject.positingDeterminedConcept();

    // "negative relating to the object is equally...sublating of the subjectivity of purpose"
    const negativeRelatingSublatingSub­jectivity = positingDeterminedConcept.negativeRelatingSublatingSub­jectivity();

    return new SublatingPresupposition(negativeRelatingSublatingSub­jectivity);
  }

  /**
   * POSITIVE REALIZATION
   *
   * "Positively, this is the realization of purpose, namely the unification
   * of the objective being with it"
   */
  getPositiveRealization(): PositiveRealization {
    // "realization of purpose...unification of the objective being with it"
    const realizationUnificationObjectiveBeing = new RealizationUnificationObjectiveBeing();

    // External determinateness becomes moment of purpose
    const externalDeterminatenessMomentPurpose = realizationUnificationObjectiveBeing.externalDeterminatenessMomentPurpose();

    return new PositiveRealization(externalDeterminatenessMomentPurpose);
  }

  /**
   * REPULSION RESOLUTION
   *
   * "Purpose is in it the impulse to its realization...the concept therefore repels itself from itself.
   * This repulsion is in general the resolution of the self-reference of the negative unity"
   */
  getRepulsionResolution(): RepulsionResolution {
    const impulseRealization = this.getImpulseRealizationStructure();
    const repulsionGeneral = this.getRepulsionGeneral();
    const excludingSingularity = this.getExcludingSingularity();
    const subjectivityParticularity = this.getSubjectivityParticularity();

    return new RepulsionResolution(impulseRealization, repulsionGeneral, excludingSingularity, subjectivityParticularity);
  }

  /**
   * IMPULSE REALIZATION STRUCTURE
   *
   * "Purpose is in it the impulse to its realization"
   */
  getImpulseRealizationStructure(): ImpulseRealizationStructure {
    // "impulse to its realization"
    const impulseRealization = new ImpulseRealizationStructureCore();

    // "the simplicity of these moments within the unity of the concept is...incommensurable"
    const simplicityMomentsIncommensurable = impulseRealization.simplicityMomentsIncommensurable();

    return new ImpulseRealizationStructure(simplicityMomentsIncommensurable);
  }

  /**
   * REPULSION GENERAL
   *
   * "This repulsion is in general the resolution of the self-reference of the negative unity"
   */
  getRepulsionGeneral(): RepulsionGeneral {
    // "resolution of the self-reference of the negative unity"
    const resolutionSelfReferenceNegativeUnity = new ResolutionSelfReferenceNegativeUnity();

    // "by virtue of which the latter is exclusive singularity"
    const exclusiveSingularity = resolutionSelfReferenceNegativeUnity.exclusiveSingularity();

    return new RepulsionGeneral(exclusiveSingularity);
  }

  /**
   * EXCLUDING SINGULARITY
   *
   * "but by this excluding the unity resolves itself, that is to say, it discloses itself,
   * for it is self-determination, the positing of itself."
   */
  getExcludingSingularity(): ExcludingSingularity {
    // "by this excluding the unity resolves itself...discloses itself"
    const excludingUnityResolvesDiscloses = new ExcludingUnityResolvesDiscloses();

    // "for it is self-determination, the positing of itself"
    const selfDeterminationPositingItself = excludingUnityResolvesDiscloses.selfDeterminationPositingItself();

    return new ExcludingSingularity(selfDeterminationPositingItself);
  }

  /**
   * SUBJECTIVITY PARTICULARITY
   *
   * "subjectivity makes itself into particularity, gives itself a content...
   * but this positing...is...immediately a presupposing"
   */
  getSubjectivityParticularity(): SubjectivityParticularity {
    // "subjectivity makes itself into particularity"
    const subjectivityMakesParticularity = new SubjectivityMakesParticularity();

    // "gives itself a content...enclosed within the unity of the concept"
    const givesContentUnityConception = subjectivityMakesParticularity.givesContentUnityConcept();

    // "this positing...is...immediately a presupposing"
    const positingImmediatelyPresupposing = givesContentUnityConception.positingImmediatelyPresupposing();

    // "referred to an indifferent, external objectivity"
    const referredIndifferentExternalObjectivity = positingImmediatelyPresupposing.referredIndifferentExternalObjectivity();

    return new SubjectivityParticularity(referredIndifferentExternalObjectivity);
  }

  /**
   * COMPLETE DIALECTICAL MOVEMENT
   */
  dialecticalMovement(): string {
    return `
    DIALECTICAL MOVEMENT - SUBJECTIVE PURPOSE:

    1. Unity repels itself from itself → Essential striving to posit externally
    2. Exempt from transition (unlike force/cause) → Cause of itself
    3. Rational concrete existence → Syllogism: Universal-Particular-Singular
    4. Finite purpose with infinite form → Extra-mundane existence
    5. Impulse to realization → Sublating presupposition → Repulsion-resolution

    RESULT: Purpose as self-determining rational activity
    TRANSITION: Subjective Purpose → Means and Realization

    Purpose is HETU - both Ground and Purpose in Sanskrit!
    The noumenal CPU-GPU architecture of rational self-determination!
    `;
  }
}

// Core supporting classes
class UnityRepelsItself {
  constructor(private determinatenessExternality: DeterminatenessExternalityWithin) {}
}

class EssentialStrivingExternal {
  constructor(private essentialStriving: EssentialStrivingImpulsePositExternal) {}
}

class ExemptFromTransition {
  constructor(
    private notForce: NotForceSubstanceCause,
    private causeItself: CauseOfItself
  ) {}
}

class RationalConcreteExistence {
  constructor(private concreteConceptObjective: ConcreteConceptObjectiveDifferenceAbsoluteUnity) {}
}

class SyllogismStructure {
  constructor(
    private universal: SelfEqualUniversal,
    private particular: MomentParticularity,
    private singular: ReflectionSingularity
  ) {}
}

class FinitePurposeInfiniteForm {
  constructor(
    private content: DeterminateContent,
    private indifference: ObjectiveIndifference,
    private presupposition: Presupposition,
    private extraMundane: ExtraMundaneExistence
  ) {}
}

class ImpulseRealization {
  constructor(
    private sublating: SublatingPresupposition,
    private positive: PositiveRealization,
    private repulsion: RepulsionResolution
  ) {}
}

export { SubjectivePurpose };
