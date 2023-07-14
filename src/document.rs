use pandoc_ast::{Block, Pandoc};

use crate::{block, inline::link, Meta};

pub fn parse(mut parse_meta: Meta) -> Pandoc {
    let mut blocks = if parse_meta.tree.goto_first_child() {
        block::parse(&mut parse_meta)
            .into_iter()
            .filter(|b| !matches!(b, Block::Null))
            .collect()
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
