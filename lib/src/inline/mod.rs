use std::collections::LinkedList;

use pandoc_ast::Inline;

use crate::Meta;

mod attached;
pub mod detached_extension;
pub mod link;

pub(super) fn parse(parse_meta: &mut Meta) -> LinkedList<Inline> {
    let mut inlines = match parse_meta.tree.node().kind() {
        "paragraph" | "paragraph_segment" => {
            if parse_meta.tree.goto_first_child() {
                parse(parse_meta)
            } else {
                LinkedList::new()
            }
        }

        "bold" => LinkedList::from([attached::parse(parse_meta, attached::Type::Bold)]),

        "italic" => LinkedList::from([attached::parse(parse_meta, attached::Type::Italic)]),

        "underline" => LinkedList::from([attached::parse(parse_meta, attached::Type::Underline)]),

        "strikethrough" => {
            LinkedList::from([attached::parse(parse_meta, attached::Type::Strikethrough)])
        }

        "spoiler" => LinkedList::from([attached::parse(parse_meta, attached::Type::Spoiler)]),

        "superscript" => {
            LinkedList::from([attached::parse(parse_meta, attached::Type::Superscript)])
        }

        "subscript" => LinkedList::from([attached::parse(parse_meta, attached::Type::Subscript)]),

        "verbatim" => LinkedList::from([attached::parse(parse_meta, attached::Type::Code)]),

        "link" => LinkedList::from([link::parse(parse_meta)]),

        "anchor_declaration" => LinkedList::from([link::parse_anchor_declaration(parse_meta)]),

        "anchor_definition" => LinkedList::from([link::parse_anchor_definition(parse_meta)]),

        "escape_sequence" => {
            if parse_meta.tree.goto_first_child() && parse_meta.tree.goto_next_sibling() {
                let char = parse_meta
                    .tree
                    .node()
                    .utf8_text(parse_meta.source)
                    .unwrap()
                    .to_owned();
                parse_meta.tree.goto_parent();
                LinkedList::from([Inline::Str(char)])
            } else {
                unreachable!()
            }
        }

        "inline_link_target" => {
            let content = if parse_meta.tree.goto_first_child() {
                parse(parse_meta).into_iter().collect()
            } else {
                vec![]
            };

            let id = format!("{} inl", to_string(&content));

            LinkedList::from([Inline::Span((id, vec![], vec![]), content)])
        }

        "_line_break" => LinkedList::from([Inline::SoftBreak]),

        "_word" => LinkedList::from([Inline::Str(
            parse_meta
                .tree
                .node()
                .utf8_text(parse_meta.source)
                .unwrap()
                .to_owned(),
        )]),

        "_begin" | "_end" | "_close" | "_open" => LinkedList::new(),

        "_space" => LinkedList::from([Inline::Space]),

        _ => {
            eprintln!("{} not implemented", parse_meta.tree.node().kind());
            LinkedList::from([Inline::Str(
                parse_meta
                    .tree
                    .node()
                    .utf8_text(parse_meta.source)
                    .unwrap()
                    .to_owned(),
            )])
        }
    };

    if parse_meta.tree.goto_next_sibling() {
        let mut following_inlines = parse(parse_meta);
        inlines.append(&mut following_inlines);
    } else {
        parse_meta.tree.goto_parent();
    };

    inlines
}

pub fn to_string(inlines: &[Inline]) -> String {
    let mut output = String::new();

    for inline in inlines {
        match inline {
            Inline::Code(_, s) | Inline::Str(s) => output.push_str(s),
            Inline::Span(_, m)
            | Inline::Link(_, m, _)
            | Inline::Emph(m)
            | Inline::Underline(m)
            | Inline::Strong(m)
            | Inline::Strikeout(m)
            | Inline::Superscript(m)
            | Inline::Subscript(m) => output.push_str(&to_string(m)),
            Inline::Space => output.push(' '),
            Inline::SoftBreak => output.push('\n'),
            _ => unreachable!(),
        }
    }

    output
}
