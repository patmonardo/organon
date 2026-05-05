DataFrame Order and Group Fixture

Namespace: dataframe::frame (sort / group_by)

00 Source
artifact: fixtures/collections/dataframe/dataframe_order_group/00-source.csv
meaning: 6-row sales table (region, product, units, price).

01 Sorted
artifact: fixtures/collections/dataframe/dataframe_order_group/01-sorted.csv
meaning: Multi-column sort: region ASC, units DESC.

02 Count
artifact: fixtures/collections/dataframe/dataframe_order_group/02-count.csv
meaning: Row count per region (group_by_count).

03 Group Expr
artifact: fixtures/collections/dataframe/dataframe_order_group/03-group-expr.csv
meaning: Mean price and total units per product via expression API.

04 Top-K
artifact: fixtures/collections/dataframe/dataframe_order_group/04-top-k.csv
meaning: Top 3 rows by units using top_k.
