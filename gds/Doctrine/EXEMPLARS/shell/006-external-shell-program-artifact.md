# Shell: External Program Artifact

File: `gds/examples/dataset_compile_external_gdsl.rs`

## Principle

This exemplar teaches the first complete external Shell program artifact. A scientific knowledge program is read from source, lowered into `ProgramFeatures`, compiled into a Dataset image, and materialized as artifact, relation, and property tables. This is where the arc stops being commentary and becomes executable doctrine.

## What It Does

1. Reads `absolute-concept-scientific-inference.gdsl`
2. Lowers external `.gdsl` source into `ProgramFeatures`
3. Compiles those features into a `DatasetCompilation`
4. Materializes artifact, relation, and property datasets
5. Prints selected forms, feature count, entrypoints, and sample features

The example does not yet execute a full scientific object pipeline. It makes the program image inspectable. That is the first victory.

## The Arc

**Whole-arc specimen**: Source -> Observation -> Reflection -> Principle -> Concept -> Judgment -> Syllogism -> Procedure.

The fixture contains the whole order:

- `source sensation perception_stream`
- `appearance empirical_appearance`
- `reflection essence_path`
- `logogenesis scientific_genesis`
- `mark quality`, `mark measure`, `mark ground`
- `concept AppearanceObject`
- `judgment determinate_appearance`
- `syllogism scientific_inference`
- `principle absolute_concept`
- `query scientific_objects`
- `procedure compile_absolute_concept`

The lowering now preserves the doctrinal Program Feature taxonomy directly: Source, Observation, Reflection, Logogenesis, Principle, Concept, Judgment, Syllogism, Query, Procedure, and their body-level marks and inferences survive as typed features.

## Key Vocabulary

- **External `.gdsl` artifact**: the persisted Shell program syntax for scientific knowledge intent. See [grammar.md](../../REFERENCES/shell/grammar.md)
- **ProgramFeatures**: the Shell-readable compiler contract extracted from internal RustScript or external `.gdsl` source. See [program-features.md](../../REFERENCES/shell/program-features.md)
- **Principle**: the nomological gate before Concept. See [principle-evaluation.md](../../REFERENCES/form/principle-evaluation.md)
- **Procedure**: entry into Objectivity as Process. See [hegel-objectivity.md](../../REFERENCES/philosophy/hegel-objectivity.md)

## Next Exemplar

**Next**: `form_applications_expository.rs`

The Shell program image must become serviceable. The next exemplar shows the Applications facade serving catalog, schema, preview, feature evaluation, attention reporting, external artifact compilation, and materialization without forcing the user into kernel internals.

## Notes for Students

**Watch for**: The printed feature list is not merely debug output. It is a visible treaty between Shell-readable program source and Dataset artifact materialization.

**Key insight**: Program images are doctrine-bearing objects. They carry the story of what the program believes scientific knowing requires.

**Try this**: Add a new `mark` or `infer` line to the fixture, then rerun the example. The materialized artifact counts should change. That is the compiler surface becoming visible.
