use rand::{Rng, SeedableRng};

use rand_xorshift::XorShiftRng;

pub trait DepthFirst {
    fn depth_first_with_rng<R: Rng + ?Sized>(&mut self, rng: &mut R);

    fn depth_first_with_seed(&mut self, seed: [u8; 16]) {
        let mut rng = XorShiftRng::from_seed(seed);
        self.depth_first_with_rng(&mut rng);
    }
    fn depth_first_with_str_seed(&mut self, seed: &str) {
        let mut seed_u8 = [0; 16];
        for (index, byte) in seed.as_bytes().iter().enumerate() {
            seed_u8[index % 16] = *byte;
        }
        let mut rng = XorShiftRng::from_seed(seed_u8);
        self.depth_first_with_rng(&mut rng);
    }

    fn depth_first(&mut self) {
        let mut rng = XorShiftRng::from_entropy();
        self.depth_first_with_rng(&mut rng);
    }
}