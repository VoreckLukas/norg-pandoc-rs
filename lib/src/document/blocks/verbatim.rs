use pandoc_ast::Block;

use crate::{Error, Meta, Result};

pub fn parse(meta: &mut Meta) -> Result<Block> {
    if !meta.tree.goto_first_child() {
        return Err(Error::MalformedTree(
            "Encountered a code block without children",
        ));
    }
    while meta.tree.node().kind() != "tag_name" {
        meta.tree.goto_next_sibling();
    }
    let tag_name = meta.tree.node().utf8_text(meta.source)?;

    let mut classes = Vec::new();
    while meta.tree.node().kind() != "ranged_verbatim_tag_content" {
        if meta.tree.node().kind() == "tag_parameters" {
            if !meta.tree.goto_first_child() {
                return Err(Error::MalformedTree(
                    "Encountered verbatim tag_parameters element without children",
                ));
            }
            loop {
                classes.push(meta.tree.node().utf8_text(meta.source)?.to_owned());
                if !meta.tree.goto_next_sibling() {
                    break;
                }
            }
            meta.tree.goto_parent();
        }
        if !meta.tree.goto_next_sibling() {
            return Err(Error::MalformedTree(
                "Encountered a code block without content",
            ));
        }
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
