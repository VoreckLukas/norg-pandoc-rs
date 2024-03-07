use std::collections::VecDeque;

use pandoc_ast::{Block, Inline};

use crate::Meta;

mod detached;
mod paragraph;

pub(super) fn parse(meta: &mut Meta) -> VecDeque<Block> {
    if !meta.tree.goto_first_child() {
        return VecDeque::new();
    }
    let node = meta.tree.node();

    let block = match node.kind() {
        "paragraph" => paragraph::parse(meta),

        "unordered_list" => detached::parse(meta, detached::Modifier::UnorderedList),

        "unordered_list_prefix" => {
            // Skip
            meta.tree.goto_next_sibling();
            return parse(meta);
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
