# Design

Nothing here calls a language model. The substrate is built around a seam where
one would go, and most of this document is about where that seam is and why it is
not in the obvious place.

## Where a model would plug in

The published successes in this area spend on the order of millions of model calls
on large clusters. At a much smaller budget the usual architecture, which asks a
model for candidate objects and scores them, buys one sample per call. That is a
bad trade when the call is the expensive part.

So the interface is [`HeuristicSource`](../harness/src/problem.rs), and what it
asks for is search strategy: destruction sizes, acceptance rules, move operators,
symmetry ansätze. Whatever comes back is handed to the CPU and run over many
restarts for as long as there is budget. One call then buys minutes of search
rather than a single guess, which on a machine with spare cores and a small API
bill is the difference between the method being worth trying and not.

There is also something a model can do here that a general solver cannot. A fair
number of records in these tables were set by annealing with no problem-specific
structure at all, which suggests the structure of individual instances is going
unused. Writing a move operator that respects the symmetry of one particular cell
is well within what a model can do, and asking it for objects instead throws that
away.

The only implementation today is `FixedHeuristic`, which is deterministic and
pulls in no dependencies. That keeps the substrate testable on its own and leaves
a baseline for any future model-driven version to be measured against.

## Correctness is not up for a vote

`Problem::is_valid` is total, decidable, and the only authority on whether a
candidate counts. Invalid candidates are dropped before scoring and never enter
the archive, and `Verdict` has two variants with no third "probably". The Lean
kernel then checks the same property again on the emitted witness, so an object
the search believes in but that does not compile is not a result.

The standard objection to LLM-driven mathematics is that these systems produce
confident wrong answers. This layering is the structural reply: the model is never
in a position to assert anything, only to suggest where to look.

## Island model

Populations evolve in isolation with periodic migration from the strongest island
into the weakest. A single global population converges early and then stops
turning anything up, whereas keeping the islands separate holds diversity for
longer. Migration writes into an island's population but never over its own best,
so a good object cannot be lost to it.

The search is single-threaded on purpose. The islands are embarrassingly parallel
and threading them would be easy, but a witness has to be reproducible from
`(seed, iterations)` alone for anyone else to check it, and thread scheduling
breaks that. Parallelism goes across runs instead.

## Ruin and recreate

The cap set neighbourhood tears out a few points, then greedily re-adds every
point that still fits, in a fresh random order. Swapping single points almost
always lands back where it started, so the rebuild is what actually moves the
search. The fill order decides what the rebuild produces, which is why it is the
part a heuristic would control.

## Missing

- No model anywhere. `HeuristicSource` has one fixed implementation.
- No self-modification. Feeding run telemetry back so the loop rewrites its own
  heuristic is the obvious next step and is the idea behind
  [Agentic Harness Engineering](https://github.com/china-qijizhifeng/agentic-harness-engineering).
  None of it is implemented here.
- No real campaign. Everything has run at calibration scale. See
  [validation.md](validation.md).
