use crate::traversal::{traverse, Order};
use tree_sitter::{Node, Tree};

use crate::editor::{Edit, Editor};
use crate::id::NodeId;

/// An [Editor] that replaces the text of a single [Node].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Replace {
    pub id: NodeId,
    pub bytes: Vec<u8>,
}

impl Editor for Replace {
    fn has_edit(&self, _tree: &Tree, node: &Node<'_>) -> bool {
        self.id.is(node)
    }

    fn edit(&self, _source: &[u8], tree: &Tree, node: &Node<'_>) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        self.bytes.clone()
    }

    fn in_order_edits(&self, _source: &[u8], tree: &Tree) -> Box<dyn Iterator<Item = Edit>> {
        if let Some(node) = traverse(tree.walk(), Order::Pre).find(|n| NodeId::new(n) == self.id) {
            Box::new(std::iter::once(Edit {
                position: node.start_byte(),
                delete: node.end_byte() - node.start_byte(),
                insert: self.bytes.clone(),
            }))
        } else {
            Box::new(std::iter::empty())
        }
    }
}
