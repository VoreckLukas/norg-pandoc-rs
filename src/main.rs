use std::{env, fs};

use tree_sitter::{Parser, TreeCursor};

mod block;
mod document;
mod inline;

pub struct Meta<'a> {
    tree: TreeCursor<'a>,
    source: &'a [u8],
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
        debug_tree(&mut tree.walk(), 0);
    }

    println!(
        "{}",
        document::parse(Meta {
            tree: tree.walk(),
            source: unparsed.as_bytes()
        })
        .to_json()
    )
}

#[cfg(debug_assertions)]
fn debug_tree(tree: &mut TreeCursor, indentlevel: usize) {
    let indent = " ".repeat(indentlevel * 3);
    eprintln!("{}{}", indent, tree.node().kind());
    if tree.goto_first_child() {
        debug_tree(tree, indentlevel + 1);
    }
    if tree.goto_next_sibling() {
        debug_tree(tree, indentlevel);
    } else {
        tree.goto_parent();
    }
}
