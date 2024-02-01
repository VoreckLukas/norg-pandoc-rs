use pandoc_ast::Block;

use crate::{inline, Meta};

pub(super) fn parse(meta: &mut Meta) -> Block {
    let node = meta.tree.node();

    let range = (node.start_byte(), node.end_byte());

    let inline = if meta.tree.goto_first_child() {
        inline::parse(meta, range).into_iter().collect()
    } else {
        vec![]
    };

    Block::Para(inline)
}
