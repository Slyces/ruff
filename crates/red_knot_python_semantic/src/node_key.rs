use ruff_python_ast::{AnyNodeRef, NodeKind};
use ruff_text_size::{Ranged, TextRange};

use serde::Serialize;

/// Compact key for a node for use in a hash map.
///
/// Compares two nodes by their kind and text range.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub(super) struct NodeKey {
    kind: NodeKind,
    #[serde(skip)]
    range: TextRange,
}

impl NodeKey {
    pub(super) fn from_node<'a, N>(node: N) -> Self
    where
        N: Into<AnyNodeRef<'a>>,
    {
        let node = node.into();
        NodeKey {
            kind: node.kind(),
            range: node.range(),
        }
    }
}
