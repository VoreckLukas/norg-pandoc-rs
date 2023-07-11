use pandoc_ast::Inline;

use crate::{inline, Meta};

pub fn parse(parse_meta: &mut Meta) -> Inline {
    let (target, desc_len) =
        if parse_meta.tree.goto_first_child() && parse_meta.tree.goto_first_child() {
            while parse_meta.tree.node().kind() == "_begin" {
                parse_meta.tree.goto_next_sibling();
            }

            match parse_meta.tree.node().kind() {
                "link_target_url" => {
                    if !parse_meta.tree.goto_next_sibling() {
                        unreachable!()
                    }

                    let url = parse_meta
                        .tree
                        .node()
                        .utf8_text(parse_meta.source)
                        .unwrap()
                        .to_owned();
                    let desc_len = url.len();
                    (url, desc_len)
                }

                "link_file_text" => {
                    let mut file = parse_meta
                        .tree
                        .node()
                        .utf8_text(parse_meta.source)
                        .unwrap()
                        .to_owned();
                    file.push_str(".norg");
                    let desc_len = file.len() - 5;
                    (file, desc_len)
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
        vec![Inline::Str(target[..desc_len].to_owned())]
    };
    parse_meta.tree.goto_parent();

    Inline::Link(
        (String::default(), vec![], vec![]),
        description,
        (target, String::new()),
    )
}
