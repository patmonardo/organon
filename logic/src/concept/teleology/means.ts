/**
 * THE MEANS - Mediating Middle Term of Purposive Syllogism
 * =======================================================
 *
 * "Through a means the purpose unites with objectivity and in objectivity unites with itself.
 * This means is the middle term of the syllogism."
 *
 * KEY STRUCTURE:
 * 1. Sublation of presupposition → Beginning of realization
 * 2. Formal middle term of formal syllogism
 * 3. Mechanical object with external purposive determination
 * 4. Totality of syllogism: Abstract activity - Means - External object
 * 5. Object penetrable by concept but still has self-subsistence
 */

interface MediatingMiddleTermPurposiveSyllogism {
  getSublationPresupposition(): SublationPresupposition;
  getFormalMiddleTerm(): FormalMiddleTerm;
  getMechanicalObjectPurpose(): MechanicalObjectPurpose;
  getTotalitySyllogism(): TotalitySyllogism;
  getPenetrableByConceptSelfSubsistence(): PenetrableByConceptSelfSubsistence;
  dialecticalMovement(): string;
}

export class Means implements MediatingMiddleTermPurposiveSyllogism {
  private sublationPresupposition: SublationPresupposition;
  private formalMiddleTerm: FormalMiddleTerm;
  private mechanicalObjectPurpose: MechanicalObjectPurpose;
  private totalitySyllogism: TotalitySyllogism;
  private penetrableByConceptSelfSubsistence: PenetrableByConceptSelfSubsistence;

  constructor() {
    this.sublationPresupposition = new SublationPresupposition();
    this.formalMiddleTerm = new FormalMiddleTerm();
    this.mechanicalObjectPurpose = new MechanicalObjectPurpose();
    this.totalitySyllogism = new TotalitySyllogism();
    this.penetrableByConceptSelfSubsistence = new PenetrableByConceptSelfSubsistence();
  }

  /**
   * SUBLATION PRESUPPOSITION - Beginning of Realization
   *
   * "its second determining is, therefore, the sublation of this presupposition as such;
   * this sublation is an immanent turning back"
   */
  getSublationPresupposition(): SublationPresupposition {
    const secondDetermining = this.getSecondDetermining();
    const immanentTurningBack = this.getImmanentTurningBack();
    const objectSoDetermined = this.getObjectSoDetermined();

    return new SublationPresupposition(secondDetermining, immanentTurningBack, objectSoDetermined);
  }

  /**
   * SECOND DETERMINING
   *
   * "the subjectivity of purpose is the absolutely negative unity;
   * its second determining is, therefore, the sublation of this presupposition as such"
   */
  getSecondDetermining(): SecondDetermining {
    // "absolutely negative unity"
    const absolutelyNegativeUnity = new AbsolutelyNegativeUnity();

    // "sublation of this presupposition as such"
    const sublationPresuppositionAsSuch = absolutelyNegativeUnity.sublationPresuppositionAsSuch();

    return new SecondDetermining(sublationPresuppositionAsSuch);
  }

  /**
   * IMMANENT TURNING BACK
   *
   * "this sublation is an immanent turning back inasmuch as that moment of the first negation
   * which is the positing of the negative over against the subject, the external object, is sublated by it."
   */
  getImmanentTurningBack(): ImmanentTurningBack {
    // "moment of the first negation...positing of the negative over against the subject"
    const firstNegationPositingNegativeAgainstSubject = new FirstNegationPositingNegativeAgainstSubject();

    // "the external object, is sublated by it"
    const externalObjectSublatedByIt = firstNegationPositingNegativeAgainstSubject.externalObjectSublatedByIt();

    return new ImmanentTurningBack(externalObjectSublatedByIt);
  }

  /**
   * OBJECT SO DETERMINED
   *
   * "This positing is therefore not yet the realized purpose itself
   * but only the beginning of this realization. The object so determined is now the means."
   */
  getObjectSoDetermined(): ObjectSoDetermined {
    // "not yet the realized purpose itself but only the beginning of this realization"
    const beginningRealization = new BeginningRealization();

    // "The object so determined is now the means"
    const objectDeterminedMeans = beginningRealization.objectDeterminedMeans();

    return new ObjectSoDetermined(objectDeterminedMeans);
  }

  /**
   * FORMAL MIDDLE TERM - External Mediation
   *
   * "The means is therefore the formal middle term of a formal syllogism;
   * it is something external to the extreme of the subjective purpose"
   */
  getFormalMiddleTerm(): FormalMiddleTerm {
    const purposeUnitesThroughMeans = this.getPurposeUnitesThroughMeans();
    const needOfMeans = this.getNeedOfMeans();
    const formalSyllogism = this.getFormalSyllogism();

    return new FormalMiddleTerm(purposeUnitesThroughMeans, needOfMeans, formalSyllogism);
  }

  /**
   * PURPOSE UNITES THROUGH MEANS
   *
   * "Through a means the purpose unites with objectivity and in objectivity unites with itself.
   * This means is the middle term of the syllogism."
   */
  getPurposeUnitesThroughMeans(): PurposeUnitesThroughMeans {
    // "purpose unites with objectivity and in objectivity unites with itself"
    const unitesWithObjectivityUnitesWithItself = new UnitesWithObjectivityUnitesWithItself();

    // "This means is the middle term of the syllogism"
    const middleTermSyllogism = unitesWithObjectivityUnitesWithItself.middleTermSyllogism();

    return new PurposeUnitesThroughMeans(middleTermSyllogism);
  }

  /**
   * NEED OF MEANS
   *
   * "Purpose is in need of a means for its realization, because it is finite, in need of a means,
   * that is to say, of a middle term that has at the same time the shape of an external existence"
   */
  getNeedOfMeans(): NeedOfMeans {
    // "in need of a means for its realization, because it is finite"
    const needMeansRealizationFinite = new NeedMeansRealizationFinite();

    // "middle term...shape of an external existence indifferent towards the purpose"
    const middleTermExternalExistenceIndifferent = needMeansRealizationFinite.middleTermExternalExistenceIndifferent();

    return new NeedOfMeans(middleTermExternalExistenceIndifferent);
  }

  /**
   * FORMAL SYLLOGISM
   *
   * "The means is therefore the formal middle term of a formal syllogism;
   * it is something external to the extreme of the subjective purpose"
   */
  getFormalSyllogism(): FormalSyllogism {
    // "formal middle term of a formal syllogism"
    const formalMiddleTermFormalSyllogism = new FormalMiddleTermFormalSyllogism();

    // "external to the extreme of the subjective purpose as also...to the extreme of the objective purpose"
    const externalSubjectiveObjectiveExtremes = formalMiddleTermFormalSyllogism.externalSubjectiveObjectiveExtremes();

    // "indifferent medius terminus that can be replaced by others"
    const indifferentMediusTerminusReplaceable = externalSubjectiveObjectiveExtremes.indifferentMediusTerminusReplaceable();

    return new FormalSyllogism(indifferentMediusTerminusReplaceable);
  }

  /**
   * MECHANICAL OBJECT PURPOSE - External Link
   *
   * "Concept and objectivity, therefore, are in the means only externally linked;
   * hence the means is only a merely mechanical object."
   */
  getMechanicalObjectPurpose(): MechanicalObjectPurpose {
    const conceptObjectivityExternallyLinked = this.getConceptObjectivityExternallyLinked();
    const referenceObjectPurpose = this.getReferenceObjectPurpose();
    const meansInheringPredicate = this.getMeansInheringPredicate();

    return new MechanicalObjectPurpose(conceptObjectivityExternallyLinked, referenceObjectPurpose, meansInheringPredicate);
  }

  /**
   * CONCEPT OBJECTIVITY EXTERNALLY LINKED
   *
   * "Concept and objectivity, therefore, are in the means only externally linked;
   * hence the means is only a merely mechanical object."
   */
  getConceptObjectivityExternallyLinked(): ConceptObjectivityExternallyLinked {
    // "only externally linked"
    const onlyExternallyLinked = new OnlyExternallyLinked();

    // "only a merely mechanical object"
    const onlyMerelyMechanicalObject = onlyExternallyLinked.onlyMerelyMechanicalObject();

    return new ConceptObjectivityExternallyLinked(onlyMerelyMechanicalObject);
  }

  /**
   * REFERENCE OBJECT PURPOSE
   *
   * "The reference of the object to purpose is a premise or the immediate reference
   * which, as we have seen, is with respect to purpose an immanent reflection"
   */
  getReferenceObjectPurpose(): ReferenceObjectPurpose {
    // "reference of the object to purpose is a premise"
    const referenceObjectPurposePremise = new ReferenceObjectPurposePremise();

    // "immediate reference...with respect to purpose an immanent reflection"
    const immediateReferenceImmanentReflection = referenceObjectPurposePremise.immediateReferenceImmanentReflection();

    return new ReferenceObjectPurpose(immediateReferenceImmanentReflection);
  }

  /**
   * MEANS INHERING PREDICATE
   *
   * "the means is an inhering predicate; its objectivity is subsumed under
   * the determination of purpose which, on account of its concreteness, is universality."
   */
  getMeansInheringPredicate(): MeansInheringPredicate {
    // "means is an inhering predicate"
    const meansInheringPredicate = new MeansInheringPredicateStructure();

    // "objectivity is subsumed under the determination of purpose"
    const objectivitySubsumedDeterminationPurpose = meansInheringPredicate.objectivitySubsumedDeterminationPurpose();

    // "which, on account of its concreteness, is universality"
    const concretenesUniversality = objectivitySubsumedDeterminationPurpose.concretenesUniversality();

    return new MeansInheringPredicate(concretenesUniversality);
  }

  /**
   * TOTALITY SYLLOGISM - Complete Structure
   *
   * "This whole middle term is thus the totality of the syllogism
   * in which the abstract activity and the external means constitute the extremes"
   */
  getTotalitySyllogism(): TotalitySyllogism {
    const wholeMiddleTermTotality = this.getWholeMiddleTermTotality();
    const universalityConnectionPurposivenessMeans = this.getUniversalityConnectionPurposivenessMeans();

    return new TotalitySyllogism(wholeMiddleTermTotality, universalityConnectionPurposivenessMeans);
  }

  /**
   * WHOLE MIDDLE TERM TOTALITY
   *
   * "This whole middle term is thus the totality of the syllogism
   * in which the abstract activity and the external means constitute the extremes,
   * while the determinateness of the object through the purpose...constitutes the middle term."
   */
  getWholeMiddleTermTotality(): WholeMiddleTermTotality {
    // "totality of the syllogism"
    const totalitySyllogism = new TotalitySyllogismStructure();

    // "abstract activity and the external means constitute the extremes"
    const abstractActivityExternalMeansExtremes = totalitySyllogism.abstractActivityExternalMeansExtremes();

    // "determinateness of the object through the purpose...constitutes the middle term"
    const determinatenessObjectPurposeMiddleTerm = abstractActivityExternalMeansExtremes.determinatenessObjectPurposeMiddleTerm();

    return new WholeMiddleTermTotality(determinatenessObjectPurposeMiddleTerm);
  }

  /**
   * UNIVERSALITY CONNECTION PURPOSIVENESS MEANS
   *
   * "But further, universality is the connection of purposiveness and the means."
   */
  getUniversalityConnectionPurposivenessMeans(): UniversalityConnectionPurposivenessMeans {
    // "universality is the connection of purposiveness and the means"
    const universalityConnectionPurposivenessMeans = new UniversalityConnectionPurposivenessMeansStructure();

    return new UniversalityConnectionPurposivenessMeans(universalityConnectionPurposivenessMeans);
  }

  /**
   * PENETRABLE BY CONCEPT SELF-SUBSISTENCE - Dialectical Tension
   *
   * "it is utterly penetrable, and it is receptive to this communication
   * because it is in itself identical with it. But...it still has self-subsistence as against the purpose."
   */
  getPenetrableByConceptSelfSubsistence(): PenetrableByConceptSelfSubsistence {
    const meansObjectTotalityConcept = this.getMeansObjectTotalityConcept();
    const purposeSubjectivitySoulObject = this.getPurposeSubjectivitySoulObject();
    const connectionConstitutePremise = this.getConnectionConstitutePremise();
    const presuppositionStillPersists = this.getPresuppositionStillPersists();

    return new PenetrableByConceptSelfSubsistence(meansObjectTotalityConcept, purposeSubjectivitySoulObject, connectionConstitutePremise, presuppositionStillPersists);
  }

  /**
   * MEANS OBJECT TOTALITY CONCEPT
   *
   * "This means is object, in itself the totality of the concept;
   * it does not have with respect to purpose any of the power of resistance"
   */
  getMeansObjectTotalityConcept(): MeansObjectTotalityConcept {
    // "in itself the totality of the concept"
    const itselfTotalityConcept = new ItselfTotalityConcept();

    // "does not have...any of the power of resistance...against another immediate object"
    const noPowerResistanceImmediateObject = itselfTotalityConcept.noPowerResistanceImmediateObject();

    // "utterly penetrable...receptive to this communication because it is in itself identical with it"
    const utterlyPenetrableReceptiveIdentical = noPowerResistanceImmediateObject.utterlyPenetrableReceptiveIdentical();

    return new MeansObjectTotalityConcept(utterlyPenetrableReceptiveIdentical);
  }

  /**
   * PURPOSE SUBJECTIVITY SOUL OBJECT
   *
   * "purpose is the subjectivity or soul of the object that has in the latter its external side."
   */
  getPurposeSubjectivitySoulObject(): PurposeSubjectivitySoulObject {
    // "Its non-self-subsistence consists precisely in its being the totality of the concept only implicitly"
    const nonSelfSubsistenceTotalityConceptImplicitly = new NonSelfSubsistenceTotalityConceptImplicitly();

    // "but the concept is being-for-itself"
    const conceptBeingForItself = nonSelfSubsistenceTotalityConceptImplicitly.conceptBeingForItself();

    // "object has the character of being powerless and of serving it"
    const objectPowerlessServing = conceptBeingForItself.objectPowerlessServing();

    // "purpose is the subjectivity or soul of the object that has in the latter its external side"
    const purposeSubjectivitySoulObjectExternalSide = objectPowerlessServing.purposeSubjectivitySoulObjectExternalSide();

    return new PurposeSubjectivitySoulObject(purposeSubjectivitySoulObjectExternalSide);
  }

  /**
   * CONNECTION CONSTITUTE PREMISE
   *
   * "The object, immediately subjected to purpose in this way, is not an extreme of the syllogism;
   * on the contrary, this connection between the two constitutes a premise of it."
   */
  getConnectionConstitutePremise(): ConnectionConstitutePremise {
    // "immediately subjected to purpose...is not an extreme of the syllogism"
    const immediatelySubjectedNotExtreme = new ImmediatelySubjectedNotExtreme();

    // "this connection between the two constitutes a premise of it"
    const connectionConstitutePremise = immediatelySubjectedNotExtreme.connectionConstitutePremise();

    return new ConnectionConstitutePremise(connectionConstitutePremise);
  }

  /**
   * PRESUPPOSITION STILL PERSISTS
   *
   * "But the means has also one side from which it still has self-subsistence as against the purpose.
   * The objectivity which in the means is bound with the purpose is still external to it"
   */
  getPresuppositionStillPersists(): PresuppositionStillPersists {
    // "still has self-subsistence as against the purpose"
    const stillSelfSubsistenceAgainstPurpose = new StillSelfSubsistenceAgainstPurpose();

    // "objectivity...bound with the purpose is still external to it"
    const objectivityBoundPurposeStillExternal = stillSelfSubsistenceAgainstPurpose.objectivityBoundPurposeStillExternal();

    // "because it is only immediately so connected; and therefore the presupposition still persists"
    const onlyImmediatelyConnectedPresuppositionPersists = objectivityBoundPurposeStillExternal.onlyImmediatelyConnectedPresuppositionPersists();

    // "The activity of the purpose through the means is for that reason still directed against this presupposition"
    const activityPurposeThroughMeansDirectedAgainstPresupposition = onlyImmediatelyConnectedPresuppositionPersists.activityPurposeThroughMeansDirectedAgainstPresupposition();

    return new PresuppositionStillPersists(activityPurposeThroughMeansDirectedAgainstPresupposition);
  }

  /**
   * COMPLETE DIALECTICAL MOVEMENT
   */
  dialecticalMovement(): string {
    return `
    DIALECTICAL MOVEMENT - THE MEANS:

    1. Sublation presupposition → Immanent turning back → Object determined as means
    2. Purpose unites through means → Need of means (finite) → Formal middle term
    3. Concept-objectivity externally linked → Mechanical object → Inhering predicate
    4. Totality of syllogism → Abstract activity-Means-External object structure
    5. Penetrable by concept → Purpose as soul of object → But presupposition persists

    RESULT: Means as dialectical middle term - penetrable yet self-subsistent
    TRANSITION: The Means → Realized Purpose (final external presupposition sublated)

    The noumenal mediation architecture - CPU-GPU coordination!
    `;
  }
}

// Core supporting classes
class SublationPresupposition {
  constructor(
    private secondDetermining: SecondDetermining,
    private immanentTurning: ImmanentTurningBack,
    private objectDetermined: ObjectSoDetermined
  ) {}
}

class FormalMiddleTerm {
  constructor(
    private purposeUnites: PurposeUnitesThroughMeans,
    private needMeans: NeedOfMeans,
    private formalSyllogism: FormalSyllogism
  ) {}
}

class MechanicalObjectPurpose {
  constructor(
    private externallyLinked: ConceptObjectivityExternallyLinked,
    private reference: ReferenceObjectPurpose,
    private inheringPredicate: MeansInheringPredicate
  ) {}
}

class TotalitySyllogism {
  constructor(
    private wholeTotality: WholeMiddleTermTotality,
    private universalityConnection: UniversalityConnectionPurposivenessMeans
  ) {}
}

class PenetrableByConceptSelfSubsistence {
  constructor(
    private objectTotality: MeansObjectTotalityConcept,
    private purposeSoul: PurposeSubjectivitySoulObject,
    private connectionPremise: ConnectionConstitutePremise,
    private presuppositionPersists: PresuppositionStillPersists
  ) {}
}

export { Means };
