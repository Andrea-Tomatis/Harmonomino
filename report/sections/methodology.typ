#import "@preview/zero:0.6.1": num
#import "introduction.typ": score-eq
#import "../constants.typ": n-features, params
#import "@preview/booktabs:0.0.4": *
#import "@preview/pillar:0.3.3": cols


= Methodology

== Game Model

We implement a Tetris simulator on a $10 times 20$ board with precomputed rotation tables
and uniformly random piece generation (no 7-bag).

== Agent Environment

The simulation employs a parallelized heuristic search to determine the optimal placement for each incoming tetromino by exhaustively evaluating the state space of possible moves. The system operates under a model where only the current piece is known, and the evaluation logic relies on a specific set of weights provided as a direct input to the simulation. For every potential coordinate on the board, along with the four possible rotations, the simulation validates that the piece can legally fit within the boundaries and ensures the gravity requirement is satisfied by confirming the piece can be locked into that specific configuration.

These valid placements are explored across all possible combinations of rotation and horizontal position in parallel, utilizing thread-safe iterators to maximize computational throughput. Each resulting board state, after accounting for row clearances, is appraised by a scoring framework that calculates a scalar fitness value as in @eq-score
$ #score-eq , $
where $f_i (s)$ are board heuristics and $w_i$ are learned weights. The move yielding the highest score is executed, updating the global game state before the next piece is generated. This cycle continues until the board reaches a terminal "game over" condition or a predefined maximum move limit is reached, providing a deterministic and high-performance methodology for assessing the efficacy of different heuristic weight configurations.

The complete optimization pipeline (simulator + optimizer) is shown in //@pipeline. Some additional text

//#image("tetris_agent_environment.png", caption: "")<pipeline>

== Heuristic Feature Set

We use #n-features board features, all computable from the current board state alone. They fall into
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

== Harmony Search Algorithm (HSA) <sec-method-hsa>

// TODO: commented out for now, because I think it is more suitable in the introduction with proper citations.
// Or maybe instead, we reference @sec-method-hsa from the introduction. (Yes, I like that)

The optimization framework utilizes a Harmony Search Algorithm (HSA) to iteratively refine the agent's heuristic weights, mimicking the process of musical improvisation to find an optimal "harmony" or set of parameters.
The process begins by initializing a Harmony Memory (HM), a population of weight vectors, where each individual is evaluated using the simulation logic described previously. To account for the inherent randomness of piece sequences, the framework can be configured to average the performance metrics over multiple stochastic runs, ensuring that the resulting weight sets are robust rather than merely lucky.

During each optimization iteration, the algorithm generates a new candidate solution by traversing the high-dimensional weight space through three distinct decision-making mechanisms. First, memory consideration allows the system to inherit values from the existing population, preserving successful traits. Second, pitch adjustment applies a localized perturbation, governed by a bandwidth parameter, to these inherited values, enabling a fine-tuned local search around known high-performing regions. Finally, random selection introduces entirely new values from the global bounds, maintaining diversity and preventing the search from becoming trapped in local optima.

The effectiveness of each generated candidate is assessed against the current population; if the newly generated candidate produces a score superior to the weakest member of the current memory, the inferior harmony is discarded and replaced. This continuous refinement loop persists until the algorithm reaches a user-defined convergence target, exhausts its iteration budget, or triggers an early-stopping condition if no significant improvement is observed over a specific duration. By managing this evolving population of strategies, the system effectively automates the discovery of complex weight configurations that maximize the agent's long-term survival and clearing efficiency.
// TODO: include HSA pseudo-code/graph

HSA maintains a harmony memory (HM) of candidate weight vectors. New candidates are created
by selecting values from HM with probability $r_"accept"$, applying pitch adjustment with
probability $r_"pa"$, or sampling randomly otherwise. The worst candidate in HM is replaced
if a better solution is found. Unless stated otherwise, we use: HM size #params.hsa_memory_size, up to #params.hsa_iterations iterations,
$r_"accept" = #params.hsa_accept_rate$, $r_"pa" = #params.hsa_pitch_adj_rate$, and bandwidth #params.hsa_bandwidth.
HSA does not employ early stopping and always exhausts its iteration budget.

== Cross-Entropy Search (CES) Algorithm <sec-method-ces>

The optimization framework also supports Cross-Entropy Search (CES), a distribution-based method that interprets the search for optimal weights as a problem of rare-event estimation. Unlike the discrete population management of Harmony Search, CES maintains a parameterized probability distribution—specifically a set of Gaussian distributions—over the space of possible weight configurations. The process begins with an initial set of means and a relatively broad standard deviation to ensure a wide exploratory reach across the high-dimensional parameter space.

In each iteration, the algorithm samples a predefined number of candidate weight sets from the current normal distributions. These candidates are then evaluated through the simulation environment to determine their fitness. To ensure the robustness of the results against the stochastic nature of the game, the framework can average these performance results over multiple runs. Once all candidates in a generation are scored, the algorithm identifies a top-tier subset known as the elite samples. The means and standard deviations of the distributions are then recalculated to fit these elite performers, effectively shifting and narrowing the search toward the most promising regions of the weight space.

To prevent the search from collapsing prematurely into a single point, a standard deviation floor is enforced, maintaining a minimum level of exploration throughout the process. The cycle of sampling, elite selection, and distribution updating repeats until a termination criterion, such as an early-stopping target or the maximum iteration limit, is satisfied. This methodology provides a mathematically principled way to refine strategies, allowing the system to converge on high-performing heuristic configurations by progressively concentrating its sampling probability around the global optimum.

CES models weights with a multivariate Gaussian. Each iteration samples $N$ candidates,
selects the top $N_"elite"$, and updates the mean and variance from the elite set. We use
$N=#params.ces_n_samples$, $N_"elite" = #params.ces_n_elite$, up to #params.ces_iterations iterations, an initial standard deviation of #params.ces_initial_std_dev, and a
minimum standard deviation floor of #params.ces_std_dev_floor. CES employs early stopping:
optimization terminates when the best fitness reaches or exceeds a target score of
#params.ces_early_stop_target, allowing seeds that converge quickly to finish well before
the iteration budget is exhausted.

== Experimental Protocol

We run #params.training_seeds training seeds for each optimizer with a simulation length of #params.training_sim_length pieces and a
maximum of #params.hsa_iterations iterations per seed. CES may terminate early when its
fitness target is reached; HSA always exhausts its full iteration budget. For
evaluation we use #params.eval_seeds fixed seeds with a simulation length of #params.eval_sim_length pieces. As a baseline we
evaluate #params.random_weights_count random weight vectors sampled uniformly from $[-1, 1]$.
We report mean, median, standard deviation, and 95% confidence intervals of cleared rows,
plus convergence, early-stopping, and weight-distribution plots.
