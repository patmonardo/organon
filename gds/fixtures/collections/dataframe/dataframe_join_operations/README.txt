DataFrame Join Operations Fixture

Namespace: dataframe::frame (join macros)

00 Source Tables
artifacts: fixtures/collections/dataframe/dataframe_join_operations/00-products.csv, fixtures/collections/dataframe/dataframe_join_operations/00-orders.csv
meaning: Products (4 rows) and Orders (5 rows) tables.

01 Inner Join
artifact: fixtures/collections/dataframe/dataframe_join_operations/01-inner.csv
meaning: Only orders with a matching product survive; product_id=5 excluded.

02 Left Join
artifact: fixtures/collections/dataframe/dataframe_join_operations/02-left.csv
meaning: All orders retained; product cols null for unknown product_id=5.

03 Full Outer Join
artifact: fixtures/collections/dataframe/dataframe_join_operations/03-full.csv
meaning: All rows from both sides; nulls fill missing keys.

04 Semi/Anti
semi: fixtures/collections/dataframe/dataframe_join_operations/04-semi.csv
anti: fixtures/collections/dataframe/dataframe_join_operations/04-anti.csv
meaning: existence filtering joins on the left frame.

05 Explicit Keys
artifact: fixtures/collections/dataframe/dataframe_join_operations/05-explicit-keys.csv
meaning: Left join across different key names via join! (product_id -> sku_id).
