//! The tests that matter here are the ones checking the harness cannot lie:
//! that `is_valid` rejects bad objects, and that the search recovers bounds the
//! literature already knows. A search that cannot rediscover a known maximum has
//! no business reporting a new one.

use ramanujan::island::search;
use ramanujan::problem::{Problem, Verdict};
use ramanujan::problems::{CapSet, CoveringDesign};
use ramanujan::Rng;

#[test]
fn rng_is_deterministic() {
    let a: Vec<u64> = (0..8).scan(Rng::new(42), |r, _| Some(r.next_u64())).collect();
    let b: Vec<u64> = (0..8).scan(Rng::new(42), |r, _| Some(r.next_u64())).collect();
    assert_eq!(a, b);
    let c: Vec<u64> = (0..8).scan(Rng::new(43), |r, _| Some(r.next_u64())).collect();
    assert_ne!(a, c);
}

#[test]
fn line_third_is_the_third_point() {
    let p = CapSet::new(3);
    // Over F_3 the three points on a line sum to zero coordinatewise.
    for a in 0..27u16 {
        for b in 0..27u16 {
            if a == b {
                continue;
            }
            let c = p.line_third(a, b);
            assert_ne!(c, a, "third point collided with a");
            assert_ne!(c, b, "third point collided with b");
            let (da, db, dc) = (p.digits(a), p.digits(b), p.digits(c));
            for i in 0..3 {
                assert_eq!((da[i] + db[i] + dc[i]) % 3, 0);
            }
        }
    }
}

#[test]
fn validity_rejects_a_line() {
    let p = CapSet::new(2);
    // 0 = (0,0), 1 = (1,0), 2 = (2,0) is a line and must be rejected.
    assert_eq!(p.is_valid(&vec![0, 1, 2]), Verdict::Invalid);
    // Two points can never form a line.
    assert_eq!(p.is_valid(&vec![0, 1]), Verdict::Valid);
}

#[test]
fn validity_rejects_duplicates_and_out_of_range() {
    let p = CapSet::new(2);
    assert_eq!(p.is_valid(&vec![0, 0]), Verdict::Invalid);
    assert_eq!(p.is_valid(&vec![0, 99]), Verdict::Invalid);
}

#[test]
fn search_only_ever_returns_valid_objects() {
    for n in 1..=4 {
        let p = CapSet::new(n);
        let found = search(&p, 7, 2_000);
        assert_eq!(
            p.is_valid(&found.object),
            Verdict::Valid,
            "search returned an invalid cap set at n={n}"
        );
        assert_eq!(found.score, found.object.len() as i64);
    }
}

/// The calibration gate. Maximum cap set sizes are known exactly for these `n`;
/// if the harness cannot recover them it is broken.
#[test]
fn recovers_known_cap_set_maxima() {
    for (n, expected) in [(1u32, 2i64), (2, 4), (3, 9), (4, 20)] {
        let p = CapSet::new(n);
        let found = search(&p, 1, 20_000);
        assert_eq!(
            found.score, expected,
            "n={n}: expected the known maximum {expected}, got {}",
            found.score
        );
        assert_eq!(found.beats_published(), Some(false));
    }
}

#[test]
fn covering_score_counts_covered_subsets() {
    let p = CoveringDesign::new(5, 3, 2, 4);
    assert_eq!(p.target_count(), 10); // C(5,2)
    // A single block {0,1,2} covers exactly the three pairs inside it.
    let one = vec![0b00111u64, 0b00111, 0b00111, 0b00111];
    assert_eq!(p.score(&one), 3);
}

#[test]
fn covering_rejects_wrong_sized_blocks() {
    let p = CoveringDesign::new(5, 3, 2, 2);
    assert_eq!(p.is_valid(&vec![0b00111, 0b00111]), Verdict::Valid);
    assert_eq!(p.is_valid(&vec![0b00011, 0b00111]), Verdict::Invalid); // popcount 2
    assert_eq!(p.is_valid(&vec![0b00111]), Verdict::Invalid); // wrong block count
}

/// `C(5,3,2) = 4` is a small known value; the search should reach full coverage.
#[test]
fn finds_a_complete_small_covering() {
    let p = CoveringDesign::new(5, 3, 2, 4);
    let found = search(&p, 3, 20_000);
    assert!(
        p.is_complete(&found.object),
        "expected full coverage of all {} pairs, covered {}",
        p.target_count(),
        found.score
    );
}

#[test]
fn emitted_lean_is_self_contained() {
    let p = CapSet::new(2);
    let found = search(&p, 1, 500);
    let lean = p.emit_lean(&found.object);
    assert!(lean.contains("import Certs.Basic"));
    assert!(lean.contains("by decide"));
    // A certificate that pulled in Mathlib would not be checkable in seconds,
    // and the reproducibility claim in the README depends on that.
    assert!(!lean.contains("Mathlib"));
}
