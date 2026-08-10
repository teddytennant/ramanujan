//! Island-model search for extremal combinatorial objects.
//!
//! The search substrate is deterministic and self-contained: no network, no model
//! calls, no external services. A language model, where one is used at all, enters
//! only through [`HeuristicSource`] — it proposes *search strategy*, never objects,
//! and never has a vote on whether a result is correct. Correctness is decided by
//! [`Problem::is_valid`] and, downstream, by a proof checker.
//!
//! See `docs/design.md` for why the seam is placed there.

pub mod certify;
pub mod island;
pub mod problem;
pub mod problems;

pub use island::{Archipelago, SearchConfig};
pub use problem::{HeuristicSource, Problem, Verdict};

/// xorshift64* — small, fast, and deterministic given a seed.
///
/// Determinism is a hard requirement, not a convenience: every published witness
/// must be reproducible from its seed alone.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Guard against the all-zero state, which is a fixed point of xorshift.
        Rng(if seed == 0 { 0x9E3779B97F4A7C15 } else { seed })
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    /// Uniform in `[0, n)`. Returns 0 for `n == 0`.
    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next_u64() % n as u64) as usize
    }

    #[inline]
    pub fn chance(&mut self, numerator: u32, denominator: u32) -> bool {
        debug_assert!(denominator > 0);
        (self.next_u64() % denominator as u64) < numerator as u64
    }

    /// In-place Fisher-Yates.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }
}
