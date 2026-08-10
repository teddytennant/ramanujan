//! Progression-free sets (cap sets) in `F_3^n`.
//!
//! A cap set contains no three distinct collinear points. Over `F_3` a line is
//! `{a, b, c}` with `a + b + c = 0`, so for any distinct `a, b` the third point is
//! *determined*: `c = -(a + b)`. That turns the defining condition into a linear
//! number of membership tests per point rather than a cubic scan over triples,
//! which is what makes greedy repair cheap enough to run inside a tight loop.
//!
//! This problem is included as the harness's calibration target. Maximum cap set
//! sizes are known exactly for `n ≤ 6`, so a search that cannot recover them is
//! broken, and any claim it makes at larger `n` is worthless. See
//! `docs/validation.md`.

use crate::problem::{Problem, Verdict};
use crate::Rng;

pub struct CapSet {
    pub n: u32,
}

impl CapSet {
    pub fn new(n: u32) -> Self {
        assert!(n >= 1 && n <= 9, "n outside supported range");
        CapSet { n }
    }

    /// Number of points in the ambient space, `3^n`.
    pub fn ambient(&self) -> usize {
        3usize.pow(self.n)
    }

    /// The unique third point on the line through `a` and `b`.
    ///
    /// For `a == b` this returns `a`; callers must exclude that case themselves.
    pub fn line_third(&self, a: u16, b: u16) -> u16 {
        let mut out = 0u16;
        let mut place = 1u16;
        let (mut a, mut b) = (a, b);
        for _ in 0..self.n {
            let (da, db) = (a % 3, b % 3);
            let dc = (3 - (da + db) % 3) % 3;
            out += dc * place;
            a /= 3;
            b /= 3;
            place *= 3;
        }
        out
    }

    /// Digit expansion, least-significant coordinate first.
    pub fn digits(&self, mut p: u16) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.n as usize);
        for _ in 0..self.n {
            out.push((p % 3) as u8);
            p /= 3;
        }
        out
    }

    /// Add every point that can be added without creating a line, in the order
    /// given. Greedy and order-dependent — the order *is* the heuristic.
    fn greedy_fill(&self, points: &mut Vec<u16>, order: &[u16], present: &mut [bool]) {
        for &p in order {
            if present[p as usize] {
                continue;
            }
            let addable = points
                .iter()
                .all(|&q| !present[self.line_third(p, q) as usize]);
            if addable {
                present[p as usize] = true;
                points.push(p);
            }
        }
    }

    fn membership(&self, points: &[u16]) -> Vec<bool> {
        let mut present = vec![false; self.ambient()];
        for &p in points {
            present[p as usize] = true;
        }
        present
    }
}

impl Problem for CapSet {
    type Object = Vec<u16>;

    fn name(&self) -> String {
        format!("capset-{}", self.n)
    }

    fn seed(&self, rng: &mut Rng) -> Vec<u16> {
        let mut order: Vec<u16> = (0..self.ambient() as u16).collect();
        rng.shuffle(&mut order);
        let mut points = Vec::new();
        let mut present = vec![false; self.ambient()];
        self.greedy_fill(&mut points, &order, &mut present);
        points.sort_unstable();
        points
    }

    fn mutate(&self, object: &Vec<u16>, rng: &mut Rng) -> Vec<u16> {
        // Ruin and recreate: tear out a few points, then greedily rebuild in a
        // fresh random order. Rebuilding is what makes this productive — a plain
        // point swap almost always lands back where it started.
        let mut points = object.clone();
        let tear = 1 + rng.below(3);
        for _ in 0..tear {
            if points.is_empty() {
                break;
            }
            let i = rng.below(points.len());
            points.swap_remove(i);
        }

        let mut present = self.membership(&points);
        let mut order: Vec<u16> = (0..self.ambient() as u16).collect();
        rng.shuffle(&mut order);
        self.greedy_fill(&mut points, &order, &mut present);
        points.sort_unstable();
        points
    }

    fn score(&self, object: &Vec<u16>) -> i64 {
        object.len() as i64
    }

    fn is_valid(&self, object: &Vec<u16>) -> Verdict {
        let ambient = self.ambient();
        if object.iter().any(|&p| p as usize >= ambient) {
            return Verdict::Invalid;
        }
        let mut seen = vec![false; ambient];
        for &p in object {
            if seen[p as usize] {
                return Verdict::Invalid; // duplicate
            }
            seen[p as usize] = true;
        }
        for (i, &a) in object.iter().enumerate() {
            for &b in &object[i + 1..] {
                if seen[self.line_third(a, b) as usize] {
                    return Verdict::Invalid;
                }
            }
        }
        Verdict::Valid
    }

    /// Exact maximum cap set sizes, known for `n ≤ 6`.
    ///
    /// Sources are recorded in `docs/targets.md`. `None` beyond that range is
    /// deliberate: a baseline that is guessed is worse than no baseline, because
    /// it invites a false record claim.
    fn published_best(&self) -> Option<i64> {
        match self.n {
            1 => Some(2),
            2 => Some(4),
            3 => Some(9),
            4 => Some(20),
            5 => Some(45),
            6 => Some(112),
            _ => None,
        }
    }

    fn emit_lean(&self, object: &Vec<u16>) -> String {
        crate::certify::capset_lean(self, object)
    }
}
