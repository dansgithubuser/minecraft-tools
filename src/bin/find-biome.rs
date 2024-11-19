// Find a specific block near a specified location.

use minecraft_tools::DimCache;

use clap::Parser;

#[derive(Parser)]
struct Args {
    region_folder: String,
    x: isize,
    z: isize,
    name: String,
    #[arg(default_value_t = 256)]
    r_max: usize,
    #[arg(default_value_t = -64)]
    y_min: isize,
    #[arg(default_value_t = 256)]
    y_max: isize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut dim = DimCache::new(args.region_folder.into());
    let mut x = args.x;
    let mut z = args.z;
    for r in 1..args.r_max {
        println!("searching {} x/z from start", r);
        for (dx, dz) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            for _ in 0..(2 * r - 1) {
                x += dx * 16;
                z += dz * 16;
                dim.block(x, 64, z);
                println!("{}", dim.chunks[&(x.div_euclid(16), z.div_euclid(16))]
                    .and_then(|chunk| chunk.biome(0, 64, 0))
                    .and_then(|biome| format!("{:?}", biome)));
            }
        }
        x += 1;
        z += 1;
    }
    println!("not found");
    Ok(())
}
