/-!
# Finite-witness certificates

Definitions of the combinatorial properties the search targets, written so that
each is a *computable `Bool`* rather than a `Prop`. Once a property is a closed
Boolean expression over an explicit finite witness, `decide` discharges it by
evaluation and the kernel's answer is the whole proof.

That restriction is the reason this file has no imports beyond Lean core. It is
also why the search deliberately avoids problems with real-valued coordinates:
certifying a packing of circles needs interval arithmetic and a substantial
library, whereas a set of integers needs nothing at all.
-/

namespace Certs

/-- Duplicate-free check. -/
def nodupB {α : Type _} [BEq α] : List α → Bool
  | [] => true
  | x :: xs => !(xs.contains x) && nodupB xs

/-- The third point on the line through `a` and `b` in `F_3^n`.

Over `F_3` a line is a triple summing to zero coordinatewise, so given two
distinct points the third is forced. -/
def lineThird (a b : List Nat) : List Nat :=
  List.zipWith (fun x y => (3 - (x + y) % 3) % 3) a b

/-- Every point has `n` coordinates, each a residue mod 3. -/
def wellFormed (n : Nat) (pts : List (List Nat)) : Bool :=
  pts.all (fun p => p.length == n && p.all (· < 3))

/-- `pts` is a cap set in `F_3^n`: no three distinct points are collinear.

Stated as: for distinct `a b ∈ pts`, the forced third point is absent. The
`a == b` guard skips the diagonal, where `lineThird a a = a` trivially. -/
def capOK (n : Nat) (pts : List (List Nat)) : Bool :=
  wellFormed n pts
    && nodupB pts
    && pts.all (fun a => pts.all (fun b => (a == b) || !(pts.contains (lineThird a b))))

/-- All sublists of `xs` with exactly `size` elements, order preserved. -/
def subsetsOfSize : Nat → List Nat → List (List Nat)
  | 0, _ => [[]]
  | _ + 1, [] => []
  | n + 1, x :: xs => (subsetsOfSize n xs).map (x :: ·) ++ subsetsOfSize (n + 1) xs

/-- `s` lies inside at least one block. -/
def coveredB (blocks : List (List Nat)) (s : List Nat) : Bool :=
  blocks.any (fun b => s.all (fun x => b.contains x))

/-- `blocks` is a covering design `C(v, k, t)`: every `t`-subset of `{0, …, v-1}`
lies inside some block, and every block is a `k`-subset. -/
def coveringOK (v k t : Nat) (blocks : List (List Nat)) : Bool :=
  blocks.all (fun b => b.length == k && b.all (· < v) && nodupB b)
    && (subsetsOfSize t (List.range v)).all (coveredB blocks)

/-! ## Sanity checks

These are not results. They are guards against a definition that is vacuously
satisfiable — a `capOK` that accepted a known line, or a `coveringOK` that
accepted an incomplete family, would make every certificate in this repository
meaningless while still compiling. -/

example : capOK 2 [[0, 0], [1, 0], [2, 0]] = false := by decide

example : capOK 2 [[0, 0], [1, 0]] = true := by decide

example : coveringOK 4 3 2 [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]] = true := by decide

example : coveringOK 4 3 2 [[0, 1, 2]] = false := by decide

end Certs
