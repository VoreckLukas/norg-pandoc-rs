use pandoc_ast::Pandoc;

use crate::{block, inline::link, Meta};

pub fn parse(mut parse_meta: Meta) -> Pandoc {
    let mut blocks = if parse_meta.tree.goto_first_child() {
        block::parse(&mut parse_meta).into()
    } else {
        vec![]
    };

    link::resolve_links(&mut blocks);

    Pandoc {
        meta: parse_meta.metadata,
        blocks,
        pandoc_api_version: vec![1, 23],
    }
}
