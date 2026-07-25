pub mod perlin;

pub trait Noise {
    fn noise(&self, x: f64, y: f64) -> f64;
    fn snoise(&self, x: f64, y: f64) -> f64;
}
