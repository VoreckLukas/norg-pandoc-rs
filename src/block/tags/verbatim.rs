use pandoc_ast::Block;

use crate::Meta;

pub fn parse(parse_meta: &mut Meta) -> Block {
    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_next_sibling() {
        unreachable!()
    }

    if parse_meta.tree.node().utf8_text(parse_meta.source).unwrap() == "code" {
        code(parse_meta)
    } else {
        todo!()
    }
}

pub fn code(parse_meta: &mut Meta) -> Block {
    loop {
        if !parse_meta.tree.goto_next_sibling() || parse_meta.tree.node().kind() != "_space" {
            break;
        }
    }

    let parameters = if parse_meta.tree.node().kind() == "tag_parameters" {
        if !parse_meta.tree.goto_first_child() {
            unreachable!()
        }

        let mut classes = vec![];
        loop {
            if parse_meta.tree.node().kind() == "tag_param" {
                classes.push(
                    parse_meta
                        .tree
                        .node()
                        .utf8_text(parse_meta.source)
                        .unwrap()
                        .to_owned(),
                );
            }
            if !parse_meta.tree.goto_next_sibling() {
                break;
            }
        }

        parse_meta.tree.goto_parent();

        (String::new(), classes, vec![])
    } else {
        (String::new(), vec![], vec![])
    };

    while parse_meta.tree.node().kind() != "ranged_verbatim_tag_content" {
        if !parse_meta.tree.goto_next_sibling() {
            unreachable!()
        }
    }

    let content = parse_meta.tree.node().utf8_text(parse_meta.source).unwrap();

    parse_meta.tree.goto_parent();

    Block::CodeBlock(parameters, content.to_owned())
}
