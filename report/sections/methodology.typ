#import "@preview/zero:0.6.1": num
#import "introduction.typ": score-eq
#import "../constants.typ": n-features, params
#import "@preview/booktabs:0.0.4": *
#import "@preview/pillar:0.3.3": cols


= Methodology

// TODO: expand a bit on methodology introduction
We implement a Tetris simulator on a $10 times 20$ board with precomputed rotation tables
and uniformly random piece generation (no 7-bag).

== Agent Environment

The simulation employs a parallelized heuristic search to determine the optimal placement for each incoming tetromino by exhaustively evaluating the state space of possible moves. The system operates under a model where only the current piece is known, and the evaluation logic relies on a specific set of weights provided as a direct input to the simulation. For every potential coordinate on the board, along with the four possible rotations, the simulation validates that the piece can legally fit within the boundaries and ensures the gravity requirement is satisfied by confirming the piece can be locked into that specific configuration.

These valid placements are explored across all possible combinations of rotation and horizontal position in parallel, utilizing thread-safe iterators to maximize computational throughput. Each resulting board state, after accounting for row clearances, is appraised by a scoring framework that calculates a scalar fitness value as in @eq-score
$ #score-eq , $
where $f_i (s)$ are board heuristics and $w_i$ are learned weights. The move yielding the highest score is executed, updating the global game state before the next piece is generated. This cycle continues until the board reaches a terminal "game over" condition or a predefined maximum move limit is reached, providing a deterministic and high-performance methodology for assessing the efficacy of different heuristic weight configurations.

The complete optimization pipeline (simulator + optimizer) is shown in @pipeline. The agent's heuristic weights are optimized by an outer loop (HSA or CES) that iteratively generates candidate weight sets, which are evaluated by the inner Tetris simulator. The simulator computes the best move for each piece based on the current board state and the provided weights, returning a fitness score that guides the optimization process.

#figure(
  image("../figures/pipeline.svg"),
  caption: [
    Optimization pipeline.
  ],
) <pipeline>

== Heuristic Feature Set

We use #n-features board features, all computable from the current board state alone. They fall into six categories:

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

This feature set is a subset used by #cite(<Romero2011TetrisHarmonySearch>, form: "prose") that does not require additional
in-game context (we exclude removed rows, landing height, and eroded pieces).

== Harmony Search Algorithm (HSA) <sec-method-hsa>

The optimization framework employs a Harmony Search Algorithm (HSA) as described in @sec-intro-hsa. As illustrated in @hsa-flowchart, the algorithm maintains a Harmony Memory (HM) of size #params.hsa_memory_size. Each iteration generates a new candidate weight vector via three mechanisms:

- *Memory Consideration*: Inheriting values from the HM with probability $r_"accept" = #params.hsa_accept_rate$.
- *Pitch Adjustment*: Applying localized perturbations to inherited values (governed by bandwidth #params.hsa_bandwidth) with probability $r_"pa" = #params.hsa_pitch_adj_rate$.
- *Random Selection*: Sampling new values globally to maintain diversity.

Candidates are evaluated by averaging performance over multiple stochastic simulation runs to ensure robustness. If a candidate outperforms the weakest individual in the HM, it is replaced. The implementation always exhausts the budget of #params.hsa_iterations iterations, as it does not employ early stopping.

#figure(
  include "../figures/hsa_flowchart.typ",
  caption: [Flowchart of the Harmony Search Algorithm (HSA). Each iteration generates a candidate by balancing memory exploitation and random exploration, replacing the worst member if fitness improves.],
) <hsa-flowchart>

== Cross-Entropy Search (CES) Algorithm <sec-method-ces>

The framework utilizes Cross-Entropy Search (CES) presented in @sec-intro-ces to optimize weights by treating the search as a rare-event estimation problem. Unlike the population-based HSA, CES maintains a parameterized multivariate Gaussian distribution over the weight space. The iterative process follows a linear sequence of sampling, evaluation, and refinement:

1. *Sampling*: $N = #params.ces_n_samples$ candidate weight sets are sampled from the current distribution.
2. *Evaluation*: Candidates are scored via simulation, with results averaged over multiple runs to mitigate game stochasticity.
3. *Refinement*: The top $N_"elite" = #params.ces_n_elite$ performers are selected to recalculate the distribution's mean and variance, shifting probability mass toward high-fitness regions.

The search begins with a standard deviation of #params.ces_initial_std_dev and enforces a floor of #params.ces_std_dev_floor to prevent premature convergence. CES employs early stopping if the best fitness reaches #params.ces_early_stop_target; otherwise, it exhausts the budget of #params.ces_iterations iterations. The procedure is summarized in @ces-flowchart.



#figure(
  scope: "parent",
  include "../figures/ces_flowchart.typ",
  caption: [Flowchart of the Cross-Entropy Search (CES) algorithm. The structure emphasizes the cyclic update of distribution parameters based on elite samples.],
) <ces-flowchart>

== Experimental Protocol

We run #params.training_seeds training seeds for each optimizer with a simulation length of #params.training_sim_length pieces and a
maximum of #params.hsa_iterations iterations per seed. CES may terminate early when its
fitness target is reached; HSA always exhausts its full iteration budget. For
evaluation we use #params.eval_seeds fixed seeds with a simulation length of #params.eval_sim_length pieces. As a baseline we
evaluate #params.random_weights_count random weight vectors sampled uniformly from $[-1, 1]$.
We report mean, median, standard deviation, and 95% confidence intervals of cleared rows,
plus convergence, early-stopping, and weight-distribution plots.
