use std::collections::LinkedList;

use pandoc_ast::Inline;

use crate::Meta;

pub fn parse(parse_meta: &mut Meta) -> LinkedList<Inline> {
    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_next_sibling() {
        unreachable!()
    }
    let item = if parse_meta.tree.node().kind() == "todo_item_urgent" {
        LinkedList::from([
            Inline::Strong(vec![Inline::Span(
                (
                    String::default(),
                    vec!["todo".to_string(), "urgent".to_string()],
                    vec![],
                ),
                vec![Inline::Str("(!)".to_string())],
            )]),
            Inline::Space,
        ])
    } else {
        eprintln!("{}", parse_meta.tree.node().kind());
        todo!()
    };

    parse_meta.tree.goto_parent();
    item
}
