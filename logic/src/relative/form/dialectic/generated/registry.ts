/*
 * GENERATED FILE — DO NOT EDIT.
 *
 * Source: src/tools/codegen-dialectic-registry.ts
 *
 * This registry links Dialectic IR ("true IR") modules into the active form layer
 * by providing a stable lookup table consumable from relative/form.
 */

import type { DialecticIR } from '@schema/dialectic';

export type DialecticIRRegistryEntry = {
  key: string;
  id: string;
  title: string;
  section: string;
  module: string;
  export: string;
};

export const dialecticIRRegistryEntries: DialecticIRRegistryEntry[] = [
  { key: '@relative/being/measure/becoming-essence/absolute-indifference-ir#absoluteIndifferenceIR', id: 'absolute-indifference-ir', title: "Absolute Indifference IR", section: "BEING - MEASURE - C. Becoming of Essence - Absolute Indifference", module: '@relative/being/measure/becoming-essence/absolute-indifference-ir', export: 'absoluteIndifferenceIR' },
  { key: '@relative/being/measure/becoming-essence/indifference-inverse-ratio-ir#indifferenceInverseRatioIR', id: 'indifference-inverse-ratio-ir', title: "Indifference and Inverse Ratio IR", section: "BEING - MEASURE - C. Becoming of Essence - Indifference and Inverse Ratio", module: '@relative/being/measure/becoming-essence/indifference-inverse-ratio-ir', export: 'indifferenceInverseRatioIR' },
  { key: '@relative/being/measure/becoming-essence/transition-essence-ir#transitionEssenceIR', id: 'transition-essence-ir', title: "Transition Essence IR", section: "BEING - MEASURE - C. Becoming of Essence - Transition into Essence", module: '@relative/being/measure/becoming-essence/transition-essence-ir', export: 'transitionEssenceIR' },
  { key: '@relative/being/measure/real-measure/measureless-ir#measurelessIR', id: 'measureless-ir', title: "Measureless IR: Progressive Determination", section: "BEING - MEASURE - B. Real Measure - The Measureless", module: '@relative/being/measure/real-measure/measureless-ir', export: 'measurelessIR' },
  { key: '@relative/being/measure/real-measure/measures-ir#measuresIR', id: 'measures-ir', title: "Measures IR: Relation of Independent Measures", section: "BEING - MEASURE - B. Real Measure - The Relation of Independent Measures", module: '@relative/being/measure/real-measure/measures-ir', export: 'measuresIR' },
  { key: '@relative/being/measure/real-measure/nodal-lines-ir#nodalLinesIR', id: 'nodal-lines-ir', title: "Nodal Lines IR: Measure-Relations and Leaps", section: "BEING - MEASURE - B. Real Measure - Nodal Lines", module: '@relative/being/measure/real-measure/nodal-lines-ir', export: 'nodalLinesIR' },
  { key: '@relative/being/measure/specific-quantity/specific-quantity-ir#specificQuantityIR', id: 'specific-quantity-ir', title: "Specific Quantity IR: Being-for-itself in Measure", section: "BEING - MEASURE - C. The Being-for-Itself in Measure", module: '@relative/being/measure/specific-quantity/specific-quantity-ir', export: 'specificQuantityIR' },
  { key: '@relative/being/measure/specific-quantity/specific-quantum-ir#specificQuantumIR', id: 'specific-quantum-ir', title: "Specific Quantum IR: Immediate Measure Dynamics", section: "BEING - MEASURE - A. The Specific Quantum", module: '@relative/being/measure/specific-quantity/specific-quantum-ir', export: 'specificQuantumIR' },
  { key: '@relative/being/measure/specific-quantity/specifying-measure-ir#specifyingMeasureIR', id: 'specifying-measure-ir', title: "Specifying Measure IR: Rule and Exponent", section: "BEING - MEASURE - B. Specifying Measure", module: '@relative/being/measure/specific-quantity/specifying-measure-ir', export: 'specifyingMeasureIR' },
  { key: '@relative/being/quality/being-for-self/attraction-ir#attractionIR', id: 'attraction-ir', title: "Attraction IR: Exclusion, Attraction, and Their Unity", section: "BEING - QUALITY - C. Being-for-Self - C. Repulsion and Attraction", module: '@relative/being/quality/being-for-self/attraction-ir', export: 'attractionIR' },
  { key: '@relative/being/quality/being-for-self/being-for-self-ir#beingForSelfIR', id: 'being-for-self-ir', title: "Being-for-Self IR: Self-Reference, Being-for-One, The One", section: "BEING - QUALITY - C. Being-for-Self", module: '@relative/being/quality/being-for-self/being-for-self-ir', export: 'beingForSelfIR' },
  { key: '@relative/being/quality/being-for-self/one-many-ir#oneManyIR', id: 'one-many-ir', title: "One and Many IR: One, Void, Repulsion, Attraction", section: "BEING - QUALITY - C. Being-for-Self - B. One and Many", module: '@relative/being/quality/being-for-self/one-many-ir', export: 'oneManyIR' },
  { key: '@relative/being/quality/being/becoming-ir#becomingIR', id: 'becoming-ir', title: "Becoming IR: Unity of Being and Nothing", section: "BEING - QUALITY - C. Becoming", module: '@relative/being/quality/being/becoming-ir', export: 'becomingIR' },
  { key: '@relative/being/quality/being/being-ir#beingIR', id: 'being-ir', title: "Being IR: Pure Being", section: "BEING - QUALITY - A. Being", module: '@relative/being/quality/being/being-ir', export: 'beingIR' },
  { key: '@relative/being/quality/being/nothing-ir#nothingIR', id: 'nothing-ir', title: "Nothing IR: Pure Nothing", section: "BEING - QUALITY - B. Nothing", module: '@relative/being/quality/being/nothing-ir', export: 'nothingIR' },
  { key: '@relative/being/quality/existence/affirmative-infinity-ir#affirmativeInfinityIR', id: 'affirmative-infinity-ir', title: "Affirmative Infinity IR: Unity, True Infinite as Becoming, True Infinite as Being", section: "I. BEING - A. QUALITY - C. Infinity - C. Affirmative Infinity", module: '@relative/being/quality/existence/affirmative-infinity-ir', export: 'affirmativeInfinityIR' },
  { key: '@relative/being/quality/existence/alternating-infinity-ir#alternatingInfinityIR', id: 'alternating-infinity-ir', title: "Alternating Infinity IR: Bad Infinite, Contradiction, Progress to Infinity", section: "I. BEING - A. QUALITY - C. Infinity - B. Alternating Determination", module: '@relative/being/quality/existence/alternating-infinity-ir', export: 'alternatingInfinityIR' },
  { key: '@relative/being/quality/existence/constitution-ir#constitutionIR', id: 'constitution-ir', title: "Constitution IR: Determination, Constitution, and Limit", section: "BEING - QUALITY - B. Finitude - b. Determination, Constitution, and Limit", module: '@relative/being/quality/existence/constitution-ir', export: 'constitutionIR' },
  { key: '@relative/being/quality/existence/existence-chapter-ir#existenceChapterIR', id: 'existence-chapter-ir', title: "Existence Chapter IR: Existence, Finitude, and Infinity", section: "BEING - QUALITY - EXISTENCE CHAPTER", module: '@relative/being/quality/existence/existence-chapter-ir', export: 'existenceChapterIR' },
  { key: '@relative/being/quality/existence/existence-ir#existenceIR', id: 'existence-ir', title: "Existence IR: Determinate Existence as Such", section: "BEING - QUALITY - A. Existence as Such", module: '@relative/being/quality/existence/existence-ir', export: 'existenceIR' },
  { key: '@relative/being/quality/existence/finitude-ir#finitudeIR', id: 'finitude-ir', title: "Finitude IR: Restriction, Ought, and Transition to Infinite", section: "BEING - QUALITY - B. Finitude - c. Finitude", module: '@relative/being/quality/existence/finitude-ir', export: 'finitudeIR' },
  { key: '@relative/being/quality/existence/infinity-ir#infinityIR', id: 'infinity-ir', title: "Infinity IR: Infinite in General", section: "BEING - QUALITY - C. Infinity - a. The Infinite in General", module: '@relative/being/quality/existence/infinity-ir', export: 'infinityIR' },
  { key: '@relative/being/quality/existence/something-and-other-ir#somethingAndOtherIR', id: 'something-and-other-ir', title: "Something and Other IR: Relational Finitude", section: "BEING - QUALITY - B. Finitude - a. Something and Other", module: '@relative/being/quality/existence/something-and-other-ir', export: 'somethingAndOtherIR' },
  { key: '@relative/being/quality/integrated-topicmap-ir#integratedTopicMapIR', id: 'quality-ir', title: "Quality IR: Being, Existence, and Being-for-itself", section: "BEING - QUALITY", module: '@relative/being/quality/integrated-topicmap-ir', export: 'integratedTopicMapIR' },
  { key: '@relative/being/quality/quality-ir#qualityIR', id: 'quality-ir', title: "Quality IR: Being, Existence, and Being-for-itself", section: "BEING - QUALITY", module: '@relative/being/quality/quality-ir', export: 'qualityIR' },
  { key: '@relative/being/quantity/quantity/limiting-quantity-ir#limitingQuantityIR', id: 'limiting-quantity-ir', title: "Limiting Quantity IR: Quantum Threshold", section: "BEING - QUANTITY - C. Limiting Quantity", module: '@relative/being/quantity/quantity/limiting-quantity-ir', export: 'limitingQuantityIR' },
  { key: '@relative/being/quantity/quantity/magnitude-ir#magnitudeIR', id: 'magnitude-ir', title: "Magnitude IR: Continuous and Discrete Magnitude", section: "BEING - QUANTITY - B. Continuous and Discrete Magnitude", module: '@relative/being/quantity/quantity/magnitude-ir', export: 'magnitudeIR' },
  { key: '@relative/being/quantity/quantity/pure-quantity-ir#pureQuantityIR', id: 'pure-quantity-ir', title: "Pure Quantity IR: Continuity and Discreteness", section: "BEING - QUANTITY - A. Pure Quantity", module: '@relative/being/quantity/quantity/pure-quantity-ir', export: 'pureQuantityIR' },
  { key: '@relative/being/quantity/quantum/infinity-ir#infinityIR', id: 'infinity-ir', title: "Infinity IR: Quantitative Infinity and Return to Quality", section: "BEING - QUANTITY - D. Quantitative Infinity", module: '@relative/being/quantity/quantum/infinity-ir', export: 'infinityIR' },
  { key: '@relative/being/quantity/quantum/number-ir#numberIR', id: 'number-ir', title: "Number IR: Quantum as Number", section: "BEING - QUANTITY - A. Number", module: '@relative/being/quantity/quantum/number-ir', export: 'numberIR' },
  { key: '@relative/being/quantity/quantum/quantum-ir#quantumIR', id: 'quantum-ir', title: "Quantum IR: Extensive and Intensive Quantum", section: "BEING - QUANTITY - C. Quantum", module: '@relative/being/quantity/quantum/quantum-ir', export: 'quantumIR' },
  { key: '@relative/being/quantity/ratio/inverse-ir#inverseIR', id: 'inverse-ir', title: "Inverse IR: The Inverse Ratio", section: "BEING - QUANTITY - D. Inverse Ratio", module: '@relative/being/quantity/ratio/inverse-ir', export: 'inverseIR' },
  { key: '@relative/being/quantity/ratio/powers-ir#powersIR', id: 'powers-ir', title: "Powers IR: Ratio of Powers and Transition to Measure", section: "BEING - QUANTITY - D. Ratio of Powers", module: '@relative/being/quantity/ratio/powers-ir', export: 'powersIR' },
  { key: '@relative/being/quantity/ratio/ratio-ir#ratioIR', id: 'ratio-ir', title: "Ratio IR: Quantitative Relation", section: "BEING - QUANTITY - D. Ratio", module: '@relative/being/quantity/ratio/ratio-ir', export: 'ratioIR' },
  { key: '@relative/concept/idea/cognition/cognition-ir#cognitionIR', id: 'cognition-ir', title: "Cognition IR: General Cognition, Theoretical Truth, Spirit Threshold", section: "CONCEPT - IDEA - B. Cognition - 1. Introduction", module: '@relative/concept/idea/cognition/cognition-ir', export: 'cognitionIR' },
  { key: '@relative/concept/idea/cognition/idea-of-the-good-ir#ideaOfTheGoodIR', id: 'idea-of-the-good-ir', title: "Idea of the Good IR: Practical Good, Ought, Absolute Transition", section: "CONCEPT - IDEA - B. Cognition - 3. The Idea of the Good", module: '@relative/concept/idea/cognition/idea-of-the-good-ir', export: 'ideaOfTheGoodIR' },
  { key: '@relative/concept/idea/cognition/idea-of-the-true-ir#ideaOfTheTrueIR', id: 'idea-of-the-true-ir', title: "Idea of the True IR: Impulse, Synthetic Method, Practical Transition", section: "CONCEPT - IDEA - B. Cognition - 2. The Idea of the True", module: '@relative/concept/idea/cognition/idea-of-the-true-ir', export: 'ideaOfTheTrueIR' },
  { key: '@relative/concept/idea/life/genus-ir#genusIR', id: 'genus-ir', title: "Genus IR: Identity/Duplication, Propagation, Transition to Cognition", section: "CONCEPT - IDEA - A. Life - C. The Genus", module: '@relative/concept/idea/life/genus-ir', export: 'genusIR' },
  { key: '@relative/concept/idea/life/life-process-ir#lifeProcessIR', id: 'life-process-ir', title: "Life-Process IR: Need/Pain, Assimilation, Genus Threshold", section: "CONCEPT - IDEA - A. Life - B. The Life-Process", module: '@relative/concept/idea/life/life-process-ir', export: 'lifeProcessIR' },
  { key: '@relative/concept/idea/life/living-individual-ir#livingIndividualIR', id: 'living-individual-ir', title: "Living Individual IR: Immediate Life, Organism, Internal Process", section: "CONCEPT - IDEA - A. Life - A. The Living Individual", module: '@relative/concept/idea/life/living-individual-ir', export: 'livingIndividualIR' },
  { key: '@relative/concept/idea/speculation/absolute-idea-ir#absoluteIdeaIR', id: 'absolute-idea-ir', title: "Absolute Idea IR: Identity, Method, Liberation", section: "CONCEPT - IDEA - C. Speculation - A. The Absolute Idea", module: '@relative/concept/idea/speculation/absolute-idea-ir', export: 'absoluteIdeaIR' },
  { key: '@relative/concept/idea/speculation/method-advance-ir#methodAdvanceIR', id: 'method-advance-ir', title: "Method Advance IR: Dialectical Movement, Third, Circle", section: "CONCEPT - IDEA - C. Speculation - B. Method - 2. The Advance", module: '@relative/concept/idea/speculation/method-advance-ir', export: 'methodAdvanceIR' },
  { key: '@relative/concept/idea/speculation/method-beginning-ir#methodBeginningIR', id: 'method-beginning-ir', title: "Method Beginning IR: Immediate Universal, Deficiency, Concrete Germ", section: "CONCEPT - IDEA - C. Speculation - B. Method - 1. The Beginning", module: '@relative/concept/idea/speculation/method-beginning-ir', export: 'methodBeginningIR' },
  { key: '@relative/concept/object/chemism/chemism-ir#chemismIR', id: 'chemism-ir', title: "Chemism IR: First Negation, Staged Sublation, Purpose", section: "CONCEPT - OBJECTIVITY - B. Chemism - C. Transition of Chemism", module: '@relative/concept/object/chemism/chemism-ir', export: 'chemismIR' },
  { key: '@relative/concept/object/chemism/object-ir#chemicalObjectIR', id: 'chemical-object-ir', title: "Chemical Object IR: Non-Indifference, Totality, Process Bridge", section: "CONCEPT - OBJECTIVITY - B. Chemism - A. The Chemical Object", module: '@relative/concept/object/chemism/object-ir', export: 'chemicalObjectIR' },
  { key: '@relative/concept/object/chemism/process-ir#chemicalProcessIR', id: 'chemical-process-ir', title: "Chemical Process IR: Affinity, Neutrality, Self-Sublation", section: "CONCEPT - OBJECTIVITY - B. Chemism - B. The Process", module: '@relative/concept/object/chemism/process-ir', export: 'chemicalProcessIR' },
  { key: '@relative/concept/object/mechanism/mechanism-ir#mechanismIR', id: 'mechanism-ir', title: "Mechanism IR: Center, Free Law, Chemism Bridge", section: "CONCEPT - OBJECTIVITY - A. Mechanism - C. Absolute Mechanism", module: '@relative/concept/object/mechanism/mechanism-ir', export: 'mechanismIR' },
  { key: '@relative/concept/object/mechanism/object-ir#mechanicalObjectIR', id: 'mechanical-object-ir', title: "Mechanical Object IR: Equilibrium, Plurality, Process Transition", section: "CONCEPT - OBJECTIVITY - A. Mechanism - A. The Mechanical Object", module: '@relative/concept/object/mechanism/object-ir', export: 'mechanicalObjectIR' },
  { key: '@relative/concept/object/mechanism/process-ir#mechanicalProcessIR', id: 'mechanical-process-ir', title: "Mechanical Process IR: Formal Cycle, Real Opposition, Lawful Product", section: "CONCEPT - OBJECTIVITY - A. Mechanism - B. The Mechanical Process", module: '@relative/concept/object/mechanism/process-ir', export: 'mechanicalProcessIR' },
  { key: '@relative/concept/object/teleology/means-ir#meansIR', id: 'means-ir', title: "Means IR: Formal Middle, Mechanical Means, Realization Bridge", section: "CONCEPT - OBJECTIVITY - C. Teleology - B. The Means", module: '@relative/concept/object/teleology/means-ir', export: 'meansIR' },
  { key: '@relative/concept/object/teleology/realized-ir#realizedPurposeIR', id: 'realized-purpose-ir', title: "Realized Purpose IR: Dominated Mechanism, External Limit, Objective Identity", section: "CONCEPT - OBJECTIVITY - C. Teleology - C. The Realized Purpose", module: '@relative/concept/object/teleology/realized-ir', export: 'realizedPurposeIR' },
  { key: '@relative/concept/object/teleology/teleology-ir#teleologyIR', id: 'teleology-ir', title: "Teleology IR: Rediscovered Purpose, Finitude, Means Bridge", section: "CONCEPT - OBJECTIVITY - C. Teleology - A. Subjective Purpose", module: '@relative/concept/object/teleology/teleology-ir', export: 'teleologyIR' },
  { key: '@relative/concept/subject/concept/particular-ir#particularIR', id: 'particular-ir', title: "Particular IR: Immanent Determinateness, Division, Singularity Transition", section: "CONCEPT - SUBJECTIVITY - A. The Concept - 2. The Particular", module: '@relative/concept/subject/concept/particular-ir', export: 'particularIR' },
  { key: '@relative/concept/subject/concept/singular-ir#singularIR', id: 'singular-ir', title: "Singular IR: Posited Singularity, Indissoluble Unity, Judgment Transition", section: "CONCEPT - SUBJECTIVITY - A. The Concept - 3. The Singular", module: '@relative/concept/subject/concept/singular-ir', export: 'singularIR' },
  { key: '@relative/concept/subject/concept/universal-ir#universalIR', id: 'universal-ir', title: "Universal IR: Pure Concept, Creative Universality, Self-Differentiation", section: "CONCEPT - SUBJECTIVITY - A. The Concept - 1. The Universal", module: '@relative/concept/subject/concept/universal-ir', export: 'universalIR' },
  { key: '@relative/concept/subject/judgment/concept-ir#conceptJudgmentIR', id: 'concept-judgment-ir', title: "Concept Judgment IR (Fourth Moment): Ought, Apodictic Truth, Idea Bridge", section: "CONCEPT - SUBJECTIVITY - B. Judgment - D. Judgment of the Concept (Idea Bridge)", module: '@relative/concept/subject/judgment/concept-ir', export: 'conceptJudgmentIR' },
  { key: '@relative/concept/subject/judgment/existence-ir#existenceIR', id: 'existence-ir', title: "Existence Judgment IR: Immediate Truth, Negative Truth, Reflection Transition", section: "CONCEPT - SUBJECTIVITY - B. Judgment - A. Judgment of Existence", module: '@relative/concept/subject/judgment/existence-ir', export: 'existenceIR' },
  { key: '@relative/concept/subject/judgment/necessity-ir#necessityIR', id: 'necessity-ir', title: "Necessity Judgment IR: Objective Universality, Disjunction, Concept Transition", section: "CONCEPT - SUBJECTIVITY - B. Judgment - C. Judgment of Necessity", module: '@relative/concept/subject/judgment/necessity-ir', export: 'necessityIR' },
  { key: '@relative/concept/subject/judgment/reflection-ir#reflectionIR', id: 'reflection-ir', title: "Reflection Judgment IR: Subsumption, Allness, Necessity Transition", section: "CONCEPT - SUBJECTIVITY - B. Judgment - B. Judgment of Reflection", module: '@relative/concept/subject/judgment/reflection-ir', export: 'reflectionIR' },
  { key: '@relative/concept/subject/syllogism/existence-ir#existenceSyllogismIR', id: 'existence-syllogism-ir', title: "Existence Syllogism IR: Immediate Form, Figure Development, Reflection Handoff", section: "CONCEPT - SUBJECTIVITY - C. Syllogism - A. Syllogism of Existence", module: '@relative/concept/subject/syllogism/existence-ir', export: 'existenceSyllogismIR' },
  { key: '@relative/concept/subject/syllogism/necessity-ir#necessitySyllogismIR', id: 'necessity-syllogism-ir', title: "Necessity Syllogism IR (Bridge): Categorical, Hypothetical, Objectivity Handoff", section: "CONCEPT - SUBJECTIVITY - C. Syllogism - C. Syllogism of Necessity (Objectivity Bridge)", module: '@relative/concept/subject/syllogism/necessity-ir', export: 'necessitySyllogismIR' },
  { key: '@relative/concept/subject/syllogism/reflection-ir#reflectionSyllogismIR', id: 'reflection-syllogism-ir', title: "Reflection Syllogism IR: Allness, Induction/Analogy, Necessity Handoff", section: "CONCEPT - SUBJECTIVITY - C. Syllogism - B. Syllogism of Reflection", module: '@relative/concept/subject/syllogism/reflection-ir', export: 'reflectionSyllogismIR' },
  { key: '@relative/essence/actuality/absolute/attribute-ir#attributeIR', id: 'attribute-ir', title: "Attribute IR: Relative Absolute, Identity Determination, Form Nullity", section: "ESSENCE - C. ACTUALITY - C. The Absolute - b. The Attribute", module: '@relative/essence/actuality/absolute/attribute-ir', export: 'attributeIR' },
  { key: '@relative/essence/actuality/absolute/exposition-ir#expositionIR', id: 'exposition-ir', title: "Exposition IR: Absolute Unity, Negative/Positive Exposition, Relative Absolute", section: "ESSENCE - C. ACTUALITY - C. The Absolute - a. The Exposition", module: '@relative/essence/actuality/absolute/exposition-ir', export: 'expositionIR' },
  { key: '@relative/essence/actuality/absolute/mode-ir#modeIR', id: 'mode-ir', title: "Mode IR: Externality Posited, Immanent Turning Back, Absolute Manifestation", section: "ESSENCE - C. ACTUALITY - C. The Absolute - c. The Mode", module: '@relative/essence/actuality/absolute/mode-ir', export: 'modeIR' },
  { key: '@relative/essence/actuality/actuality/absolute-necessity-ir#absoluteNecessityIR', id: 'absolute-necessity-ir', title: "Absolute Necessity IR: Determinate Necessity, Truth, Substance Transition", section: "ESSENCE - C. ACTUALITY - C. Absolute Necessity", module: '@relative/essence/actuality/actuality/absolute-necessity-ir', export: 'absoluteNecessityIR' },
  { key: '@relative/essence/actuality/actuality/contingency-ir#contingencyIR', id: 'contingency-ir', title: "Contingency IR: Formal Actuality, Groundless/Grounded Unity, Necessity", section: "ESSENCE - C. ACTUALITY - A. Contingency", module: '@relative/essence/actuality/actuality/contingency-ir', export: 'contingencyIR' },
  { key: '@relative/essence/actuality/actuality/relative-necessity-ir#relativeNecessityIR', id: 'relative-necessity-ir', title: "Relative Necessity IR: Real Actuality, Condition Completion, Presupposed Necessity", section: "ESSENCE - C. ACTUALITY - B. Actuality - Relative Necessity", module: '@relative/essence/actuality/actuality/relative-necessity-ir', export: 'relativeNecessityIR' },
  { key: '@relative/essence/actuality/substance/reciprocity-action-ir#reciprocityActionIR', id: 'reciprocity-action-ir', title: "Reciprocity Action IR: Mechanism Sublation, Freedom, Concept Triad", section: "ESSENCE - C. ACTUALITY - Absolute Relation - c. Reciprocity of Action", module: '@relative/essence/actuality/substance/reciprocity-action-ir', export: 'reciprocityActionIR' },
  { key: '@relative/essence/actuality/substance/relation-causality-ir#relationCausalityIR', id: 'relation-causality-ir', title: "Relation Causality IR: Formal Causality, Extinguishing, Reciprocity Passover", section: "ESSENCE - C. ACTUALITY - Absolute Relation - b. Causality", module: '@relative/essence/actuality/substance/relation-causality-ir', export: 'relationCausalityIR' },
  { key: '@relative/essence/actuality/substance/relation-substantiality-ir#relationSubstantialityIR', id: 'relation-substantiality-ir', title: "Relation Substantiality IR: Substance, Power, Causal Passover", section: "ESSENCE - C. ACTUALITY - Absolute Relation - a. Substantiality", module: '@relative/essence/actuality/substance/relation-substantiality-ir', export: 'relationSubstantialityIR' },
  { key: '@relative/essence/appearance/relation/force-expression-ir#forceExpressionIR', id: 'force-expression-ir', title: "Force-Expression IR: Force, Solicitation, Infinity", section: "B. APPEARANCE - 3. Essential Relation - c. Force and Expression", module: '@relative/essence/appearance/relation/force-expression-ir', export: 'forceExpressionIR' },
  { key: '@relative/essence/appearance/relation/outer-inner-ir#outerInnerIR', id: 'outer-inner-ir', title: "Outer-Inner IR: One Identity, Immediate Conversion, Actuality", section: "B. APPEARANCE - 3. Essential Relation - b. Outer and Inner", module: '@relative/essence/appearance/relation/outer-inner-ir', export: 'outerInnerIR' },
  { key: '@relative/essence/appearance/relation/whole-parts-ir#wholePartsIR', id: 'whole-parts-ir', title: "Whole-Parts IR: Essential Relation, Reciprocal Conditioning, Tautology", section: "B. APPEARANCE - 3. Essential Relation - a. Whole and Parts", module: '@relative/essence/appearance/relation/whole-parts-ir', export: 'wholePartsIR' },
  { key: '@relative/essence/appearance/thing/dissolution-ir#dissolutionIR', id: 'dissolution-ir', title: "Dissolution IR: Absolute Porosity, Interpenetration, Appearance", section: "B. APPEARANCE - 1. The Thing - c. Dissolution", module: '@relative/essence/appearance/thing/dissolution-ir', export: 'dissolutionIR' },
  { key: '@relative/essence/appearance/thing/matter-ir#matterIR', id: 'matter-ir', title: "Matter IR: Property to Matter, Porous Matter", section: "B. APPEARANCE - 1. The Thing - b. Matter", module: '@relative/essence/appearance/thing/matter-ir', export: 'matterIR' },
  { key: '@relative/essence/appearance/thing/thing-ir#thingIR', id: 'thing-ir', title: "Thing IR: Concrete Existence, Thing-in-Itself, Property", section: "ESSENCE - B. APPEARANCE - A. THE THING", module: '@relative/essence/appearance/thing/thing-ir', export: 'thingIR' },
  { key: '@relative/essence/appearance/world/disappearance-ir#disappearanceIR', id: 'disappearance-ir', title: "Disappearance IR: Opposition, Law Realized, World Foundered", section: "B. APPEARANCE - 2. The World - c. Dissolution of Appearance", module: '@relative/essence/appearance/world/disappearance-ir', export: 'disappearanceIR' },
  { key: '@relative/essence/appearance/world/law-ir#lawIR', id: 'law-ir', title: "Law IR: Appearance, Law, Kingdom of Laws", section: "B. APPEARANCE - 2. The World - b. Law of Appearance", module: '@relative/essence/appearance/world/law-ir', export: 'lawIR' },
  { key: '@relative/essence/appearance/world/world-ir#worldIR', id: 'world-ir', title: "World IR: Kingdom of Laws, Suprasensible World, Opposition", section: "B. APPEARANCE - 2. The World - a. World-In-Itself", module: '@relative/essence/appearance/world/world-ir', export: 'worldIR' },
  { key: '@relative/essence/reflection/essence/essence-ir#essenceIR', id: 'essence-ir', title: "Essence IR: Truth of Being, Absolute Negativity, Reflection", section: "ESSENCE - A. ESSENCE AS REFLECTION WITHIN ITSELF", module: '@relative/essence/reflection/essence/essence-ir', export: 'essenceIR' },
  { key: '@relative/essence/reflection/essence/reflection-ir#reflectionIR', id: 'reflection-ir', title: "Reflection IR: Essence as Reflection, Positing, External, Determining", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. Reflection", module: '@relative/essence/reflection/essence/reflection-ir', export: 'reflectionIR' },
  { key: '@relative/essence/reflection/essence/shine-ir#shineIR', id: 'shine-ir', title: "Shine IR: Being as Shine and Infinite Determinateness", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 1. Shine", module: '@relative/essence/reflection/essence/shine-ir', export: 'shineIR' },
  { key: '@relative/essence/reflection/foundation/contradiction-ir#contradictionIR', id: 'contradiction-ir', title: "Contradiction IR: Opposition, Posited Contradiction, Ground", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - C. Contradiction", module: '@relative/essence/reflection/foundation/contradiction-ir', export: 'contradictionIR' },
  { key: '@relative/essence/reflection/foundation/difference-ir#differenceIR', id: 'difference-ir', title: "Difference IR: Absolute Difference, Diversity, Opposition", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - B. Difference", module: '@relative/essence/reflection/foundation/difference-ir', export: 'differenceIR' },
  { key: '@relative/essence/reflection/foundation/identity-ir#identityIR', id: 'identity-ir', title: "Identity IR: Essence as Simple Self-Identity", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 2. The Determinations of Reflection - A. Identity", module: '@relative/essence/reflection/foundation/identity-ir', export: 'identityIR' },
  { key: '@relative/essence/reflection/ground/absolute-ir#absoluteIR', id: 'absolute-ir', title: "Absolute Ground IR: Form, Matter, Content", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - a. Absolute Ground", module: '@relative/essence/reflection/ground/absolute-ir', export: 'absoluteIR' },
  { key: '@relative/essence/reflection/ground/condition-ir#conditionIR', id: 'condition-ir', title: "Condition IR: Unconditioned, Procession, Concrete Existence", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - c. Condition", module: '@relative/essence/reflection/ground/condition-ir', export: 'conditionIR' },
  { key: '@relative/essence/reflection/ground/determinate-ir#determinateIR', id: 'determinate-ir', title: "Determinate Ground IR: Formal, Real, Complete, Conditioning", section: "A. ESSENCE AS REFLECTION WITHIN ITSELF - 3. Ground - b. Determinate Ground", module: '@relative/essence/reflection/ground/determinate-ir', export: 'determinateIR' },
];

export const dialecticIRRegistry: Record<string, DialecticIRRegistryEntry> = Object.fromEntries(
  dialecticIRRegistryEntries.map(e => [e.key, e])
);

export const dialecticIRIdIndex: Record<string, string[]> = {
  "absolute-idea-ir": ["@relative/concept/idea/speculation/absolute-idea-ir#absoluteIdeaIR"],
  "absolute-indifference-ir": ["@relative/being/measure/becoming-essence/absolute-indifference-ir#absoluteIndifferenceIR"],
  "absolute-ir": ["@relative/essence/reflection/ground/absolute-ir#absoluteIR"],
  "absolute-necessity-ir": ["@relative/essence/actuality/actuality/absolute-necessity-ir#absoluteNecessityIR"],
  "affirmative-infinity-ir": ["@relative/being/quality/existence/affirmative-infinity-ir#affirmativeInfinityIR"],
  "alternating-infinity-ir": ["@relative/being/quality/existence/alternating-infinity-ir#alternatingInfinityIR"],
  "attraction-ir": ["@relative/being/quality/being-for-self/attraction-ir#attractionIR"],
  "attribute-ir": ["@relative/essence/actuality/absolute/attribute-ir#attributeIR"],
  "becoming-ir": ["@relative/being/quality/being/becoming-ir#becomingIR"],
  "being-for-self-ir": ["@relative/being/quality/being-for-self/being-for-self-ir#beingForSelfIR"],
  "being-ir": ["@relative/being/quality/being/being-ir#beingIR"],
  "chemical-object-ir": ["@relative/concept/object/chemism/object-ir#chemicalObjectIR"],
  "chemical-process-ir": ["@relative/concept/object/chemism/process-ir#chemicalProcessIR"],
  "chemism-ir": ["@relative/concept/object/chemism/chemism-ir#chemismIR"],
  "cognition-ir": ["@relative/concept/idea/cognition/cognition-ir#cognitionIR"],
  "concept-judgment-ir": ["@relative/concept/subject/judgment/concept-ir#conceptJudgmentIR"],
  "condition-ir": ["@relative/essence/reflection/ground/condition-ir#conditionIR"],
  "constitution-ir": ["@relative/being/quality/existence/constitution-ir#constitutionIR"],
  "contingency-ir": ["@relative/essence/actuality/actuality/contingency-ir#contingencyIR"],
  "contradiction-ir": ["@relative/essence/reflection/foundation/contradiction-ir#contradictionIR"],
  "determinate-ir": ["@relative/essence/reflection/ground/determinate-ir#determinateIR"],
  "difference-ir": ["@relative/essence/reflection/foundation/difference-ir#differenceIR"],
  "disappearance-ir": ["@relative/essence/appearance/world/disappearance-ir#disappearanceIR"],
  "dissolution-ir": ["@relative/essence/appearance/thing/dissolution-ir#dissolutionIR"],
  "essence-ir": ["@relative/essence/reflection/essence/essence-ir#essenceIR"],
  "existence-chapter-ir": ["@relative/being/quality/existence/existence-chapter-ir#existenceChapterIR"],
  "existence-ir": ["@relative/being/quality/existence/existence-ir#existenceIR","@relative/concept/subject/judgment/existence-ir#existenceIR"],
  "existence-syllogism-ir": ["@relative/concept/subject/syllogism/existence-ir#existenceSyllogismIR"],
  "exposition-ir": ["@relative/essence/actuality/absolute/exposition-ir#expositionIR"],
  "finitude-ir": ["@relative/being/quality/existence/finitude-ir#finitudeIR"],
  "force-expression-ir": ["@relative/essence/appearance/relation/force-expression-ir#forceExpressionIR"],
  "genus-ir": ["@relative/concept/idea/life/genus-ir#genusIR"],
  "idea-of-the-good-ir": ["@relative/concept/idea/cognition/idea-of-the-good-ir#ideaOfTheGoodIR"],
  "idea-of-the-true-ir": ["@relative/concept/idea/cognition/idea-of-the-true-ir#ideaOfTheTrueIR"],
  "identity-ir": ["@relative/essence/reflection/foundation/identity-ir#identityIR"],
  "indifference-inverse-ratio-ir": ["@relative/being/measure/becoming-essence/indifference-inverse-ratio-ir#indifferenceInverseRatioIR"],
  "infinity-ir": ["@relative/being/quality/existence/infinity-ir#infinityIR","@relative/being/quantity/quantum/infinity-ir#infinityIR"],
  "inverse-ir": ["@relative/being/quantity/ratio/inverse-ir#inverseIR"],
  "law-ir": ["@relative/essence/appearance/world/law-ir#lawIR"],
  "life-process-ir": ["@relative/concept/idea/life/life-process-ir#lifeProcessIR"],
  "limiting-quantity-ir": ["@relative/being/quantity/quantity/limiting-quantity-ir#limitingQuantityIR"],
  "living-individual-ir": ["@relative/concept/idea/life/living-individual-ir#livingIndividualIR"],
  "magnitude-ir": ["@relative/being/quantity/quantity/magnitude-ir#magnitudeIR"],
  "matter-ir": ["@relative/essence/appearance/thing/matter-ir#matterIR"],
  "means-ir": ["@relative/concept/object/teleology/means-ir#meansIR"],
  "measureless-ir": ["@relative/being/measure/real-measure/measureless-ir#measurelessIR"],
  "measures-ir": ["@relative/being/measure/real-measure/measures-ir#measuresIR"],
  "mechanical-object-ir": ["@relative/concept/object/mechanism/object-ir#mechanicalObjectIR"],
  "mechanical-process-ir": ["@relative/concept/object/mechanism/process-ir#mechanicalProcessIR"],
  "mechanism-ir": ["@relative/concept/object/mechanism/mechanism-ir#mechanismIR"],
  "method-advance-ir": ["@relative/concept/idea/speculation/method-advance-ir#methodAdvanceIR"],
  "method-beginning-ir": ["@relative/concept/idea/speculation/method-beginning-ir#methodBeginningIR"],
  "mode-ir": ["@relative/essence/actuality/absolute/mode-ir#modeIR"],
  "necessity-ir": ["@relative/concept/subject/judgment/necessity-ir#necessityIR"],
  "necessity-syllogism-ir": ["@relative/concept/subject/syllogism/necessity-ir#necessitySyllogismIR"],
  "nodal-lines-ir": ["@relative/being/measure/real-measure/nodal-lines-ir#nodalLinesIR"],
  "nothing-ir": ["@relative/being/quality/being/nothing-ir#nothingIR"],
  "number-ir": ["@relative/being/quantity/quantum/number-ir#numberIR"],
  "one-many-ir": ["@relative/being/quality/being-for-self/one-many-ir#oneManyIR"],
  "outer-inner-ir": ["@relative/essence/appearance/relation/outer-inner-ir#outerInnerIR"],
  "particular-ir": ["@relative/concept/subject/concept/particular-ir#particularIR"],
  "powers-ir": ["@relative/being/quantity/ratio/powers-ir#powersIR"],
  "pure-quantity-ir": ["@relative/being/quantity/quantity/pure-quantity-ir#pureQuantityIR"],
  "quality-ir": ["@relative/being/quality/integrated-topicmap-ir#integratedTopicMapIR","@relative/being/quality/quality-ir#qualityIR"],
  "quantum-ir": ["@relative/being/quantity/quantum/quantum-ir#quantumIR"],
  "ratio-ir": ["@relative/being/quantity/ratio/ratio-ir#ratioIR"],
  "realized-purpose-ir": ["@relative/concept/object/teleology/realized-ir#realizedPurposeIR"],
  "reciprocity-action-ir": ["@relative/essence/actuality/substance/reciprocity-action-ir#reciprocityActionIR"],
  "reflection-ir": ["@relative/concept/subject/judgment/reflection-ir#reflectionIR","@relative/essence/reflection/essence/reflection-ir#reflectionIR"],
  "reflection-syllogism-ir": ["@relative/concept/subject/syllogism/reflection-ir#reflectionSyllogismIR"],
  "relation-causality-ir": ["@relative/essence/actuality/substance/relation-causality-ir#relationCausalityIR"],
  "relation-substantiality-ir": ["@relative/essence/actuality/substance/relation-substantiality-ir#relationSubstantialityIR"],
  "relative-necessity-ir": ["@relative/essence/actuality/actuality/relative-necessity-ir#relativeNecessityIR"],
  "shine-ir": ["@relative/essence/reflection/essence/shine-ir#shineIR"],
  "singular-ir": ["@relative/concept/subject/concept/singular-ir#singularIR"],
  "something-and-other-ir": ["@relative/being/quality/existence/something-and-other-ir#somethingAndOtherIR"],
  "specific-quantity-ir": ["@relative/being/measure/specific-quantity/specific-quantity-ir#specificQuantityIR"],
  "specific-quantum-ir": ["@relative/being/measure/specific-quantity/specific-quantum-ir#specificQuantumIR"],
  "specifying-measure-ir": ["@relative/being/measure/specific-quantity/specifying-measure-ir#specifyingMeasureIR"],
  "teleology-ir": ["@relative/concept/object/teleology/teleology-ir#teleologyIR"],
  "thing-ir": ["@relative/essence/appearance/thing/thing-ir#thingIR"],
  "transition-essence-ir": ["@relative/being/measure/becoming-essence/transition-essence-ir#transitionEssenceIR"],
  "universal-ir": ["@relative/concept/subject/concept/universal-ir#universalIR"],
  "whole-parts-ir": ["@relative/essence/appearance/relation/whole-parts-ir#wholePartsIR"],
  "world-ir": ["@relative/essence/appearance/world/world-ir#worldIR"],
};

export function getDialecticIRMeta(key: string): DialecticIRRegistryEntry | undefined {
  return dialecticIRRegistry[key];
}

export function getDialecticIRKeysById(id: string): string[] {
  return dialecticIRIdIndex[id] ?? [];
}

/**
 * Load a DialecticIR by registry key.
 *
 * Note: This uses dynamic import of the source module specifier recorded in the registry.
 * In built JS output, resolution depends on your bundler/aliasing strategy.
 */
export async function loadDialecticIR(key: string): Promise<DialecticIR> {
  const meta = getDialecticIRMeta(key);
  if (!meta) throw new Error('Unknown DialecticIR key: ' + key);
  const mod = await import(meta.module);
  const ir = (mod as any)[meta.export] as unknown;
  return ir as DialecticIR;
}
