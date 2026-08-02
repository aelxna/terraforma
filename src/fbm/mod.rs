use crate::noise::Noise;

#[inline]
fn apply_contrast(x: f64, c: f64) -> f64 {
    let centered = x - 0.5;
    (c * centered) + 0.5
}

#[inline]
fn invert(x: f64) -> f64 {
    1.0 - x.abs()
}

// for each octave, add noise of decreasing amplitude and increasing frequency
#[inline]
pub fn fbm<N: Noise>(
    x: f64,
    y: f64,
    period: f64,
    hurst: f64,
    lacunarity: f64,
    octaves: usize,
    contrast: f64,
    exp: f64,
    offset: f64,
    ridges: usize,
    n: &N,
) -> f64 {
    let mut total: f64 = 0.0;
    let mut amp_total: f64 = 0.0;
    let mut amp: f64 = 1.0;
    let mut freq: f64 = 1.0 / period;
    let gain: f64 = f64::powf(lacunarity, -hurst);
    for _ in 0..octaves {
        if ridges == 0 {
            total += amp * n.noise(x * freq, y * freq);
        } else if ridges == 1 {
            // ridges should use signed noise
            total += amp * n.snoise(x * freq, y * freq);
        } else {
            // valleys should use abs signed noise
            total += amp * n.snoise(x * freq, y * freq).abs();
        }
        amp_total += amp;
        amp *= gain;
        freq *= lacunarity;
    }
    total /= amp_total; // guarantee within the range 0-1
    if ridges == 1 {
        total = invert(total);
    }
    // add cases for common exp values
    let powered = match exp {
        1.0 => total,
        2.0 => total * total,
        _ => f64::powf(total, exp)
    };
    // c * total^exp + offset
    (apply_contrast(powered, contrast) + offset).clamp(0.0, 1.0)
}
