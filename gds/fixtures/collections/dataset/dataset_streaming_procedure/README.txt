Dataset Streaming Procedure Fixture

Namespace: dataset::streaming

00 Source
artifact: fixtures/collections/dataset/dataset_streaming_procedure/00-source.csv
meaning: 10-row named Dataset used as streaming source.

01 Iteration
artifact: fixtures/collections/dataset/dataset_streaming_procedure/01-iteration.txt
meaning: StreamingDataset with batch_size=3, full row coverage.

02 Plan Streaming
artifact: fixtures/collections/dataset/dataset_streaming_procedure/02-plan-streaming.txt
meaning: with_plan drives batch size and Select transform per batch.
