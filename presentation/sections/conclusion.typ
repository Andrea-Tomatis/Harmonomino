#import "@preview/touying:0.6.1": *
#import "@preview/clear-tub:0.2.0": *
#import "../constants.typ": *

= Conclusion

== Findings

#slide(composer: (1fr, 1fr))[
  === Contributions
  - Both HSA and CES successfully optimize Tetris agent weights
  - #n-features feature evaluation covers pile, hole, well, row, and block metrics
  - Subset of weights converge to stable values across runs
  - Automated pipeline enables reproducible experiments
][
  === Limitations
  - Single-piece lookahead only
  - Fixed game length caps observable performance
  - No T-spin or hold-piece strategies
]

== Future Work

- Multi-piece lookahead and hold-piece integration
- Hybrid algorithms combining HSA exploration with CES exploitation
- Neural-network evaluation functions trained on optimized weights
- Transfer learning across board sizes and rule variants

