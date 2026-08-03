use crate::noise::*;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct ValueNoise {
    P: Vec<usize>,
}

impl ValueNoise {
    // initialize P from given seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut permutations: Vec<usize> = (0..256).collect();
        permutations.shuffle(&mut rng);
        permutations.extend_from_slice(&permutations.clone()); // repeat in case of overflow
        Self { P: permutations }
    }

    // determine gradient vectors and find dot product between distance and gradient
    #[inline]
    fn get_values(&self, unit: [usize; 2]) -> (f64, f64, f64, f64) {
        let [ux, uy] = unit;

        // pick gradient vectors for each corner
        let tl = self.P[self.P[ux] + uy];
        let tr = self.P[self.P[ux + 1] + uy];
        let bl = self.P[self.P[ux] + uy + 1];
        let br = self.P[self.P[ux + 1] + uy + 1];

        // dot(gradient, distance)
        let tlf = tl as f64 / 255.0;
        let trf = tr as f64 / 255.0;
        let blf = bl as f64 / 255.0;
        let brf = br as f64 / 255.0;

        (tlf, trf, blf, brf)
    }

}

impl Noise for ValueNoise {
    // returns a noise value between 0 and 1
    #[inline]
    fn noise(&self, x: f64, y: f64) -> f64 {
        // determine location within a box
        let internal: [f64; 2] = [x - x.floor(), y - y.floor()];
        // determine in which box coordinates are located
        let unit: [usize; 2] = [(x as usize) & 255, (y as usize) & 255];

        let (tl, tr, bl, br) = self.get_values(unit);

        let faded: [f64; 2] = internal.map(|i| fade(i));
        bilinear_interpolate(tl, tr, bl, br, faded)
    }

    // returns a noise value between -1 and 1
    #[inline]
    fn snoise(&self, x: f64, y: f64) -> f64 {
        (self.noise(x, y) * 2.0) - 1.0
    }
}
