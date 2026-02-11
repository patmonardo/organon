# Dataset Resource Registry + Corpus Readers

This note sketches a minimal, resource-first registry and a corpus-reader trait that fits our Dataset SDK.

## Goals

- Keep resources as first-class: URLs + checksums + optional metadata.
- Make corpus readers a thin, resource-backed view with fileid selection and lazy reading.
- Avoid re-creating DataFrame utilities; let Polars handle table ops.
- Keep writers in app space unless they are generic, format-level utilities.
- Emphasize registry now, keep catalog integration in view.

## Registry Schema (Resource-First)

A resource is a named dataset payload, typically a zip archive and a directory of files after extraction.

```
Resource {
  name: str
  urls: [str]
  sha256: str
  archive_format: "zip" | "tar" | "none" | "auto"
  layout: "archive-root" | "nested-dir" | "custom"
  default_globs: [str] // optional, for table discovery
  metadata: {
    license: str?
    citation: str?
    description: str?
    homepage: str?
    tags: [str]?
    features: [str]? // e.g. "text", "table", "graph"
  }
}
```

Notes:

- `urls` aligns with Skrub and allows multiple mirrors.
- `sha256` is optional but preferred for integrity.
- `default_globs` is used by catalog discovery; if absent, fall back to extension scanning.

## Corpus Reader Trait (Resource-Backed)

A corpus reader is a view over a resource, with file selection and optional tokenization.

```
CorpusReader {
  resource: ResourceRef
  fileids(): [str]
  open(fileid): Read
  raw(fileids?): String
  abspaths(fileids?): [Path]
  encoding(fileid): String?
}
```

Optional higher-level views (implemented by concrete readers):

- `words(fileids?) -> [str]`
- `sents(fileids?) -> [[str]]`
- `paras(fileids?) -> [[[str]]]`
- `records(fileids?) -> DataFrame` (table-backed corpora only)

Key simplifications vs NLTK:

- No global `nltk.data.path` equivalent; only the registry + data home.
- No dynamic class mutation; use explicit lazy wrappers where needed.
- Streaming views should be a separate helper (e.g., `CorpusStreamView`) instead of magic in the reader.

## Module Placement

Recommended split:

- `gds/src/collections/dataset/stdlib/resources.rs`
  - Resource registry + fetch/unzip + checksum

- `gds/src/collections/dataset/stdlib/corpus/*`
  - Generic corpus interfaces + simple readers
  - Examples: plaintext, wordlist, table-backed

- `gds/src/collections/dataset/corpus/*` (core)
  - Only if we need the traits in core without stdlib. Otherwise keep all corpus readers under stdlib.

## Registry vs Catalog (Master Catalog Path)

Registry is the authoritative list of known resources and their metadata. It is stable, declarative, and can be merged into a top-level "master catalog" later.

Catalog is an operational index for concrete artifacts (paths, formats, schemas) and can change as data is ingested or rewritten. It is closer to a database system catalog and should be layered on top of registry rather than replacing it.

Short-term focus:

- Use registry as the primary API for datasets and corpora.
- Treat catalog as optional, used only when a resource has been materialized into tables.

Long-term direction:

- Provide a top-level master catalog view that can union registry entries, catalog tables, and other cataloged items (graphs, corpora, models), similar to a database metadata schema.
- Keep registry and catalog independent so we can ignore catalog during early corpus work.

## Writers Boundary

Corpus writers are app-level by default. If we add any:

- Keep them format-level and stateless (e.g., write wordlist or plain text).
- Avoid coupling to resource registry logic.

## Decision Checklist

- Do we want corpus readers always resource-backed?
- Do we need fileid globbing in registry metadata or in the reader?
- Which minimal readers do we want first (plaintext, wordlist, table)?
- When do we introduce a master catalog view over registry + catalog?
