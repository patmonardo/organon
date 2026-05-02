# Collections Dataset: XML and HTML Semantic Form

File: `gds/examples/collections_dataset_xml_html.rs`

## Principle

This exemplar teaches markup as semantic form. XML and HTML are not only document formats; they are explicit tree surfaces. The Dataset parser lowers markup into a tree so document structure can become available for later semantic marking.

## What It Does

1. Defines a compact markup document
2. Tokenizes it with `MarkupTokenizer`
3. Parses tokens with `MarkupParser`
4. Reads the first parse from the forest
5. Prints the bracketed tree representation

The example parallels the JSON exemplar. The point is not the format; the point is that structured source material becomes observed form.

## The Arc

**Stage 2: Observation.**

Markup parsing belongs to Observation. It prepares the document structure for later Reflection. The tags, attributes, and text content are not yet concepts. They are appearance made explicit.

## Key Vocabulary

- **MarkupTokenizer**: turns markup text into token stream. See [structured-sources.md](../REFERENCES/collections-dataset/structured-sources.md)
- **MarkupParser**: builds semantic tree form from markup tokens. See [structured-sources.md](../REFERENCES/collections-dataset/structured-sources.md)
- **Document tree**: an observed structure that can later carry marks. See [tree-structures.md](../REFERENCES/collections-dataset/tree-structures.md)

## Next Exemplar

**Next**: Reflection and Principle exemplars.

The structured-source exemplars have completed the Observation support layer. The next curriculum movement should add examples that explicitly realize seven-moment Reflection, Principle evaluation, and Concept emergence.

## Notes for Students

**Watch for**: The markup parser treats tags and attributes as structure, not as strings to be searched. That is the difference between text processing and semantic observation.

**Key insight**: Observation is not passive. It is the active production of form from source.
