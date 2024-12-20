use std::str::Utf8Error;

use pandoc_ast::Inline;

use crate::Meta;

#[derive(PartialEq, Eq)]
pub enum AttachedType {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Superscript,
    Subscript,
    Code,
}

pub fn parse(meta: &mut Meta, attached_type: AttachedType) -> Result<Inline, Utf8Error> {
    if attached_type != AttachedType::Code {
        let content = super::parse(meta)?;
        Ok(match attached_type {
            AttachedType::Bold => Inline::Strong(content),
            AttachedType::Italic => Inline::Emph(content),
            AttachedType::Underline => Inline::Underline(content),
            AttachedType::Strikethrough => Inline::Strikeout(content),
            AttachedType::Spoiler => Inline::Span(
                (String::default(), vec![String::from("spoiler")], Vec::new()),
                content,
            ),
            AttachedType::Superscript => Inline::Superscript(content),
            AttachedType::Subscript => Inline::Subscript(content),
            AttachedType::Code => unreachable!("Is handled earlier"),
        })
    } else {
        let content = {
            let content = meta.tree.node().utf8_text(meta.source).unwrap();
            content[1..content.len() - 1].to_owned()
        };

        Ok(Inline::Code(
            (String::default(), Vec::new(), Vec::new()),
            content,
        ))
    }
}
