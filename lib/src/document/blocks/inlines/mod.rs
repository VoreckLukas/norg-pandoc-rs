use std::str::Utf8Error;

use attached::AttachedType;
use either::Either;
use pandoc_ast::Inline;

use crate::Meta;

mod attached;
mod link;

pub fn parse(meta: &mut Meta) -> Result<Vec<Inline>, Utf8Error> {
    pub fn parse_inline(meta: &mut Meta) -> Result<Option<Either<Inline, Vec<Inline>>>, Utf8Error> {
        Ok(match meta.tree.node().kind() {
            "paragraph" | "paragraph_segment" => Some(Either::Right(parse(meta)?)),

            "_line_break" => Some(Either::Left(Inline::SoftBreak)),
            "_word" => Some(Either::Left(word(meta))),
            "_space" => Some(Either::Left(Inline::Space)),
            "escape_sequence" => Some(Either::Left(escape_sequence(meta)?)),

            "bold" => Some(Either::Left(attached::parse(meta, AttachedType::Bold)?)),
            "italic" => Some(Either::Left(attached::parse(meta, AttachedType::Italic)?)),
            "underline" => Some(Either::Left(attached::parse(
                meta,
                AttachedType::Underline,
            )?)),
            "strikethrough" => Some(Either::Left(attached::parse(
                meta,
                AttachedType::Strikethrough,
            )?)),
            "spoiler" => Some(Either::Left(attached::parse(meta, AttachedType::Spoiler)?)),
            "superscript" => Some(Either::Left(attached::parse(
                meta,
                AttachedType::Superscript,
            )?)),
            "subscript" => Some(Either::Left(attached::parse(
                meta,
                AttachedType::Subscript,
            )?)),
            "verbatim" => Some(Either::Left(attached::parse(meta, AttachedType::Code)?)),

            "link" => Some(Either::Left(link::parse(meta)?)),

            "_begin" | "_end" | "_open" | "_close" => {
                if meta.tree.goto_next_sibling() {
                    parse_inline(meta)?
                } else {
                    None
                }
            }

            _ => todo!("{}", meta.tree.node().kind()),
        })
    }

    let mut inlines = Vec::new();
    if meta.tree.goto_first_child() {
        while let Some(inline) = parse_inline(meta)? {
            match inline {
                Either::Left(inline) => inlines.push(inline),
                Either::Right(mut segment) => inlines.append(&mut segment),
            }
            if !meta.tree.goto_next_sibling() {
                break;
            }
        }
        meta.tree.goto_parent();
    }
    Ok(inlines)
}

pub fn word(meta: &mut Meta) -> Inline {
    Inline::Str(meta.tree.node().utf8_text(meta.source).unwrap().to_owned())
}

pub fn escape_sequence(meta: &mut Meta) -> Result<Inline, Utf8Error> {
    meta.tree.goto_first_child();
    meta.tree.goto_next_sibling();
    let char = meta
        .tree
        .node()
        .utf8_text(meta.source)
        .map(str::to_owned)
        .map(Inline::Str);
    meta.tree.goto_parent();
    char
}
