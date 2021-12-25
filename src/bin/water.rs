// Just a demonstration. Make a top-down plot of water at y=62.

use minecraft_tools::{BlockResult, DimCache};

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
    let mut dim = DimCache::new(matches.value_of("region folder").unwrap().into());
    let x_c = matches.value_of("x").unwrap().parse::<isize>()?;
    let z_c = matches.value_of("z").unwrap().parse::<isize>()?;
    let size = matches.value_of("size").unwrap().parse::<usize>()? as isize;
    let x_i = x_c - size / 2;
    let x_f = x_i + size;
    let z_i = z_c - size / 2;
    let z_f = z_i + size;
    let mut img = image::ImageBuffer::new(size as u32, size as u32);
    for x in x_i..x_f {
        println!("rendering x={}, z={}..{}", x, z_i, z_f);
        for z in z_i..z_f {
            let mut r = 0;
            let mut b = 0;
            match dim.block(x, 62, z) {
                BlockResult::NoRegion => (),
                BlockResult::NoChunk => r = 128,
                BlockResult::NoBlock => r = 255,
                BlockResult::Block(block) => {
                    if block.name() == "minecraft:water" {
                        b = 255;
                    }
                }
            };
            img.put_pixel(
                (x - x_i) as u32,
                (z - z_i) as u32,
                image::Rgba::<u8>([r, 0, b, 255]),
            );
        }
    }
    img.save("water.png").unwrap();
    Ok(())
}
