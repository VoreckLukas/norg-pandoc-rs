use std::str::Utf8Error;

use pandoc_ast::Pandoc;
use thiserror::Error;
use tree_sitter::{Parser, TreeCursor};

mod document;

/// Metadata used to parse the tree
struct Meta<'a> {
    /// Used to walk the tree
    tree: TreeCursor<'a>,
    /// The source text
    source: &'a [u8],
}

/// Errors preventing the document from being converted
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Utf8Error(#[from] Utf8Error),
    /// The treesitter tree was malformed. This is most likely a treesitter bug
    #[error("The treesitter tree was malformed. This is most likely a treesitter bug\n{0}")]
    MalformedTree(&'static str),
}

/// A result where the error type is [Error]
pub type Result<T> = std::result::Result<T, Error>;

/// Parses the given source into a pandoc ast
///
/// Please supply the api version the tree is for
pub fn parse(source: &str, pandoc_api_version: Vec<u32>) -> Option<Result<Pandoc>> {
    let language = tree_sitter_norg::language();
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("The language is always valid");

    parser.parse(source, None).map(|tree| {
        document::parse(
            Meta {
                tree: tree.walk(),
                source: source.as_bytes(),
            },
            pandoc_api_version,
        )
    })
}

/// Debug prints the tree
fn debug(meta: &mut Meta) -> ! {
    fn print(meta: &mut Meta, indent: u8) {
        for _ in 0..indent {
            print!("  ");
        }
        println!("{}", meta.tree.node().kind());
        if meta.tree.goto_first_child() {
            loop {
                print(meta, indent + 1);
                if !meta.tree.goto_next_sibling() {
                    break;
                }
            }
            meta.tree.goto_parent();
        }
    }

    print(meta, 0);
    panic!()
}
