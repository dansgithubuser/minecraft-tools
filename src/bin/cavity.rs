// Find and plot cave connectivity from a given location.

use minecraft_tools::{BlockResult, DimCache};

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

const CODE_AIR: u8 = 0;
const CODE_TORCH: u8 = 1;
const CODE_BANNER: u8 = 2;
const CODE_OTHER: u8 = 255;

fn passable(block: &BlockResult) -> bool {
    let short_name = block.short_name();
    short_name == "air"
        || short_name.contains("torch")
        || short_name == "ladder"
        || short_name.contains("door")
        || short_name.contains("banner")
}

fn code(block: &BlockResult) -> u8 {
    let short_name = block.short_name();
    if short_name == "air" {
        return CODE_AIR;
    }
    if short_name.contains("torch") {
        return CODE_TORCH;
    }
    if short_name.contains("banner") {
        return CODE_BANNER;
    }
    CODE_OTHER
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Minecraft Cave Connectivity Plotter")
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .args_from_usage(
            "<region folder>
            <x>
            <z>",
        )
        .get_matches();
    let mut dim = DimCache::new(matches.value_of("region folder").unwrap().into());
    let x_c = matches.value_of("x").unwrap().parse::<isize>()?;
    let z_c = matches.value_of("z").unwrap().parse::<isize>()?;
    // check column of blocks below sea level at specified location, queue passable blocks
    let mut queue = VecDeque::<(isize, isize, isize)>::new();
    for y in 0..62 {
        if passable(&dim.block(x_c, y, z_c)) {
            queue.push_back((x_c, y, z_c));
        }
    }
    // process queue, accumulate cave blocks, and keep track of cave extent
    let mut cave = HashMap::<(isize, isize, isize), u8>::new();
    let mut x_i = x_c;
    let mut x_f = x_c;
    let mut z_i = z_c;
    let mut z_f = z_c;
    let mut progress = 0;
    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap(); for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    let x_n = x + dx;
                    let y_n = y + dy;
                    let z_n = z + dz;
                    if y_n > 62 {
                        continue;
                    }
                    if cave.contains_key(&(x_n, y_n, z_n)) {
                        continue;
                    }
                    let block = dim.block(x_n, y_n, z_n);
                    if passable(&block) {
                        queue.push_back((x_n, y_n, z_n));
                        cave.insert((x_n, y_n, z_n), code(&block));
                        x_i = min(x_i, x_n);
                        x_f = max(x_f, x_n);
                        z_i = min(z_i, z_n);
                        z_f = max(z_f, z_n);
                    }
                }
            }
        }
        if cave.len() - progress > 1000 {
            println!("{} blocks connected", cave.len());
            progress = cave.len();
        }
    }
    // plot
    let mut img = image::ImageBuffer::from_pixel(
        (x_f - x_i + 1) as u32,
        (z_f - z_i + 1) as u32,
        image::Rgba::<u8>([0, 0, 0, 255]),
    );
    let mut keep = HashSet::<(u32, u32)>::new();
    for ((x, y, z), code) in cave {
        // skip if kept
        let x_im = (x - x_i) as u32;
        let z_im = (z - z_i) as u32;
        if keep.contains(&(x_im, z_im)) {
            continue;
        }
        // calculate color and put pixel
        let w = 64 + y as u8 * 2;
        let mut r = w;
        let mut g = w;
        let mut b = w;
        if code == CODE_TORCH {
            r += 64;
            g += 64;
        }
        if code == CODE_BANNER {
            r = 255;
            g = 0;
            b = 255;
        }
        img.put_pixel(x_im, z_im, image::Rgba::<u8>([r, g, b, 255]));
        // keep pixel
        if code == CODE_BANNER {
            keep.insert((x_im, z_im));
        }
    }
    img.save("cavity.png").unwrap();
    Ok(())
}
