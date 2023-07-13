use std::{cell::RefCell, collections::HashMap, rc::Rc};

use pandoc_ast::{Block, Inline};

use crate::{inline, Meta};

pub(super) fn parse(parse_meta: &mut Meta) -> Inline {
    let (target, description) =
        if parse_meta.tree.goto_first_child() && parse_meta.tree.goto_first_child() {
            parse_target(parse_meta)
        } else {
            unreachable!()
        };

    let description = if parse_meta.tree.goto_parent()
        && parse_meta.tree.goto_next_sibling()
        && parse_meta.tree.goto_first_child()
    {
        inline::parse(parse_meta).into_iter().collect()
    } else {
        description
    };
    parse_meta.tree.goto_parent();

    Inline::Link((String::default(), vec![], vec![]), description, target)
}

fn parse_target(parse_meta: &mut Meta) -> ((String, String), Vec<Inline>) {
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
            ((url.clone(), String::new()), vec![Inline::Str(url)])
        }

        "link_file_text" => {
            let mut file = parse_meta
                .tree
                .node()
                .utf8_text(parse_meta.source)
                .unwrap()
                .to_owned();
            let mut description = vec![Inline::Str(file.clone())];
            file.push_str(".norg");

            loop {
                if !parse_meta.tree.goto_next_sibling() || parse_meta.tree.node().kind() != "_end" {
                    break;
                }
            }

            if parse_meta.tree.node().kind() != "_end" {
                let ((sub_target, _), mut sub_description) = parse_target(parse_meta);
                file.push_str(&sub_target);
                description.push(Inline::Space);
                description.append(&mut sub_description);
            }

            ((file, String::new()), description)
        }

        "link_target_external_file" => {
            if !parse_meta.tree.goto_next_sibling() {
                unreachable!()
            }

            let file = parse_meta
                .tree
                .node()
                .utf8_text(parse_meta.source)
                .unwrap()
                .replace("~", dirs::home_dir().unwrap().to_str().unwrap());
            let description = vec![Inline::Str(file.clone())];

            ((file, String::new()), description)
        }

        "link_target_line_number" => {
            if !parse_meta.tree.goto_next_sibling() {
                unreachable!()
            }

            let line_number = parse_meta.tree.node().utf8_text(parse_meta.source).unwrap();
            (
                (format!("#L{line_number}"), String::new()),
                vec![Inline::Str(format!("Line {line_number}"))],
            )
        }

        s if s.starts_with("link_target_heading") => {
            let nesting = {
                let nesting_index = parse_meta
                    .tree
                    .node()
                    .kind()
                    .chars()
                    .position(|c| c.is_ascii_digit())
                    .unwrap();
                &parse_meta.tree.node().kind()[nesting_index..]
            };
            if !parse_meta.tree.goto_next_sibling() || !parse_meta.tree.goto_first_child() {
                unreachable!()
            }
            let heading: Vec<_> = inline::parse(parse_meta).into_iter().collect();

            let heading_id = {
                let mut heading_str = inline::to_string(&heading);
                heading_str.push_str(nesting);
                heading_str
            };

            ((format!("#{heading_id}"), String::new()), heading)
        }

        "link_target_generic" => {
            if !parse_meta.tree.goto_next_sibling() || !parse_meta.tree.goto_first_child() {
                unreachable!()
            }

            let description: Vec<_> = inline::parse(parse_meta).into_iter().collect();

            let target = inline::to_string(&description);

            ((format!("#{target}"), String::from("Magic")), description)
        }

        _ => todo!(),
    }
}

pub(crate) fn resolve_links(blocks: &mut [Block]) {
    let mut targets = HashMap::new();
    let links = Rc::new(RefCell::new(Vec::new()));
    let empty_anchors = Rc::new(RefCell::new(Vec::new()));
    let mut anchor_definitions = HashMap::new();

    link_resolver_blocks(
        blocks,
        &mut targets,
        links.clone(),
        empty_anchors.clone(),
        &mut anchor_definitions,
    );

    // Resolve magic char links
    for link in Rc::try_unwrap(links).unwrap().into_inner() {
        if let Some(target) = targets.get(&link.0 .0[1..]) {
            link.0 .0 = format!("#{target}");
        }
        link.0 .1 = link.0 .1.replace("Magic", "");
        if link.0 .1 == "Anchor" {
            anchor_definitions.insert(link.1, link.0);
        }
    }

    // Resolve anchors
    for anchor in Rc::try_unwrap(empty_anchors).unwrap().into_inner() {
        if let Some(target) = anchor_definitions.get(&anchor.1) {
            anchor.0 .0 = target.0.clone();
        }
        anchor.0 .1 = String::new();
    }
    for anchor in anchor_definitions.values_mut() {
        anchor.1 = String::new();
    }
}

fn link_resolver_blocks<'a>(
    blocks: &'a mut [Block],
    targets: &mut HashMap<String, &'a str>,
    links: Rc<RefCell<Vec<(&'a mut (String, String), String)>>>,
    empty_anchors: Rc<RefCell<Vec<(&'a mut (String, String), String)>>>,
    anchor_definitions: &mut HashMap<String, &'a mut (String, String)>,
) {
    for block in blocks {
        match block {
            Block::Para(inlines) | Block::Plain(inlines) => link_resolver_inlines(
                inlines,
                links.clone(),
                empty_anchors.clone(),
                anchor_definitions,
            ),
            Block::Div(_, blocks) | Block::BlockQuote(blocks) => link_resolver_blocks(
                blocks,
                targets,
                links.clone(),
                empty_anchors.clone(),
                anchor_definitions,
            ),
            Block::BulletList(list) | Block::OrderedList(_, list) => {
                for item in list {
                    link_resolver_blocks(
                        item,
                        targets,
                        links.clone(),
                        empty_anchors.clone(),
                        anchor_definitions,
                    );
                }
            }
            Block::Header(_, (id, _, _), content) => {
                let text = inline::to_string(content);
                targets.insert(text, id);
                link_resolver_inlines(
                    content,
                    links.clone(),
                    empty_anchors.clone(),
                    anchor_definitions,
                );
            }
            _ => unreachable!(),
        }
    }
}

fn link_resolver_inlines<'a>(
    inlines: &'a mut [Inline],
    links: Rc<RefCell<Vec<(&'a mut (String, String), String)>>>,
    empty_anchors: Rc<RefCell<Vec<(&'a mut (String, String), String)>>>,
    anchor_definitions: &mut HashMap<String, &'a mut (String, String)>,
) {
    for inline in inlines {
        match inline {
            Inline::Space | Inline::SoftBreak | Inline::Code(_, _) | Inline::Str(_) => { /* ignore */
            }
            Inline::Underline(inlines)
            | Inline::Strong(inlines)
            | Inline::Strikeout(inlines)
            | Inline::Superscript(inlines)
            | Inline::Subscript(inlines)
            | Inline::Span(_, inlines)
            | Inline::Emph(inlines) => link_resolver_inlines(
                inlines,
                links.clone(),
                empty_anchors.clone(),
                anchor_definitions,
            ),
            Inline::Link(_, description, target) => {
                if target.1.contains("Magic") {
                    (*links)
                        .borrow_mut()
                        .push((target, inline::to_string(description)));
                } else if target.1 == "Anchor" {
                    if target.0.is_empty() {
                        (*empty_anchors)
                            .borrow_mut()
                            .push((target, inline::to_string(description)));
                    } else {
                        anchor_definitions.insert(inline::to_string(description), target);
                    }
                }
                link_resolver_inlines(
                    description,
                    links.clone(),
                    empty_anchors.clone(),
                    anchor_definitions,
                );
            }
            _ => unreachable!(),
        }
    }
}

pub(super) fn parse_anchor_declaration(parse_meta: &mut Meta) -> Inline {
    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_first_child() {
        unreachable!()
    }

    let description = inline::parse(parse_meta).into_iter().collect();

    parse_meta.tree.goto_parent();

    Inline::Link(
        (String::default(), vec![], vec![]),
        description,
        (String::new(), String::from("Anchor")),
    )
}

pub(super) fn parse_anchor_definition(parse_meta: &mut Meta) -> Inline {
    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_first_child() {
        unreachable!()
    }

    let description = inline::parse(parse_meta).into_iter().collect();

    if !parse_meta.tree.goto_next_sibling() || !parse_meta.tree.goto_first_child() {
        unreachable!()
    }

    let (mut target, _) = parse_target(parse_meta);
    target.1.push_str("Anchor");

    parse_meta.tree.goto_parent();

    Inline::Link((String::default(), vec![], vec![]), description, target)
}
