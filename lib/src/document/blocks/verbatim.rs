use std::str::Utf8Error;

use pandoc_ast::Block;

use crate::Meta;

pub fn parse(meta: &mut Meta) -> Result<Block, Utf8Error> {
    meta.tree.goto_first_child();
    while meta.tree.node().kind() != "tag_name" {
        meta.tree.goto_next_sibling();
    }
    let tag_name = meta.tree.node().utf8_text(meta.source)?;

    let mut classes = Vec::new();
    while meta.tree.node().kind() != "ranged_verbatim_tag_content" {
        if meta.tree.node().kind() == "tag_parameters" {
            meta.tree.goto_first_child();
            loop {
                classes.push(meta.tree.node().utf8_text(meta.source)?.to_owned());
                if !meta.tree.goto_next_sibling() {
                    break;
                }
            }
            meta.tree.goto_parent();
        }
        meta.tree.goto_next_sibling();
    }
    let content = meta.tree.node().utf8_text(meta.source)?.to_owned();
    meta.tree.goto_parent();

    Ok(Block::CodeBlock(
        (
            String::new(),
            classes,
            vec![("tag_name".to_owned(), tag_name.to_owned())],
        ),
        content,
    ))
}
