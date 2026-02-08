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

== Harmony Search Algorithm
// Slide 1: Introduction to Harmony Search
#slide[
  == Harmony Search (HS) Algorithm
  
  Introduced by Geem et al. (2001), HS is a metaheuristic inspired by the *musical improvisation process*.

  *Musicians seek "pleasing harmony" through three strategies:*

  + *Memory:* Playing a known piece from memory.
  + *Variation:* Playing something similar with slight adjustments.
  + *Randomness:* Composing freely from random notes.
]

---

// Slide 2: Core Mechanisms
== Core Mechanisms of HS
#slide[
  
  The algorithm maintains a *Harmony Memory (HM)* containing a population of solution vectors.

  #table(
    columns: (1fr, 2fr),
    stroke: none,
    fill: (x, y) => if calc.even(y) { gray.lighten(90%) },
    [*Mechanism*], [*Description*],
    [HM Consideration], [Copying a value from HM with probability $r_"accept"$],
    [Pitch Adjustment], [Perturbing a value with probability $r_"pa"$],
    [Randomization], [Sampling a completely new random value],
  )

  #v(1em)
  > *Key Advantage:* Unlike Genetic Algorithms (which use two parents), HS considers *all* solutions in the HM simultaneously.
]

---

// Slide 3: Application to Tetris
== HS in Tetris Weight Optimization
#slide[
  
  Romero et al. (2011) pioneered the use of HS for Tetris:

  * *Feature Set:* #n-features board feature functions.
  * *Configuration:* Harmony Memory size of 5.
  * *Results: - Efficiently discovered high-quality weight configurations.
    - Achieved a spawned-pieces-to-cleared-rows ratio near the *theoretical optimum of 2.5*.

  #align(center + bottom)[
    #block(
      fill: blue.lighten(90%),
      inset: 8pt,
      radius: 4pt
    )
  ]
]

== Flowchart
#slide[

  #figure(
  include "../../report/figures/hsa_flowchart.typ", 
  )
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
