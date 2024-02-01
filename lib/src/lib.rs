use pandoc_ast::Pandoc;
use tree_sitter::{Parser, TreeCursor};

mod block;
mod document;
mod inline;

struct Meta<'a> {
    tree: TreeCursor<'a>,
    source: &'a [u8],
}

/// Takes in a source string and a target api version and returns the texts pandoc ast
pub fn parse(text: &str, api_version: Vec<u32>) -> Pandoc {
    let tree = {
        let norg_language = tree_sitter_norg::language();
        let mut parser = Parser::new();
        parser.set_language(norg_language).unwrap();

        parser.parse(text, None).unwrap()
    };

    let mut meta = Meta {
        tree: tree.walk(),
        source: text.as_bytes(),
    };

    #[cfg(feature = "debug")]
    {
        debug_tree(&mut meta, 0);
    }

    document::parse(&mut meta, api_version)
}

#[cfg(feature = "debug")]
fn debug_tree(parse_meta: &mut Meta, indentlevel: usize) {
    let indent = " ".repeat(indentlevel * 3);
    println!(
        //"{indent}{:?}",
        "{indent}{:?}: {:?}",
        parse_meta.tree.node().kind(),
        parse_meta.tree.node().utf8_text(parse_meta.source).unwrap()
    );
    if parse_meta.tree.goto_first_child() {
        debug_tree(parse_meta, indentlevel + 1);
    }
    if parse_meta.tree.goto_next_sibling() {
        debug_tree(parse_meta, indentlevel);
    } else {
        parse_meta.tree.goto_parent();
    }
}
