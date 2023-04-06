use tree_sitter::{Node, Tree};

// TODO(lb): Perhaps make an associated error type so that these operations can
// fail?

/// Modify a tree-sitter parse tree when printing.
pub trait Editor {
    /// Does this editor have an edit for this node?
    fn has_edit(&self, tree: &Tree, node: &Node) -> bool;

    /// Edit this node (precondition: [`Editor::has_edit`]).
    fn edit(&self, source: &[u8], tree: &Tree, node: &Node) -> Vec<u8>;
}
