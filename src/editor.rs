use tree_sitter::{Node, Tree};
use tree_sitter_traversal::{traverse, Order};

#[derive(Debug)]
pub struct Edit {
    pub position: usize,
    pub delete: usize,
    pub insert: Vec<u8>,
}

// TODO(lb): Perhaps make an associated error type so that these operations can
// fail?

pub fn default_in_order<'a, E: Editor>(
    editor: &'a E,
    source: &'a [u8],
    tree: &'a Tree,
) -> impl Iterator<Item = Edit> + 'a {
    traverse(tree.walk(), Order::Pre).filter_map(|n| {
        if editor.has_edit(tree, &n) {
            Some(Edit {
                position: n.start_byte(),
                delete: n.end_byte() - n.start_byte(),
                insert: editor.edit(source, tree, &n),
            })
        } else {
            None
        }
    })
}

/// Modify a tree-sitter parse tree when printing.
pub trait Editor {
    /// Does this editor have an edit for this node?
    fn has_edit(&self, tree: &Tree, node: &Node) -> bool;

    /// Edit this node (precondition: [`Editor::has_edit`]).
    fn edit(&self, source: &[u8], tree: &Tree, node: &Node) -> Vec<u8>;

    /// Get all edits to this tree, in order.
    ///
    /// Edits must be ordered by start byte and must not overlap.
    ///
    /// [`default_in_order`] does an in-order traversal of the tree.
    fn in_order_edits<'a>(
        &'a self,
        source: &'a [u8],
        tree: &'a Tree,
    ) -> Box<dyn Iterator<Item = Edit> + 'a>;
}
