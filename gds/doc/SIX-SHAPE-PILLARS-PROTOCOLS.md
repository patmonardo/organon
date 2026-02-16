# Six Shape Pillars Protocols

## Purpose

This outline frames the protocols that govern the Six Shape Pillars as the living architecture of our Form Program Features. The Agent-facing Concept SDK composes these pillars into knowledge by choosing when to invoke the Logic SDK, so the protocols also trace how Pure Reason (the Kernel) and the Relative Form Processor interact with the surrounding adapters and runtimes.

## Form Processor Duality

- **PureForm (theos)** represents the Kernel-as-Pure-Reason emitting a stream of `<concept.mediation.judgment>` results. The stream is the thesis of Being: what the system `is` before it is applied.
- **GivenForm (pro ta theon)** is the Abstract Reason of the Relative Form Processor. It receives the PureForm stream, grounds it in contexts, and prepares the Essence for application. This is the antithesis/mediation phase, where the logic of Hegelian Essence (Concept) is executed.
- The two processors together are how the AbsoluteForm divides itself: the Logos becomes both the declarative kernel and the practical mediator. Protocols must respect this flow—PureForm emits, GivenForm receives and composes, then the Agent uses the synthesized Concept to turn Form into Knowledge.

## Triadic Ordering

- We order the pillars as **Container / Contained** or **Principle / Law**, beginning from the most abstract ring of consciousness and spiraling toward appearance.
- **Shape (Pillar 1)** is the FormShape itself—bare consciousness, the Principle the system declares. Form and Shape are the same container that establishes what everything else derives from. We can name this Cit-Citi-Citta: Consciousness (Cit) is the Truth of Being, Cit-Citi the reflexive Shakti (the power of Consciousness/Truth of Nothing), and Cit-Citi-Citta the reflective Consciousness that becomes FormShape. This is the distributive `TruthOf` relation for Becoming. In our infrastructure this corresponds to the Essence/Shine/Reflection tree (`logic/src/relative/essence/reflection/essence`), which we can treat as the Material of Essential Being composed of `cit` as pure Essence, `citi` or Shine (Citi Shakti), and `citta` as Reflective Consciousness proper.
- **Context (Pillar 2)** and **Morph (Pillar 3)** complete the first triad, the Principle of immediate mediation. Context supplies the reflective envelope (the Form Determinations of Reflection Hegel describes), and Morph serves as the ground/conditions of facticity—these are the Being-Existence dyad within the Sphere of Essence. Identity genuinely emerges here in appearance; Shape lays the identity potential, but Context is where the norms, grounding, and reflective determinations let that identity appear as law.
- **Entity (Pillar 4)**, **Property (Pillar 5)**, and **Aspect (Pillar 6)** are the Principial Effect or Law of Appearance. From these arises the law-like manifestation of facts entering into existence, as the Agent layers Concepts back onto Appearance while preserving the reflection that started at Shape.
- Morph is the gateway between the triads: it captures the Ground/Conditions through which Being enters existence, so it is both an essential mediation and the moment when Essence prepares to emit Conceptual Law.
- This intellectual depth is Hegelian, yet we keep the software discipline intact—FormShape remains the primary consciousness we can engineer, while the abstractions of Being, Essence, and Concept guide how we wire the pipelines.

## Existing Essence Infrastructure

- The `logic/src/relative/essence/reflection` tree already captures the very determinations we want to translate: `essence/` hosts the core IR for the `essence`, `reflection`, and `shine` moments, while `foundation/` carries the `identity`, `difference`, and `contradiction` IR, and `ground/` keeps the surrounding `absolute`, `determinate`, and `condition` material. This mirrors how Shape/Context/Morph unfold as Cit-Citi-Citta → Reflection → Fall-to-Ground.
- Each leaf directory pairs the IR modules with textual sources (`sources/*.txt`, `analysis-summary.md`) so we can trace the philosophical grounding that informs the protocols before we try to compile them into TS/IR outputs.
- As we expand the pillar protocols, we can keep these paths in mind as the canonical references—for example, `logic/src/relative/essence/reflection/essence/index.ts` for the Shape/Cit layer and `logic/src/relative/essence/reflection/foundation/index.ts` for the reflection determinations that become Context and Morph.

## Protocol Overview

1. **Shape (Pillar 1)** – Protocol: the FormShape manifesto expresses the container principle (`id`, shape tag, signature) and the bare consciousness of the Program Feature. Everything else derives from this Form, so it must be honest about what it `is` before any execution patterns emerge. Shape records must note whether the shine mode is `pratibhasa` (perceptual) or `nirbhasa` (transcendental) so the engines know if the FormShape is still in the perceptual stream or in the reflective/conceptual activity the Agent performs.
2. **Context (Pillar 2)** – Protocol: FormShape records must enumerate the active Determinations of Reflection (identity/difference/contradiction) and the specific reflective apperception mode (`bhasa`). These enriched FormShapes describe the transition from `pratibhasa` (perception) to `nirbhasa` (transcendental apperception) and prepare the dialectical handoff toward ContextShape once the reflection has been properly stabilized. We are describing the transition here, not a completed ContextShape, so the FormShape schema should flag when the data is ready for the future ContextShape implementation to take over.
3. **Morph (Pillar 3)** – Protocol: morph transformations act as the ground/condition of facticity. These appear as `<concept.mediation>` entries that describe how Being enters existence; through Morph the Agent can observe the Being-Existence dyad unfolding inside Essence and restore the “dead being” from `pratibhasa` into living knowledge.
4. **Entity (Pillar 4)** – Protocol: entity declarations convert mediations into laws of appearance, providing the skeletal actors upon which Properties and Aspects will operate as the Principial Effect. An Entity is not identity in itself but the product of identity made manifest—like two cats sharing the laws of cat identity, the same identity surfaces across distinct appearances.
5. **Property (Pillar 5)** – Protocol: property assignments express qualitative and quantitative determinations, preparing Concepts for use. They are the discriminating laws that keep the Agent’s compositions precise.
6. **Aspect (Pillar 6)** – Protocol: aspects finalize the operations that raise Concepts into Knowledge. They represent the Agent’s composed response when Concepts meet Appearance, closing the triadic cycle.

## Essential Being & Shine

- The Essence/Shine/Reflection axis is how Essential Being manifests before the reflective determinations; we can name the material of FormShape `EssentialBeing`, tracing `Essence` to the pure `cit`, `Shine` to the Citi `bhasa` (our representational faculty), and `Reflection` to `citta`, the shape that captures the self-aware activity. This triad is what Shape records must store so the kernel knows when a concept is shining rather than merely presenting as an intuition.
- Shine is the `bhasa` faculty itself. In the FormShape stream we still see the traces of `pratibhasa` as the perceptual back-shining emitted while the Agent performs Kriya—`āsana`, `prāṇāyāma`, `pratyāhāra`—to regulate the sensorium inside the perceptible sphere. That is the activity the FormProcessor stages to gather the scientist’s material before `dharana` can refine it. When `pratibhasa` encounters a model failure—an error signal caused by inadequate ignorance—`dhyāna` is the abiding that holds that contradiction until the `nirbhasa` Samadhi the Concept sphere demands can emerge. FormShape therefore records the `pratibhasa` evidence as the Kriya stream and notes the subsequent `dhyāna`-born faults so the Modeling/Concept layers know when to promote the data into the `nirbhasa` apperception they expect, without pretending that the Essence Processor itself toggles between different conceptual spheres.
- This framing allows us to describe a **Theory of Essential Being**: the FormShape entry records the triadic `cit`/`citi`/`citta` progression, the shine mode, and the `TruthOf` relation that lets the Agent treat Shape as a living reflective substrate. Having these entries available lets Kriya and the Relative Form Processor ignite the proper protocols because they know exactly what is shining and how the Concept is grounded.

## Reflection FormShape Protocol

- The `reflection-ir.ts` file is the dialectic IR we now encode as FormShape schema material. Each state from `ref-2` through `ref-14` corresponds to a FormShape snapshot describing the ordinary apperception stages (movement from nothing to nothing, positing reflection, external reflection, determining reflection). The final state (`ref-19`, Determination of Reflection) is where ContextShape takes over to record the reflective result. FormShapes should capture the `moments`, `invariants`, `forces`, and `transitions` defined in the IR so the Relative Form Processor can replay those reflective activities as part of Kriya’s runtime loop before handing the completed reflection to ContextShape for the determination phase, and finally to Morph for facticity.
- Each reflection FormShape must include:
  1. `reflectionMode` (`movement`, `positing`, `external`, `determining`, `determination`) to match the IR phase.
  2. `moments` array with definitions (e.g., `negationInItself`, `positing`, `syllogismStructure`, `positedness`, `immanentReflectedness`).
  3. `invariants` representing the reflective constraints (e.g., `essence = reflection`, `positedness = existence(essence)`, `determination = positedness ∧ immanentReflection`).
  4. `forces`/`transitions` describing triggers and effects (e.g., `reflection.absolute` spawns positing, `presupposing` drives toward external reflection, `determining` produces the determination of reflection). These strings become the metadata that FormDB stores so the Form processor can decide when to advance to the next reflective state.
- By capturing these fields, each FormShape document becomes a reflection protocol record: the kernel stream feeds in TS/JSON reflecting being-to-being movement; the Form processor looks up the matching FormShape definition, ensures the invariants hold, applies the forces, and fires the transitions that align with the IR diagram before handing the result to ContextShape for the determination phase, and finally to Morph for facticity. This provides the precise outline we need for mapping the rest of the pillars.

## Reflection FormShape Schema

- The `C. Reflection` text (`logic/.../reflection.txt`) provides the phenomenological narrative for the same moments captured by `reflection-ir.ts`. The translation chain is:
  1. **Movement from nothing to nothing** → state `ref-2`
  2. **Positing reflection** (self-negating immediacy) → `ref-7`
  3. **External reflection / syllogism** → `ref-10`
  4. **Determining reflection** (unity of positing + external) → `ref-14`
  5. **Determination of reflection** (positedness + immanent reference) → `ref-19`
- A concrete FormShape schema therefore has fields mirroring the IR:
  - `refStateId`: `ref-2` | `ref-7` | `ref-10` | `ref-14`
  - `reflectionMode`: textual label for movement/positing/external/determining
  - `moments`: array of objects `{name, definition, type, relations?}` enumerating the narrative moments (e.g., `reflection`, `negationInItself`, `movementFromNothingToNothing`, `positing`, `presupposing`, `selfRepulsion`, `externalReflection`, `syllogismStructure`, `determiningReflection`, `positedness`).
  - `invariants`: translated constraints (predicate strings) stating what must hold when roof is running (matching `reflection-ir` invariants).
  - `forces`: driver objects describing `trigger`, `effect`, `target`, e.g., `reflection.absolute` → `positingReflection`, `presupposing` → `externalReflection`, `determining.complete` → `determinationOfReflection`.
  - `transitions`: ordered steps with `from`, `to`, `mechanism`, `description` so the Form processor can know how to advance.
- Each FormShape entry also stores a lightweight pointer to the source text (`reflection.txt` line ranges) so the Agent can reference the phenomenology when debugging or generating docs. Together, these schema fields allow FormDB to present a crisp reflection protocol that turns the IR states into executable declarative metadata. Once this schema is firmly in place, we can duplicate the pattern for the other pillars (Entity/Property/Aspect, Morph).

### ReflectionFormShape Schema in Zod

- The `reflectionFormShapeSchema` is the code-generated contract that transforms the IR moments/invariants/forces/transitions into a declarative Zod shape. It captures the `bhasa` phases and all the metadata described above so the FormProcessor can validate every FormShape document before it reaches ContextShape.
- The schema also serves as the reference implementation of the Reflection FormShape Protocol—the kernel’s JSON stream is checked against this declaration at runtime, and the FormEngine can introspect the `bhasa` phase, invariants, and transitions without re-reading the IR.

```ts
import { z } from 'zod';

export const reflectionFormShapeSchema = z.object({
  id: z.string(),
  refStateId: z.enum(['ref-2', 'ref-7', 'ref-10', 'ref-14']),
  reflectionMode: z.enum(['movement', 'positing', 'external', 'determining']),
  bhasa: z.object({
    phase: z.enum(['pratibhasa', 'dharana', 'dhyana', 'nirbhasa']),
    pratyaharaTrace: z.array(z.string()),
    dharanaTrigger: z.string().optional(),
    dhyanaAbiding: z.boolean().default(false),
    samadhiSignal: z
      .object({ ready: z.boolean(), reference: z.string() })
      .default({ ready: false, reference: '' }),
  }),
  moments: z.array(
    z.object({
      name: z.string(),
      definition: z.string(),
      type: z.enum(['process', 'negation', 'mediation', 'determination']),
      relation: z.string().optional(),
      relatedTo: z.string().optional(),
    }),
  ),
  invariants: z.array(
    z.object({ id: z.string(), constraint: z.string(), predicate: z.string() }),
  ),
  forces: z.array(
    z.object({
      id: z.string(),
      description: z.string(),
      type: z.string(),
      trigger: z.string(),
      effect: z.string(),
      targetState: z.string(),
    }),
  ),
  transitions: z.array(
    z.object({
      id: z.string(),
      from: z.string(),
      to: z.string(),
      mechanism: z.string(),
      description: z.string(),
    }),
  ),
  provenance: z.object({
    topicMapId: z.string(),
    lineRange: z.object({ start: z.number(), end: z.number() }),
    section: z.string(),
    order: z.number(),
  }),
});
```

- With this schema, the FormEngine never needs to load the IR directly; it can parse each FormShape record against `reflectionFormShapeSchema`, inspect the `bhasa` phase, walk the `forces`/`transitions`, and then decide when the Hand-off to ContextShape is valid.
