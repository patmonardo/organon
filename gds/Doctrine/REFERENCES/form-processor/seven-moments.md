# The Seven Moments of Reflection

This reference explains the canonical structure of the Reflection stage in the
Reflection → Principle → Concept → Judgment → Syllogism → Procedure arc.

---

## The Arc

```
Source → Observation → Reflection (7 moments) → Principle → Concept → Judgment → Syllogism → Procedure
```

Reflection is not a single moment. It is a seven-moment sequence that the entity passes through
before it can be evaluated for Principle.

---

## The Six Moments of Relative Reflection

These are stages where the entity's appearance is refined into essence structure.

### Moment 1: Being

**The entity as first-given.** What is immediately present?

The entity enters Reflection with its raw appearance: the data as observed, the text as written,
the signal as received. Nothing has been structured yet.

**Dataset output**: `(entity_id, moment, "being", essence_mark, raw_value)`

---

### Moment 2: Identity

**The entity marks itself as singular.** What makes this one entity rather than another?

The identity field is fixed. The entity has a proper name, a unique ID, a stable reference.
Identity is the answer to "which one?"

**Dataset output**: `(entity_id, moment, "identity", identity_field, identity_value)`

---

### Moment 3: Opposition

**The entity negates what it is not.** What is outside this entity?

Opposition is the first determination. The entity is distinguished from everything that is
not it. This is the binary opposition: self vs. other.

**Dataset output**: `(entity_id, moment, "opposition", negated_mark, opposite_value)`

---

### Moment 4: Contradiction

**The entity holds together opposites.** What tensions does it contain?

Contradiction is not error; it is depth. The entity is neither pure sameness nor pure difference.
It is both and neither. It contains internal tension.

**Dataset output**: `(entity_id, moment, "contradiction", tension_mark, resolution_space)`

---

### Moment 5: Ground (First Moment of Ground)

**The entity finds its ground in condition.** What must hold for this entity to be possible?

Ground is what supports the entity. It is the necessary condition, the reason, the medium
in which the entity can exist.

**Dataset output**: `(entity_id, moment, "ground-condition", ground_mark, condition_value)`

---

### Moment 6: Ground-as-Condition (Second Moment of Ground)

**The ground becomes explicit and evaluated.** Does the supporting condition hold?

Where Moment 5 identified the ground, Moment 6 evaluates it. The condition is checked.
The ground is measured. The entity is now fully determined by what supports it.

**Dataset output**: `(entity_id, moment, "ground-evaluated", condition_mark, evaluated_result)`

---

## The Seventh Moment: Absolute Reflection

This is the threshold. It is **not yet Concept**, but it is the preparation.

### Moment 7: Absolute Reflection

**The entity stands at the threshold of scientific objecthood.** Can it proceed to Principle?

Absolute Reflection is where the kernel and the agent meet. Both must traverse this moment
with zero-copy access to shape data. The entity is now fully articulated by the six relative
moments. The question is: does it satisfy the program's Principle?

If yes, Concept emerges.
If no, the entity remains at the threshold.

**Dataset output**: `(entity_id, moment, "absolute-reflection", threshold_mark, status: "ready_for_principle")`

**ZeroCopy requirement**: Both the Rust kernel (PureForm) and the TS agent (Relative Form Processor)
must read the entity's shape at Moment 7 without serialization. The shape data is the common
language. Principle evaluation happens in that shared space.

---

## Reading the Reflection Table

A complete reflection table looks like:

```
entity_id  | moment                | stage_name              | essence_mark                | value
-----------|----------------------|-------------------------|-----------------------------|-----------
ent_123    | 1                    | being                   | appearance::raw             | raw_datum
ent_123    | 2                    | identity                | identity::unique            | ent_123
ent_123    | 3                    | opposition              | opposition::not_other       | ent_456
ent_123    | 4                    | contradiction           | tension::sameness_diff      | partial
ent_123    | 5                    | ground-condition        | ground::medium              | condition_1
ent_123    | 6                    | ground-evaluated        | evaluation::holds           | true
ent_123    | 7                    | absolute-reflection     | threshold::ready            | threshold_met
```

Reading across the columns:
- **entity_id**: which entity is this?
- **moment**: which of the 7 stages?
- **stage_name**: the name of that stage (being, identity, opposition, ...)
- **essence_mark**: the semantic token applied at that moment
- **value**: the resulting value

This table tells the complete story of how an entity passed through essence articulation.

---

## Why Seven Moments?

The seven moments are not arbitrary. They follow from Hegelian dialectic and Fichte's deduction:

1. **Thesis** (Being)
2. **Antithesis** (Opposition)
3. **Synthesis** (Contradiction held together)
4. **Negation of negation** (Ground emerges as the resolution)
5. **Ground as condition** (the ground is identified)
6. **Ground as evaluated** (the condition is measured)
7. **Absolute** (all moments are held together at the threshold)

Seven moments are the classical triadic form expanded to full articulation and return to unity.

---

## What Happens After Absolute Reflection

At Moment 7, the entity is fully articulated but not yet determined as scientific object.

The **Principle** stage comes next. Principle asks: "Does this entity satisfy our nomological
requirements?"

If Principle says yes, then **Concept** emerges: the entity becomes a named, determinate,
scientific object.

If Principle says no, the entity remains at Moment 7. It may re-enter Observation or remain
in the threshold state. But it does not proceed to Concept.

This is why Principle comes _before_ Concept. An entity must be approved by Principle before
it can be named as a scientific object.

---

## Implementation Notes

- The reflection table is mandatory. Every program that uses the `reflection` feature must emit
  this 7-moment artifact.
- Moment 7 is the zero-copy boundary. Design your shape data so that the kernel and agent can
  both read it at this moment without marshaling.
- Provenance must carry through all 7 moments. Each moment knows where the entity came from.
- The reflection table is auditable. A user should be able to read it and understand exactly
  how the entity was articulated.
