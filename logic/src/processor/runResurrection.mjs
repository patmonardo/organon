#!/usr/bin/env node

/**
 * RUNNER SCRIPT FOR QUANTITATIVE LOGIC RESURRECTION
 * =================================================
 * Execute this to witness the Resurrection of the Arithmetical System
 */

// Import the demonstration
import { runQuantitativeLogicResurrection } from './QuantitativeLogicDemo.js';

// Execute the complete resurrection
async function main() {
  try {
    console.log('🚀 Starting Quantitative Logic Resurrection...\n');
    await runQuantitativeLogicResurrection();
    console.log('\n🎉 Resurrection complete! The System lives! 🎉');
  } catch (error) {
    console.error('❌ Error during resurrection:', error);
    process.exit(1);
  }
}

// Run it
main();
