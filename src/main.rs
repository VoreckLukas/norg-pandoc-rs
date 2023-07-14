use std::{env, fs};

use pandoc_ast::{Map, MetaValue};
use tree_sitter::{Parser, TreeCursor};

mod block;
mod document;
mod inline;

pub struct Meta<'a> {
    tree: TreeCursor<'a>,
    source: &'a [u8],
    metadata: Map<String, MetaValue>,
}

fn main() {
    let language = tree_sitter_norg::language();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let file = env::args().nth(1).expect("No input file");
    let unparsed = fs::read_to_string(file).expect("Cannot read file");
    let tree = parser.parse(&unparsed, None).unwrap();

    #[cfg(debug_assertions)]
    {
        debug_tree(
            &mut Meta {
                tree: tree.walk(),
                source: unparsed.as_bytes(),
                metadata: Map::default(),
            },
            0,
        );
    }

    println!(
        "{}",
        document::parse(Meta {
            tree: tree.walk(),
            source: unparsed.as_bytes(),
            metadata: Map::default()
        })
        .to_json()
    )
}

#[cfg(debug_assertions)]
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
