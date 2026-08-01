use std::error::Error;
use std::fmt;

use crate::types::graph::MappedNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeCursorError {
    start: MappedNodeId,
    length: usize,
    node_count: usize,
}

impl fmt::Display for NodeCursorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "node range starting at {} with length {} exceeds node count {}",
            self.start, self.length, self.node_count
        )
    }
}

impl Error for NodeCursorError {}

/// Reusable cursor over a contiguous range of mapped node identities.
pub trait NodeCursor: fmt::Debug {
    fn reset(&mut self, start: MappedNodeId, length: usize) -> Result<(), NodeCursorError>;
    fn size(&self) -> usize;
    fn remaining(&self) -> usize;
    fn next_node(&mut self) -> Option<MappedNodeId>;
}

#[derive(Debug, Clone)]
pub struct MappedNodeRangeCursor {
    node_count: usize,
    next: MappedNodeId,
    size: usize,
    remaining: usize,
}

impl MappedNodeRangeCursor {
    pub fn new(node_count: usize) -> Self {
        Self {
            node_count,
            next: MappedNodeId::ZERO,
            size: 0,
            remaining: 0,
        }
    }
}

impl NodeCursor for MappedNodeRangeCursor {
    fn reset(&mut self, start: MappedNodeId, length: usize) -> Result<(), NodeCursorError> {
        let end = start
            .to_usize()
            .and_then(|start| start.checked_add(length));
        if end.is_none_or(|end| end > self.node_count) {
            return Err(NodeCursorError {
                start,
                length,
                node_count: self.node_count,
            });
        }
        self.next = start;
        self.size = length;
        self.remaining = length;
        Ok(())
    }

    fn size(&self) -> usize {
        self.size
    }

    fn remaining(&self) -> usize {
        self.remaining
    }

    fn next_node(&mut self) -> Option<MappedNodeId> {
        if self.remaining == 0 {
            return None;
        }
        let node = self.next;
        self.remaining -= 1;
        if self.remaining > 0 {
            self.next = self.next.checked_add(1)?;
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_can_be_reset_and_reused() {
        let mut cursor = MappedNodeRangeCursor::new(5);
        cursor.reset(MappedNodeId::new(1), 3).unwrap();
        assert_eq!(
            std::iter::from_fn(|| cursor.next_node()).collect::<Vec<_>>(),
            vec![
                MappedNodeId::new(1),
                MappedNodeId::new(2),
                MappedNodeId::new(3),
            ]
        );

        cursor.reset(MappedNodeId::new(4), 1).unwrap();
        assert_eq!(cursor.next_node(), Some(MappedNodeId::new(4)));
        assert_eq!(cursor.next_node(), None);
    }

    #[test]
    fn cursor_rejects_ranges_outside_mapped_space() {
        let mut cursor = MappedNodeRangeCursor::new(2);
        assert!(cursor.reset(MappedNodeId::new(1), 2).is_err());
    }
}
