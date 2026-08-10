//! Covering designs `C(v, k, t)`.
//!
//! A covering design is a family of `k`-subsets ("blocks") of a `v`-set such that
//! every `t`-subset lies inside at least one block. The covering number is the
//! least number of blocks that suffices.
//!
//! The search is posed at a *fixed* block count `b`: maximise the number of
//! covered `t`-subsets. Full coverage at some `b` establishes an upper bound of
//! `b` on the covering number, so improving a record means succeeding at a `b`
//! strictly below the published value. That comparison is left to the caller
//! rather than folded into [`Problem::published_best`], because the two numbers
//! measure different things and conflating them is exactly how a false record
//! claim gets made.

use crate::problem::{Problem, Verdict};
use crate::Rng;

pub struct CoveringDesign {
    pub v: u32,
    pub k: u32,
    pub t: u32,
    /// Number of blocks the search is allowed to use.
    pub blocks: usize,
    targets: Vec<u64>,
}

impl CoveringDesign {
    pub fn new(v: u32, k: u32, t: u32, blocks: usize) -> Self {
        assert!(v <= 63, "v must fit in a u64 mask");
        assert!(t <= k && k <= v, "require t <= k <= v");
        let targets = subsets_of_size(v, t);
        CoveringDesign {
            v,
            k,
            t,
            blocks,
            targets,
        }
    }

    /// Total number of `t`-subsets that must be covered.
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Whether this family covers every `t`-subset.
    pub fn is_complete(&self, object: &[u64]) -> bool {
        self.score(&object.to_vec()) == self.targets.len() as i64
    }

    fn random_block(&self, rng: &mut Rng) -> u64 {
        let mut elems: Vec<u32> = (0..self.v).collect();
        rng.shuffle(&mut elems);
        elems
            .iter()
            .take(self.k as usize)
            .fold(0u64, |m, &e| m | (1u64 << e))
    }
}

/// All bitmasks over `0..v` with exactly `size` bits set, in ascending order.
pub fn subsets_of_size(v: u32, size: u32) -> Vec<u64> {
    let mut out = Vec::new();
    for mask in 0u64..(1u64 << v) {
        if mask.count_ones() == size {
            out.push(mask);
        }
    }
    out
}

impl Problem for CoveringDesign {
    type Object = Vec<u64>;

    fn name(&self) -> String {
        format!("covering-{}-{}-{}-b{}", self.v, self.k, self.t, self.blocks)
    }

    fn seed(&self, rng: &mut Rng) -> Vec<u64> {
        (0..self.blocks).map(|_| self.random_block(rng)).collect()
    }

    fn mutate(&self, object: &Vec<u64>, rng: &mut Rng) -> Vec<u64> {
        let mut next = object.clone();
        if next.is_empty() {
            return next;
        }
        let i = rng.below(next.len());
        let block = next[i];

        // Swap one element out for one not currently in the block, preserving
        // the popcount invariant so the result stays a legal k-subset.
        let inside: Vec<u32> = (0..self.v).filter(|&e| block & (1 << e) != 0).collect();
        let outside: Vec<u32> = (0..self.v).filter(|&e| block & (1 << e) == 0).collect();
        if inside.is_empty() || outside.is_empty() {
            return next;
        }
        let drop = inside[rng.below(inside.len())];
        let add = outside[rng.below(outside.len())];
        next[i] = (block & !(1u64 << drop)) | (1u64 << add);
        next
    }

    fn score(&self, object: &Vec<u64>) -> i64 {
        self.targets
            .iter()
            .filter(|&&target| object.iter().any(|&b| target & !b == 0))
            .count() as i64
    }

    fn is_valid(&self, object: &Vec<u64>) -> Verdict {
        let full = if self.v == 64 { !0u64 } else { (1u64 << self.v) - 1 };
        let ok = object.len() == self.blocks
            && object
                .iter()
                .all(|&b| b & !full == 0 && b.count_ones() == self.k);
        if ok {
            Verdict::Valid
        } else {
            Verdict::Invalid
        }
    }

    fn emit_lean(&self, object: &Vec<u64>) -> String {
        crate::certify::covering_lean(self, object)
    }
}
