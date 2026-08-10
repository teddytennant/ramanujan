# ramanujan

Searches for extremal combinatorial objects and emits a Lean 4 proof for each one
it finds.

LLM-guided program search has produced real mathematics.
[FunSearch](https://www.nature.com/articles/s41586-023-06924-6) improved a cap set
lower bound and later systems applied the same idea to other extremal problems.
What none of them produce is a proof. You get an object and the authors'
evaluator, so you either trust the evaluator or reimplement it yourself.

This repo is an attempt at the missing half. Every object the search reports comes
with a Lean 4 certificate, and nothing is treated as found until `lake build`
accepts it.

## Status

The search substrate and the certification path both work and are tested end to
end. Nothing new has been found. The search recovers known values on small
instances, it has never run long enough or against a hard enough cell for a new
result to be plausible, and there are no record claims here.

## Example run

```
$ cargo run --release -- capset 4 30000 1
capset-4: best 20 (seed 1, 30000 iterations)
  matches published best of 20
  certificate written to ../lean/Certs/Generated/Capset4.lean

$ cd ../lean && lake build
✔ Built Certs.Generated.Capset4 (2.2s)
```

`Capset4.lean` is standalone: the witness as a literal, the predicate imported
from `Certs.Basic`, and

```lean
theorem capset4_20_isCap : capOK 4 capset4_20 = true := by decide
```

`decide` evaluates the predicate on the witness inside the kernel. The file
imports core Lean and nothing else, so checking it needs an `elan` install and a
couple of seconds.

## Why only finite witnesses

Every target has to come with a decidable predicate over an explicit finite
witness, which makes "there is an object of size N with property P" a finite
computation rather than a proof obligation. A cap set is a list of vectors over
`F_3`, a covering design is a list of blocks, a linear code is a generator matrix.
Anything with real coordinates, like circle packing or Heilbronn triangle, would
need interval arithmetic and a large supporting library to certify, so it is out
of scope.

## Baselines

Rediscovering a known bound and reporting it as a record is the easiest way to
embarrass yourself in this area, so every problem carries its published best or
explicitly carries none, and each run prints whether the score beat it, matched
it, or fell short. Where there is no maintained table to cite, the harness says
that no baseline is declared instead of guessing one. Sources are in
[docs/targets.md](docs/targets.md).

## Layout

| Path | What |
|---|---|
| `harness/` | Rust search substrate: island model, problems, Lean emitter |
| `lean/Certs/Basic.lean` | The predicates, as computable `Bool`s. Core Lean only |
| `lean/Certs/Generated/` | Emitted certificates |
| `docs/design.md` | Architecture, and where a model would plug in |
| `docs/targets.md` | Selection criteria and baselines with sources |
| `docs/validation.md` | The calibration gate and what it covers |

## Build

```bash
cd harness && cargo test --release     # 10 tests, including the calibration gate
cd ../lean && lake build               # verifies every certificate
```

Lean 4.33.0 via `elan`. There is no Mathlib dependency, so a cold build takes
seconds.

## License

MIT.
