# NLP DSL Parity Map

This document tracks the DSL surface for Token, Stem, Parse, FeatStruct, and Tag.
It is focused on data shapes and accessors only. Algorithms live in NLP Algo modules.

## Token

- DSL: `Token`, `TokenSpan`, `TokenKind`, `to_struct_expr()`
- Expressions: `token_*_expr`
- Notes: tokenization algorithms are out of scope here

## Stem

- DSL: `Stem`, `StemKind`, `to_struct_expr()`
- Expressions: `stem_*_expr`
- Notes: stemming and lemmatization algorithms are out of scope here

## Parse

- DSL: `Parse`, `ParseKind`, `ParseForest`, `to_struct_expr()`
- Expressions: `parse_*_expr`
- Notes: parsers, grammars, and scoring algorithms are out of scope here

## FeatStruct

- DSL: `FeatStruct`, `FeatValue`, `FeatPath`, `FeatReentranceId`, `FeatStructSet`
- Helpers: `format_featstruct`, `parse_featstruct` (round-trips the DSL format)
- Notes: unification and full NLTK parser remain in NLP Algo modules

## Tag

- DSL: `Tag`, `to_struct_expr()`
- Expressions: `tag_*_expr`
- Notes: tagging algorithms are out of scope here

## Tree Integration

- Tree helpers: `align_tags_by_index`, `align_tags_by_span`
- Notes: alignment is leaf-order based and does not perform retokenization
