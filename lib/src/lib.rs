use std::{fs, path::Path};

use pandoc_ast::{Map, MetaValue, Pandoc};
use tree_sitter::{Parser, TreeCursor};

mod block;
mod document;
mod inline;

pub struct Meta<'a> {
    tree: TreeCursor<'a>,
    source: &'a [u8],
    metadata: Map<String, MetaValue>,
    target_format: &'a str,
}

pub fn parse<P: AsRef<Path>>(file: P, target_format: &str) -> Pandoc {
    let language = tree_sitter_norg::language();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let unparsed = fs::read_to_string(file).expect("Cannot read file");
    let tree = parser.parse(&unparsed, None).unwrap();

    #[cfg(debug)]
    {
        debug_tree(
            &mut Meta {
                tree: tree.walk(),
                source: unparsed.as_bytes(),
                metadata: Map::default(),
                target_format,
            },
            0,
        );
    }

    document::parse(Meta {
        tree: tree.walk(),
        source: unparsed.as_bytes(),
        metadata: Map::default(),
        target_format,
    })
}

#[cfg(debug)]
fn debug_tree(parse_meta: &mut Meta, indentlevel: usize) {
    let indent = " ".repeat(indentlevel * 3);
    eprintln!(
        "{indent}{}",
        //"{indent}{}: {}",
        parse_meta.tree.node().kind(),
        //parse_meta.tree.node().utf8_text(parse_meta.source).unwrap()
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