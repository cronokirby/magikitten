use ck_meow::Meow;
use rand_core::{CryptoRng, RngCore};

/// The size of the seeds our RNG consumes, in bytes.
pub const SEED_SIZE: usize = 32;
/// A label we use to domain separate our RNG.
const CHALLENGE_RNG_CONTEXT: &[u8] = b"magikitten v0.1.0 challenge rng";

/// A pseudo-random number generator.
///
/// This RNG is initialized with a seed, and from that point generates bits
/// deterministically from that seed. Crucially, these bits are determined
/// solely by that seed, and not by how they're pulled from the RNG.
/// Pulling bytes by chunks of 8, or chunks of 16, or 32, etc. will yield
/// the same bytes.
///
/// Treating the RNG as an unstructured stream makes it more easy to produce consistent
/// results. Refactoring the code to use a larger buffer won't change the results,
/// for example.
pub struct MeowRng {
    meow: Meow,
}

impl MeowRng {
    /// Create a new RNG from a seed.
    pub fn new(seed: &[u8; SEED_SIZE]) -> Self {
        let mut meow = Meow::new(CHALLENGE_RNG_CONTEXT);
        meow.key(seed, false);

        // This is a bit of a hack, so that we can use more = true for subsequent
        // operations.
        meow.prf(&mut [], false);

        Self { meow }
    }
}

impl RngCore for MeowRng {
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

impl CryptoRng for MeowRng {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rng_is_prefix() {
        let seed = [0xFF; 32];
        let mut rng0 = MeowRng::new(&seed);
        let mut rng1 = MeowRng::new(&seed);

        let mut data0 = [0; 32];
        rng0.fill_bytes(&mut data0);
        let mut data1 = [0; 64];
        rng1.fill_bytes(&mut data1);

        assert_eq!(&data0, &data1[..32]);
    }
}
