use crate::Rng;

/// What the search is allowed to conclude about a candidate.
///
/// There is deliberately no `Probably` variant. A candidate either satisfies the
/// defining predicate or it does not, and the check is total and decidable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Valid,
    Invalid,
}

/// A combinatorial search problem with a decidable validity predicate.
///
/// Implementors must satisfy one contract above all: `is_valid` is the *only*
/// authority on correctness, it is total, and it never consults anything outside
/// the object itself. Scores guide search; they do not establish anything.
pub trait Problem: Sync {
    /// The object under search — a set, a family of blocks, a colouring.
    type Object: Clone + Send;

    /// Human-readable identifier, e.g. `capset-4` or `covering-9-4-3`.
    fn name(&self) -> String;

    /// A random starting point. Need not be good; must be valid.
    fn seed(&self, rng: &mut Rng) -> Self::Object;

    /// Produce a neighbour. May return an invalid object — the caller checks.
    fn mutate(&self, object: &Self::Object, rng: &mut Rng) -> Self::Object;

    /// Objective, higher is better. Only compared, never interpreted.
    fn score(&self, object: &Self::Object) -> i64;

    /// The decidable defining predicate. Sole authority on correctness.
    fn is_valid(&self, object: &Self::Object) -> Verdict;

    /// Best value in the published literature, where one is known.
    ///
    /// This exists to keep the search honest. A result is only interesting
    /// relative to a citable baseline, and a baseline that is merely remembered
    /// is worse than none at all — see `docs/targets.md`.
    fn published_best(&self) -> Option<i64> {
        None
    }

    /// Emit a self-contained Lean 4 certificate for this object.
    ///
    /// The emitted text must depend on nothing but Lean core: a reader checks it
    /// by running `lake build`, not by trusting this program.
    fn emit_lean(&self, object: &Self::Object) -> String;
}

/// The seam where a language model may enter the loop.
///
/// It supplies *heuristics* — the parameters and moves that shape a local search —
/// and never candidate objects. The reason is economic as much as epistemic: one
/// proposal here buys minutes of CPU search rather than a single guess, which is
/// what makes the method viable on a small budget. See `docs/design.md`.
///
/// Nothing in this crate calls a model. The default implementation is fixed and
/// deterministic, so the substrate is testable and reproducible on its own.
pub trait HeuristicSource {
    /// How many elements to tear out before rebuilding, given the current plateau
    /// length. Returning a larger number escapes deeper local optima at higher cost.
    fn destruction_size(&self, plateau: u32, rng: &mut Rng) -> usize;

    /// Whether to accept a strictly worse neighbour, given how long we have stalled.
    fn accept_worse(&self, plateau: u32, rng: &mut Rng) -> bool;
}

/// A fixed, dependency-free heuristic: ruin-and-recreate with a plateau-scaled
/// destruction size and occasional sideways moves.
#[derive(Clone, Copy, Debug, Default)]
pub struct FixedHeuristic;

impl HeuristicSource for FixedHeuristic {
    fn destruction_size(&self, plateau: u32, rng: &mut Rng) -> usize {
        // Widen the tear as the search stalls, then reset. The cap keeps a long
        // plateau from degenerating into a full restart, which throws away
        // structure the island has already paid for.
        let scale = 1 + (plateau / 64).min(6) as usize;
        1 + rng.below(scale)
    }

    fn accept_worse(&self, plateau: u32, rng: &mut Rng) -> bool {
        plateau > 32 && rng.chance(1, 20)
    }
}
