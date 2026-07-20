use anyhow::{Context, Result};
use std::fs;
use crate::util::*;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub image: Image,
    pub fbm: Fbm,
    pub options: Options,
}

#[serde_inline_default]
#[derive(Deserialize, Debug, Default)]
pub struct Image {
    #[serde_inline_default(1)]
    pub length: usize,
    #[serde_inline_default(1)]
    pub width: usize
}

#[serde_inline_default]
#[derive(Deserialize, Debug, Default)]
pub struct Fbm {
    #[serde_inline_default(1.0)]
    pub period: f64,
    #[serde_inline_default(0.5)]
    pub hurst: f64,
    #[serde_inline_default(2.0)]
    pub lacunarity: f64,
    #[serde_inline_default(1)]
    pub octaves: usize,
} 

#[serde_inline_default]
#[derive(Deserialize, Debug, Default)]
pub struct Options {
    #[serde_inline_default(String::from(""))]
    seed: String,
    #[serde_inline_default(String::from("perlin"))]
    noise: String,
    #[serde_inline_default(0)]
    pub mode: usize,
    #[serde_inline_default(1.0)]
    pub contrast: f64,
    #[serde_inline_default(1.0)]
    pub exp: f64,
    #[serde_inline_default(0.0)]
    pub offset: f64,
}

impl Config {
    pub fn from(fd: &str) -> Result<(Self, u64, usize)> {
        // deserialize
        let config: Config = toml::from_str(
            &(fs::read_to_string(fd)
            .with_context(|| format!("Failed to read file {}", fd))?)
        ).unwrap();

        // convert seed to u64 or generate
        let seed = if config.options.seed == "" {
            random_seed()
        } else {
            config.options.seed.parse::<u64>()
                .unwrap_or(hash_seed(&config.options.seed))
        };

        // set noise flag based on string
        let noise = 0;

        Ok((config, seed, noise))
    }
}
