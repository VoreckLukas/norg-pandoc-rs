use pandoc_ast::Block;

use crate::{inline, Meta};

pub(super) fn parse(meta: &mut Meta) -> Block {
    let inline = if meta.tree.goto_first_child() {
        inline::parse(meta).into_iter().collect()
    } else {
        vec![]
    };

    Block::Para(inline)
}
