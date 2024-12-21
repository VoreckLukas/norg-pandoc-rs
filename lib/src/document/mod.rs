use std::str::Utf8Error;

use blocks::inlines::link;
use pandoc_ast::Pandoc;

use crate::Meta;

mod blocks;

pub fn parse(mut meta: Meta, pandoc_api_version: Vec<u32>) -> Result<Pandoc, Utf8Error> {
    blocks::parse(&mut meta).map(|mut blocks| {
        link::link_anchors(&mut blocks);
        Pandoc {
            meta: meta.metadata,
            blocks,
            pandoc_api_version,
        }
    })
}
