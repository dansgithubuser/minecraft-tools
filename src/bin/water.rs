// Just a demonstration. Make a top-down plot of water at y=62.

use minecraft_tools::{BlockResult, DimCache};

use clap::Parser;

#[derive(Parser)]
struct Args {
    region_folder: String,
    x: isize,
    z: isize,
    size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut dim = DimCache::new(args.region_folder.into());
    let x_c = args.x;
    let z_c = args.z;
    let size = args.size as isize;
    let x_i = x_c - size / 2;
    let x_f = x_i + size;
    let z_i = z_c - size / 2;
    let z_f = z_i + size;
    let mut img = image::ImageBuffer::new(size as u32, size as u32);
    for x in x_i..x_f {
        println!("rendering x={}, z={}..{}", x, z_i, z_f);
        for z in z_i..z_f {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            match dim.block(x, 62, z) {
                BlockResult::NoRegion => (),
                BlockResult::NoChunk => r = 128,
                BlockResult::NoBlock => r = 255,
                BlockResult::Block(block) => {
                    if block.name() == "minecraft:water" {
                        r = 128;
                        g = 128;
                        b = 255;
                    } else {
                        g = 128;
                    }
                }
            };
            img.put_pixel(
                (x - x_i) as u32,
                (z - z_i) as u32,
                image::Rgba::<u8>([r, g, b, 255]),
            );
        }
    }
    img.save("water.png").unwrap();
    Ok(())
}
