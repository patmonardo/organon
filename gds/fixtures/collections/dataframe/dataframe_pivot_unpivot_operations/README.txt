DataFrame Pivot/Unpivot Operations Fixture

Namespace: dataframe::reshape

00 Source
artifact: fixtures/collections/dataframe/dataframe_pivot_unpivot_operations/00-source.csv
meaning: baseline wide table for reshape operations.

01 Unpivot
artifact: fixtures/collections/dataframe/dataframe_pivot_unpivot_operations/01-unpivot.csv
meaning: unpivot! converts qty/revenue into long metric/value rows.

02 Melt
artifact: fixtures/collections/dataframe/dataframe_pivot_unpivot_operations/02-melt.csv
meaning: melt! alias path with id/value terminology.

03 Long Summary
artifact: fixtures/collections/dataframe/dataframe_pivot_unpivot_operations/03-summary.csv
meaning: group_by! + agg! summarize long-form metric totals.

04 Pivot Boundary
artifact: fixtures/collections/dataframe/dataframe_pivot_unpivot_operations/04-pivot-boundary.txt
meaning: explicit eager pivot boundary note in wrapper surface.
