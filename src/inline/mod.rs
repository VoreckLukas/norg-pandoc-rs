use std::collections::LinkedList;

use pandoc_ast::Inline;

use crate::Meta;

mod attached;
mod link;

pub fn parse(parse_meta: &mut Meta) -> LinkedList<Inline> {
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
