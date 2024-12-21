use std::{iter, str::Utf8Error};

use pandoc_ast::Block;

use crate::Meta;

pub fn parse(meta: &mut Meta) -> Result<Block, Utf8Error> {
    let nesting = {
        let number_index = meta
            .tree
            .node()
            .kind()
            .chars()
            .position(|c| c.is_ascii_digit())
            .expect("There is always a number in the heading kind");
        meta.tree.node().kind()[number_index..]
            .parse()
            .expect("This is always a number")
    };

    let mut content = super::parse(meta)?.into_iter();

    let Some(Block::Para(heading)) = content.next() else {
        unreachable!("First element is always a paragraph segment")
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
