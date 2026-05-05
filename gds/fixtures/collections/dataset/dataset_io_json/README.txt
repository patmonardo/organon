Dataset IO JSON Fixture

Namespace: dataset::io + dataset::namespaces::dataset

00 Source JSON
artifacts: fixtures/collections/dataset/dataset_io_json/00-source.json, fixtures/collections/dataset/dataset_io_json/00-source.ndjson
meaning: Same table written in JSON and NDJSON formats.

01 Detect
artifact: fixtures/collections/dataset/dataset_io_json/01-detect.txt
meaning: detect_format_from_path results for json/ndjson/csv.

02 Loaded
artifact: fixtures/collections/dataset/dataset_io_json/02-loaded.csv
meaning: Dataset::from_json round-trip into tabular view.

03 Namespace IO
artifact: fixtures/collections/dataset/dataset_io_json/03-namespace-io.txt
meaning: io_path/io_url constructors for Dataset IO expressions.
