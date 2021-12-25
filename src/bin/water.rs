// Just a demonstration. Make a top-down plot of water at y=62.

use fastanvil::{CCoord, Chunk, JavaChunk, RCoord, Region, RegionFileLoader, RegionLoader};

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Minecraft Water Plotter")
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .args_from_usage(
            "<region folder>
            <x>
            <z>
            <size>",
        )
        .get_matches();
    let loader =
        RegionFileLoader::<JavaChunk>::new(matches.value_of("region folder").unwrap().into());
    let x_c = matches.value_of("x").unwrap().parse::<isize>()?;
    let z_c = matches.value_of("z").unwrap().parse::<isize>()?;
    let size = matches.value_of("size").unwrap().parse::<usize>()? as isize;
    let x_i = x_c - size / 2;
    let x_f = x_i + size;
    let z_i = z_c - size / 2;
    let z_f = z_i + size;
    let mut regions = HashMap::<(isize, isize), Option<Box<dyn Region<JavaChunk>>>>::new();
    let mut chunks = HashMap::<(isize, isize), Option<JavaChunk>>::new();
    let mut img = image::ImageBuffer::new(size as u32, size as u32);
    for x in x_i..x_f {
        println!("rendering x={}, z={}..{}", x, z_i, z_f);
        for z in z_i..z_f {
            let chunk_x = x.div_euclid(16);
            let chunk_z = z.div_euclid(16);
            if !chunks.contains_key(&(chunk_x, chunk_z)) {
                let region_x = chunk_x.div_euclid(32);
                let region_z = chunk_z.div_euclid(32);
                if !regions.contains_key(&(region_x, region_z)) {
                    println!("loading region {} {}", region_x, region_z);
                    regions.insert(
                        (region_x, region_z),
                        loader.region(RCoord(region_x), RCoord(region_z)),
                    );
                }
                let region = match &regions[&(region_x, region_z)] {
                    Some(v) => v,
                    None => continue,
                };
                let chunk_rel_x = chunk_x.rem_euclid(32);
                let chunk_rel_z = chunk_z.rem_euclid(32);
                chunks.insert(
                    (chunk_x, chunk_z),
                    region.chunk(CCoord(chunk_rel_x), CCoord(chunk_rel_z)),
                );
            }
            let chunk = match &chunks[&(chunk_x, chunk_z)] {
                Some(v) => v,
                None => continue,
            };
            let block_rel_x = x.rem_euclid(16) as usize;
            let block_rel_z = z.rem_euclid(16) as usize;
            let mut r = 0;
            let mut b = 0;
            match chunk.block(block_rel_x, 62, block_rel_z) {
                Some(v) => {
                    if v.name() == "minecraft:water" {
                        b = 255;
                    }
                }
                None => r = 255,
            };
            img.put_pixel((x - x_i) as u32, (z - z_i) as u32, image::Rgba::<u8>([r, 0, b, 255]));
        }
    }
    img.save("water.png").unwrap();
    Ok(())
}
