use crate::util::{dot2, lerp};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::f64::consts::SQRT_2;

const GRADIENTS: [[f64; 2]; 4] = [[1.0, 1.0], [-1.0, 1.0], [1.0, -1.0], [-1.0, -1.0]];
const BOUND: f64 = 1.0 / SQRT_2;

#[derive(Debug)]
pub struct Perlin {
    P: Vec<usize>,
}

impl Perlin {
    // initialize P from given seed
    #[inline]
    pub fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut permutations: Vec<usize> = (0..256).collect();
        permutations.shuffle(&mut rng);
        permutations.extend_from_slice(&permutations.clone()); // repeat in case of overflow
        Perlin { P: permutations }
    }

    // determine gradient vectors and find dot product between distance and gradient
    #[inline]
    fn grad(&self, trunc: [f64; 2], unit: [usize; 2]) -> (f64, f64, f64, f64) {
        let [x, y] = trunc;
        let [ux, uy] = unit;

        // pick gradient vectors for each corner
        let gtl = GRADIENTS[self.P[self.P[ux] + uy] & 3];
        let gtr = GRADIENTS[self.P[self.P[ux + 1] + uy] & 3];
        let gbl = GRADIENTS[self.P[self.P[ux] + uy + 1] & 3];
        let gbr = GRADIENTS[self.P[self.P[ux + 1] + uy + 1] & 3];

        // dot(gradient, distance)
        let dtl = dot2(gtl, [x, y]);
        let dtr = dot2(gtr, [x - 1.0, y]);
        let dbl = dot2(gbl, [x, y - 1.0]);
        let dbr = dot2(gbr, [x - 1.0, y - 1.0]);

        (dtl, dtr, dbl, dbr)
    }

    // returns a noise value between -1 and 1
    #[inline]
    pub fn snoise(&self, x: f64, y: f64) -> f64 {
        // determine location within a box
        let trunc: [f64; 2] = [x - x.floor(), y - y.floor()];
        // determine in which box coordinates are located
        let unit: [usize; 2] = [(x as usize) & 255, (y as usize) & 255];
        // apply fade function to truncated coordinates
        let faded: [f64; 2] = trunc.map(|i| fade(i));

        let (dtl, dtr, dbl, dbr) = self.grad(trunc, unit);

        bilinear_interpolate(dtl, dtr, dbl, dbr, faded) / BOUND
    }

    // normalize the output of snoise to be between 0 and 1
    #[inline]
    pub fn noise(&self, x: f64, y: f64) -> f64 {
        normalize(self.snoise(x, y))
    }
}

// replace linear scale with a smoother function
#[inline]
fn fade(t: f64) -> f64 {
    ((6.0 * t - 15.0) * t + 10.0) * t * t * t
}

#[inline]
fn bilinear_interpolate(dtl: f64, dtr: f64, dbl: f64, dbr: f64, faded: [f64; 2]) -> f64 {
    let x1 = lerp(dtl, dtr, faded[0]);
    let x2 = lerp(dbl, dbr, faded[0]);

    lerp(x1, x2, faded[1])
}

#[inline]
fn normalize(x: f64) -> f64 {
    ((x + 1.0) / 2.0).clamp(0.0, 1.0)
}
