use pandoc_ast::Block;

use crate::{Meta, Result};

mod heading;
pub mod inlines;
mod list;
mod quote;
mod verbatim;

pub fn parse(meta: &mut Meta) -> Result<Vec<Block>> {
    fn parse_block(meta: &mut Meta) -> Result<Option<Block>> {
        Ok(match meta.tree.node().kind() {
            "paragraph" => Some(paragraph(meta)?),
            "paragraph_segment" => Some(paragraph(meta)?),
            "generic_list" => Some(list::parse(meta)?),
            "quote" => Some(quote::parse(meta)?),
            s if s.contains("heading") && !s.contains("prefix") => Some(heading::parse(meta)?),
            "ranged_verbatim_tag" => Some(verbatim::parse(meta)?),

            s if s.contains("prefix") => {
                if meta.tree.goto_next_sibling() {
                    parse_block(meta)?
                } else {
                    None
                }
            }
            "strong_paragraph_delimiter"
            | "weak_paragraph_delimiter"
            | "_line_break"
            | "_paragraph_break" => {
                if meta.tree.goto_next_sibling() {
                    parse_block(meta)?
                } else {
                    None
                }
            }
            _ => todo!("{}", meta.tree.node().kind()),
        })
    }
    let mut blocks = Vec::new();
    if meta.tree.goto_first_child() {
        while let Some(block) = parse_block(meta)? {
            blocks.push(block);
            if !meta.tree.goto_next_sibling() {
                break;
            }
        }
        meta.tree.goto_parent();
    }
    Ok(blocks)
}

fn paragraph(meta: &mut Meta) -> Result<Block> {
    Ok(Block::Para(inlines::parse(meta)?))
}
