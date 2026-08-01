#![allow(clippy::module_inception)]

mod batch_node_iterable;
mod filtered_id_map;
mod ids;
mod id_map;
mod node_iterator;
mod partial_id_map;
mod simple;

pub use batch_node_iterable::{BatchNodeIterable, NodeIdBatch, NodeIdBatchIter};
pub use filtered_id_map::FilteredIdMap;
pub use id_map::{IdMap, NodeLabelConsumer, NO_TYPE, START_NODE_ID};
pub use ids::{MappedNodeId, OriginalNodeId, RelationshipIndex};
pub use node_iterator::{NodeConsumer, NodeIdIterator, NodeIterator, NodeIteratorExt};
pub use partial_id_map::{EmptyPartialIdMap, PartialIdMap};
pub use simple::{SimpleIdMap, SimpleIdMapError};

pub use crate::task::concurrency::Concurrency;

/// Property value type (matches Java GDS Long)
pub type PropertyValue = i64;

/// Algorithm weight type (matches Java GDS Long)
pub type Weight = i64;

/// Count and size type (matches Java GDS Long)
pub type Count = i64;

/// Floating point value type (only when you need actual floating point)
pub type FloatValue = f64;
