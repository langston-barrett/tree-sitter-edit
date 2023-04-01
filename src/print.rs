use std::io;
use std::io::Write;

use tree_sitter::{Node, Tree};

use crate::editor::Editor;

#[derive(Debug)]
struct Edit {
    // from node.start_byte()
    position: usize,
    deleted_length: usize,
    inserted_text: Vec<u8>,
}

fn collect_edits(tree: &Tree, node: &Node, source: &[u8], editor: &impl Editor) -> Vec<Edit> {
    let mut edits = Vec::new();
    let mut nodes = Vec::new();
    nodes.push(*node);
    while let Some(node) = nodes.pop() {
        if !editor.contains_edit(tree, &node) {
            continue;
        } else if editor.has_edit(tree, &node) {
            debug_assert!(node.end_byte() >= node.start_byte());
            edits.push(Edit {
                position: node.start_byte(),
                deleted_length: node.end_byte() - node.start_byte(),
                inserted_text: editor.edit(source, tree, &node),
            });
        } else {
            nodes.extend(node.children(&mut tree.walk()));
        }
    }
    edits
}

fn merge_edits(w: &mut impl Write, source: &[u8], edits: &[Edit]) -> Result<bool, io::Error> {
    let mut start = 0;
    for edit in edits {
        w.write_all(&source[start..edit.position])?;
        w.write_all(&edit.inserted_text)?;
        start = edit.position + edit.deleted_length;
    }
    w.write_all(&source[start..source.len()])?;
    Ok(!edits.is_empty())
}

/// # Errors
///
/// Errors if [`write!`] returns an error.
pub fn render(
    w: &mut impl Write,
    tree: &Tree,
    source: &[u8],
    editor: &impl Editor,
) -> Result<bool, io::Error> {
    let edits = collect_edits(tree, &tree.root_node(), source, editor);
    merge_edits(w, source, edits.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editors::{Delete, Id, Replace};
    use crate::id::NodeId;

    fn vec_str(v: &Vec<u8>) -> &str {
        std::str::from_utf8(&v).unwrap()
    }

    fn parse(src: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_c::language())
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
        assert_eq!(src, vec_str(&r))
    }

    #[test]
    fn parse_then_render_main_id() {
        let src = r#"int main(int argc, char *argv[]) { return 0; }"#;
        let r = parse_then_render(src, &Id::new());
        assert_eq!(src, vec_str(&r))
    }

    #[test]
    fn parse_then_render_main_omit() {
        let src = r#"int main(int argc, char *argv[]) { return 0; }"#;
        let tree = parse(src);
        let editor = Delete::new(NodeId::new(&tree.root_node()));
        let r = do_render(&tree, src, &editor);
        assert_eq!("", vec_str(&r))
    }

    fn find_kind(tree: &Tree, node: &Node, kind: &str) -> Option<NodeId> {
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
        assert_eq!(edited, vec_str(&r))
    }
}
