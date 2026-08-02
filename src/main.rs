#![allow(non_snake_case)]

pub mod util;
pub mod parse;
pub mod fbm;
pub mod noise;

use crate::noise::*;
use crate::noise::perlin::Perlin;
use crate::noise::value::ValueNoise;
use crate::fbm::fbm;
use crate::parse::*;
use anyhow::{Result, Context, bail};
use csv::Writer;
use indicatif::ProgressBar;
use rayon::prelude::*;
use image::{ImageBuffer, Luma};
use std::env;

pub type Luma16 = Luma<u16>;
pub type Gray16Image = ImageBuffer<Luma16, Vec<u16>>;

fn run<N: Noise + Send + Sync>(n: &N, config: &Config) -> Gray16Image {
    let progress = ProgressBar::new((config.image.length * config.image.width) as u64);
    let heightmap = Gray16Image::from_par_fn(
        config.image.length as u32, 
        config.image.width as u32, 
        |i, j| {
            let val: f64 = fbm(
                i as f64,
                j as f64,
                config.fbm.period,
                config.fbm.hurst,
                config.fbm.lacunarity,
                config.fbm.octaves,
                config.options.contrast,
                config.options.exp,
                config.options.offset,
                config.options.mode,
                n,
            );
            progress.inc(1);
            let val_u16: u16 = (val * 65536.0) as u16;
            Luma16::from([val_u16])
        });
    progress.finish();
    heightmap
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Missing command line arguments");
    }

    let (config, seed) = Config::from(&args[1]).with_context(|| "Failed to generate config")?;

    println!("Building heightmap with seed {}...", seed);
    let heightmap = match config.options.noise.as_str() {
        "perlin" => run(&Perlin::new(seed), &config),
        "value" => run(&ValueNoise::new(seed), &config),
        _ => run(&Perlin::new(seed), &config)
    };
    println!("Saving image...");
    heightmap.save("heightmap.png").with_context(|| "Failed to save output image")?;
    println!("Done!");

    Ok(())
}
