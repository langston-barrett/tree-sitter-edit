use tree_sitter::{Node, Tree};

use crate::editor::{Edit, Editor};

/// The [Editor] that makes no changes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {}

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Editor for Id {
    #[must_use]
    fn has_edit(&self, _tree: &Tree, _node: &Node<'_>) -> bool {
        false
    }

    fn edit(&self, _source: &[u8], _tree: &Tree, _node: &Node<'_>) -> Vec<u8> {
        debug_assert!(false);
        Vec::new()
    }

    fn in_order_edits(&self, _source: &[u8], _tree: &Tree) -> Box<dyn Iterator<Item = Edit>> {
        Box::new(std::iter::empty())
    }
}
