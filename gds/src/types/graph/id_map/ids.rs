use std::fmt;

use serde::Deserialize;
use serde::Serialize;

macro_rules! unsigned_graph_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(
            Debug,
            Default,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(u64);

        impl $name {
            pub const ZERO: Self = Self(0);

            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            pub const fn get(self) -> u64 {
                self.0
            }

            pub fn checked_add(self, amount: usize) -> Option<Self> {
                let amount = u64::try_from(amount).ok()?;
                self.0.checked_add(amount).map(Self)
            }

            pub fn to_usize(self) -> Option<usize> {
                usize::try_from(self.0).ok()
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<usize> for $name {
            type Error = std::num::TryFromIntError;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                u64::try_from(value).map(Self)
            }
        }

        impl TryFrom<i64> for $name {
            type Error = std::num::TryFromIntError;

            fn try_from(value: i64) -> Result<Self, Self::Error> {
                u64::try_from(value).map(Self)
            }
        }

        impl TryFrom<$name> for usize {
            type Error = std::num::TryFromIntError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                usize::try_from(value.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

/// External or database identity of a node.
#[repr(transparent)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct OriginalNodeId(i64);

impl OriginalNodeId {
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> i64 {
        self.0
    }
}

impl From<i64> for OriginalNodeId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<OriginalNodeId> for i64 {
    fn from(value: OriginalNodeId) -> Self {
        value.0
    }
}

impl fmt::Display for OriginalNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

unsigned_graph_id!(MappedNodeId);
unsigned_graph_id!(RelationshipIndex);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_ids_preserve_their_domains() {
        assert_eq!(OriginalNodeId::new(-7).get(), -7);
        assert_eq!(MappedNodeId::new(7).get(), 7);
        assert_eq!(RelationshipIndex::new(7).get(), 7);
    }

    #[test]
    fn mapped_ids_advance_without_primitive_arithmetic() {
        assert_eq!(MappedNodeId::new(4).checked_add(3), Some(MappedNodeId::new(7)));
        assert_eq!(MappedNodeId::new(u64::MAX).checked_add(1), None);
    }
}
