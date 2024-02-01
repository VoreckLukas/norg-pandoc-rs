use pandoc_ast::{Map, Pandoc};

use crate::{block, Meta};

pub(super) fn parse(meta: &mut Meta, pandoc_api_version: Vec<u32>) -> Pandoc {
    let blocks = if meta.tree.goto_first_child() {
        block::parse(meta).into()
    } else {
        vec![]
    };

    Pandoc {
        meta: Map::new(),
        blocks,
        pandoc_api_version,
    }
}
