use crate::perlin::Perlin;

#[inline]
pub fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

#[inline]
pub fn dot2(u: [f64; 2], v: [f64; 2]) -> f64 {
    u[0] * v[0] + u[1] * v[1]
}

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
pub fn fbm(
    x: f64,
    y: f64,
    period: f64,
    hurst: f64,
    lacunarity: f64,
    contrast: f64,
    exp: f64,
    offset: f64,
    octaves: usize,
    ridges: i32,
    p: &Perlin,
) -> f64 {
    let mut total: f64 = 0.0;
    let mut amp_total: f64 = 0.0;
    let mut amp: f64 = 1.0;
    let mut freq: f64 = 1.0 / period;
    let gain: f64 = f64::powf(lacunarity, -hurst);
    for _ in 0..octaves {
        if ridges == 0 {
            total += amp * p.noise(x * freq, y * freq);
        } else if ridges == 1 {
            // ridges should use signed noise
            total += amp * p.snoise(x * freq, y * freq);
        } else {
            // valleys should use abs signed noise
            total += amp * p.snoise(x * freq, y * freq).abs();
        }
        amp_total += amp;
        amp *= gain;
        freq *= lacunarity;
    }
    total /= amp_total; // guarantee within the range 0-1
    if ridges == 1 {
        total = invert(total);
    }
    // c * total^exp + offset
    (apply_contrast(f64::powf(total, exp), contrast) + offset).clamp(0.0, 1.0)
}
