use std::iter;

use pandoc_ast::Block;

use crate::{Error, Meta, Result};

pub fn parse(meta: &mut Meta) -> Result<Block> {
    let nesting = {
        let number_index = meta
            .tree
            .node()
            .kind()
            .chars()
            .position(|c| c.is_ascii_digit())
            .ok_or(Error::MalformedTree("Encountered heading without nesting"))?;
        meta.tree.node().kind()[number_index..]
            .parse()
            .map_err(|_| Error::MalformedTree("Couldn't parse heading nesting"))?
    };

    let mut content = super::parse(meta)?.into_iter();

    let Some(Block::Para(heading)) = content.next() else {
        return Err(Error::MalformedTree(
            "Encountered heading without a paragraph as first element",
        ));
    };
    Ok(Block::Div(
        (String::new(), Vec::new(), Vec::new()),
        iter::once(Block::Header(
            nesting,
            (
                format!("heading{nesting}{}", {
                    meta.tree.goto_first_child();
                    meta.tree.goto_next_sibling();
                    let source = meta.tree.node().utf8_text(meta.source)?;
                    meta.tree.goto_parent();
                    source
                }),
                Vec::new(),
                Vec::new(),
            ),
            heading,
        ))
        .chain(content)
        .collect(),
    ))
}
