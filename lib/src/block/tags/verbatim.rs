use std::{
    io::Write,
    process::{Command, Stdio},
};

use pandoc_ast::{Block, MetaValue};

use crate::Meta;

pub fn parse(parse_meta: &mut Meta) -> Block {
    if !parse_meta.tree.goto_first_child() || !parse_meta.tree.goto_next_sibling() {
        unreachable!()
    }

    if parse_meta.tree.node().utf8_text(parse_meta.source).unwrap() == "code" {
        code(parse_meta)
    } else if parse_meta.tree.node().utf8_text(parse_meta.source).unwrap() == "document.meta" {
        meta(parse_meta)
    } else {
        general(parse_meta)
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

pub fn meta(parse_meta: &mut Meta) -> Block {
    while parse_meta.tree.node().kind() != "ranged_verbatim_tag_content" {
        if !parse_meta.tree.goto_next_sibling() {
            unreachable!()
        }
    }

    if parse_meta.tree.goto_first_child() {
        if parse_meta.tree.goto_first_child() {
            loop {
                if parse_meta.tree.node().kind() == "paragraph_segment" {
                    let mut key_value = parse_meta
                        .tree
                        .node()
                        .utf8_text(parse_meta.source)
                        .unwrap()
                        .split(':');
                    let key = if let Some(key) = key_value.next() {
                        if key == "authors" {
                            String::from("author")
                        } else {
                            key.to_owned()
                        }
                    } else {
                        unreachable!()
                    };
                    let value = key_value.collect::<String>().trim().to_owned();
                    parse_meta
                        .metadata
                        .insert(key, MetaValue::MetaString(value));
                }

                if !parse_meta.tree.goto_next_sibling() {
                    break;
                }
            }
        }
        parse_meta.tree.goto_parent();
        parse_meta.tree.goto_parent();
    }

    parse_meta.tree.goto_parent();

    Block::Null
}

pub fn general(parse_meta: &mut Meta) -> Block {
    let mut classes = vec![parse_meta
        .tree
        .node()
        .utf8_text(parse_meta.source)
        .unwrap()
        .to_owned()];

    loop {
        if !parse_meta.tree.goto_next_sibling() || parse_meta.tree.node().kind() != "_space" {
            break;
        }
    }

    if parse_meta.tree.node().kind() == "tag_parameters" {
        if !parse_meta.tree.goto_first_child() {
            unreachable!()
        }

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
    }

    while parse_meta.tree.node().kind() != "ranged_verbatim_tag_content" {
        if !parse_meta.tree.goto_next_sibling() {
            unreachable!()
        }
    }

    let content = parse_meta.tree.node().utf8_text(parse_meta.source).unwrap();

    if classes.first().unwrap() == "table" {
        let mut pandoc_command = Command::new("pandoc")
            .arg("--from=gfm")
            .arg("--to=json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Couldn't spawn pandoc");
        let mut stdin = pandoc_command.stdin.take().unwrap();
        stdin
            .write_all(content.as_bytes())
            .expect("Couldnt write to pandocs stdin");
        stdin.flush().unwrap();
        drop(stdin);
        if let Ok(output) = pandoc_command.wait_with_output() {
            let output = String::from_utf8_lossy(&output.stdout);
            let table = pandoc_ast::Pandoc::from_json(&output).blocks;
            if table.len() == 1 && matches!(table.first().unwrap(), Block::Table(_, _, _, _, _, _))
            {
                parse_meta.tree.goto_parent();
                return Block::Div((String::new(), Vec::from(&classes[1..]), vec![]), table);
            }
        }
    }

    parse_meta.tree.goto_parent();

    Block::CodeBlock((String::new(), classes, vec![]), content.to_string())
}
