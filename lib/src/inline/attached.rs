use pandoc_ast::Inline;

use crate::{inline, Meta};

pub enum Type {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Superscript,
    Subscript,
    Code,
}

pub(super) fn parse(parse_meta: &mut Meta, attached_type: Type) -> Inline {
    if !matches!(attached_type, Type::Code) {
        let content = if parse_meta.tree.goto_first_child() {
            inline::parse(parse_meta).into_iter().collect()
        } else {
            // There should be contents
            unreachable!()
        };

        match attached_type {
            Type::Bold => Inline::Strong(content),
            Type::Italic => Inline::Emph(content),
            Type::Underline => Inline::Underline(content),
            Type::Strikethrough => Inline::Strikeout(content),
            Type::Spoiler => Inline::Span(
                (String::default(), vec![String::from("spoiler")], vec![]),
                content,
            ),
            Type::Superscript => Inline::Superscript(content),
            Type::Subscript => Inline::Subscript(content),
            Type::Code => unreachable!(),
        }
    } else {
        let content = {
            let content = parse_meta.tree.node().utf8_text(parse_meta.source).unwrap();
            content[1..content.len() - 1].to_owned()
        };

        Inline::Code((String::default(), vec![], vec![]), content)
    }
}
