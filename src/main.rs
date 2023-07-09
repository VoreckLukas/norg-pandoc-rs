use std::{env, fs};

use pandoc_ast::{Block, Inline, Map, Pandoc};
use tree_sitter::{Parser, TreeCursor};

fn main() {
    let language = tree_sitter_norg::language();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let file = env::args().nth(1).expect("No input file");
    let unparsed = fs::read_to_string(file).expect("Cannot read file");
    let tree = parser.parse(&unparsed, None).unwrap();

    #[cfg(debug_assertions)]
    {
        debug_tree(&mut tree.walk(), 0);
    }

    if let AstNode::Document(document) = parse_tree(&mut tree.walk(), &unparsed) {
        println!("{}", document.to_json());
    } else {
        unreachable!();
    };
}

enum AstNode {
    Document(Pandoc),
    Blocks(Vec<Block>),
    Inlines(Vec<Inline>),
}

fn parse_tree(walker: &mut TreeCursor, source: &str) -> AstNode {
    let node = walker.node();
    match node.kind() {
        "document" => {
            let blocks = if walker.goto_first_child() {
                if let AstNode::Blocks(blocks) = parse_tree(walker, source) {
                    blocks
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            AstNode::Document(Pandoc {
                meta: Map::default(),
                blocks,
                pandoc_api_version: vec![1, 23],
            })
        }

        "paragraph" => {
            let mut curblocks = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    vec![Block::Para(inlines)]
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            if walker.goto_next_sibling() {
                if let AstNode::Blocks(mut blocks) = parse_tree(walker, source) {
                    curblocks.append(&mut blocks);
                }
            } else {
                walker.goto_parent();
            }

            AstNode::Blocks(curblocks)
        }

        "paragraph_segment" => {
            let mut inlines = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            if walker.goto_next_sibling() {
                inlines.push(Inline::SoftBreak);
                if let AstNode::Inlines(mut new_inlines) = parse_tree(walker, source) {
                    inlines.append(&mut new_inlines);
                } else {
                    eprintln!(
                        "{} {}-{}",
                        walker.node().kind(),
                        walker.node().range().start_point,
                        walker.node().range().end_point
                    );
                    unreachable!()
                }
            } else {
                walker.goto_parent();
            }

            AstNode::Inlines(inlines)
        }

        "bold" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Strong(contents)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "italic" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Emph(contents)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "underline" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Underline(contents)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "strikethrough" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Strikeout(contents)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "spoiler" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Span(
                (String::default(), vec![String::from("spoiler")], vec![]),
                contents,
            )];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "superscript" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Superscript(contents)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "subscript" => {
            let contents = if walker.goto_first_child() {
                if let AstNode::Inlines(inlines) = parse_tree(walker, source) {
                    inlines
                } else {
                    unreachable!()
                }
            } else {
                vec![]
            };

            let mut inlines = vec![Inline::Subscript(contents)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "verbatim" => {
            let content = node.utf8_text(source.as_bytes()).unwrap();

            let mut inlines = vec![Inline::Code(
                (String::default(), vec![], vec![]),
                content[1..content.len() - 1].to_owned(),
            )];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "link" => {
            let target = if walker.goto_first_child()
                && walker.goto_first_child()
                && walker.goto_next_sibling()
                && walker.goto_next_sibling()
            {
                let target = walker
                    .node()
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_owned();

                walker.goto_parent();

                target
            } else {
                unreachable!()
            };

            let description = if walker.goto_next_sibling() {
                if walker.goto_first_child()
                    && walker.goto_next_sibling()
                    && walker.goto_first_child()
                {
                    let description = if let AstNode::Inlines(inlines) = parse_tree(walker, source)
                    {
                        inlines
                    } else {
                        unreachable!()
                    };

                    walker.goto_parent();

                    description
                } else {
                    unreachable!()
                }
            } else {
                vec![Inline::Str(target.clone())]
            };
            walker.goto_parent();

            let mut inlines = vec![Inline::Link(
                (String::default(), vec![], vec![]),
                description,
                (target, String::default()),
            )];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "escape_sequence" => {
            let text = if walker.goto_first_child() && walker.goto_next_sibling() {
                let text = walker
                    .node()
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_owned();
                walker.goto_parent();
                text
            } else {
                unreachable!()
            };

            let mut inlines = vec![Inline::Str(text)];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "_word" => {
            let mut inlines = vec![Inline::Str(
                node.utf8_text(source.as_bytes()).unwrap().to_owned(),
            )];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "_space" => {
            let mut inlines = vec![Inline::Space];

            parse_inlines(&mut inlines, walker, source);

            AstNode::Inlines(inlines)
        }

        "_paragraph_break" => {
            if walker.goto_next_sibling() {
                parse_tree(walker, source)
            } else {
                AstNode::Blocks(vec![])
            }
        }

        "_line_break" => {
            if walker.goto_next_sibling() {
                parse_tree(walker, source)
            } else {
                AstNode::Inlines(vec![])
            }
        }

        "_open" => {
            if !walker.goto_next_sibling() {
                unreachable!()
            }
            parse_tree(walker, source)
        }

        "_close" => {
            if walker.goto_next_sibling() {
                unreachable!()
            }
            walker.goto_parent();
            AstNode::Inlines(vec![])
        }

        _ => panic!("{} not implemented", node.kind()),
    }
}

fn parse_inlines(inlines: &mut Vec<Inline>, walker: &mut TreeCursor, source: &str) {
    if walker.goto_next_sibling() {
        if let AstNode::Inlines(mut new_inlies) = parse_tree(walker, source) {
            inlines.append(&mut new_inlies);
        } else {
            unreachable!()
        }
    } else {
        walker.goto_parent();
    }
}

#[cfg(debug_assertions)]
fn debug_tree(tree: &mut TreeCursor, indentlevel: usize) {
    let indent = " ".repeat(indentlevel * 3);
    eprintln!("{indent}{}", tree.node().kind());
    if tree.goto_first_child() {
        debug_tree(tree, indentlevel + 1);
    }
    if tree.goto_next_sibling() {
        debug_tree(tree, indentlevel);
    } else {
        tree.goto_parent();
    }
}
