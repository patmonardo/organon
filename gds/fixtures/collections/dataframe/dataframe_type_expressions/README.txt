DataFrame Type Expressions Fixture

Namespace: dataframe::datatype_expr

tree dtype: Struct({'label': String, 'children': List(Struct({'label': String, 'span_start': UInt32, 'span_end': UInt32})), 'span_start': UInt32, 'span_end': UInt32})
label dtype: String
child label dtype: String
token inner dtype: Int64
embedding inner dtype: Float32
signed user id dtype: Int64
tree contract dtype: Struct({'label': String, 'children': List(Struct({'label': String, 'children': List(Struct({'label': String, 'span_start': UInt32, 'span_end': UInt32})), 'span_start': UInt32, 'span_end': UInt32})), 'span_start': UInt32, 'span_end': UInt32})

This fixture exercises collect_dtype over nested Struct/List/Array storage.
