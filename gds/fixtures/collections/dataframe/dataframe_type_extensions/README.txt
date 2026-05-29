DataFrame Type Extensions Fixture

Namespace: dataframe::datatypes::extension + dataframe::expressions::ext

registered gds.tree: Class("TreeClass")
extension name: gds.tree
extension storage: Struct({'label': String, 'children': List(Struct({'label': String, 'span_start': UInt32, 'span_end': UInt32})), 'span_start': UInt32, 'span_end': UInt32})
extension metadata: Some("class=Tree;version=seed")
storage contract: Struct({'label': String, 'children': List(Struct({'label': String, 'children': List(Struct({'label': String, 'span_start': UInt32, 'span_end': UInt32})), 'span_start': UInt32, 'span_end': UInt32})), 'span_start': UInt32, 'span_end': UInt32})

Current gap: this local registry does not yet materialize a Polars DataType::Extension.
The expression namespace currently lowers to storage-level cast/identity behavior.
