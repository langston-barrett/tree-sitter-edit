use tree_sitter::{Node, Tree};

use crate::editor::{default_in_order, Edit, Editor};

/// An [Editor] that merges the edits from two [Editor]s, preferring the left
/// one.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeftBiasedOr<L, R>
where
    L: Editor,
    R: Editor,
{
    pub left: L,
    pub right: R,
}

impl<L: Editor, R: Editor> Editor for LeftBiasedOr<L, R> {
    fn has_edit(&self, tree: &Tree, node: &Node<'_>) -> bool {
        self.left.has_edit(tree, node) || self.right.has_edit(tree, node)
    }

    fn edit(&self, source: &[u8], tree: &Tree, node: &Node<'_>) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        if self.left.has_edit(tree, node) {
            self.left.edit(source, tree, node)
        } else {
            debug_assert!(self.right.has_edit(tree, node));
            self.right.edit(source, tree, node)
        }
    }

    fn in_order_edits<'a>(
        &'a self,
        source: &'a [u8],
        tree: &'a Tree,
    ) -> Box<dyn Iterator<Item = Edit> + 'a> {
        Box::new(default_in_order(self, source, tree))
    }
}
