use attached::AttachedType;
use pandoc_ast::Inline;

use crate::{Error, Meta, Result};

mod attached;
pub mod link;

pub fn parse(meta: &mut Meta) -> Result<Vec<Inline>> {
    pub fn parse_inline(meta: &mut Meta) -> Result<Option<Inline>> {
        Ok(match meta.tree.node().kind() {
            "paragraph" | "paragraph_segment" => Some(Inline::Span(
                (String::new(), Vec::new(), Vec::new()),
                parse(meta)?,
            )),

            "_line_break" => Some(Inline::SoftBreak),
            "_word" => Some(word(meta)?),
            "_space" => Some(Inline::Space),
            "escape_sequence" => Some(escape_sequence(meta)?),

            "bold" => Some(attached::parse(meta, AttachedType::Bold)?),
            "italic" => Some(attached::parse(meta, AttachedType::Italic)?),
            "underline" => Some(attached::parse(meta, AttachedType::Underline)?),
            "strikethrough" => Some(attached::parse(meta, AttachedType::Strikethrough)?),
            "spoiler" => Some(attached::parse(meta, AttachedType::Spoiler)?),
            "superscript" => Some(attached::parse(meta, AttachedType::Superscript)?),
            "subscript" => Some(attached::parse(meta, AttachedType::Subscript)?),
            "verbatim" => Some(attached::parse(meta, AttachedType::Code)?),

            "link" => Some(link::parse(meta)?),
            "anchor_declaration" => Some(link::anchor_declaration(meta)?),
            "anchor_definition" => Some(link::anchor_definition(meta)?),

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
            inlines.push(inline);
            if !meta.tree.goto_next_sibling() {
                break;
            }
        }
        meta.tree.goto_parent();
    }
    Ok(inlines)
}

pub fn word(meta: &mut Meta) -> Result<Inline> {
    meta.tree
        .node()
        .utf8_text(meta.source)
        .map(|word| Inline::Str(word.to_owned()))
        .map_err(Into::into)
}

pub fn escape_sequence(meta: &mut Meta) -> Result<Inline> {
    if !(meta.tree.goto_first_child() && meta.tree.goto_next_sibling()) {
        return Err(Error::MalformedTree(
            "Encountered an escape sequence without content",
        ));
    }
    let char = meta
        .tree
        .node()
        .utf8_text(meta.source)
        .map(str::to_owned)
        .map(Inline::Str)
        .map_err(Into::into);
    meta.tree.goto_parent();
    char
}
