use pandoc_ast::Inline;

use crate::{inline, Meta};

#[derive(PartialEq)]
pub(super) enum Modifier {
    Bold,
    Italic,
    Spoiler,
    Verbatim,
    Underline,
    Strikethrough,
    Superscript,
    Subscript,
}

pub(super) fn parse(meta: &mut Meta, modifier: Modifier) -> Inline {
    if modifier != Modifier::Verbatim {
        meta.tree.goto_first_child();
        let inlines = inline::parse(meta).into_iter().collect();

        match modifier {
            Modifier::Bold => Inline::Strong(inlines),
            Modifier::Italic => Inline::Emph(inlines),
            Modifier::Spoiler => Inline::Span(
                (String::default(), vec![String::from("spoiler")], vec![]),
                inlines,
            ),
            Modifier::Underline => Inline::Underline(inlines),
            Modifier::Strikethrough => Inline::Strikeout(inlines),
            Modifier::Superscript => Inline::Superscript(inlines),
            Modifier::Subscript => Inline::Subscript(inlines),
            Modifier::Verbatim => unreachable!(),
        }
    } else {
        meta.tree.goto_first_child();

        let start_byte = meta.tree.node().end_byte();

        while meta.tree.node().kind() != "verbatim_close" {
            meta.tree.goto_next_sibling();
        }

        let end_byte = meta.tree.node().start_byte();

        meta.tree.goto_parent();

        let text = String::from_utf8_lossy(&meta.source[start_byte..end_byte]);

        Inline::Code((String::default(), vec![], vec![]), text.to_string())
    }
}
