/**
 * QUANTITY - The Determinateness Indifferent to Being
 * =================================================
 *
 * "Quantity is sublated being-for-itself."
 *
 * The transition from Quality to Quantity: where determinateness
 * becomes indifferent to being - a limit which is just as much no limit.
 */

interface QuantityMoments {
  continuity: Continuity;
  discreteness: Discreteness;
  limitingOfQuantity: LimitingOfQuantity;
}

export class Quantity implements DialecticalMoment {
  private continuity: Continuity;
  private discreteness: Discreteness;

  constructor() {
    // "Quantity is sublated being-for-itself"
    this.continuity = new Continuity();
    this.discreteness = new Discreteness();
  }

  /**
   * A. PURE QUANTITY
   * "Quantity is sublated being-for-itself. The repelling one that behaved only
   * negatively towards the excluded one, now that it has gone over in connection with it,
   * behaves towards the other as identical to itself"
   */
  pureQuantity(): PureQuantity {
    // Being-for-itself has passed over into attraction
    const attraction = this.sublateBeingForItself();

    // "The absolute obduracy of the one has melted away into this unity"
    const unity = attraction.meltObduracy();

    // "as unity of the self-externality, it is unity with itself"
    return new PureQuantity(unity.unityWithItself());
  }

  /**
   * B. CONTINUOUS AND DISCRETE MAGNITUDE
   * "Quantity contains the two moments of continuity and discreteness."
   */
  continuousAndDiscreteMagnitude(): ContinuousAndDiscreteMagnitude {
    // "It is to be posited in both, in each as its determination"
    const continuousMagnitude = this.positContinuity();
    const discreteMagnitude = this.positDiscreteness();

    return new ContinuousAndDiscreteMagnitude(
      continuousMagnitude,
      discreteMagnitude
    );
  }

  /**
   * C. THE LIMITING OF QUANTITY
   * "Discrete magnitude has, first, the one for its principle
   * and, second, is a plurality of ones; third, it is essentially continuous"
   */
  limitingOfQuantity(): Quantum {
    // "it is posited as one magnitude, and the 'one' is its determinateness"
    const oneMagnitude = this.positAsOneMagnitude();

    // "a 'one' which, in this posited and determinate existence, excludes, is a limit to the unity"
    const limit = oneMagnitude.excludingLimit();

    // "Real, discrete quantity is thus one quantity, or quantum: quantity as an existence and a something"
    return new Quantum(limit.asExistenceAndSomething());
  }

  private sublateBeingForItself(): Attraction {
    // "being-for-itself has passed over into attraction"
    return new Attraction();
  }

  private positContinuity(): ContinuousMagnitude {
    return this.continuity.asWholeMagnitude();
  }

  private positDiscreteness(): DiscreteMagnitude {
    return this.discreteness.asWholeMagnitude();
  }

  private positAsOneMagnitude(): OneMagnitude {
    return new OneMagnitude(this.discreteness.principleOfOne());
  }

  // Dialectical transition to Quantum
  transcend(): Quantum {
    return this.limitingOfQuantity();
  }
}

/**
 * PURE QUANTITY - Sublated Being-For-Itself
 * "Pure quantity is real being-for-itself turned back into itself,
 * with as yet no determinateness in it: a compact, infinite unity
 * which continues itself into itself."
 */
class PureQuantity {
  private compactInfiniteUnity: CompactInfiniteUnity;

  constructor(unityWithItself: UnityWithItself) {
    this.compactInfiniteUnity = new CompactInfiniteUnity(unityWithItself);
  }

  continuesIntoItself(): SelfContinuation {
    return this.compactInfiniteUnity.selfContinuation();
  }
}

/**
 * CONTINUITY - Attraction as Moment of Quantity
 * "Attraction is in this way the moment of continuity in quantity."
 */
class Continuity {
  /**
   * "Continuity is therefore simple, self-same reference to itself
   * unbroken by any limit or exclusion; not, however, immediate unity
   * but the unity of ones which have existence for themselves."
   */
  selfSameReference(): SelfSameReference {
    // "Still contained in it is the outside-one-another of plurality"
    const outsideOneAnother = new OutsideOneAnother();

    // "though at the same time as something without distinctions, unbroken"
    return new SelfSameReference(outsideOneAnother.withoutDistinctions());
  }

  asWholeMagnitude(): ContinuousMagnitude {
    // "Continuity is only the compact unity holding together as unity of the discrete"
    return new ContinuousMagnitude(this.compactUnityOfDiscrete());
  }

  private compactUnityOfDiscrete(): CompactUnity {
    return new CompactUnity();
  }
}

/**
 * DISCRETENESS - Repulsion as Moment in Quantity
 * "In continuity, therefore, magnitude immediately possesses
 * the moment of discreteness, repulsion as now a moment in quantity."
 */
class Discreteness {
  /**
   * "Discreteness is, for its part, a discreteness of confluents,
   * of ones that do not have the void to connect them"
   */
  discretenessOfConfluents(): DiscretenessOfConfluents {
    // "not the negative, but their own steady advance"
    return new DiscretenessOfConfluents(this.steadyAdvance());
  }

  asWholeMagnitude(): DiscreteMagnitude {
    // "Discrete magnitude is therefore the one-outside-the-other of
    // the many ones as of a same; not the many ones in general,
    // but posited rather as the many of a unity."
    return new DiscreteMagnitude(this.manyOfAUnity());
  }

  principleOfOne(): PrincipleOfOne {
    return new PrincipleOfOne();
  }

  private steadyAdvance(): SteadyAdvance {
    return new SteadyAdvance();
  }

  private manyOfAUnity(): ManyOfAUnity {
    return new ManyOfAUnity();
  }
}

// Supporting types capturing Hegel's precise logical movements
class Attraction {
  meltObduracy(): Unity { return new Unity(); }
}
class Unity {
  unityWithItself(): UnityWithItself { return new UnityWithItself(); }
}
class UnityWithItself {}
class CompactInfiniteUnity {
  constructor(private unity: UnityWithItself) {}
  selfContinuation(): SelfContinuation { return new SelfContinuation(); }
}
class SelfContinuation {}

class OutsideOneAnother {
  withoutDistinctions(): WithoutDistinctions { return new WithoutDistinctions(); }
}
class WithoutDistinctions {}
class SelfSameReference {
  constructor(private without: WithoutDistinctions) {}
}

class ContinuousMagnitude {
  constructor(private compactUnity: CompactUnity) {}
}
class DiscreteMagnitude {
  constructor(private manyOfUnity: ManyOfAUnity) {}
}
class CompactUnity {}
class ManyOfAUnity {}

class DiscretenessOfConfluents {
  constructor(private advance: SteadyAdvance) {}
}
class SteadyAdvance {}
class PrincipleOfOne {}

class OneMagnitude {
  constructor(private principle: PrincipleOfOne) {}
  excludingLimit(): ExcludingLimit { return new ExcludingLimit(); }
}
class ExcludingLimit {
  asExistenceAndSomething(): ExistenceAndSomething { return new ExistenceAndSomething(); }
}
class ExistenceAndSomething {}

class Quantum {
  constructor(private existence: ExistenceAndSomething) {}
}

export { Quantity, PureQuantity, Continuity, Discreteness, Quantum };
