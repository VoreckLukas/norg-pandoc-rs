use std::collections::LinkedList;

use pandoc_ast::Inline;
use regex::Regex;

use crate::Meta;

/// Paragraph segments aren't tree sitter nodes but in between nodes so we need
/// the start and end of where to check for segments and recursively
/// pass that into further checks
///
/// LinkedList is used because concatenating two lists *should* be cheap in theory
/// TODO benchmarking
pub(super) fn parse(meta: &mut Meta, mut range: (usize, usize)) -> LinkedList<Inline> {
    let whitespace_pattern = Regex::new(r"\s+").unwrap();

    let node = meta.tree.node();

    let mut inlines = if node.range().start_byte > range.0 {
        let mut inlines = LinkedList::new();

        let segment = String::from_utf8_lossy(&meta.source[range.0..node.start_byte()]);
        range.0 = node.end_byte();

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

    match node.kind() {
        "\n" => inlines.push_back(Inline::SoftBreak),

        _ => {
            eprintln!("{:?} not implemented", node.kind());
            inlines.push_back(Inline::Str(node.utf8_text(meta.source).unwrap().to_owned()))
        }
    }

    if meta.tree.goto_next_sibling() {
        let mut following = parse(meta, range);
        inlines.append(&mut following);
    } else {
        if range.0 < range.1 {
            let segment = String::from_utf8_lossy(&meta.source[range.0..range.1]);

            whitespace_pattern.split(&segment).for_each(|word| {
                if !word.is_empty() {
                    inlines.push_back(Inline::Str(word.to_string()));
                    inlines.push_back(Inline::Space);
                }
            });
            inlines.pop_back();
        }
        if let Some(back) = inlines.back() {
            if matches!(back, Inline::SoftBreak) {
                inlines.pop_back();
            }
        }
        meta.tree.goto_parent();
    }

    inlines
}
