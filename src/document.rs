use pandoc_ast::{Map, Pandoc};

use crate::{block, Meta};

pub fn parse(mut parse_meta: Meta) -> Pandoc {
    let blocks = if parse_meta.tree.goto_first_child() {
        block::parse(&mut parse_meta).into()
    } else {
        vec![]
    };

    Pandoc {
        meta: Map::default(),
        blocks,
        pandoc_api_version: vec![1, 23],
    }
}
