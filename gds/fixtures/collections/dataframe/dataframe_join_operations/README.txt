DataFrame Join Operations Fixture

Namespace: dataframe::frame (join / join_on)

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

04 join_on
artifact: fixtures/collections/dataframe/dataframe_join_operations/04-join-on.csv
meaning: Convenience same-key join; products LEFT JOIN inventory on product_id.
