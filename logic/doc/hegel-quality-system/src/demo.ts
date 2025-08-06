/**
 * DEMONSTRATION OF HEGEL'S QUALITATIVE LOGIC AS GUNA THEORY
 * ==========================================================
 *
 * Simple demonstration of the complete dialectical system
 * translating Hegel's Logic of Quality into Guna Theory
 */

import { Being } from './being';
import { Existence, QualitativeProperty } from './existence';
import { BeingForSelf } from './being-for-self';

/**
 * Complete demonstration of the Qualitative Logic system
 */
export function demonstrateQualitativeLogic(): void {
  console.log('='.repeat(70));
  console.log('HEGEL\'S QUALITATIVE LOGIC AS GUNA THEORY');
  console.log('Translation of Greater Logic Section on Quality');
  console.log('='.repeat(70));

  // I. BEING (SATTVA GUNA)
  console.log('\nI. BEING (SATTVA GUNA) - Pure Luminous Existence');
  console.log('-'.repeat(50));

  const being = new Being('pure-being-demo');
  console.log(`Name: ${being.name}`);
  console.log(`Guna: ${being.guna} (pure luminous consciousness)`);
  console.log(`Character: ${being.being_character}`);
  console.log(`Content: ${being.qualitative_content.guna_character}`);
  console.log(`\nDescription: ${being.description}`);

  console.log(`\nLogical Beginning: ${being.asLogicalBeginning()}`);
  console.log(`\nContradiction: ${being.getInternalContradiction()}`);
  console.log(`\nTransition: ${being.transitionToNothing()}`);

  // II. EXISTENCE (TAMAS GUNA)
  console.log('\n\nII. EXISTENCE (TAMAS GUNA) - Determinate Limited Being');
  console.log('-'.repeat(55));

  const redQuality = new QualitativeProperty('red', 'The quality of redness that limits and determines');
  const existence = new Existence('red-existence-demo', redQuality);

  console.log(`Name: ${existence.name}`);
  console.log(`Guna: ${existence.guna} (determinate limitation)`);
  console.log(`Character: ${existence.existence_character}`);
  console.log(`Content: ${existence.qualitative_content.guna_character}`);
  console.log(`\nDescription: ${existence.description}`);

  console.log(`\nDeterminateness: ${existence.getDeterminateness()}`);
  console.log(`\nQualitative Structure: ${existence.getQualitativeStructure()}`);
  console.log(`\nContradiction: ${existence.getInternalContradiction()}`);
  console.log(`\nTransition: ${existence.transitionToBeingForSelf()}`);

  // III. BEING-FOR-SELF (RAJAS GUNA)
  console.log('\n\nIII. BEING-FOR-SELF (RAJAS GUNA) - Dynamic Self-Relating Unity');
  console.log('-'.repeat(65));

  const beingForSelf = new BeingForSelf('unity-demo', being, existence);

  console.log(`Name: ${beingForSelf.name}`);
  console.log(`Guna: ${beingForSelf.guna} (dynamic combination)`);
  console.log(`Character: ${beingForSelf.unity_character}`);
  console.log(`Content: ${beingForSelf.qualitative_content.guna_character}`);
  console.log(`\nDescription: ${beingForSelf.description}`);

  console.log(`\nDialectical Unity: ${beingForSelf.getDialecticalUnity()}`);
  console.log(`\nSelf-Relation: ${beingForSelf.getSelfRelationalStructure()}`);
  console.log(`\nTranscendence: ${beingForSelf.transcendenceOfExistence()}`);
  console.log(`\nInfinity: ${beingForSelf.getInfinityStructure()}`);
  console.log(`\nGuna Synthesis: ${beingForSelf.getGunasSynthesis()}`);
  console.log(`\nTransition to Quantity: ${beingForSelf.transitionToQuantity()}`);

  // COMPLETE GUNA ANALYSIS
  console.log('\n\n' + '='.repeat(70));
  console.log('COMPLETE GUNA ANALYSIS OF QUALITATIVE LOGIC');
  console.log('='.repeat(70));

  console.log(`
SATTVA (BEING):
- Pure, luminous, unlimited consciousness
- Immediate presence without determination
- Contains all possibilities in pure form
- Dialectical moment: THESIS

TAMAS (EXISTENCE):
- Determinate, limited, bounded being
- Mediated through quality and negation
- Actualizes specific possibilities through limitation
- Dialectical moment: ANTITHESIS

RAJAS (BEING-FOR-SELF):
- Dynamic, combining, self-relating principle
- Unifies Sattva and Tamas in active process
- Creates infinite self-development through self-relation
- Dialectical moment: SYNTHESIS

QUALITATIVE LOGIC vs FORMAL LOGIC:
- Formal Logic: Abstracts from all content (empty forms)
- Qualitative Logic: Conditioned by Guna content (substantial determinations)
- Formal: Identity as A = A (tautology)
- Qualitative: Identity as Being = Being through Sattva content
- Formal: Contradiction as external exclusion
- Qualitative: Contradiction as internal self-development

RESULT: Hegel's Logic of Quality = Theory of Three Gunas
  `);

  console.log('='.repeat(70));
  console.log('DEMONSTRATION COMPLETE');
  console.log('Qualitative Logic successfully translated as Guna Theory');
  console.log('='.repeat(70));
}

// Run the demonstration
if (require.main === module) {
  demonstrateQualitativeLogic();
}
