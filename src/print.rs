use std::io;
use std::io::Write;

use tree_sitter::Tree;

use crate::editor::Editor;

/// Render edits from [`Editor::in_order_edits`].
///
/// If an edit begins inside of a previous edit, then skip it.
///
/// # Errors
///
/// Errors if [`write!`] returns an error.
pub fn render(
    w: &mut impl Write,
    tree: &Tree,
    source: &[u8],
    editor: &impl Editor,
) -> Result<bool, io::Error> {
    let mut changed = false;
    let mut start = 0;
    for edit in editor.in_order_edits(source, tree) {
        if edit.position < start {
            continue;
        }
        changed = true;
        // Write everything up to the start of this edit
        w.write_all(&source[start..edit.position])?;
        w.write_all(&edit.insert)?;
        start = edit.position + edit.delete;
    }
    w.write_all(&source[start..source.len()])?;
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use tree_sitter::Node;

    use super::*;
    use crate::editors::{Delete, Id, Replace};
    use crate::id::NodeId;

    fn vec_str(v: &[u8]) -> &str {
        std::str::from_utf8(v).unwrap()
    }

    fn parse(src: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .expect("Error loading C grammar");
        parser.parse(src, None).expect("Failed to parse test")
    }

    fn do_render(tree: &Tree, src: &str, editor: &impl Editor) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        render(&mut v, tree, src.as_bytes(), editor).expect("I/O error on a vector?");
        v
    }

    fn parse_then_render(src: &str, editor: &impl Editor) -> Vec<u8> {
        do_render(&parse(src), src, editor)
    }

    #[test]
    fn parse_then_render_nil() {
        let src = r#""#;
        let r = parse_then_render(src, &Id {});
        assert_eq!(src, vec_str(&r));
    }

    #[test]
    fn parse_then_render_main_id() {
        let src = r#"int main(int argc, char *argv[]) { return 0; }"#;
        let r = parse_then_render(src, &Id::new());
        assert_eq!(src, vec_str(&r));
    }

    #[test]
    fn parse_then_render_main_omit() {
        let src = r#"int main(int argc, char *argv[]) { return 0; }"#;
        let tree = parse(src);
        let editor = Delete::new(NodeId::new(&tree.root_node()));
        let r = do_render(&tree, src, &editor);
        assert_eq!("", vec_str(&r));
    }

    fn find_kind(tree: &Tree, node: &Node<'_>, kind: &str) -> Option<NodeId> {
        if node.kind() == kind {
            return Some(NodeId::new(node));
        }
        for child in node.children(&mut tree.walk()) {
            if let Some(n) = find_kind(tree, &child, kind) {
                return Some(n);
            }
        }
        None
    }

    #[test]
    fn parse_then_render_replace_binary_expr() {
        let src = r#"int main(int argc, char *argv[]) { return 0 + 0; }"#;
        let tree = parse(src);
        let binop = find_kind(&tree, &tree.root_node(), "binary_expression").unwrap();
        let editor = Replace {
            id: binop,
            bytes: "1".as_bytes().to_vec(),
        };
        let edited = r#"int main(int argc, char *argv[]) { return 1; }"#;
        let r = do_render(&tree, src, &editor);
        assert_eq!(edited, vec_str(&r));
    }

    #[test]
    fn parse_then_render_replace_binary_expr_bigger() {
        let src = r#"int main(int argc, char *argv[]) { return 0 + 0; }"#;
        let tree = parse(src);
        let binop = find_kind(&tree, &tree.root_node(), "binary_expression").unwrap();
        let editor = Replace {
            id: binop,
            bytes: "100 + 100000".as_bytes().to_vec(),
        };
        let edited = r#"int main(int argc, char *argv[]) { return 100 + 100000; }"#;
        let r = do_render(&tree, src, &editor);
        assert_eq!(edited, vec_str(&r));
    }
}
