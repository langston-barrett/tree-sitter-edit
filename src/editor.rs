use crate::traversal::{traverse, Order};
use tree_sitter::{Node, Tree};

#[derive(Debug)]
pub struct Edit {
    pub position: usize,
    pub delete: usize,
    pub insert: Vec<u8>,
}

// TODO(lb): Perhaps make an associated error type so that these operations can
// fail?

/// Modify a tree-sitter parse tree when printing.
pub trait Editor {
    /// Does this editor have an edit for this node?
    #[must_use]
    fn has_edit(&self, tree: &Tree, node: &Node<'_>) -> bool;

    /// Edit this node (precondition: [`Editor::has_edit`]).
    fn edit(&self, source: &[u8], tree: &Tree, node: &Node<'_>) -> Vec<u8>;

    /// Get all edits to this tree, in order.
    ///
    /// Edits must be ordered by start byte and must not overlap.
    ///
    /// [`default_in_order`] does an in-order traversal of the tree.
    fn in_order_edits<'a>(
        &'a self,
        source: &'a [u8],
        tree: &'a Tree,
    ) -> Box<dyn Iterator<Item = Edit> + 'a> {
        Box::new(traverse(tree.walk(), Order::Pre).filter_map(|n| {
            if self.has_edit(tree, &n) {
                Some(Edit {
                    position: n.start_byte(),
                    delete: n.end_byte() - n.start_byte(),
                    insert: self.edit(source, tree, &n),
                })
            } else {
                None
            }
        }))
    }
}
