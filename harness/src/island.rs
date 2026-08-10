use crate::problem::{FixedHeuristic, HeuristicSource, Problem, Verdict};
use crate::Rng;

#[derive(Clone, Debug)]
pub struct SearchConfig {
    /// Number of isolated populations. Isolation is the mechanism that preserves
    /// diversity; a single global population converges early and stops finding
    /// anything. Six to ten is a reasonable range.
    pub islands: usize,
    /// Total mutation attempts across all islands.
    pub iterations: u64,
    /// Copy the global best into the weakest island this often. Zero disables
    /// migration entirely, which is the correct setting when measuring whether
    /// isolation is doing any work.
    pub migrate_every: u64,
    pub seed: u64,
}

impl Default for SearchConfig {
    fn default() -> Self {
        SearchConfig {
            islands: 8,
            iterations: 200_000,
            migrate_every: 20_000,
            seed: 1,
        }
    }
}

struct Island<O> {
    current: O,
    current_score: i64,
    best: O,
    best_score: i64,
    plateau: u32,
}

/// The outcome of a search: the best valid object found, and enough context to
/// judge whether it means anything.
#[derive(Clone, Debug)]
pub struct Found<O> {
    pub object: O,
    pub score: i64,
    /// Best value in the literature, if the problem declares one.
    pub published_best: Option<i64>,
    pub iterations: u64,
    pub seed: u64,
}

impl<O> Found<O> {
    /// Whether this strictly exceeds the published baseline.
    ///
    /// `None` when the problem declares no baseline — in which case the result is
    /// uninterpretable as a record claim, and must not be presented as one.
    pub fn beats_published(&self) -> Option<bool> {
        self.published_best.map(|b| self.score > b)
    }
}

/// A collection of isolated populations searching the same problem.
///
/// Deliberately single-threaded. The islands are embarrassingly parallel and a
/// threaded version is straightforward, but a run must be reproducible from
/// `(seed, iterations)` alone for a witness to be independently checkable, and
/// thread scheduling would destroy that. Parallelism belongs across runs.
pub struct Archipelago<'a, P: Problem, H: HeuristicSource = FixedHeuristic> {
    problem: &'a P,
    heuristic: H,
    config: SearchConfig,
}

impl<'a, P: Problem> Archipelago<'a, P, FixedHeuristic> {
    pub fn new(problem: &'a P, config: SearchConfig) -> Self {
        Archipelago {
            problem,
            heuristic: FixedHeuristic,
            config,
        }
    }
}

impl<'a, P: Problem, H: HeuristicSource> Archipelago<'a, P, H> {
    pub fn with_heuristic(problem: &'a P, heuristic: H, config: SearchConfig) -> Self {
        Archipelago {
            problem,
            heuristic,
            config,
        }
    }

    pub fn run(&mut self) -> Found<P::Object> {
        let mut rng = Rng::new(self.config.seed);
        let n = self.config.islands.max(1);

        let mut islands: Vec<Island<P::Object>> = (0..n)
            .map(|_| {
                let seed_obj = self.problem.seed(&mut rng);
                debug_assert_eq!(
                    self.problem.is_valid(&seed_obj),
                    Verdict::Valid,
                    "Problem::seed must return a valid object"
                );
                let s = self.problem.score(&seed_obj);
                Island {
                    current: seed_obj.clone(),
                    current_score: s,
                    best: seed_obj,
                    best_score: s,
                    plateau: 0,
                }
            })
            .collect();

        for step in 0..self.config.iterations {
            let idx = (step as usize) % n;
            let candidate = {
                let isl = &islands[idx];
                self.problem.mutate(&isl.current, &mut rng)
            };

            // Invalid candidates are discarded outright. They are not scored,
            // not counted, and never reach the archive.
            if self.problem.is_valid(&candidate) == Verdict::Invalid {
                islands[idx].plateau = islands[idx].plateau.saturating_add(1);
                continue;
            }

            let score = self.problem.score(&candidate);
            let isl = &mut islands[idx];

            let accept = score >= isl.current_score
                || self.heuristic.accept_worse(isl.plateau, &mut rng);

            if accept {
                isl.current = candidate.clone();
                isl.current_score = score;
            }

            if score > isl.best_score {
                isl.best = candidate;
                isl.best_score = score;
                isl.plateau = 0;
            } else {
                isl.plateau = isl.plateau.saturating_add(1);
            }

            if self.config.migrate_every > 0
                && step > 0
                && step % self.config.migrate_every == 0
            {
                self.migrate(&mut islands);
            }
        }

        let winner = islands
            .iter()
            .max_by_key(|i| i.best_score)
            .expect("at least one island");

        Found {
            object: winner.best.clone(),
            score: winner.best_score,
            published_best: self.problem.published_best(),
            iterations: self.config.iterations,
            seed: self.config.seed,
        }
    }

    /// Reseed the weakest island from the strongest. The weakest island's own
    /// best is kept, so migration can never lose a result.
    fn migrate(&self, islands: &mut [Island<P::Object>]) {
        let (Some(best), Some(worst)) = (
            islands
                .iter()
                .enumerate()
                .max_by_key(|(_, i)| i.best_score)
                .map(|(i, _)| i),
            islands
                .iter()
                .enumerate()
                .min_by_key(|(_, i)| i.best_score)
                .map(|(i, _)| i),
        ) else {
            return;
        };
        if best == worst {
            return;
        }
        let donor = islands[best].best.clone();
        let donor_score = islands[best].best_score;
        islands[worst].current = donor;
        islands[worst].current_score = donor_score;
        islands[worst].plateau = 0;
    }
}

/// Search a problem with default settings and a given seed.
pub fn search<P: Problem>(problem: &P, seed: u64, iterations: u64) -> Found<P::Object> {
    let config = SearchConfig {
        seed,
        iterations,
        ..SearchConfig::default()
    };
    Archipelago::new(problem, config).run()
}
