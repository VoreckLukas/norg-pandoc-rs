use pandoc_ast::{Map, Pandoc};
use tree_sitter::Parser;

pub fn parse(source: &str, pandoc_api_version: Vec<u32>) -> Option<Pandoc> {
    let language = tree_sitter_norg::language();
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("The language is always valid");

    parser.parse(source, None).map(|tree| Pandoc {
        meta: Map::new(),
        blocks: todo!(),
        pandoc_api_version,
    })
}
