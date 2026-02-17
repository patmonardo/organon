# Pass 1 Acceptance — Integrated Reflection Graph

Date: 2026-02-17
Status: ACCEPTED
Operator: pat@logos

## Terminology Correction (Hegelian)

- This implementation is Hegelian (Logic of Essence / Reflection), not Kantian.
- Earlier “Kantian” wording was a temporary scaffold and is treated as superseded terminology.
- Layer IDs are retained for continuity, but semantic interpretation is: Principle -> Law (determinate negation) -> Science of Reflection.

## Verified Runtime Metrics

- Nodes: 947
- Relationships: 1596

## Layer Cardinalities (accepted)

- `principle-citta` (`PRINCIPLE`): 3
- `law-consciousness` (`LAW`): 7
- `science-reflection` (`SCIENCE`): 65

## Source Chunk Counts (accepted)

- `source-identity`: 6
- `source-difference`: 15
- `source-contradiction`: 10
- `source-essence`: 12
- `source-reflection`: 19
- `source-shine`: 10
- `source-determinate-ground`: 14
- `source-absolute-ground`: 16
- `source-condition`: 16

Total chunks: 118

## Provenance Integrity (accepted)

`TXT -> CHUNK_SEGMENT -> TOPIC -> INTEGRATED_CHUNK`

- Chain counts match source chunk counts for all 9 sources.
- No missing source in provenance chain checks.

## Cross-Source Transition Set (accepted)

1. `idn-6 --LAYER_NEGATION--> diff-2`
2. `diff-15 --SUBLATES--> ctr-1`
3. `ctr-10 --SUBLATES--> ref-2`
4. `ref-15 --NEXT--> det-1`
5. `det-14 --SPIRALS_TO--> abs-1`
6. `abs-16 --SPIRALS_TO--> con-1`

## Acceptance Invariants for Pass 2+

- Layer cardinalities remain at 3 / 7 / 65 unless intentionally re-specified.
- Source chunk totals remain 118 unless source/topic maps change.
- Provenance chain counts must equal source chunk counts.
- Cross-source transition set must include all six accepted transitions above.

## Baseline Scripts Used

- `run-all-loader.sh`
- `run-post-checks.sh`
