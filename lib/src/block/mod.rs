use std::collections::VecDeque;

use pandoc_ast::{Block, Inline};

use crate::Meta;

mod paragraph;

/// Parse blocks in a document
pub(super) fn parse(parse_meta: &mut Meta) -> VecDeque<Block> {
    let block = match parse_meta.tree.node().kind() {
        "paragraph" => paragraph::parse(parse_meta),

        "\n" => {
            // skip
            if parse_meta.tree.goto_next_sibling() {
                return parse(parse_meta);
            } else {
                return VecDeque::new();
            }
        }

        _ => {
            eprintln!("{:?} not implemented", parse_meta.tree.node().kind());
            Block::Plain(vec![Inline::Str(
                parse_meta
                    .tree
                    .node()
                    .utf8_text(parse_meta.source)
                    .unwrap()
                    .to_owned(),
            )])
        }
    };

    if parse_meta.tree.goto_next_sibling() {
        let mut next_blocks = parse(parse_meta);
        if !matches!(block, Block::Null) {
            next_blocks.push_front(block);
        }
        next_blocks
    } else {
        parse_meta.tree.goto_parent();
        if !matches!(block, Block::Null) {
            VecDeque::from([block])
        } else {
            VecDeque::default()
        }
    }
}
