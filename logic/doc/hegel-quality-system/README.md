# Hegel's Qualitative Logic as Guna Theory

## Overview

This project provides a complete TypeScript implementation of Hegel's Logic of Quality from the *Greater Logic*, translated through the lens of **Guna Theory** (the theory of three fundamental qualities from Indian philosophy).

## Core Insight

**Qualitative Logic is conditioned by Gunas for content**, whereas Formal Logic abstracts from all content. This translation reveals that Hegel's dialectical categories are not empty forms but substantial determinations qualified by the three Gunas:

- **Sattva** (Being) - Pure luminous existence
- **Tamas** (Existence) - Determinate limited being  
- **Rajas** (Being-for-Self) - Dynamic self-relating unity

## Dialectical Structure

### I. Being (Sattva Guna) - THESIS
- **Character**: Pure luminous consciousness
- **Determination**: Immediate, undifferentiated presence
- **Contradiction**: Pure indeterminateness reveals itself as emptiness
- **Transition**: To Nothing/Existence (Tamas)

### II. Existence (Tamas Guna) - ANTITHESIS  
- **Character**: Determinate, limited, bounded being
- **Determination**: Being qualified by specific properties
- **Contradiction**: Seeks to overcome its limitations
- **Transition**: To Being-for-Self (Rajas)

### III. Being-for-Self (Rajas Guna) - SYNTHESIS
- **Character**: Dynamic, self-relating, combining principle
- **Determination**: Unity that relates to itself as both subject and object
- **Achievement**: Concrete universality through self-relation
- **Transition**: To Quantity (beyond quality)

## Key Features

### Qualitative vs Formal Logic
- **Formal Logic**: Abstract, contentless forms (A = A)
- **Qualitative Logic**: Guna-conditioned substantial determinations
- **Advance**: From empty tautologies to living, self-developing content

### Guna Integration
Each dialectical moment is qualified by its corresponding Guna:
- Categories have substantial qualitative content
- Transitions are driven by Guna contradictions
- Synthesis achieved through Rajas activity

### TypeScript Implementation
- Type-safe dialectical categories
- Guna-qualified interfaces and classes  
- Complete dialectical movement with method chains
- Demonstration system showing logical development

## File Structure

```
src/
├── types/quality.ts          # Core type definitions and Guna schema
├── being.ts                  # Being as Sattva Guna implementation
├── existence.ts              # Existence as Tamas Guna implementation  
├── being-for-self.ts         # Being-for-Self as Rajas Guna implementation
├── index.ts                  # Main system exports and integration
└── demo.ts                   # Complete demonstration of the system
```

## Usage

```typescript
import { demonstrateQualitativeLogic } from './demo';

// Run complete demonstration
demonstrateQualitativeLogic();

// Or use individual components
import { Being, Existence, BeingForSelf } from './index';

const being = new Being();
const existence = new Existence('red-thing', redQuality);
const beingForSelf = new BeingForSelf('unity', being, existence);

console.log(beingForSelf.getGunasSynthesis());
```

## Philosophical Significance

This implementation demonstrates that:

1. **Hegel's Logic has substantial content** (not just formal structures)
2. **Dialectical categories are Guna-qualified** (each has qualitative determination)
3. **Logical development follows Guna dynamics** (Sattva → Tamas → Rajas)
4. **Quality precedes Quantity** in logical development
5. **Self-relation achieves concrete universality** through Rajas synthesis

## Relationship to Source Materials

Based on translation of Hegel's *Science of Logic* sections on:
- Pure Being and Nothing
- Determinate Being and Quality  
- Being-for-Self and the One
- Qualitative vs Quantitative Logic

Integrated with Guna theory from classical Indian philosophy to provide substantial content for dialectical categories.

## Future Development

This Qualitative Logic system forms the foundation for:
- **Essence Logic** (Second Order Logic with reflection)
- **Concept Logic** (Higher Order Logic with self-determining universality)
- **Complete System** (Being → Essence → Concept as BEC framework)

The goal is a complete computational implementation of Hegel's entire *Logic* as a living, self-developing system of categories.

### Usage:
The example usage at the end creates instances of `Quality`, `Existence`, and `BeingForSelf`, and prints the description of `BeingForSelf`, demonstrating the relationships between these concepts. 

This structure can be expanded with additional methods and properties as needed to further explore the philosophical implications of these concepts.
