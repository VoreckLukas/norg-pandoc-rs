use pandoc_ast::{Map, Pandoc};

use crate::{block, inline::link, Meta};

pub fn parse(mut parse_meta: Meta) -> Pandoc {
    let mut blocks = if parse_meta.tree.goto_first_child() {
        block::parse(&mut parse_meta).into()
    } else {
        vec![]
    };

    link::resolve_magic_links(&mut blocks);

    Pandoc {
        meta: Map::default(),
        blocks,
        pandoc_api_version: vec![1, 23],
    }
}
