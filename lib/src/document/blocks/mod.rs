use std::str::Utf8Error;

use pandoc_ast::Block;

use crate::Meta;

mod inlines;

pub fn parse(meta: &mut Meta) -> Result<Vec<Block>, Utf8Error> {
    fn parse_block(meta: &mut Meta) -> Result<Option<Block>, Utf8Error> {
        Ok(match meta.tree.node().kind() {
            "paragraph" => Some(paragraph(meta)?),
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
    let inlines = if meta.tree.goto_first_child() {
        let inlines = inlines::parse(meta)?;
        meta.tree.goto_parent();
        inlines
    } else {
        Vec::new()
    };

    Ok(Block::Para(inlines))
}
