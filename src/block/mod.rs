use std::collections::VecDeque;

use pandoc_ast::{Block, Inline};

use crate::Meta;

mod heading;
mod list;
mod paragraph;
mod quote;
mod tags;

pub fn parse(parse_meta: &mut Meta) -> VecDeque<Block> {
    let block = match parse_meta.tree.node().kind() {
        "paragraph" => paragraph::parse(parse_meta),

        "generic_list" => list::parse(parse_meta),

        "quote" => quote::parse(parse_meta),

        s if s.starts_with("heading") => heading::parse(parse_meta),

        "ranged_verbatim_tag" => tags::verbatim::parse(parse_meta),

        "weak_paragraph_delimiter" => Block::Null,

        "_line_break" | "_paragraph_break" => {
            return if parse_meta.tree.goto_next_sibling() {
                parse(parse_meta)
            } else {
                parse_meta.tree.goto_parent();
                VecDeque::default()
            }
        }

        _ => {
            eprintln!("{} not implemented", parse_meta.tree.node().kind());
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
        next_blocks.push_front(block);
        next_blocks
    } else {
        parse_meta.tree.goto_parent();
        if matches!(block, Block::Null) {
            VecDeque::default()
        } else {
            VecDeque::from([block])
        }
    }
}
