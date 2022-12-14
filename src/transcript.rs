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

/// Represents a transcript.
///
/// A transcript is, in essence, a way of making a public coin protocol
/// non-interactive. The transcript can absorb messages, and then produce
/// challenges based on the messages in response.
///
/// ## Basic Flow
///
/// The basic flow of using a transcript involves initializing it, adding
/// in messages, and then generating challenges:
///
/// ```rust
/// # use magikitten::Transcript;
/// # use rand_core::RngCore;
/// let mut transcript = Transcript::new(b"my protocol");
/// transcript.message(b"message0", b"hello world!");
/// let c0 = transcript.challenge(b"challenge0").next_u64();
/// transcript.message(b"message1", b"hello again!");
/// let c1 = transcript.challenge(b"challenge1").next_u64();
/// ```
///
/// Notice that you can mix adding messages and extracting challenges.
pub struct Transcript {
    meow: Meow,
}

impl Transcript {
    /// Return a cloned version of this transcript.
    fn cloned(&self) -> Self {
        Self {
            meow: self.meow.clone(),
        }
    }

    /// Initialize a new transcript.
    ///
    /// This also takes a string describing the protocol the transcript is
    /// being used for. This is used for domain separation.
    ///
    /// Note that for most situations, constructions should simply accept
    /// a transcript as input, rather than creating it themselves. This allows
    /// a scheme to be used in various contexts, including in sequential composition
    /// with other schemes.
    pub fn new(protocol: &'static [u8]) -> Self {
        let mut meow = Meow::new(protocol);
        // To prevent potential shenanigans if the protocol string overlaps
        // with subsequent metadata.
        meow.ratchet();
        Self { meow }
    }

    /// Add a message to this transcript.
    ///
    /// You can also add a label to distinguish this message from others.
    ///
    /// The labels used for different objects in a transcript should, ideally,
    /// be unique. It's ok if some labels are prefixes of others.
    pub fn message(&mut self, label: &'static [u8], data: &[u8]) {
        self.feed_meta_len(label.len(), false);
        self.meow.meta_ad(label, true);
        self.feed_meta_len(data.len(), true);
        self.meow.ad(data, false);
    }

    /// Generate a challenge given the transcript so far.
    ///
    /// This challenge takes the form of an infinite stream of bytes, represented
    /// as an RNG.
    pub fn challenge(&mut self, label: &'static [u8]) -> MeowRng {
        let mut seed = [0u8; SEED_SIZE];
        self.feed_meta_len(label.len(), false);
        self.meow.meta_ad(label, true);
        self.feed_meta_len(0, true);
        self.meow.prf(&mut seed, false);
        MeowRng::new(&seed)
    }

    /// Create a forked version of this transcript.
    ///
    /// This is often useful in the context of cryptographic protocols. You
    /// might want to verify multiple proofs generated at the some point
    /// in the transcript, but by different people. You can use this primitive
    /// to fork the transcript to check those proofs, with some domain separation
    /// identifying each person.
    /// 
    /// Forking without domain separation is intentionally not possible, to prevent
    /// potential misuse where the same randomness is generated in different contexts.
    pub fn forked(&self, label: &'static [u8], data: &[u8]) -> Self {
        let mut out = self.cloned();
        out.message(label, data);
        out
    }
}

impl Transcript {
    /// Feed in a length as metadata.
    fn feed_meta_len(&mut self, len: usize, more: bool) {
        let (data, size) = serialize_len(len);
        self.meow.meta_ad(&data[..size], more);
    }
}

#[cfg(test)]
mod test {
    use rand_core::RngCore;

    use super::{serialize_len, Transcript};

    #[test]
    fn test_serialize_len() {
        for size in 1..4 {
            let len = (1 << (7 * size)) - 1;
            let mut expected = [0u8; 10];
            for e_i in &mut expected[..size - 1] {
                *e_i = 0xFF;
            }
            expected[size - 1] = 0x7F;
            assert_eq!(serialize_len(len), (expected, size));
        }
    }

    #[test]
    fn test_changing_label_gives_different_results() {
        let mut t0 = Transcript::new(b"protocol");
        t0.message(b"label A", b"message");
        let x0 = t0.challenge(b"challenge").next_u64();

        let mut t1 = Transcript::new(b"protocol");
        t1.message(b"label B", b"message");
        let x1 = t0.challenge(b"challenge").next_u64();

        assert_ne!(x0, x1);
    }

    #[test]
    fn test_changing_message_gives_different_results() {
        let mut t0 = Transcript::new(b"protocol");
        t0.message(b"label", b"message A");
        let x0 = t0.challenge(b"challenge").next_u64();

        let mut t1 = Transcript::new(b"protocol");
        t1.message(b"label", b"message B");
        let x1 = t0.challenge(b"challenge").next_u64();

        assert_ne!(x0, x1);
    }

    #[test]
    fn test_changing_protocol_gives_different_results() {
        let mut t0 = Transcript::new(b"protocol A");
        t0.message(b"label", b"message");
        let x0 = t0.challenge(b"challenge").next_u64();

        let mut t1 = Transcript::new(b"protocol B");
        t1.message(b"label", b"message");
        let x1 = t0.challenge(b"challenge").next_u64();

        assert_ne!(x0, x1);
    }
}
