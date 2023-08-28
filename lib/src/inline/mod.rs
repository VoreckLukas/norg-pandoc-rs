use std::collections::LinkedList;

use pandoc_ast::Inline;
use regex::Regex;

use crate::Meta;

pub(super) fn parse(parse_meta: &mut Meta, mut range: (usize, usize)) -> LinkedList<Inline> {
    let whitespace_pattern = Regex::new(r"\s+").unwrap();

    let mut inlines = if parse_meta.tree.node().range().start_byte > range.0 {
        let mut inlines = LinkedList::new();
        // The paragraph segments arent a tree sitter node but in between nodes
        let segment = String::from_utf8_lossy(
            &parse_meta.source[range.0..parse_meta.tree.node().start_byte()],
        );
        range.0 = parse_meta.tree.node().end_byte();

        whitespace_pattern.split(&segment).for_each(|word| {
            if !word.is_empty() {
                inlines.push_back(Inline::Str(word.to_string()));
                inlines.push_back(Inline::Space);
            }
        });
        inlines.pop_back();

        inlines
    } else {
        LinkedList::new()
    };

    match parse_meta.tree.node().kind() {
        "" => { /* ignore */ }

        "\n" => inlines.push_back(Inline::SoftBreak),

        _ => {
            eprintln!("{:?} not implemented", parse_meta.tree.node().kind());
            inlines.push_back(Inline::Str(
                parse_meta
                    .tree
                    .node()
                    .utf8_text(parse_meta.source)
                    .unwrap()
                    .to_owned(),
            ))
        }
    };

    if parse_meta.tree.goto_next_sibling() {
        let mut following_inlines = parse(parse_meta, range);
        inlines.append(&mut following_inlines);
    } else {
        if range.0 < range.1 {
            // The paragraph segments arent a tree sitter node but in between nodes
            let segment = String::from_utf8_lossy(&parse_meta.source[range.0..range.1]);

            whitespace_pattern.split(&segment).for_each(|word| {
                inlines.push_back(Inline::Str(word.to_string()));
                inlines.push_back(Inline::Space);
            });
            inlines.pop_back();
        }
        if let Some(back) = inlines.back() {
            if matches!(back, Inline::SoftBreak) {
                inlines.pop_back();
            }
        }
        parse_meta.tree.goto_parent();
    };

    inlines
}
