use tree_sitter::Node;

/// [Newtype][newtype] around [usize], holding values from [`Node::id`].
///
/// [newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    pub id: usize,
}

impl NodeId {
    #[must_use]
    pub fn new(node: &Node<'_>) -> Self {
        NodeId { id: node.id() }
    }

    #[must_use]
    pub fn get(&self) -> usize {
        self.id
    }

    #[must_use]
    pub fn is(&self, node: &Node<'_>) -> bool {
        node.id() == self.get()
    }
}
