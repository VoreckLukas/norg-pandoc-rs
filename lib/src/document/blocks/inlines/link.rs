use std::str::Utf8Error;

use pandoc_ast::Inline;

use crate::Meta;

pub fn parse(meta: &mut Meta) -> Result<Inline, Utf8Error> {
    if meta.tree.goto_first_child() {
        if meta.tree.goto_first_child() {
            if meta.tree.goto_next_sibling() && meta.tree.goto_next_sibling() {
                return meta.tree.node().utf8_text(meta.source).and_then(|target| {
                    meta.tree.goto_parent();
                    if meta.tree.goto_next_sibling() {
                        super::parse(meta).map(|description| {
                            meta.tree.goto_parent();
                            Inline::Link(
                                (String::new(), Vec::new(), Vec::new()),
                                description,
                                (target.to_owned(), String::new()),
                            )
                        })
                    } else {
                        meta.tree.goto_parent();
                        Ok(Inline::Link(
                            (String::new(), Vec::new(), Vec::new()),
                            vec![Inline::Str(target.to_owned())],
                            (target.to_owned(), String::new()),
                        ))
                    }
                });
            } else {
                meta.tree.goto_parent();
                meta.tree.goto_parent();
            }
        } else {
            meta.tree.goto_parent();
        }
    }
    Ok(Inline::Link(
        (String::new(), Vec::new(), Vec::new()),
        Vec::new(),
        (String::new(), String::new()),
    ))
}
