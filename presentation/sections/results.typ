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
#slide[
  == Harmony Search (HS) Algorithm
  
  Introduced by Geem et al. (2001), HS is a metaheuristic inspired by the *musical improvisation process*.

  *Musicians seek "pleasing harmony" through three strategies:*

  + *Memory:* Playing a known piece from memory.
  + *Variation:* Playing something similar with slight adjustments.
  + *Randomness:* Composing freely from random notes.
]

---

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

---

== HSA Hyperparameter Sensitivity
#slide[
  
  Analysis of Bandwidth, Pitch Adjustment Rate ($r_"pa"$), and Max Iterations.

  #grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 1em,
    [
      #figure(image("../../report/figures/benchmark_bandwidth.pdf", width: 100%))
      *Bandwidth:* Excessive values disrupt fine-tuning; very small values risk local optima.
    ],
    [
      #figure(image("../../report/figures/benchmark_pitch_adj_rate.pdf", width: 100%))
      *Pitch Adj. Rate:* Shows minimal impact on final performance in this configuration.
    ],
    [
      #figure(image("../../report/figures/benchmark_iterations.pdf", width: 100%))
      *Max Iterations:* Clear diminishing returns observed beyond roughly 170 iterations.
    ]
  )
  
  #v(1fr)
  #block(
    fill: gray.lighten(90%), 
    inset: 8pt, 
    radius: 4pt,
    width: 100%,
    [*Note:* While relative performance is stable, these parameters primarily influence the *rate* of convergence and search robustness.]
  )
]

---

== Convergence Stability (DBSCAN)
#slide[
  
  Are the agents finding the same solution, or many different ones?

  #figure(
    grid(
      columns: (1fr, 1.2fr),
      gutter: 10pt,
      image("../../report/figures/k_distance_elbow.pdf", width: 100%), 
      image("../../report/figures/dbscan_stability.pdf", width: 100%),
    ),
    caption: [DBSCAN identifies a single primary cluster of "good" solutions.],
  )

  - *The "Elbow":* Identified at $epsilon approx 1.35$.
]

== Convergence Stability (DBSCAN)
#slide[

  #figure(
      image("../../report/figures/weight_categories.pdf", width: 80%),
    caption: [Most likely category found with DBSCAN per weight.],
  )
  - *Primary Cluster:* Most seeds converge to a similar region in the high-dimensional weight space, validating the robustness of the heuristic set.
]

== Theoretical Divergence (Error Analysis)
#slide[
  
  As games scale, the "Absolute Error" relative to the theoretical maximum increases.

  #grid(
    columns: (1fr, 1.2fr),
    gutter: 1em,
    [
      *The Plateau Effect:*
      - *Short games (< 500 lines):* Error remains near zero.
      - *Long games (> 750 lines):* Error grows sharply, exceeding 1750 by line 5000.
      
      *Conclusion:* Current linear heuristics cannot fully compensate for board "exhaustion" in long-horizon play.
    ],
    figure(
      image("../../report/figures/consistency_error.pdf", width: 100%),
      caption: [Absolute error vs. game length.],
    )
  )
]

== Simulation Consistency
#slide[
  We tested the agent against a theoretical performance model to check for "plateaus."

  #grid(
    columns: (1fr, 1fr),
    gutter: 1em,
    figure(
      image("../../report/figures/consistency_test.pdf", width: 100%),
      caption: [Performance vs. Theoretical Max],
    ),
    [
      *Findings:*
      - *Short Term:* Near-perfect alignment with theory up to ~500 rows.
      - *Long Term:* Error grows significantly after 750 rows.
      - *Implication:* The agents encounter "unsolvable" board states or structural constraints not captured by the simple linear heuristic model.
    ]
  )
]



== Cross-Entropy: Distribution Learning
#slide[
  
  
  Unlike HS, which tracks individual "members", CES tracks a *probability distribution* (mean $mu$ and variance $sigma^2$) that represents where the best weights likely live.

  + *Sampling:* Generate a large batch of candidate weight vectors (e.g., 100) by sampling from the current Gaussian distribution.
  + *The "Elite" Selection:* Test every candidate in a Tetris simulation. Select the top 10% (the "Elite Set").
  + *Distribution Shift:* Calculate the new $mu$ and $sigma^2$ based *only* on the Elite Set. 
  
  The distribution literally "moves" and "shrinks" toward the highest-scoring regions of the fitness landscape over multiple generations.
]



== Maintaining Exploration (Noisy CES)
#slide[
  
  A major challenge in Tetris is the *stochastic noise*—a weight might perform well just because it got "lucky" pieces. This often leads to *Variance Collapse*.

  * *The Failure:* The variance ($sigma^2$) shrinks to zero too quickly. The algorithm becomes "blind" to other possibilities and stops exploring.
  * *The Fix:* *Additive Noise.* We manually inject noise into the update rule:
    $ sigma^2_(t+1) = sigma^2_"elite" + Z(t) $
  * *The Benefit: By ensuring the standard deviation never drops below a certain threshold, the search is forced to remain wide enough to find general, robust weights rather than "lucky" ones.
]

== Comparison: HS vs. CES
#slide[
  
  
  #table(
    columns: (1fr, 1fr, 1fr),
    inset: 8pt,
    fill: (x, y) => if y == 0 { gray.lighten(60%) } else if calc.even(y) { gray.lighten(90%) },
    [*Feature*], [*Harmony Search*], [*Cross-Entropy*],
    [Representation], [Individual vectors], [Probability distribution],
    [Improvement], [Replaces worst member], [Updates mean and variance],
    [Diversity], [Randomization ($r_"rand"$)], [Additive noise ($Z$)],
    [Strength], [Simple, fast updates], [Excellent in high dimensions]
  )
]

== Flowchart
#slide[
  #align(horizon + center)[
    #figure(
      scale(180%, include "../../report/figures/ces_flowchart.typ"),
    )
  ]
]



== Performance Comparison HSA vs. CES
#slide[
  
  Both optimization methods significantly outperform the random baseline, with CES showing a slight edge in raw performance.

  #columns(2)[
    #table(
      columns: (auto, auto, auto),
      inset: 6pt,
      stroke: 0.5pt + gray,
      fill: (x, y) => if y == 0 { gray.lighten(60%) },
      [*Method*], [*Mean*], [*Median*],
      [CES], [#summary.ces.mean], [#summary.ces.median],
      [HSA], [#summary.hsa.mean], [#summary.hsa.median],
      [Random], [#summary.random.mean], [#summary.random.median],
    )
    
    *Key Takeaways:*
    - CES and HSA distributions overlap significantly.
    - Both maintain a high "floor": lower quartiles exceed the best baseline results.
    - Performance approaches theoretical limits for short-horizon games.
  ]

  #figure(
    image("../../report/figures/rows_cleared_distribution.pdf", width: 70%),
    caption: [Clear separation between optimized agents and baseline.],
  )
]

---

== Computational Performance Comparison
#slide[
    #grid(
    columns: (1.2fr, 1fr),
    gutter: 1.5em,
    figure(
      image("../../report/figures/speed_comparison.pdf", width: 100%),
      caption: [Execution time per iteration (seconds).],
    ),
    [
      *Harmony Search (HSA):*
      - *Fast & Stable:* 12–19 seconds per iteration.
      - Lower overhead allows for the high iteration counts (#params.hsa_iterations) required for convergence.

      *Cross-Entropy (CES):*
      - *Resource Intensive:* 35–58 seconds per iteration.
      - The complexity stems from sampling and simulating large batches to update the distribution.
    ]
  )
]

== Convergence and Search Efficiency
#slide[
  
  
  There is a massive disparity in how quickly each algorithm "solves" the weight space.

  #grid(
    columns: (1.2fr, 1fr),
    gutter: 1em,
    figure(
      image("../../report/figures/fitness_over_iter.pdf", width: 100%),
      caption: [CES (Rapid) vs. HSA (Gradual)],
    ),
    [
      *Cross-Entropy (CES):*
      - *Extremely Efficient:* Typically converges in $< 5$ iterations.
      - Rapidly narrows sampling distribution.
      - *Trade-off:* Higher CPU cost per iteration (35–58s).
    ]
  )
]

== Convergence and Search Efficiency
#slide[
  
  
  There is a massive disparity in how quickly each algorithm "solves" the weight space.

  #grid(
    columns: (1.2fr, 1fr),
    gutter: 1em,
    figure(
      image("../../report/figures/fitness_over_iter.pdf", width: 100%),
      caption: [CES (Rapid) vs. HSA (Gradual)],
    ),
    [
      *Harmony Search (HSA):*
      - *Steady Improvement:* Requires the full #params.hsa_iterations budget.
      - *Trade-off:* Lower CPU cost per iteration (12–19s).
    ]
  )
]

---

== Learned Weight Analysis
#slide[
  
  Which board features actually matter for long-term survival?

  #grid(
    columns: (1fr, 1fr),
    gutter: 1em,
    figure(
      image("../../report/figures/weight_mean_std.pdf", width: 100%),
      caption: [Weight stability across independent runs.],
    ),
    [
      *Stable Features (Low $sigma$):*
      - #fmt-weights(stable-weights)
      - These are universally critical for board quality.

      *Volatile Features (High $sigma$):*
      - #fmt-weights(hv-weights)
      - High variance suggests the landscape has multiple "good" local optima or redundant features.
    ]
  )
]

---

== Feature Relationships and Directionality
#slide[
  #grid(
    columns: (1fr, 1.2fr),
    gutter: 10pt,
    [
      *Structure of the Solution:*
      - *Positive Correlation:* Height-related features (Pile Height & Blocks Above).
      - *Negative Trends:* "Transition" features (Row/Col) consistently move toward negative weights to penalize surface instability.
      - *Independence:* Most off-diagonal correlations are weak, validating the choice of features.
    ],
    figure(
      image("../../report/figures/weight_correlation.pdf", width: 100%),
      caption: [Pearson correlation of learned weights.],
    )
  )
]

---


== Weight Distribution Analysis
#slide[

  #grid(
    columns: (1.2fr, 1fr),
    gutter: 1em,
    figure(
      image("../../report/figures/weights_distribution.pdf", width: 100%),
      caption: [CES shows higher consistency (tighter violins).],
    ),
    [
      *Key Observations:*
      - *CES Consistency:* Generally produces tighter clusters, suggesting it finds a more precise "global" region.
      - *HSA Diversity:* Wider distributions indicate HSA explores a broader range of the solution landscape.
      - *Directionality:* Both agree on the polarity of key features (e.g., negative weights for transitions).
    ]
  )

]

---

