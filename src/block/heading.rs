use pandoc_ast::Block;

use crate::{block, inline, Meta};

pub fn parse(parse_meta: &mut Meta) -> Block {
    let nesting = {
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

    let text = if parse_meta.tree.goto_first_child()
        && parse_meta.tree.goto_next_sibling()
        && parse_meta.tree.goto_first_child()
    {
        inline::parse(parse_meta).into_iter().collect()
    } else {
        unreachable!()
    };

    let header = Block::Header(nesting, (String::default(), vec![], vec![]), text);
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
