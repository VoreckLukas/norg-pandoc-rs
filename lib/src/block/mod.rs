use std::collections::VecDeque;

use pandoc_ast::{Block, Inline};

use crate::Meta;

mod paragraph;

pub(super) fn parse(meta: &mut Meta) -> VecDeque<Block> {
    let node = meta.tree.node();

    let block = match node.kind() {
        "paragraph" => paragraph::parse(meta),

        "\n" => {
            return if meta.tree.goto_next_sibling() {
                parse(meta)
            } else {
                VecDeque::new()
            }
        }

        _ => {
            eprintln!("{:?} not implemented", node.kind());
            Block::Plain(vec![Inline::Str(
                node.utf8_text(meta.source).unwrap().to_owned(),
            )])
        }
    };

    if meta.tree.goto_next_sibling() {
        let mut next_blocks = parse(meta);
        next_blocks.push_front(block);
        next_blocks
    } else {
        meta.tree.goto_parent();
        VecDeque::from([block])
    }
}
