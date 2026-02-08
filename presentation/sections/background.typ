#import "@preview/touying:0.6.1": *
#import "@preview/clear-tub:0.2.0": *
#import "../constants.typ": *

= Background & Motivation

== Motivation

- Tetris is a well-studied AI benchmark with NP-hard piece placement
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

Board → #n-features evaluation functions → weighted sum → best placement
