# Magikitten

<img
 width="33%"
 align="right"
 src="https://cronokirby.com/projects/2022/magikitten/cover.jpg"/>

A system for making public-coin protocols non-interactive, using
[Meow](https://github.com/cronokirby/meow).

This library is also heavily inspired by [Merlin](https://merlin.cool),
and provides a similar construction, albeit with some differences.

# Usage

The essence of using the library is that you initialize a `Transcript` object,
and then alternate between feeding in data and extracting out randomness:

```rust
use rand_core::RngCore;
use magikitten::Transcript;

let mut transcript = Transcript::new(b"my cool protocol");
transcript.message(b"message0", b"hello world!");
let c0 = transcript.challenge(b"challenge0").next_u64();
transcript.message(b"message1", b"hello world again!");
let c1 = transcript.challenge(b"challenge1").next_u64();
```

The `challenge` function ratchets the state of the transcript, and then
gives you an object implementing `RngCore`, allowing you to extract arbitrary
randomness.
This randomness depends (unpredictably) on the state of the transcript so far, but is otherwise
deterministic.
The label you use to create the challenge RNG also affects the transcript,
but then the state of the RNG is independent, and the way you use the RNG
doesn't affect the transcript.
Implementing `RngCore` lets you use the challenge RNG to generate arbitrarily
complex objects.
For example, you can do rejection sampling, or other tricky techniques.

## Sequential Composition

In most situations, you should write schemes to accept a `Transcript`
as an argument, rather than creating it themselves.

This allows schemes to be composed sequentially, by having one scheme use
the running transcript that another scheme interacted with.

# Differences with Merlin

This library is heavily inspired by [Merlin](https://merlin.cool),
which essentially serves the same use case.
There are a few differences though.

### Magikitten uses a reduced round permutation

Rather than using Keccak with 24 rounds, we use KitTen, which is Keccak
with only 10 rounds.
This makes the protocol faster, but obviously more vulnerable to potential
advances in the cryptanalysis of Keccak.

See [Meow](https://github.com/cronokirby/meow) for some more rationale
on using KitTen.

### Challenges are a stream of bytes

Merlin requires you to specify the length of a challenge, and it just generates
a challenge of that length.
Magikitten, on the other hand, gives you an arbitrary RNG object.
I think this makes the library easier to use, since many functions
can easily be written to accept an object implementing the `RngCore` trait.
Some sampling algorithms, like rejection sampling, don't work well
if you need to know the length of the bits you need to sample in advance.

Also, the RNG object is seeded from the transcript, but then becomes independent.
With an independent state, the way you query bits of the RNG doesn't affect
the rest of the transcript.
This is more intuitive, since seemingly equivalent behavior can't change
the results.
For example, if you do rejection sampling using a buffer of bytes,
increasing this buffer size won't affect the results you get with Magikitten,
because the RNG can be treated as simply an infinite stream of bytes.

With Merlin, the length of each RNG query is recorded, and so different query
patterns for the RNG change the result.

### No support for private RNG

At the moment, Magikitten doesn't support private randomness.
This is a feature that might be added in the future though.
