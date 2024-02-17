use std::collections::LinkedList;

use pandoc_ast::Inline;

use crate::Meta;

mod attached;
mod link;

/// Paragraph segments aren't tree sitter nodes but in between nodes so we need
/// the start and end of where to check for segments and recursively
/// pass that into further checks
///
/// LinkedList is used because concatenating two lists *should* be cheap in theory
/// TODO benchmarking
pub(super) fn parse(meta: &mut Meta) -> LinkedList<Inline> {
    let node = meta.tree.node();

    let mut inlines = match node.kind() {
        "paragraph_break" | "soft_break" => LinkedList::from([Inline::SoftBreak]),
        "whitespace" => LinkedList::from([Inline::Space]),

        "bold" => LinkedList::from([attached::parse(meta, attached::Modifier::Bold)]),
        "italic" => LinkedList::from([attached::parse(meta, attached::Modifier::Italic)]),
        "underline" => LinkedList::from([attached::parse(meta, attached::Modifier::Underline)]),
        "strikethrough" => {
            LinkedList::from([attached::parse(meta, attached::Modifier::Strikethrough)])
        }
        "spoiler" => LinkedList::from([attached::parse(meta, attached::Modifier::Spoiler)]),
        "superscript" => LinkedList::from([attached::parse(meta, attached::Modifier::Superscript)]),
        "subscript" => LinkedList::from([attached::parse(meta, attached::Modifier::Subscript)]),
        "verbatim" => LinkedList::from([attached::parse(meta, attached::Modifier::Verbatim)]),

        "link" => LinkedList::from([link::parse(meta)]),

        "word" | "punctuation" => {
            LinkedList::from([Inline::Str(node.utf8_text(meta.source).unwrap().to_owned())])
        }
        "escape_sequence" => {
            let content = node.utf8_text(meta.source).unwrap();
            LinkedList::from([Inline::Str(content[1..].to_owned())])
        }

        s if s.ends_with("close") || s.ends_with("open") => {
            // Ignore opening and closing tags
            LinkedList::new()
        }

        _ => {
            eprintln!("{:?} not implemented", node.kind());
            LinkedList::from([Inline::Str(node.utf8_text(meta.source).unwrap().to_owned())])
        }
    };

    if meta.tree.goto_next_sibling() {
        let mut following = parse(meta);
        inlines.append(&mut following);
    } else {
        if let Some(back) = inlines.back() {
            if matches!(back, Inline::SoftBreak) {
                inlines.pop_back();
            }
        }
        meta.tree.goto_parent();
    }

    inlines
}
