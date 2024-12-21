use std::str::Utf8Error;

use pandoc_ast::{Block, ListNumberDelim, ListNumberStyle};

use crate::Meta;

enum ListType {
    Unordered,
    Ordered,
}

const DEFAULT_ORDERED_ATTR: (i64, ListNumberStyle, ListNumberDelim) = (
    1,
    ListNumberStyle::DefaultStyle,
    ListNumberDelim::DefaultDelim,
);

pub fn parse(meta: &mut Meta) -> Result<Block, Utf8Error> {
    if meta.tree.goto_first_child() {
        parse_list(meta).map(|(mut list, kind, nesting)| match kind {
            ListType::Unordered => {
                for _ in 1..nesting {
                    list = vec![vec![Block::BulletList(list)]]
                }
                Block::BulletList(list)
            }
            ListType::Ordered => {
                for _ in 1..nesting {
                    list = vec![vec![Block::OrderedList(DEFAULT_ORDERED_ATTR, list)]]
                }
                Block::OrderedList(DEFAULT_ORDERED_ATTR, list)
            }
        })
    } else {
        unreachable!()
    }
}

fn parse_list(meta: &mut Meta) -> Result<(Vec<Vec<Block>>, ListType, usize), Utf8Error> {
    let mut list = Vec::new();

    let kind = match meta.tree.node().kind() {
        s if s.starts_with("unordered_list") => ListType::Unordered,
        s if s.starts_with("ordered_list") => ListType::Ordered,
        _ => todo!("{}", meta.tree.node().kind()),
    };

    let mut top_nesting = None;

    loop {
        let nesting = {
            let number_index = meta
                .tree
                .node()
                .kind()
                .chars()
                .position(|c| c.is_ascii_digit())
                .expect("Nesting is always in the kind");
            meta.tree.node().kind()[number_index..]
                .parse()
                .expect("This is always a number")
        };

        let mut content = if meta.tree.goto_first_child() && meta.tree.goto_next_sibling() {
            vec![super::paragraph(meta)?]
        } else {
            unreachable!()
        };

        if meta.tree.goto_next_sibling() {
            let (mut list, kind, sub_nesting) = parse_list(meta)?;
            match kind {
                ListType::Unordered => {
                    for _ in nesting..sub_nesting - 1 {
                        list = vec![vec![Block::BulletList(list)]]
                    }
                    content.push(Block::BulletList(list));
                }
                ListType::Ordered => {
                    for _ in nesting..sub_nesting - 1 {
                        list = vec![vec![Block::OrderedList(DEFAULT_ORDERED_ATTR, list)]]
                    }
                    content.push(Block::OrderedList(DEFAULT_ORDERED_ATTR, list));
                }
            }
        } else {
            meta.tree.goto_parent();
        }

        match top_nesting.as_mut() {
            Some(top_nesting) => {
                for _ in nesting..*top_nesting {
                    match kind {
                        ListType::Unordered => list = vec![vec![Block::BulletList(list)]],
                        ListType::Ordered => {
                            list = vec![vec![Block::OrderedList(DEFAULT_ORDERED_ATTR, list)]];
                        }
                    }
                }
                *top_nesting = nesting;
                list.push(content);
            }
            None => {
                list.push(content);
                top_nesting = Some(nesting)
            }
        }

        if !meta.tree.goto_next_sibling() {
            meta.tree.goto_parent();
            break;
        }
    }

    Ok((
        list,
        kind,
        top_nesting.expect("At this point there is a top nesting"),
    ))
}
