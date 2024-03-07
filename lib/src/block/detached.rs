use pandoc_ast::Block;

use crate::Meta;

pub(super) enum Modifier {
    UnorderedList,
}

pub(super) fn parse(meta: &mut Meta, modifier: Modifier) -> Block {
    meta.tree.goto_first_child();

    let mut items = vec![];

    loop {
        items.push(super::parse(meta).into());

        if !meta.tree.goto_next_sibling() {
            break;
        }
    }

    meta.tree.goto_parent();
    match modifier {
        Modifier::UnorderedList => Block::BulletList(items),
    }
}
