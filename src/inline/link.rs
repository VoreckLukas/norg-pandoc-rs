use pandoc_ast::Inline;

use crate::{inline, Meta};

pub fn parse(parse_meta: &mut Meta) -> Inline {
    let target = if parse_meta.tree.goto_first_child()
        && parse_meta.tree.goto_first_child()
        && parse_meta.tree.goto_next_sibling()
    {
        match parse_meta.tree.node().kind() {
            "link_target_url" => {
                if !parse_meta.tree.goto_next_sibling() {
                    unreachable!()
                }

                parse_meta
                    .tree
                    .node()
                    .utf8_text(parse_meta.source)
                    .unwrap()
                    .to_owned()
            }

            _ => todo!(),
        }
    } else {
        unreachable!()
    };

    let description = if parse_meta.tree.goto_parent()
        && parse_meta.tree.goto_next_sibling()
        && parse_meta.tree.goto_first_child()
    {
        inline::parse(parse_meta).into_iter().collect()
    } else {
        vec![Inline::Str(target.clone())]
    };
    parse_meta.tree.goto_parent();

    Inline::Link(
        (String::default(), vec![], vec![]),
        description,
        (target, String::new()),
    )
}
