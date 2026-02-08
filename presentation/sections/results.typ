#import "@preview/touying:0.6.1": *
#import "@preview/clear-tub:0.2.0": *
#import "../constants.typ": *

= Approach & Results

== Algorithms

#slide(composer: (1fr, 1fr))[
  === Harmony Search (HSA)
  - Population of #params.hsa_memory_size weight vectors
  - #params.hsa_iterations improvisation rounds
  - HMCR: #params.hsa_accept_rate, PAR: #params.hsa_pitch_adj_rate
  - Bandwidth: #params.hsa_bandwidth
][
  === Cross-Entropy Search (CES)
  - #params.ces_n_samples candidates per generation
  - Top #params.ces_n_elite elite selection
  - #params.ces_iterations generations
  - Gaussian sampling + shrinkage
]

== Performance

#align(center,
  image("../../report/figures/rows_cleared_distribution.pdf", width: 75%)
)

#slide(composer: (1fr, 1fr))[
  === HSA
  Mean: #summary.hsa.mean rows \
  Median: #summary.hsa.median rows
][
  === CES
  Mean: #summary.ces.mean rows \
  Median: #summary.ces.median rows
]

== Convergence

#align(center,
  image("../../report/figures/fitness_over_iter.pdf", width: 80%)
)

== Weight Analysis

#align(center,
  image("../../report/figures/weight_mean_std.pdf", width: 75%)
)

*Most stable:* #fmt-weights(stable-weights) \
*High variance:* #fmt-weights(hv-weights)

== Parameter Sensitivity

#slide(composer: (1fr, 1fr, 1fr))[
  #image("../../report/figures/benchmark_bandwidth.pdf", width: 100%)
][
  #image("../../report/figures/benchmark_iterations.pdf", width: 100%)
][
  #image("../../report/figures/benchmark_pitch_adj_rate.pdf", width: 100%)
]
