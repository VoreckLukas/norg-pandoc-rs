use std::str::Utf8Error;

use pandoc_ast::Pandoc;

use crate::Meta;

mod blocks;

pub fn parse(mut meta: Meta, pandoc_api_version: Vec<u32>) -> Result<Pandoc, Utf8Error> {
    blocks::parse(&mut meta).map(|blocks| Pandoc {
        meta: meta.metadata,
        blocks,
        pandoc_api_version,
    })
}
