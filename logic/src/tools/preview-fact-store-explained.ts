import { compileFactStorePrototypeBundle } from '@relative/core/compiler/fact-store-prototype';

function bullet(text: string): string {
  return `- ${text}`;
}

async function main(): Promise<void> {
  const bundle = await compileFactStorePrototypeBundle();
  const context = bundle.contextShape;
  const morph = bundle.morphShape;
  const factStore = bundle.factStore;

  const resolvedCount = context.determinations.filter((d) => d.resolved).length;
  const totalCount = context.determinations.length;

  const lines: string[] = [
    'FactStore Operational Preview',
    '',
    '1) What the FactStore must do',
    bullet(
      'Ingest compiled FormShape state from Reflection (currently ref-14).',
    ),
    bullet(
      'Carry Context determinations (identity, difference, contradiction) with explicit source states.',
    ),
    bullet(
      'Apply Morph conditioned-genesis actions to marked subgraphs before facticity is asserted.',
    ),
    bullet(
      'Emit a concrete facticity judgment only when all conditions are at hand.',
    ),
    '',
    '2) How this prototype currently works',
    bullet(
      `Form source: ${factStore.formShape.id} (${factStore.formShape.refStateId}).`,
    ),
    bullet(
      `Context source: ${context.id}; determinations resolved ${resolvedCount}/${totalCount}.`,
    ),
    bullet(
      `Morph source: ${morph.id}; condition state ${morph.conditionStateId}.`,
    ),
    bullet(`Facticity state: ${factStore.facticity.state}.`),
    bullet(`Last transition: ${factStore.facticity.lastTransition}.`),
    '',
    '3) Runtime responsibilities of FactStore',
    bullet(
      'Preserve provenance: every determination and transition remains traceable to its IR source state.',
    ),
    bullet(
      'Store contradiction policies as actionable subgraph rules (revise, attach, sublate).',
    ),
    bullet(
      'Reject concrete assertions when conditionedGenesis flags are false.',
    ),
    bullet('Expose evidence chain for debugging and scientific audit.'),
    '',
    '4) Why this matters for the triad',
    bullet('Form contributes reflective structure.'),
    bullet('Context contributes determination logic.'),
    bullet('Morph contributes conditioned existence logic.'),
    bullet(
      'FactStore is the synthesized operational unity of those three moments.',
    ),
    '',
    '5) Current prototype snapshot (JSON)',
    JSON.stringify(bundle, null, 2),
  ];

  console.log(lines.join('\n'));
}

main().catch((error) => {
  console.error('Failed to render FactStore explanation preview:', error);
  process.exit(1);
});
