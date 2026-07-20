use std::hash::{DefaultHasher, Hash, Hasher};

#[inline]
pub fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

#[inline]
pub fn dot2(u: [f64; 2], v: [f64; 2]) -> f64 {
    u[0] * v[0] + u[1] * v[1]
}

#[inline]
pub fn random_seed() -> u64 {
    rand::random::<u64>()
}

#[inline]
pub fn hash_seed(str: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);

    hasher.finish()
}
