use tree_sitter::{Node, Tree};

use crate::editor::Editor;
use crate::id::NodeId;

/// An [Editor] that deletes the text of a single [Node].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Delete {
    id: NodeId,
}

impl Delete {
    #[must_use]
    pub fn new(id: NodeId) -> Self {
        Delete { id }
    }
}

impl Editor for Delete {
    #[must_use]
    fn has_edit(&self, _tree: &Tree, node: &Node) -> bool {
        self.id.is(node)
    }

    fn edit(&self, _source: &[u8], tree: &Tree, node: &Node) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        Vec::new()
    }
}
