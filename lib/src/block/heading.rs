use std::collections::LinkedList;

use pandoc_ast::Block;

use crate::{block, inline, Meta};

pub(super) fn parse(parse_meta: &mut Meta) -> Block {
    let nesting: i64 = {
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

    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_next_sibling() {
        unreachable!()
    }

    let (mut text, extension_length) =
        if parse_meta.tree.node().kind() == "detached_modifier_extension" {
            let item = inline::detached_extension::parse(parse_meta);
            parse_meta.tree.goto_next_sibling();
            let len = item.len();
            (item, len)
        } else {
            (LinkedList::new(), 0)
        };

    let text: Vec<_> = if parse_meta.tree.goto_first_child() {
        text.append(&mut inline::parse(parse_meta));
        text.into_iter().collect()
    } else {
        unreachable!()
    };

    let id = {
        let mut id = inline::to_string(&text[extension_length..]);
        id.push_str(&nesting.to_string());
        id
    };
    let header = Block::Header(nesting, (id, vec![], vec![]), text);
    let content = if parse_meta.tree.goto_next_sibling() {
        let mut content = block::parse(parse_meta);
        content.push_front(header);
        content.into()
    } else {
        parse_meta.tree.goto_parent();
        vec![header]
    };
    Block::Div((String::default(), vec![], vec![]), content)
}
