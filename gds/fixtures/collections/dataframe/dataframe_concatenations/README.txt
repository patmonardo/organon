DataFrame Concatenations Fixture

Namespace: dataframe::concatenations

00 Source Batches
jan: fixtures/collections/dataframe/dataframe_concatenations/00-jan.csv
feb: fixtures/collections/dataframe/dataframe_concatenations/00-feb.csv
mar: fixtures/collections/dataframe/dataframe_concatenations/00-mar.csv
meaning: monthly partitions with one stable schema.

01 Concatenated Rows
artifact: fixtures/collections/dataframe/dataframe_concatenations/01-all-sales.csv
meaning: concat_rows! stacks all batch rows in one frame.

02 Enriched/Ordered
artifact: fixtures/collections/dataframe/dataframe_concatenations/02-ordered.csv
meaning: mutate! + concat_str helper create route labels.

03 Route Rollup
artifact: fixtures/collections/dataframe/dataframe_concatenations/03-rollup.csv
meaning: group_by! with agg! summarizes totals after concatenation.
