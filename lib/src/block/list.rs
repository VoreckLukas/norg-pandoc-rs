use std::collections::LinkedList;

use pandoc_ast::{Block, Inline, ListNumberDelim, ListNumberStyle};

use crate::{inline, Meta};

enum ListType {
    Unordered,
    Ordered,
}

const DEFAULT_ORDERED_ATTR: (i64, ListNumberStyle, ListNumberDelim) = (
    1,
    ListNumberStyle::DefaultStyle,
    ListNumberDelim::DefaultDelim,
);

pub fn parse(parse_meta: &mut Meta) -> Block {
    if parse_meta.tree.goto_first_child() {
        let (list, kind, nesting) = parse_list(parse_meta);
        let mut list = list.into_iter().collect();
        match kind {
            ListType::Unordered => {
                for _ in 1..nesting {
                    list = vec![vec![Block::BulletList(list)]];
                }
                Block::BulletList(list)
            }
            ListType::Ordered => {
                for _ in 1..nesting {
                    list = vec![vec![Block::OrderedList(DEFAULT_ORDERED_ATTR, list)]];
                }
                Block::OrderedList(DEFAULT_ORDERED_ATTR, list)
            }
        }
    } else {
        unreachable!()
    }
}

fn parse_list(parse_meta: &mut Meta) -> (LinkedList<Vec<Block>>, ListType, usize) {
    let kind = match parse_meta.tree.node().kind() {
        s if s.starts_with("unordered_list") => ListType::Unordered,
        s if s.starts_with("ordered_list") => ListType::Ordered,
        _ => unreachable!(),
    };

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

    let mut content = if parse_meta.tree.goto_first_child() && parse_meta.tree.goto_next_sibling() {
        let mut inlines = if parse_meta.tree.node().kind() == "detached_modifier_extension" {
            if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_next_sibling() {
                unreachable!()
            }
            let item = if parse_meta.tree.node().kind() == "todo_item_urgent" {
                LinkedList::from([
                    Inline::Strong(vec![Inline::Span(
                        (
                            String::default(),
                            vec!["todo".to_string(), "urgent".to_string()],
                            vec![],
                        ),
                        vec![Inline::Str("(!)".to_string())],
                    )]),
                    Inline::Space,
                ])
            } else {
                eprintln!("{}", parse_meta.tree.node().kind());
                todo!()
            };

            parse_meta.tree.goto_parent();
            parse_meta.tree.goto_next_sibling();
            item
        } else {
            LinkedList::new()
        };
        if !parse_meta.tree.goto_first_child() {
            unreachable!()
        }
        inlines.append(&mut inline::parse(parse_meta));
        vec![Block::Para(inlines.into_iter().collect())]
    } else {
        unreachable!()
    };

    if parse_meta.tree.goto_next_sibling() {
        let (list, kind, sub_nesting) = parse_list(parse_meta);
        let mut list = list.into_iter().collect();
        match kind {
            ListType::Unordered => {
                for _ in nesting..sub_nesting - 1 {
                    list = vec![vec![Block::BulletList(list)]];
                }
                content.push(Block::BulletList(list))
            }
            ListType::Ordered => {
                for _ in nesting..sub_nesting - 1 {
                    list = vec![vec![Block::OrderedList(DEFAULT_ORDERED_ATTR, list)]];
                }
                content.push(Block::OrderedList(DEFAULT_ORDERED_ATTR, list));
            }
        }
    } else {
        parse_meta.tree.goto_parent();
    }

    let mut list = LinkedList::from([content]);

    if parse_meta.tree.goto_next_sibling() {
        let (mut next_list, _, sib_nesting) = parse_list(parse_meta);
        for _ in sib_nesting..nesting {
            match kind {
                ListType::Unordered => {
                    list = LinkedList::from([vec![Block::BulletList(list.into_iter().collect())]])
                }
                ListType::Ordered => {
                    list = LinkedList::from([vec![Block::OrderedList(
                        DEFAULT_ORDERED_ATTR,
                        list.into_iter().collect(),
                    )]])
                }
            }
        }
        nesting = sib_nesting;
        list.append(&mut next_list);
    } else {
        parse_meta.tree.goto_parent();
    }

    (list, kind, nesting)
}
