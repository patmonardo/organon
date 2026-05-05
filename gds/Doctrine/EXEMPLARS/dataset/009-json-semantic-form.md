# Collections Dataset: JSON Semantic Form

File: `gds/examples/dataset_io_json.rs`

## Principle

This exemplar teaches that structured data enters Dataset as semantic form, not opaque bytes. JSON is already a syntax tree. The Dataset layer tokenizes it, parses it, and exposes the resulting tree as inspectable structure that can later be marked, reflected, and compiled.

## What It Does

1. Defines a compact JSON document
2. Tokenizes the source text with `JsonTokenizer`
3. Parses tokens with `JsonParser`
4. Reads the first parse from the forest
5. Prints the bracketed tree representation

The example is intentionally small so the reader can see the full transformation from text to semantic form.

## The Arc

**Stage 2: Observation.**

JSON parsing is Observation in the strict sense: the source becomes structured appearance. Nothing has yet passed through Principle or Concept. The program has only learned what shape the given data presents.

## Key Vocabulary

- **Tokenizer**: turns source text into a stream of typed tokens. See [structured-sources.md](../../REFERENCES/dataset/structured-sources.md)
- **Parser**: turns tokens into parse trees or forests. See [structured-sources.md](../../REFERENCES/dataset/structured-sources.md)
- **Parse forest**: a collection of possible parsed semantic forms. See [tree-structures.md](../../REFERENCES/dataset/tree-structures.md)

## Next Exemplar

**Next**: `dataset_io_xml_html.rs`

JSON shows strict structured data. XML/HTML shows markup: a different surface, but the same doctrine. Source becomes observed structure before it becomes knowledge.

## Notes for Students

**Watch for**: The printed bracketed tree is not decorative. It is the first visible semantic form of the source.

**Key insight**: Parsing is a doctrine stage. It belongs to Observation because it determines what can later be marked and reflected.
