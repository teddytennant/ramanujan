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

## Record tables

Filter 1 removes more candidates than expected, and several tables that still get
cited as live are frozen or gone. Checked 2026-08-10, and worth re-checking before
relying on any row.

| Table | Status |
|---|---|
| Constant-weight binary codes `A(n,d,w)`, [Brouwer](https://aeb.win.tue.nl/codes/Andw.html) | Live, `Last-Modified: 2026-08-09`, submissions by plain email. Best fit. |
| Best-known linear codes, [codetables.de](https://codetables.de) (Grassl) | Live but slow; the [changelog](https://www.codetables.de/updates.html) stops at 2025-12-11. Certificates are MAGMA construction recipes rather than matrices, so verifying one yourself needs MAGMA. |
| Covering designs, [La Jolla](https://ljcr.dmgordon.org/cover/table.html) | Retired. Submissions closed 2026-03-01 and the database is frozen and archived on [Zenodo](https://zenodo.org/records/19735294). |
| Covering designs, [coveringrepository.com](https://coveringrepository.com) (Acerbi) | Active successor, but it fails filter 1 in practice: downloads and submissions sit behind a subscription and uploading needs a Windows-only validator. |
| Self-dual codes, [Harada and Munemasa](https://www.math.is.tohoku.ac.jp/~munemasa/selfdualcodes.htm) | Live at a moderate pace. Plausible second choice. |
| Small Ramsey numbers, Radziszowski EJC DS1 | Live but revised every couple of years, which is too slow to enter. |
| Sphere and spherical packings, [Sloane](https://neilsloane.com/packings/) | Fails filter 3 on real coordinates. |
| Costas arrays, Golomb rulers | Frontier is dead. OGR-28 finished in 2022 with no successor planned, and these are exhaustive-proof problems rather than incremental ones. |
| Turán and packing designs (`ccrwest.org`) | Domain parked, no successor found. |

`harness/src/problems/covering.rs` stays in the tree as a second worked example of
the certificate pattern. It is no longer a candidate target.

## The pick

Constant-weight binary codes `A(n,d,w)`. Free, actively maintained, discrete, and
the certificate is either an explicit codeword list or a cyclic orbit generator,
both of which are finite checks that suit `decide`.

The softest cells share a signature: the lower bound is inherited from `n−1`
because no code was ever built for that cell, so beating it takes exactly one more
codeword. These were read off the table on 2026-08-10 and should be re-derived
before anyone relies on them.

| Cell | LB | UB | Where the LB comes from |
|---|---|---|---|
| `A(28,12,9)` | 39 | 45 | inherited from `A(27,12,9)` |
| `A(37,10,6)` | 37 | 43 | inherited from `A(36,10,6)` |
| `A(21,10,10)` | 38 | 42 | inherited from `A(20,10,10)` |
| `A(30,14,14)` | 58 | 95 | inherited from `A(29,14,14)` |

## Prior art

[Rosin, *Automated Discovery of Improved Constant Weight Binary Codes*,
arXiv:2603.00174](https://arxiv.org/abs/2603.00174) (February 2026) improved
`A(n,d,w)` lower bounds for 24 cells with tabu search driven by strategies from an
automated protocol, and published
[code and results](https://github.com/Constructive-Codes/CWBC). Same idea, same
table, so read it before starting anything here.

The distinction this project can claim is narrow. Rosin's codes ship with C
verification programs, whereas the aim here is a certificate the Lean kernel
accepts. That is better verification of a known method, not a new method.

One thing the literature understates: well over a hundred lower bounds arrived on
the table by email in July and August 2026 alone. Read the table rather than the
papers to judge whether a cell is live.
