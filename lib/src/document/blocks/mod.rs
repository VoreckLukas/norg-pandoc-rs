use std::str::Utf8Error;

use pandoc_ast::Block;

use crate::Meta;

mod inlines;
mod list;
mod quote;

pub fn parse(meta: &mut Meta) -> Result<Vec<Block>, Utf8Error> {
    fn parse_block(meta: &mut Meta) -> Result<Option<Block>, Utf8Error> {
        Ok(match meta.tree.node().kind() {
            "paragraph" => Some(paragraph(meta)?),
            "generic_list" => Some(list::parse(meta)?),
            "quote" => Some(quote::parse(meta)?),

            "_line_break" | "_paragraph_break" => {
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

fn paragraph(meta: &mut Meta) -> Result<Block, Utf8Error> {
    Ok(Block::Para(inlines::parse(meta)?))
}
