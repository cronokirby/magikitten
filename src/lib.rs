#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
mod rng;
mod transcript;

pub use rng::MeowRng;
pub use transcript::Transcript;
