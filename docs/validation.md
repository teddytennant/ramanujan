# Validation

A search that cannot reproduce values the literature already has is not going to
produce a new one, and if it appears to, the likelier explanation is a bug in the
validity predicate. So the first thing that runs is a calibration test rather than
a document: `recovers_known_cap_set_maxima` in `harness/tests/search.rs` fails the
build if the search misses a known maximum.

## What it covers

Cap set maxima in `F_3^n` from a cold start, 20,000 iterations, seed 1:

| n | known max | found |
|---|---|---|
| 1 | 2 | 2 |
| 2 | 4 | 4 |
| 3 | 9 | 9 |
| 4 | 20 | 20 |

`n = 4` is the only row carrying any weight, since 20 is the exact maximum and the
search reaches it from a random start in well under a second. These are small
instances that suit greedy repair, so the test establishes that the harness is not
broken and nothing past that.

## Certification

```
$ ./target/release/ramanujan capset 4 30000 1
capset-4: best 20 (seed 1, 30000 iterations)
  matches published best of 20
  certificate written to ../lean/Certs/Generated/Capset4.lean

$ ./target/release/ramanujan covering 7 3 2 7 200000 3
covering-7-3-2-b7: covered 21/21 2-subsets with 7 blocks (seed 3, 200000 iterations)
  complete covering: C(7,3,2) <= 7
  certificate written to ../lean/Certs/Generated/Covering732B7.lean

$ cd ../lean && lake build
✔ Built Certs.Generated.Covering732B7 (326ms)
✔ Built Certs.Generated.Capset4 (2.2s)
```

Both certificates check with `decide` over core Lean, about two and a half seconds
from cold. `C(7,3,2) = 7` is the Fano plane, which is why it is worth keeping
around: the answer is known, so a regression in the emitter or the predicate turns
into a failed build.

## Checking the checker

A predicate that accepted everything would make every certificate meaningless and
still compile green, so `Basic.lean` carries negative examples beside the positive
ones:

```lean
example : capOK 2 [[0, 0], [1, 0], [2, 0]] = false := by decide
example : coveringOK 4 3 2 [[0, 1, 2]] = false := by decide
```

The first is a line in `F_3^2` and has to be rejected; the second is an incomplete
family. The Rust side does the same in `validity_rejects_a_line`,
`validity_rejects_duplicates_and_out_of_range`, and
`covering_rejects_wrong_sized_blocks`.

## What this does not show

- Nothing has run at a scale where a new result would be plausible.
- Nothing has run against a cell whose answer is unknown.
- No model has been in the loop, so there is no evidence either way about what one
  would add over `FixedHeuristic`.
