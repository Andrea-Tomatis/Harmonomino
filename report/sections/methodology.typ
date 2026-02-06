#import "introduction.typ": score-eq
#import "@preview/booktabs:0.0.4": *
#import "@preview/pillar:0.3.3": cols


= Methodology

== Game Model

We implement a Tetris simulator on a $10 times 20$ board with precomputed rotation tables
and uniformly random piece generation (no 7-bag). The agent considers only the current
piece with no lookahead. These constraints keep evaluation deterministic and fast while
preserving the core placement challenges.

== Agent Evaluation

For each piece, the agent enumerates all rotations and column placements, applies a hard
lock, clears full rows, and scores the resulting board. The move with the highest score is
selected.
The score is calculated as in @eq-score, a linear combination

$ #score-eq , $

where $f_i (s)$ are board heuristics and $w_i$ are learned weights.

== Heuristic Feature Set

We use 16 board features, all computable from the current board state alone. They fall into
six categories:

// Two-column Typst table for features
#table(
  ..cols("l|r"),
  [*Height/Surface*], [pile height\ altitude difference\ smoothness], midrule(),
  [*Holes*], [holes\ connected holes\ highest hole\ row holes\ hole depth], midrule(),
  [*Wells*], [max well depth\ sum of well depths], midrule(),
  [*Transitions*], [row transitions\ column transitions], midrule(),
  [*Blocks*], [total blocks\ weighted blocks\ blocks above highest hole], midrule(),
  [*Rows*], [potential rows],
)

This feature set matches the subset used by #cite(<Romero2011TetrisHarmonySearch>, form: "prose") that does not require additional
in-game context (we exclude removed rows, landing height, and eroded pieces).

== Harmony Search (HS)

// TODO: get numbers from data
HS maintains a harmony memory (HM) of candidate weight vectors. New candidates are created
by selecting values from HM with probability $r_"accept"$, applying pitch adjustment with
probability $r_"pa"$, or sampling randomly otherwise. The worst candidate in HM is replaced
if a better solution is found. Unless stated otherwise, we use: HM size 5, 500 iterations,
$r_"accept" = 0.95$, $r_"pa" = 0.99$, and bandwidth 0.1.

== Cross-Entropy Search (CES)

// TODO: get numbers from data
CES models weights with a multivariate Gaussian. Each iteration samples $N$ candidates,
selects the top $N_"elite"$, and updates the mean and variance from the elite set. We use
$N=50$, $N_"elite" = 10$, 500 iterations, an initial standard deviation of 10.0, and a
minimum standard deviation floor of 0.01.

== Experimental Protocol

// TODO: get numbers from data
We run 10 training seeds for each optimizer with a simulation length of 1000 pieces. For
evaluation we use 30 fixed seeds with a simulation length of 2000 pieces. As a baseline we
evaluate 30 random weight vectors sampled uniformly from $[-1, 1]$.
We report mean, median, standard deviation, and 95% confidence intervals of cleared rows,
plus convergence and weight-distribution plots.
