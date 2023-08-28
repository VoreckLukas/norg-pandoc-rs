use pandoc_ast::Block;

use crate::{inline, Meta};

pub(super) fn parse(parse_meta: &mut Meta) -> Block {
    let range = (
        parse_meta.tree.node().start_byte(),
        parse_meta.tree.node().end_byte(),
    );
    let inline = if parse_meta.tree.goto_first_child() {
        inline::parse(parse_meta, range).into_iter().collect()
    } else {
        vec![]
    };

    Block::Para(inline)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use pandoc_ast::{Block, Inline};

    #[test]
    fn paragraph() {
        let input = "this\nis\na\nparagraph\n\nand so\nis   this\n\nand this";
        let parsed = crate::parse(input, "", vec![], Path::new(""));
        assert_eq!(
            parsed.blocks,
            vec![
                Block::Para(vec![
                    Inline::Str("this".to_owned()),
                    Inline::SoftBreak,
                    Inline::Str("is".to_owned()),
                    Inline::SoftBreak,
                    Inline::Str("a".to_owned()),
                    Inline::SoftBreak,
                    Inline::Str("paragraph".to_owned()),
                ]),
                Block::Para(vec![
                    Inline::Str("and".to_owned()),
                    Inline::Space,
                    Inline::Str("so".to_owned()),
                    Inline::SoftBreak,
                    Inline::Str("is".to_owned()),
                    Inline::Space,
                    Inline::Str("this".to_owned()),
                ]),
                Block::Para(vec![
                    Inline::Str("and".to_owned()),
                    Inline::Space,
                    Inline::Str("this".to_owned()),
                ])
            ]
        );
    }
}
