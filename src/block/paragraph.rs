use pandoc_ast::Block;

use crate::{inline, Meta};

pub fn parse(parse_meta: &mut Meta) -> Block {
    let inline = if parse_meta.tree.goto_first_child() {
        inline::parse(parse_meta).into_iter().collect()
    } else {
        vec![]
    };

    // Skip the paragraph break
    parse_meta.tree.goto_next_sibling();

    Block::Para(inline)
}
