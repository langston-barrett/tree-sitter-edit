use tree_sitter::{Node, Tree};

use crate::editor::Editor;
use crate::id::NodeId;

/// An [Editor] that replaces the text of a single [Node].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Replace {
    pub id: NodeId,
    pub bytes: Vec<u8>,
}

impl Editor for Replace {
    fn has_edit(&self, _tree: &Tree, node: &Node) -> bool {
        self.id.is(node)
    }

    fn edit(&self, _source: &[u8], tree: &Tree, node: &Node) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        self.bytes.clone()
    }
}
