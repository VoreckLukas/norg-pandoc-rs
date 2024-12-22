use blocks::inlines::link;
use pandoc_ast::{Map, Pandoc};

use crate::{Meta, Result};

mod blocks;

pub fn parse(mut meta: Meta, pandoc_api_version: Vec<u32>) -> Result<Pandoc> {
    blocks::parse(&mut meta).map(|mut blocks| {
        link::link_anchors(&mut blocks);
        Pandoc {
            meta: Map::new(),
            blocks,
            pandoc_api_version,
        }
    })
}
