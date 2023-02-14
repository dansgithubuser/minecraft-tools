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
                x += dx;
                z += dz;
                for y in args.y_min..args.y_max {
                    if dim.block(x, y, z).has_name(&args.name) {
                        println!("found at ({}, {}, {})", x, y, z);
                        return Ok(());
                    }
                }
            }
        }
        x += 1;
        z += 1;
    }
    println!("not found");
    Ok(())
}
