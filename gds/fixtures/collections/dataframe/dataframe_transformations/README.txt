DataFrame Transformations Fixture

Namespace: dataframe::frame (transformations workbook)

00 Source
artifact: fixtures/collections/dataframe/dataframe_transformations/00-source.csv
meaning: baseline wide table with duplicates and NaN quality values.

01 Order + Mutate
artifact: fixtures/collections/dataframe/dataframe_transformations/01-order-mutate.csv
meaning: sorted rows plus weighted/confidence derived columns.

02 Unpivot
artifact: fixtures/collections/dataframe/dataframe_transformations/02-unpivot.csv
meaning: wide metrics reshaped into long metric/value records.

03 Melt
artifact: fixtures/collections/dataframe/dataframe_transformations/03-melt.csv
meaning: alias path to unpivot with feature/score naming.

04 Null/NaN Normalization
artifact: fixtures/collections/dataframe/dataframe_transformations/04-cleaned.csv
meaning: NaN/null repair and required-field null dropping.

05 Distinct
artifact: fixtures/collections/dataframe/dataframe_transformations/05-distinct.csv
meaning: deduplicated domain/split/doc rows.

05 Summary
artifact: fixtures/collections/dataframe/dataframe_transformations/05-summary.csv
meaning: grouped means and row counts per domain/split.

06 Top-K
artifact: fixtures/collections/dataframe/dataframe_transformations/06-top-k.csv
meaning: highest weighted rows.

06 Bottom-K
artifact: fixtures/collections/dataframe/dataframe_transformations/06-bottom-k.csv
meaning: lowest weighted rows.

07 Dummies
artifact: fixtures/collections/dataframe/dataframe_transformations/07-dummies.csv
meaning: one-hot expansion of domain/split categories.

07 Sampled
artifact: fixtures/collections/dataframe/dataframe_transformations/07-sampled.csv
meaning: deterministic sample for quick iteration.

07 Gather Every
artifact: fixtures/collections/dataframe/dataframe_transformations/07-gather-every.csv
meaning: stride-based thinning of rows.

08 Pivot Boundary
artifact: fixtures/collections/dataframe/dataframe_transformations/08-pivot-boundary.txt
meaning: explicit wrapper gap contract for eager pivot.

09 Fill Strategy
artifact: fixtures/collections/dataframe/dataframe_transformations/09-fill-null-strategy.csv
meaning: strategy-path null fill surface documentation.
