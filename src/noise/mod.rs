pub mod perlin;
pub mod value;

use crate::util::lerp;

pub trait Noise {
    fn noise(&self, x: f64, y: f64) -> f64;
    fn snoise(&self, x: f64, y: f64) -> f64;
}
//
// replace linear scale with a smoother function
#[inline]
fn fade(t: f64) -> f64 {
    ((6.0 * t - 15.0) * t + 10.0) * t * t * t
}

#[inline]
fn bilinear_interpolate(tl: f64, tr: f64, bl: f64, br: f64, p: [f64; 2]) -> f64 {
    let x1 = lerp(tl, tr, p[0]);
    let x2 = lerp(bl, br, p[0]);

    lerp(x1, x2, p[1])
}
