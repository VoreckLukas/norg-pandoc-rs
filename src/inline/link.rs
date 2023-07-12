use pandoc_ast::Inline;

use crate::{inline, Meta};

pub fn parse(parse_meta: &mut Meta) -> Inline {
    let (target, description) =
        if parse_meta.tree.goto_first_child() && parse_meta.tree.goto_first_child() {
            parse_target(parse_meta)
        } else {
            unreachable!()
        };

    let description = if parse_meta.tree.goto_parent()
        && parse_meta.tree.goto_next_sibling()
        && parse_meta.tree.goto_first_child()
    {
        inline::parse(parse_meta).into_iter().collect()
    } else {
        description
    };
    parse_meta.tree.goto_parent();

    Inline::Link((String::default(), vec![], vec![]), description, target)
}

fn parse_target(parse_meta: &mut Meta) -> ((String, String), Vec<Inline>) {
    while parse_meta.tree.node().kind() == "_begin" {
        parse_meta.tree.goto_next_sibling();
    }

    match parse_meta.tree.node().kind() {
        "link_target_url" => {
            if !parse_meta.tree.goto_next_sibling() {
                unreachable!()
            }

            let url = parse_meta
                .tree
                .node()
                .utf8_text(parse_meta.source)
                .unwrap()
                .to_owned();
            ((url.clone(), String::new()), vec![Inline::Str(url)])
        }

        "link_file_text" => {
            let mut file = parse_meta
                .tree
                .node()
                .utf8_text(parse_meta.source)
                .unwrap()
                .to_owned();
            let mut description = vec![Inline::Str(file.clone())];
            file.push_str(".norg");

            loop {
                if !parse_meta.tree.goto_next_sibling() || parse_meta.tree.node().kind() != "_end" {
                    break;
                }
            }

            if parse_meta.tree.node().kind() != "_end" {
                let ((sub_target, _), mut sub_description) = parse_target(parse_meta);
                file.push_str(&sub_target);
                description.push(Inline::Space);
                description.append(&mut sub_description);
            }

            ((file, String::new()), description)
        }

        "link_target_line_number" => {
            if !parse_meta.tree.goto_next_sibling() {
                unreachable!()
            }

            let line_number = parse_meta.tree.node().utf8_text(parse_meta.source).unwrap();
            (
                (format!("#L{line_number}"), String::new()),
                vec![Inline::Str(format!("Line {line_number}"))],
            )
        }

        s if s.starts_with("link_target_heading") => {
            let nesting = {
                let nesting_index = parse_meta
                    .tree
                    .node()
                    .kind()
                    .chars()
                    .position(|c| c.is_ascii_digit())
                    .unwrap();
                &parse_meta.tree.node().kind()[nesting_index..]
            };
            if !parse_meta.tree.goto_next_sibling() {
                unreachable!()
            }
            let heading: Vec<_> = inline::parse(parse_meta).into_iter().collect();
            let heading_id = {
                let mut heading_str = inline::to_string(&heading);
                heading_str.push_str(nesting);
                heading_str
            };

            ((format!("#{heading_id}"), String::new()), heading)
        }

        _ => todo!(),
    }
}
