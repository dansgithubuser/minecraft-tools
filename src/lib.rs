use fastanvil::{CCoord, Chunk, JavaChunk, RCoord, Region, RegionFileLoader, RegionLoader};

use std::collections::HashMap;
use std::path::PathBuf;

pub enum BlockResult<'a> {
    NoRegion,
    NoChunk,
    NoBlock,
    Block(&'a fastanvil::Block),
}

impl BlockResult<'_> {
    pub fn short_name(&self) -> &str {
        match self {
            BlockResult::NoRegion => "no-region",
            BlockResult::NoChunk => "no-chunk",
            BlockResult::NoBlock => "no-block",
            BlockResult::Block(block) => &block.name()[10..],
        }
    }

    pub fn has_name(&self, item_id: &str) -> bool {
        match self {
            BlockResult::Block(block) => block.name() == item_id,
            _ => false,
        }
    }
}

pub struct DimCache {
    loader: RegionFileLoader<JavaChunk>,
    regions: HashMap<(isize, isize), Option<Box<dyn Region<JavaChunk>>>>,
    chunks: HashMap<(isize, isize), Option<JavaChunk>>,
}

impl DimCache {
    pub fn new(region_folder: PathBuf) -> Self {
        Self {
            loader: RegionFileLoader::<JavaChunk>::new(region_folder),
            regions: HashMap::<(isize, isize), Option<Box<dyn Region<JavaChunk>>>>::new(),
            chunks: HashMap::<(isize, isize), Option<JavaChunk>>::new(),
        }
    }

    pub fn block(&mut self, x: isize, y: isize, z: isize) -> BlockResult {
        let chunk_x = x.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        if !self.chunks.contains_key(&(chunk_x, chunk_z)) {
            let region_x = chunk_x.div_euclid(32);
            let region_z = chunk_z.div_euclid(32);
            if !self.regions.contains_key(&(region_x, region_z)) {
                self.regions.insert(
                    (region_x, region_z),
                    self.loader.region(RCoord(region_x), RCoord(region_z)),
                );
            }
            let region = match &self.regions[&(region_x, region_z)] {
                Some(v) => v,
                None => return BlockResult::NoRegion,
            };
            let chunk_rel_x = chunk_x.rem_euclid(32);
            let chunk_rel_z = chunk_z.rem_euclid(32);
            self.chunks.insert(
                (chunk_x, chunk_z),
                region.chunk(CCoord(chunk_rel_x), CCoord(chunk_rel_z)),
            );
        }
        let chunk = match &self.chunks[&(chunk_x, chunk_z)] {
            Some(v) => v,
            None => return BlockResult::NoChunk,
        };
        let block_rel_x = x.rem_euclid(16) as usize;
        let block_rel_z = z.rem_euclid(16) as usize;
        match chunk.block(block_rel_x, y, block_rel_z) {
            Some(block) => BlockResult::Block(&block),
            None => BlockResult::NoBlock,
        }
    }
}
