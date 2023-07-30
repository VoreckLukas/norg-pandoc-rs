use std::collections::LinkedList;

use pandoc_ast::Block;

use crate::{inline, Meta};

pub(super) fn parse(parse_meta: &mut Meta) -> Block {
    if parse_meta.tree.goto_first_child() {
        let (quote, nesting) = parse_quote(parse_meta);
        let mut quote = quote.into_iter().collect();
        for _ in 1..nesting {
            quote = vec![Block::BlockQuote(quote)];
        }
        Block::BlockQuote(quote)
    } else {
        unreachable!()
    }
}

fn parse_quote(parse_meta: &mut Meta) -> (LinkedList<Block>, usize) {
    let mut nesting = {
        let number_index = parse_meta
            .tree
            .node()
            .kind()
            .chars()
            .position(|c| c.is_ascii_digit())
            .unwrap();
        parse_meta.tree.node().kind()[number_index..]
            .parse()
            .unwrap()
    };

    let mut content = if parse_meta.tree.goto_first_child()
        && parse_meta.tree.goto_next_sibling()
        && parse_meta.tree.goto_first_child()
    {
        LinkedList::from([Block::Para(inline::parse(parse_meta).into_iter().collect())])
    } else {
        unreachable!()
    };

    if parse_meta.tree.goto_next_sibling() {
        let (quote, sub_nesting) = parse_quote(parse_meta);
        let mut quote = quote.into_iter().collect();
        for _ in nesting..sub_nesting - 1 {
            quote = vec![Block::BlockQuote(quote)];
        }
        content.push_back(Block::BlockQuote(quote))
    } else {
        parse_meta.tree.goto_parent();
    }

    if parse_meta.tree.goto_next_sibling() {
        let (mut next_quote, sib_nesting) = parse_quote(parse_meta);
        for _ in sib_nesting..nesting {
            content = LinkedList::from([Block::BlockQuote(content.into_iter().collect())]);
        }
        nesting = sib_nesting;
        content.append(&mut next_quote);
    } else {
        parse_meta.tree.goto_parent();
    }

    (content, nesting)
}
