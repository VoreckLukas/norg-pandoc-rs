use pandoc_ast::Pandoc;

use crate::{block, Meta};

/// Parse a norg document
pub(super) fn parse(mut parse_meta: Meta, api_version: Vec<u32>) -> Pandoc {
    let blocks = if parse_meta.tree.goto_first_child() {
        block::parse(&mut parse_meta).into()
    } else {
        vec![]
    };

    Pandoc {
        meta: parse_meta.metadata,
        blocks,
        pandoc_api_version: api_version,
    }
}
