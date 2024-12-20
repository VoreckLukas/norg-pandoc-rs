use std::str::Utf8Error;

use pandoc_ast::{Map, MetaValue, Pandoc};
use tree_sitter::{Parser, TreeCursor};

mod document;

struct Meta<'a> {
    tree: TreeCursor<'a>,
    source: &'a [u8],
    metadata: Map<String, MetaValue>,
}

pub fn parse(source: &str, pandoc_api_version: Vec<u32>) -> Option<Result<Pandoc, Utf8Error>> {
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
                metadata: Map::new(),
            },
            pandoc_api_version,
        )
    })
}
