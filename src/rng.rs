use ck_meow::Meow;
use rand_core::RngCore;

pub const SEED_SIZE: usize = 32;
const CHALLENGE_RNG_CONTEXT: &[u8] = b"magikitten v0.1.0 challenge rng";

pub struct ChallengeRng {
    meow: Meow,
}

impl ChallengeRng {
    pub fn new(seed: &[u8; SEED_SIZE]) -> Self {
        let mut meow = Meow::new(CHALLENGE_RNG_CONTEXT);
        meow.key(seed, false);

        // This is a bit of a hack, so that we can use more = true for subsequent
        // operations.
        meow.prf(&mut [], false);

        Self { meow }
    }
}

impl RngCore for ChallengeRng {
    fn next_u32(&mut self) -> u32 {
        rand_core::impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        rand_core::impls::next_u64_via_fill(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.meow.prf(dest, true);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rng_is_prefix() {
        let seed = [0xFF; 32];
        let mut rng0 = ChallengeRng::new(&seed);
        let mut rng1 = ChallengeRng::new(&seed);

        let mut data0 = [0; 32];
        rng0.fill_bytes(&mut data0);
        let mut data1 = [0; 64];
        rng1.fill_bytes(&mut data1);

        assert_eq!(&data0, &data1[..32]);
    }
}
