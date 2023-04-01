use tree_sitter::{Node, Tree};

// TODO(lb): Perhaps make an associated error type so that these operations can
// fail?

/// Modify a tree-sitter parse tree when printing.
pub trait Editor {
    /// Does this editor have an edit for this node?
    fn has_edit(&self, tree: &Tree, node: &Node) -> bool;

    /// Does this editor have an edit for some descendant of this node?
    fn contains_edit(&self, tree: &Tree, node: &Node) -> bool {
        let mut nodes = Vec::with_capacity(node.child_count());
        nodes.push(*node);
        while let Some(node) = nodes.pop() {
            if self.has_edit(tree, &node) {
                return true;
            }
            nodes.extend(node.children(&mut tree.walk()));
        }
        false
    }

    /// Edit this node (precondition: [`Editor::has_edit`]).
    fn edit(&self, source: &[u8], tree: &Tree, node: &Node) -> Vec<u8>;
}
