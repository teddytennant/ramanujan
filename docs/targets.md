# Target selection

Which cell to attack matters more than the algorithm or the model, and it is easy
to burn a month on one that was never going to move.

## Filters

A target has to pass all four.

1. There is a maintained record table to cite, rather than a number remembered
   from a paper. Someone else has to be able to check the claim.
2. The table is still being updated. If nothing has changed in thirty years the
   problem is finished, very hard, or abandoned, and none of those help.
3. The object is discrete: finite fields, finite sets, integers, colourings.
4. The certificate is a finite check, so `decide` closes the goal.

Filter 3 rules out circle packing and the Heilbronn triangle problem, which
otherwise fit well, since both are searchable and both have maintained tables.
Certifying either means interval arithmetic over the reals and a large supporting
library, where a cap set is a list of small integers that `decide` evaluates in
seconds with no imports.

## Picking a cell

Flagship cells are a bad bet. The famous constants have absorbed far more compute
than this project has, in some cases from these same methods.

The useful signal is where a bound came from, not how old it is. Age on its own
says very little: plenty of thirty-year-old records are old because the value is
optimal and the visible gap is lower-bound weakness rather than an upper bound
that is soft. A bound inherited from a smaller parameter, or one set by a generic
local search that ran out of time, is a much better sign. So is a table that
simply stops. "Known for n ≤ 7, open for n = 8" is the ideal entry, because
filling the next cell is citable and much easier than improving a celebrated
bound.

Set a generation budget and a required rate of improvement per cell before
starting, abandon on plateau, and log every target abandoned. Letting one cell eat
the entire budget is the usual way this kind of project fails.

## Baselines declared

Cap set maxima in `F_3^n`, exact for `n ≤ 6` ([OEIS A090245](https://oeis.org/A090245);
`n = 6` is due to Potechin):

| n | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| max | 2 | 4 | 9 | 20 | 45 | 112 |

Past `n = 6` the code declares no baseline, deliberately. A guessed baseline is
worse than none because it invites a false record claim, so the harness prints
`not a record claim` rather than staying quiet.

## Domains worth evaluating

None of these are implemented yet. This is the shortlist the filters produce.

- **Covering designs.** The [La Jolla Covering Repository](https://ccrwest.org/cover.html)
  maintains `C(v,k,t)` records, the certificate is a finite set of blocks, and the
  records move regularly under heuristic search. Best fit for the filters, and
  there is a partial implementation in `harness/src/problems/covering.rs`.
- **Best-known linear codes.** [codetables.de](https://codetables.de) (Grassl).
  The certificate is a generator matrix over `GF(q)`, finite to verify, and the
  table is well maintained. Coding theorists have been over it thoroughly, which
  cuts both ways.
- **Small Ramsey lower bounds.** Radziszowski's *Small Ramsey Numbers* dynamic
  survey (EJC DS1). Lower bounds are explicit colourings and finite to verify, and
  many entries were set by computer search in the 1990s and 2000s under budgets a
  laptop now dwarfs, so some are plausibly stale.
- **Other maintained-record objects.** Costas arrays, Golomb rulers, difference
  sets, van der Waerden and Schur numbers. Same shape: an explicit object and a
  maintained table.
