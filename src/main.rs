pub mod perlin;
pub mod util;

use crate::perlin::Perlin;
use crate::util::fbm;
use anyhow::{Result, bail};
use csv::Writer;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::env;

struct Cli {
    length: usize,
    width: usize,
    period: f64,
    hurst: f64,
    lacunarity: f64,
    octaves: usize,
    contrast: f64,
    exp: f64,
    offset: f64,
    ridges: i32,
    seed: u64,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 12 {
        bail!("Missing command line arguments");
    }
    let cli: Cli = Cli {
        length: args[1].parse::<usize>()?,
        width: args[2].parse::<usize>()?,
        period: args[3].parse::<f64>()?,
        hurst: args[4].parse::<f64>()?,
        lacunarity: args[5].parse::<f64>()?,
        octaves: args[6].parse::<usize>()?,
        contrast: args[7].parse::<f64>()?,
        exp: args[8].parse::<f64>()?,
        offset: args[9].parse::<f64>()?,
        ridges: args[10].parse::<i32>()?,
        seed: args[11].parse::<u64>()?,
    };

    let progress = ProgressBar::new((cli.length * cli.width) as u64);
    let p: Perlin = Perlin::new(cli.seed);

    println!("Building heightmap with seed {}...", cli.seed);
    let heightmap: Vec<f64> = (0..cli.width)
        .into_par_iter()
        .flat_map(|i| (0..cli.length).into_par_iter().map(move |j| (i, j)))
        .map(|(i, j)| {
            let val: f64 = fbm(
                i as f64,
                j as f64,
                cli.period,
                cli.hurst,
                cli.lacunarity,
                cli.contrast,
                cli.exp,
                cli.offset,
                cli.octaves,
                cli.ridges,
                &p,
            );
            progress.inc(1);
            val
        })
        .collect();
    progress.finish();
    println!("Done!\n");

    let progress = ProgressBar::new((cli.length * cli.width) as u64);

    println!("Saving to heightmap.csv...");
    let mut writer = Writer::from_path("heightmap.csv")?;
    for row in 0..cli.width {
        let row = &heightmap[cli.length * row..][..cli.length];
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
