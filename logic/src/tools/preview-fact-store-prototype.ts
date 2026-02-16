import { compileFactStorePrototypeBundle } from '@relative/core/compiler/fact-store-prototype';

async function main(): Promise<void> {
  const bundle = await compileFactStorePrototypeBundle();
  console.log(JSON.stringify(bundle, null, 2));
}

main().catch((error) => {
  console.error('Failed to compile FactStore prototype bundle:', error);
  process.exit(1);
});
