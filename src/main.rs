pub mod perlin;
pub mod util;
pub mod parse;
pub mod fbm;

use crate::perlin::Perlin;
use crate::fbm::fbm;
use crate::parse::*;
use anyhow::{Result, Context, bail};
use csv::Writer;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Missing command line arguments");
    }

    let (config, seed, noise) = Config::from(&args[1]).with_context(|| "Failed to generate config")?;

    let p: Perlin = Perlin::new(seed); 
    let progress = ProgressBar::new((config.image.length * config.image.width) as u64);

    println!("Building heightmap with seed {}...", seed);
    let heightmap: Vec<f64> = (0..config.image.width)
        .into_par_iter()
        .flat_map(|i| (0..config.image.length).into_par_iter().map(move |j| (i, j)))
        .map(|(i, j)| {
            let val: f64 = fbm(
                i as f64,
                j as f64,
                config.fbm.period,
                config.fbm.hurst,
                config.fbm.lacunarity,
                config.options.contrast,
                config.options.exp,
                config.options.offset,
                config.fbm.octaves,
                config.options.mode,
                &p,
            );
            progress.inc(1);
            val
        })
        .collect();
    progress.finish();
    println!("Done!\n");

    let progress = ProgressBar::new((config.image.length * config.image.width) as u64);

    println!("Saving to heightmap.csv...");
    let mut writer = Writer::from_path("heightmap.csv")?;
    for row in 0..config.image.width {
        let row = &heightmap[config.image.length * row..][..config.image.length];
        for cell in row {
            writer.write_field(format!("{}", cell))?;
            progress.inc(1);
        }
        writer.write_record(None::<&[u8]>)?;
    }
    progress.finish();
    println!("Done!");

    Ok(())
}
