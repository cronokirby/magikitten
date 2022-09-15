use ck_meow::Meow;

use crate::rng::{MeowRng, SEED_SIZE};

fn serialize_len(len: usize) -> ([u8; 10], usize) {
    let mut len64 = u64::to_le(u64::try_from(len).expect("failed to convert length to u64"));

    let mut out = [0u8; 10];
    let mut size = 1u8;

    let mut lo = (len64 & 0x7F) as u8;
    for out_i in &mut out {
        *out_i = lo;
        len64 >>= 7;
        lo = (len64 & 0x7F) as u8;
        if lo == 0 {
            break;
        }
        *out_i |= 0x80;
        size += 1;
    }
    (out, size as usize)
}

pub struct Transcript {
    meow: Meow,
}

impl Transcript {
    /// Generate a challenge given the transcript so far.
    ///
    /// This challenge takes the form of an infinite stream of bytes, represented
    /// as an RNG.
    pub fn challenge(&mut self, label: &'static str) -> MeowRng {
        let mut seed = [0u8; SEED_SIZE];
        self.meow.prf(&mut seed, false);
        MeowRng::new(&seed)
    }
}
