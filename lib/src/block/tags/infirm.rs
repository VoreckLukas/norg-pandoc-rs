use pandoc_ast::{Block, Inline};

use crate::Meta;

pub(in crate::block) fn parse(parse_meta: &mut Meta) -> Block {
    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_next_sibling() {
        unreachable!()
    }

    if parse_meta.tree.node().utf8_text(parse_meta.source).unwrap() == "image" {
        if !parse_meta.tree.goto_next_sibling() || !parse_meta.tree.goto_next_sibling() {
            unreachable!()
        }

        let target = parse_meta
            .tree
            .node()
            .utf8_text(parse_meta.source)
            .unwrap()
            .to_owned();

        parse_meta.tree.goto_parent();

        Block::Plain(vec![Inline::Image(
            (String::new(), vec![], vec![]),
            vec![],
            (target, String::default()),
        )])
    } else {
        unimplemented!()
    }
}
