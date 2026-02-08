#import "@preview/touying:0.6.1": *
#import "@preview/clear-tub:0.2.0": *
#import "../constants.typ": *

= Background & Motivation

== Motivation

- Tetris is a subproblem in the category of tiling problems and is a well-studied AI benchmark with NP-hard piece placement
- The game is simple to understand but difficult to master, making it an ideal testbed for optimization techniques.
- Hand-tuning evaluation weights is infeasible for #n-features features
- Metaheuristic search offers a practical alternative to exhaustive optimization
#pause
- *Goal:* automatically discover weight vectors that maximize rows cleared

== Research Questions

- *RQ1* --- How effective is Harmony Search (HSA) at optimizing Tetris agent weights?
- *RQ2* --- How does Cross-Entropy Search (CES) compare to HSA?
- *RQ3* --- Do the optimized weights converge to stable values?

== Agent Architecture

#align(center, image("../../report/figures/pipeline.svg", width: 85%))

The agent that plays Tetris consists of the following components:

Board → #n-features evaluation functions → weighted sum → best placement

Mathematically the objective function to maximize with respect to the weight vector $w$ can be expressed as
$ V(s) = sum_(i=1)^#n-features w_i dot f_i (s), $
where every evaluation function $f_i$ maps a board state $s$ to an integer value.

// FIX: Find different way to fit on one slide
#table(
  columns: (auto, 1fr),
  inset: 10pt,
  align: horizon,
  fill: (x, y) => if y == 0 { gray.lighten(50%) } else if calc.even(y) { gray.lighten(90%) },
  stroke: 0.5pt + gray,

  [*ID*], [*Evaluation Function*],
  [ef01], [Pile Height],
  [ef02], [Holes],
  [ef03], [Connected Holes],
  [ef05], [Altitude Diff],
  [ef06], [Max Well Depth],
  [ef07], [Sum of Wells],
  [ef09], [Blocks],
  [ef10], [Weighted Blocks],
  [ef11], [Row Transitions],
  [ef12], [Col Transitions],
  [ef13], [Highest Hole],
  [ef14], [Blocks Above Highest],
  [ef15], [Potential Rows],
  [ef16], [Smoothness],
  [ef18], [Row Holes],
  [ef19], [Hole Depth],
)

