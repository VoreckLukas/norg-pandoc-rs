use pandoc_ast::Inline;

use crate::Meta;

pub(super) fn parse(meta: &mut Meta) -> Inline {
    let mut target = None;
    let mut content = None;
    let attrs = (String::new(), vec![], vec![]);

    meta.tree.goto_first_child();

    loop {
        match meta.tree.node().kind() {
            "uri" => target = Some(meta.tree.node().utf8_text(meta.source).unwrap().to_owned()),
            "description" => {
                let inline = if meta.tree.goto_first_child() {
                    super::parse(meta).into_iter().collect()
                } else {
                    vec![]
                };
                content = Some(inline);
            }
            "{" | "}" | "[" | "]" => { /* Do nothing */ }
            _ => eprintln!("{} not implemented, ignoring", meta.tree.node().kind()),
        }

        if !meta.tree.goto_next_sibling() {
            break;
        }
    }

    meta.tree.goto_parent();

    Inline::Link(
        attrs,
        content.unwrap_or(vec![]),
        (target.unwrap_or(String::new()), String::new()),
    )
}
