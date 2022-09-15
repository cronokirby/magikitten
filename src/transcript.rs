use ck_meow::Meow;

use crate::rng::{MeowRng, SEED_SIZE};

struct Transcript {
    meow: Meow,
}

impl Transcript {
    /// Generate a challenge given the transcript so far.
    ///
    /// This challenge takes the form of an infinite stream of bytes, represented
    /// as an RNG.
    pub fn challenge(&mut self) -> MeowRng {
        let mut seed = [0u8; SEED_SIZE];
        self.meow.prf(&mut seed, false);
        MeowRng::new(&seed)
    }
}
