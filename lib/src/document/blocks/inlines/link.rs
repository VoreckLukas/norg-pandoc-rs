use std::{collections::HashMap, str::Utf8Error};

use pandoc_ast::{Block, Inline};

use crate::Meta;

fn get_target(meta: &mut Meta) -> Result<(String, String, String), Utf8Error> {
    let mut target = Vec::new();
    let mut description = Vec::new();
    let mut link_type = Vec::new();

    'outer: loop {
        while meta.tree.node().kind() == "_begin" || meta.tree.node().kind() == "_end" {
            if !meta.tree.goto_next_sibling() {
                break 'outer;
            }
        }

        match meta.tree.node().kind() {
            "link_target_url" => {
                meta.tree.goto_next_sibling();
                let url = meta.tree.node().utf8_text(meta.source)?.to_string();
                target.push(url.clone());
                description.push(url);
                link_type.push("url");
            }
            "link_file_text" => {
                let mut file = String::from(meta.tree.node().utf8_text(meta.source)?);
                file.push_str(".norg");
                target.push(file.clone());
                description.push(file);
                link_type.push("file");
            }
            "link_target_line_number" => {
                meta.tree.goto_next_sibling();
                let line = meta.tree.node().utf8_text(meta.source)?.to_string();
                target.push(line.clone());
                description.push(line);
                link_type.push("line");
            }
            s if s.contains("link_target_heading") => {
                let nesting: i64 = {
                    let number_index = meta
                        .tree
                        .node()
                        .kind()
                        .chars()
                        .position(|c| c.is_ascii_digit())
                        .expect("There is always a number in the heading kind");
                    meta.tree.node().kind()[number_index..]
                        .parse()
                        .expect("This is always a number")
                };

                meta.tree.goto_next_sibling();

                let heading = meta.tree.node().utf8_text(meta.source)?;

                target.push(format!("#heading{nesting}{heading}"));
                description.push(heading.to_string());
                link_type.push("heading");
            }
            _ => todo!("{}", meta.tree.node().kind()),
        }

        if !meta.tree.goto_next_sibling() {
            break 'outer;
        }
    }

    Ok((target.join("#"), description.join("#"), link_type.join("#")))
}

pub fn parse(meta: &mut Meta) -> Result<Inline, Utf8Error> {
    if meta.tree.goto_first_child() {
        if meta.tree.goto_first_child() {
            let (target, description, link_type) = get_target(meta)?;

            meta.tree.goto_parent();
            return if meta.tree.goto_next_sibling() {
                super::parse(meta).map(|description| {
                    meta.tree.goto_parent();
                    Inline::Link(
                        (
                            String::new(),
                            Vec::new(),
                            vec![("link_type".to_string(), link_type)],
                        ),
                        description,
                        (target, String::new()),
                    )
                })
            } else {
                meta.tree.goto_parent();
                Ok(Inline::Link(
                    (
                        String::new(),
                        Vec::new(),
                        vec![("link_type".to_string(), link_type)],
                    ),
                    vec![Inline::Str(description)],
                    (target, String::new()),
                ))
            };
        } else {
            meta.tree.goto_parent();
        }
    }
    Ok(Inline::Link(
        (String::new(), Vec::new(), Vec::new()),
        Vec::new(),
        (String::new(), String::new()),
    ))
}

pub fn anchor_declaration(meta: &mut Meta) -> Result<Inline, Utf8Error> {
    meta.tree.goto_first_child();
    let id = meta.tree.node().utf8_text(meta.source)?;
    let description = super::parse(meta)?;
    meta.tree.goto_parent();

    Ok(Inline::Link(
        (
            String::new(),
            Vec::new(),
            vec![
                (String::from("anchor"), String::from("declaration")),
                (String::from("anchor_id"), id.to_string()),
            ],
        ),
        description,
        (String::new(), String::new()),
    ))
}

pub fn anchor_definition(meta: &mut Meta) -> Result<Inline, Utf8Error> {
    meta.tree.goto_first_child();
    let id = meta.tree.node().utf8_text(meta.source)?;
    let description = super::parse(meta)?;
    meta.tree.goto_next_sibling();
    meta.tree.goto_first_child();
    let (target, _, link_type) = get_target(meta)?;
    meta.tree.goto_parent();
    meta.tree.goto_parent();

    Ok(Inline::Link(
        (
            String::new(),
            Vec::new(),
            vec![
                (String::from("anchor"), String::from("definition")),
                (String::from("link_type"), link_type),
                (String::from("anchor_id"), id.to_string()),
            ],
        ),
        description,
        (target, String::new()),
    ))
}

pub fn link_anchors(blocks: &mut [Block]) {
    fn get_anchor_definitions_in_blocks(
        blocks: &[Block],
        anchor_definition: &mut HashMap<String, (String, String)>,
    ) {
        blocks.iter().for_each(|block| match block {
            Block::Plain(inlines) | Block::Para(inlines) | Block::Header(_, _, inlines) => {
                get_anchor_definitions_in_inlines(inlines, anchor_definition)
            }
            Block::LineBlock(inlines) => inlines
                .iter()
                .for_each(|inlines| get_anchor_definitions_in_inlines(inlines, anchor_definition)),
            Block::CodeBlock(_, _)
            | Block::RawBlock(_, _)
            | Block::HorizontalRule
            | Block::Null => { /* ignore */ }
            Block::BlockQuote(blocks) | Block::Figure(_, _, blocks) | Block::Div(_, blocks) => {
                get_anchor_definitions_in_blocks(blocks, anchor_definition);
            }
            Block::OrderedList(_, blocks) | Block::BulletList(blocks) => blocks
                .iter()
                .for_each(|blocks| get_anchor_definitions_in_blocks(blocks, anchor_definition)),
            Block::DefinitionList(definitions) => {
                definitions.iter().for_each(|(inlines, blocks)| {
                    get_anchor_definitions_in_inlines(inlines, anchor_definition);
                    blocks.iter().for_each(|blocks| {
                        get_anchor_definitions_in_blocks(blocks, anchor_definition)
                    });
                })
            }
            Block::Table(_, _, _, _, _, _) => todo!(),
        });
    }
    fn get_anchor_definitions_in_inlines(
        inlines: &[Inline],
        anchor_definition: &mut HashMap<String, (String, String)>,
    ) {
        inlines.iter().for_each(|inline| match inline {
            Inline::Str(_)
            | Inline::Code(_, _)
            | Inline::Space
            | Inline::SoftBreak
            | Inline::LineBreak
            | Inline::Math(_, _)
            | Inline::RawInline(_, _) => { /* Ignore */ }
            Inline::Emph(inlines)
            | Inline::Underline(inlines)
            | Inline::Strong(inlines)
            | Inline::Strikeout(inlines)
            | Inline::Superscript(inlines)
            | Inline::Subscript(inlines)
            | Inline::SmallCaps(inlines)
            | Inline::Quoted(_, inlines)
            | Inline::Image(_, inlines, _)
            | Inline::Span(_, inlines) => {
                get_anchor_definitions_in_inlines(inlines, anchor_definition)
            }
            Inline::Cite(citations, inlines) => {
                citations.iter().for_each(|citation| {
                    get_anchor_definitions_in_inlines(&citation.citationPrefix, anchor_definition);
                    get_anchor_definitions_in_inlines(&citation.citationSuffix, anchor_definition);
                });
                get_anchor_definitions_in_inlines(inlines, anchor_definition);
            }
            Inline::Link(attributes, inlines, target) => {
                if attributes
                    .2
                    .contains(&(String::from("anchor"), String::from("definition")))
                {
                    anchor_definition.insert(
                        attributes
                            .2
                            .iter()
                            .find(|(key, _)| key == "anchor_id")
                            .map(|(_, value)| value.clone())
                            .expect("This always exists"),
                        (
                            target.0.clone(),
                            attributes
                                .2
                                .iter()
                                .find(|(key, _)| key == "link_type")
                                .map(|(_, value)| value.clone())
                                .expect("This always exists"),
                        ),
                    );
                }
                get_anchor_definitions_in_inlines(inlines, anchor_definition);
            }
            Inline::Note(blocks) => get_anchor_definitions_in_blocks(blocks, anchor_definition),
        });
    }
    fn set_anchor_definitions_in_blocks(
        blocks: &mut [Block],
        anchor_definition: &HashMap<String, (String, String)>,
    ) {
        blocks.iter_mut().for_each(|block| match block {
            Block::Plain(inlines) | Block::Para(inlines) | Block::Header(_, _, inlines) => {
                set_anchor_definitions_in_inlines(inlines, anchor_definition)
            }
            Block::LineBlock(inlines) => inlines
                .iter_mut()
                .for_each(|inlines| set_anchor_definitions_in_inlines(inlines, anchor_definition)),
            Block::CodeBlock(_, _)
            | Block::RawBlock(_, _)
            | Block::HorizontalRule
            | Block::Null => { /* ignore */ }
            Block::BlockQuote(blocks) | Block::Figure(_, _, blocks) | Block::Div(_, blocks) => {
                set_anchor_definitions_in_blocks(blocks, anchor_definition);
            }
            Block::OrderedList(_, blocks) | Block::BulletList(blocks) => blocks
                .iter_mut()
                .for_each(|blocks| set_anchor_definitions_in_blocks(blocks, anchor_definition)),
            Block::DefinitionList(definitions) => {
                definitions.iter_mut().for_each(|(inlines, blocks)| {
                    set_anchor_definitions_in_inlines(inlines, anchor_definition);
                    blocks.iter_mut().for_each(|blocks| {
                        set_anchor_definitions_in_blocks(blocks, anchor_definition)
                    });
                })
            }
            Block::Table(_, _, _, _, _, _) => todo!(),
        });
    }
    fn set_anchor_definitions_in_inlines(
        inlines: &mut [Inline],
        anchor_definition: &HashMap<String, (String, String)>,
    ) {
        inlines.iter_mut().for_each(|inline| match inline {
            Inline::Str(_)
            | Inline::Code(_, _)
            | Inline::Space
            | Inline::SoftBreak
            | Inline::LineBreak
            | Inline::Math(_, _)
            | Inline::RawInline(_, _) => { /* Ignore */ }
            Inline::Emph(inlines)
            | Inline::Underline(inlines)
            | Inline::Strong(inlines)
            | Inline::Strikeout(inlines)
            | Inline::Superscript(inlines)
            | Inline::Subscript(inlines)
            | Inline::SmallCaps(inlines)
            | Inline::Quoted(_, inlines)
            | Inline::Image(_, inlines, _)
            | Inline::Span(_, inlines) => {
                set_anchor_definitions_in_inlines(inlines, anchor_definition)
            }
            Inline::Cite(citations, inlines) => {
                citations.iter_mut().for_each(|citation| {
                    set_anchor_definitions_in_inlines(
                        &mut citation.citationPrefix,
                        anchor_definition,
                    );
                    set_anchor_definitions_in_inlines(
                        &mut citation.citationSuffix,
                        anchor_definition,
                    );
                });
                set_anchor_definitions_in_inlines(inlines, anchor_definition);
            }
            Inline::Link(attributes, inlines, target) => {
                if attributes
                    .2
                    .contains(&(String::from("anchor"), String::from("declaration")))
                {
                    let id = attributes
                        .2
                        .iter()
                        .find(|(key, _)| key == "anchor_id")
                        .map(|(_, value)| value)
                        .expect("This always exists");
                    if let Some((anchor_target, link_type)) = anchor_definition.get(id) {
                        attributes
                            .2
                            .push((String::from("link_type"), link_type.clone()));
                        target.0 = anchor_target.clone();
                    }
                }
                set_anchor_definitions_in_inlines(inlines, anchor_definition);
            }
            Inline::Note(blocks) => set_anchor_definitions_in_blocks(blocks, anchor_definition),
        });
    }

    let mut anchor_definitions = HashMap::new();
    get_anchor_definitions_in_blocks(blocks, &mut anchor_definitions);
    set_anchor_definitions_in_blocks(blocks, &anchor_definitions);
}
