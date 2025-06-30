use tree_sitter::{Node, Tree};
use tree_sitter_traversal::{traverse, Order};

use crate::editor::{Edit, Editor};
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
    fn has_edit(&self, _tree: &Tree, node: &Node<'_>) -> bool {
        self.id.is(node)
    }

    fn edit(&self, _source: &[u8], tree: &Tree, node: &Node<'_>) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        Vec::new()
    }

    fn in_order_edits(&self, _source: &[u8], tree: &Tree) -> Box<dyn Iterator<Item = Edit> + '_> {
        if let Some(node) = traverse(tree.walk(), Order::Pre).find(|n| NodeId::new(n) == self.id) {
            Box::new(std::iter::once(Edit {
                position: node.start_byte(),
                delete: node.end_byte() - node.start_byte(),
                insert: Vec::new(),
            }))
        } else {
            Box::new(std::iter::empty())
        }
    }
}
