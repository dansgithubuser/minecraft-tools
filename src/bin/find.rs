// Find and specific block near a specified location.

use minecraft_tools::DimCache;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Minecraft Block Finder")
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .args_from_usage(
            "<region folder>
            <x>
            <z>
            <name>
            --r-max",
        )
        .get_matches();
    let mut dim = DimCache::new(matches.value_of("region folder").unwrap().into());
    let mut x = matches.value_of("x").unwrap().parse::<isize>()?;
    let mut z = matches.value_of("z").unwrap().parse::<isize>()?;
    let name = matches.value_of("name").unwrap();
    let r_max = matches.value_of("r-max").unwrap_or("256").parse::<usize>()?;
    for r in 1..r_max {
        println!("searching {} x/z from start", r);
        for (dx, dz) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            for _ in 0..(2 * r - 1) {
                x += dx;
                z += dz;
                for y in -64..256 {
                    if dim.block(x, y, z).has_name(name) {
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
