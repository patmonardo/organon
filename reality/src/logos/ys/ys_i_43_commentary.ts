import { DatasetUnit, makeUnitId } from '../registry/canon'

export const CHUNKS_I_43_COMMENTARY = [
  {
    id: 'ys-i-43-rules-for-disjunction',
    title: 'Rules for disjunction: into principles; each both unity and disjunction',
    source:
      '(paraphrase) Disjunction proceeds into principles; each principle is equally a principle of unity and of disjunction.',
  },
  {
    id: 'ys-i-43-genetic-empirical-schema',
    title: 'Deduction: genetic schema of the total empirical domain',
    source:
      '(paraphrase) From the disjunction of principles, deduce a general schema of the empirical field according to the form of its genetic principle.',
  },
  {
    id: 'ys-i-43-only-in-light',
    title: 'Only in/through light: oneness-beyond as projection, contemplation of light',
    source:
      '(paraphrase) Neither in A nor in the point; the beyond “in itself” is nothing for us; it exists only through/in light as its projection; contemplation of light.',
  },
  {
    id: 'ys-i-43-derive-higher-principle',
    title: 'Further step: deriving the oneness/disjunction of principles of division',
    source:
      '(paraphrase) Proceed to derive the principle of oneness and disjunction of materially different principles of division—opening a new side of the investigation.',
  },
  {
    id: 'ys-i-43-representation-kills-light',
    title: 'Observation via representative objectifies (kills) the light',
    source:
      '(paraphrase) Observing light through a representative alienates and kills it as primordial; analysis shifts from “what” to “how” (genetic view of observing).',
  },
  {
    id: 'ys-i-43-image-imaged-mutual-implication',
    title: 'Conceptual manifestness: image ⇔ imaged; oneness of content',
    source:
      '(paraphrase) Image requires imaged, and imaged requires image; absolute oneness of content remains, disjunction appears only in the fulfillment of thinking.',
  },
  {
    id: 'ys-i-43-concealed-oneness-principle',
    title: 'Principle = concealed oneness lived in the act of seeing',
    source:
      '(paraphrase) Principle no longer in light or its image, but in the oneness between them, realized only in the act; “primordial concept” as lived immediate oneness/disjunction.',
  },
  {
    id: 'ys-i-43-primordial-concept',
    title: 'Primordial concept more original than light; light as its disjunct',
    source:
      '(paraphrase) What appeared original (light) arises as a disjunct of the primordial concept; hence the concept is more original in the genetic order.',
  },
  {
    id: 'ys-i-43-concept-implicit-content',
    title: 'Concept now has implicit, self‑subsistent content (not just divider)',
    source:
      '(paraphrase) Formerly a dividing principle expiring in light, the concept now bears its own undeniable, unchangeable content; division only conditions its life/appearance.',
  },
  {
    id: 'ys-i-43-concept-same-substantial-being',
    title: 'Same substantial being: now prior to intuition, principle of objectifying intuition',
    source:
      '(paraphrase) The concept’s content is the same substantial being once projected from intuition, now prior to all intuition and grounding objectifying intuition itself.',
  },
  {
    id: 'ys-i-43-concept-grounds-appearance',
    title: 'Self‑grounding: image and imaged posited absolutely; appearance expresses inner being',
    source:
      '(paraphrase) The single concept grounds its appearance from its essence; image and imaged are organically posited through one another; appearance is exponent of inner being.',
  },
  {
    id: 'ys-i-43-organic-through-one-another',
    title: 'Being‑for‑itself = inner organization of through‑one‑another; absolute oneness self‑grounded',
    source:
      '(paraphrase) Its permanent being‑for‑itself equals its inner organization of the through‑one‑another (essential, not externally constructed); absolute oneness is grounded through itself.',
  },
  {
    id: 'ys-i-43-organic-oneness-primordial-concept',
    title: 'Inner organic oneness of the primordial concept',
    source:
      '(paraphrase) The oneness we need: the primordial concept’s inner organic oneness that remains one in content while disjoining only in the vital fulfillment of thinking.',
  },
  {
    id: 'ys-i-43-mutual-positing-truth-in-itself',
    title: 'Mutual positing (image ⇔ imaged) as truth in itself without priority',
    source:
      '(paraphrase) Image, as image, posits imaged—and vice versa—not by temporal priority but as truth in itself; abstraction from our insight reveals their identical content.',
  },
  {
    id: 'ys-i-43-critique-and-post-factum',
    title: 'Critique of “and”: post factum synthesis; antecedent/consequent dissolve into appearance',
    source:
      '(paraphrase) The empty “and” merely links two halves post factum; determination by sequence produces antecedent/consequent, which on inspection dissolve into appearance.',
  },
  {
    id: 'ys-i-43-through-one-another-condition',
    title: 'Through-one-another as common condition holding all inference',
    source:
      '(paraphrase) What remains as condition is the through-one-another that initially holds every inference and leaves the consequence relation free as it appears.',
  }
]

export const HLOS_I_43_COMMENTARY = [
  {
    id: 'ys-i-43-hlo-rules-for-disjunction',
    chunkId: 'ys-i-43-rules-for-disjunction',
    label: 'Disjunction into principles; each is unity ∧ disjunction',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.10')",
      "tag('category','rules')",

      'disjoin(into: principles)',
      'forAll(p in principles): simultaneouslyPrinciple(p, unity, disjunction)'
    ]
  },
  {
    id: 'ys-i-43-hlo-genetic-empirical-schema',
    chunkId: 'ys-i-43-genetic-empirical-schema',
    label: 'Genetic schema of empirical domain',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.11')",
      "tag('category','genesis/schema')",

      'deduce(schema(empiricalTotality) from geneticPrinciple)',
      'announce(newExplanation)'
    ]
  },
  {
    id: 'ys-i-43-hlo-only-in-light',
    chunkId: 'ys-i-43-only-in-light',
    label: 'Oneness-beyond exists only in/through light (projection)',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.12')",
      "tag('category','light')",

      '¬locus(A) ∧ ¬locus(point)',
      'beyondInItself := nothingForUs',
      'existsOnlyThrough(light) ∧ in(light) as projection',
      'contemplate(light)'
    ]
  },
  {
    id: 'ys-i-43-hlo-derive-higher-principle',
    chunkId: 'ys-i-43-derive-higher-principle',
    label: 'Toward principle-formation: oneness/disjunction of divisions',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.13')",
      "tag('category','principle/derivation')",

      'derive(principle(oneness ∧ disjunction) for materiallyDifferent(divisions))',
      'open(newSideOfInvestigation)',
      'prefigure(nirvicāra: principleFormation)'
    ]
  },
  {
    id: 'ys-i-43-hlo-representation-kills-light',
    chunkId: 'ys-i-43-representation-kills-light',
    label: 'Observation via proxy ⇒ objectification (killing) of light',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.14')",
      "tag('category','representation')",

      'observe(light) via representative ⇒ objectify(light) ∧ kill(primordialLife(light))',
      'representativeWithout(represented) == nothing'
    ]
  },
  {
    id: 'ys-i-43-hlo-image-imaged-mutual-implication',
    chunkId: 'ys-i-43-image-imaged-mutual-implication',
    label: 'Image ⇔ imaged; oneness of content; disjunction in fulfillment',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.15')",
      "tag('category','manifestness')",

      'requires(image, imaged) ∧ requires(imaged, image)',
      'sameContent := absoluteOneness(content(image,imaged))',
      'disjunction := in(fulfillment(thinking)) not(inContent)',
      'prefer(subjectiveObjectiveDisjunction)'
    ]
  },
  {
    id: 'ys-i-43-hlo-concealed-oneness-principle',
    chunkId: 'ys-i-43-concealed-oneness-principle',
    label: 'Principle = concealed oneness lived in the act of seeing',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.16')",
      "tag('category','principle/oneness')",

      'principle := concealedOnenessBetween(image, imaged)',
      'realizedBy(actOfThinking(viewOfLight))',
      'cannotDescribeFurther; onlyLive(immediately)'
    ]
  },
  {
    id: 'ys-i-43-hlo-primordial-concept',
    chunkId: 'ys-i-43-primordial-concept',
    label: 'Primordial concept > light (genetically); light as its disjunct',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')",
      "tag('cycle','ys:vitarka:A.1')","tag('order','2.17')",
      "tag('category','primordial-concept')",

      'primordialConcept := content(concealedOneness)',
      'light(arisesAs, disjunctOf(primordialConcept))',
      'moreOriginal(primordialConcept, than=light)'
    ]
  },
  {
    id: 'ys-i-43-hlo-concept-implicit-content',
    chunkId: 'ys-i-43-concept-implicit-content',
    label: 'Concept acquires implicit, self‑subsistent content',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.18')","tag('category','concept/content')",

      'was(dividingPrinciple ⇒ expiresIn(light))',
      'now(concept.content := selfSubsistent ∧ undeniable ∧ unchangeable)',
      'division ⇒ onlyConditions(life/appearance)'
    ]
  },
  {
    id: 'ys-i-43-hlo-concept-same-substantial-being',
    chunkId: 'ys-i-43-concept-same-substantial-being',
    label: 'Same substantial being, now prior to intuition',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.19')","tag('category','being/prior')",

      'concept.content == substantialBeing(previouslyProjected(from:Intuition))',
      'now(priorTo(Intuition)) ∧ principle(of objectifyingIntuition)'
    ]
  },
  {
    id: 'ys-i-43-hlo-concept-grounds-appearance',
    chunkId: 'ys-i-43-concept-grounds-appearance',
    label: 'Concept grounds its own appearance; image ⟺ imaged',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.20')","tag('category','appearance/ground')",

      'grounds(appearance(concept), by=essence(concept))',
      'positAbsolutely(image, imaged) as organic(throughOneAnother)',
      'appearance ⇒ exponent(innerBeing)'
    ]
  },
  {
    id: 'ys-i-43-hlo-organic-through-one-another',
    chunkId: 'ys-i-43-organic-through-one-another',
    label: 'Being‑for‑itself = inner organization (through‑one‑another); self‑grounded oneness',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.21')","tag('category','oneness/self-ground')",

      'beingForItself == innerOrganization(throughOneAnother) // essential, not externally constructed',
      'absoluteOneness ⇒ groundsThrough(itself)'
    ]
  },
  {
    id: 'ys-i-43-hlo-organic-oneness-primordial-concept',
    chunkId: 'ys-i-43-organic-oneness-primordial-concept',
    label: 'Primordial concept: inner organic oneness',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.22')","tag('category','oneness/organic')",

      'organicOneness(primordialConcept)',
      'contentOne := remains(one) while disjunction := only(in fulfillment(thinking))'
    ]
  },
  {
    id: 'ys-i-43-hlo-mutual-positing-truth-in-itself',
    chunkId: 'ys-i-43-mutual-positing-truth-in-itself',
    label: 'Image ⇔ imaged as truth in itself; no priority',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.23')","tag('category','mutual-positing')",

      'truthInItself(image ⇔ imaged)',
      'noPriority(image, imaged)',
      'sameContent(image, imaged)'
    ]
  },
  {
    id: 'ys-i-43-hlo-critique-and-post-factum',
    chunkId: 'ys-i-43-critique-and-post-factum',
    label: 'Critique of “and”; post factum synthesis; sequence-forms as appearance',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.24')","tag('category','post-factum')",

      'reject(expletiveAnd)',
      'diagnose(postFactumSynthesis)',
      'sequencePlaceDetermines(antecedent, consequent) ⇒ dissolvesInto(appearance)'
    ]
  },
  {
    id: 'ys-i-43-hlo-through-one-another-condition',
    chunkId: 'ys-i-43-through-one-another-condition',
    label: 'Through-one-another holds inference; consequence left free',
    clauses: [
      "tag('lens','fichte')","tag('method','samapatti')",
      "tag('mode','vitarka')","tag('phase','nirvitarka')",
      "tag('plane','dyadic')","tag('role','reflection')",
      "tag('faculty','buddhi')","tag('cycle','ys:vitarka:A.1')",
      "tag('order','2.25')","tag('category','through-one-another')",

      'condition := throughOneAnother',
      'holdsTogether(allInference)',
      'leaves(consequenceRelation, free)'
    ]
  }
]

export const YS_I_43_COMMENTARY_UNIT: DatasetUnit = {
  id: makeUnitId('i.43.commentary'),
  title: 'YS I.43 — Commentary (Fichtean analysis)',
  scope: 'being-only',
  logosMode: 'prajna',
  synthesis: 'pre-factum',
  faculty: 'buddhi',
  lens: 'fichte',
  chunks: CHUNKS_I_43_COMMENTARY as any,
  hlos: HLOS_I_43_COMMENTARY as any,
}
