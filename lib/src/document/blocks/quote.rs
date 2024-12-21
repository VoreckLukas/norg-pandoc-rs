use std::str::Utf8Error;

use pandoc_ast::Block;

use crate::Meta;

pub fn parse(meta: &mut Meta) -> Result<Block, Utf8Error> {
    if meta.tree.goto_first_child() {
        parse_quote(meta).map(|(mut quote, nesting)| {
            for _ in 1..nesting {
                quote = vec![Block::BlockQuote(quote)]
            }
            Block::BlockQuote(quote)
        })
    } else {
        unreachable!()
    }
}

fn parse_quote(meta: &mut Meta) -> Result<(Vec<Block>, usize), Utf8Error> {
    let mut content = Vec::new();

    let mut top_nesting = None;

    loop {
        let nesting = {
            let number_index = meta
                .tree
                .node()
                .kind()
                .chars()
                .position(|c| c.is_ascii_digit())
                .expect("Nesting is always in the kind");
            meta.tree.node().kind()[number_index..]
                .parse()
                .expect("This is always a number")
        };

        match top_nesting.as_mut() {
            Some(top_nesting) => {
                for _ in nesting..*top_nesting {
                    content = vec![Block::BlockQuote(content)]
                }
                *top_nesting = nesting;
            }
            None => top_nesting = Some(nesting),
        }

        if meta.tree.goto_first_child() && meta.tree.goto_next_sibling() {
            content.push(super::paragraph(meta)?)
        } else {
            unreachable!()
        };

        if meta.tree.goto_next_sibling() {
            let (mut quote, sub_nesting) = parse_quote(meta)?;
            for _ in nesting..sub_nesting - 1 {
                quote = vec![Block::BlockQuote(quote)]
            }
            content.push(Block::BlockQuote(quote));
        } else {
            meta.tree.goto_parent();
        }

        if !meta.tree.goto_next_sibling() {
            meta.tree.goto_parent();
            break;
        }
    }

    Ok((
        content,
        top_nesting.expect("At this point there should be a top nesting"),
    ))
}
