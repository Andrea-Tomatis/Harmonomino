#import "@preview/intextual:0.1.1": *
#import "../constants.typ": n-features

#let score-eq = $V(s) = sum_(i=1)^#n-features w_i dot f_i (s)$

= Introduction

Tetris, the iconic puzzle video game created by Alexey Pajitnov in 1984, has attracted
substantial interest from the artificial intelligence and optimization research communities.
In Tetris, a sequence of Tetromino pieces,
geometric shapes composed of four connected squares, // TODO: add actual definition
falls from the top of a $10 × 20$ game board. The player must rotate and
translate each piece to form complete horizontal rows, which are then cleared. The game
terminates when the accumulated pieces prevent new pieces from entering the board, making
the objective to clear as many rows as possible.

// TODO: change prose citation to use "... et al."
#cite(<Demaine2003TetrisHard>, form: "prose") proved that in the offline version of Tetris,
maximizing the number of cleared rows, maximizing the number of simultaneous four-row
clears ("Tetrises"), and minimizing the maximum height of occupied cells are all NP-complete
problems. Furthermore, these objectives are inapproximable to within a factor of
$p^(1-epsilon)$ for any $epsilon > 0$, where $p$ is the number of pieces in the sequence.
This NP-completeness result establishes Tetris as an interesting testbed for metaheuristic
optimization algorithms, as no polynomial-time algorithm can guarantee an optimal solution.

// TODO: add something like:
//from @Algorta2019TetrisSurvey: "Tetris is estimated to have 7 × 2^200 states"
// anyways, this is a great paper to read more into.

== Related Work

// TODO: add reference(s) (maybe not Romero)
The dominant approach to building Tetris-playing agents relies on a _state-evaluation
function_: a linear combination of weighted board features that scores each possible
placement. Given $n$ feature functions $f_i (s)$ mapping a board state $s$ to a real value,
and corresponding weights $w_i$, the agent selects the move that maximizes

$ #score-eq . $ <eq-score>

The optimization problem then reduces to finding the weight vector $bold(w)$ that yields the
highest number of cleared rows @Romero2011TetrisHarmonySearch.

A variety of metaheuristic and machine learning approaches have been applied to this weight
optimization problem. #cite(<Bohm2005Evolutionary>, form: "prose") used evolutionary algorithms,
evolving a population of weight vectors via selection, crossover, and mutation, and
demonstrated that relatively simple feature sets can produce competent agents.
#cite(<Chen2009AntColony>, form: "prose") applied ant colony optimization
(ACO) to Tetris using a set of feature functions, reporting results competitive with other
methods.

// TODO: maybe split into full section on ce?
An impressive result was achieved by #cite(<Szita2006NoisyCE>, form: "prose"), who applied the
noisy cross-entropy method to Tetris. By injecting noise into the cross-entropy update rule, they prevented
premature convergence of the sampling distribution.
// TODO: see
// #cite(<Thiery2009CEImprovements>, form: "prose")
// #cite(<Gabillon2013ADP>, form: "prose")
// #cite(<Langenhoven2010SwarmTetris>, form: "prose")

== The Harmony Search Algorithm

The Harmony Search (HS) algorithm, introduced by #cite(<Geem2001HarmonySearch>, form: "prose") in
2001, is a metaheuristic optimization algorithm inspired by the improvisation process of
musicians. When musicians seek to create pleasing harmony, they may (1) play a known piece
from memory, (2) play something similar to a known piece with slight variations, or (3)
compose freely from random notes. These three strategies correspond to the three core
mechanisms of HS: harmony memory consideration, pitch adjustment, and randomization.
// TODO: this is kind of from @Romero2011TetrisHarmonySearch, check @Yang2009HSMetaheuristic instead

The algorithm maintains a harmony memory (HM), a population of solution vectors
analogous to a set of musical compositions. In each iteration, a new solution is constructed
by, for each variable, either copying a value from a randomly selected existing
solution in HM (with probability $r_"accept"$) or sampling a random value. Copied values may
then be perturbed by a small amount (with probability $r_"pa"$). If the new candidate outperforms
the worst solution in HM, it replaces it. HS does not require derivative information, and considers all existing solutions when generating a new candidate, unlike genetic algorithms, which typically recombine
only two parents @Geem2001HarmonySearch @Yang2009HSMetaheuristic. // TODO: better cite sources

#cite(<Romero2011TetrisHarmonySearch>, form: "prose") were the first to apply Harmony Search to the
Tetris weight optimization problem. Using 19 board feature functions and a harmony memory of
size 5, their system demonstrated that HS can efficiently discover high-quality weight configurations, achieving a spawned-pieces-to-cleared-rows ratio approaching the theoretical optimum of 2.5.

== Research Questions
// TODO: incorportate better in rest of text
This work is guided by three research questions:

- *RQ1:* Can metaheuristic optimization converge to high-quality Tetris agents using only board-state features? <rq1>
- *RQ2:* How does Harmony Search compare to Cross-Entropy Search under identical feature sets and simulation conditions? <rq2>
- *RQ3:* What structure exists in the learned weight space --- are certain features consistently emphasized? <rq3>

== Contributions

This work presents _Harmonomino_, a Tetris agent optimization system implemented in Rust
Our contributions are:
that builds upon and extends the approach of #cite(<Romero2011TetrisHarmonySearch>, form: "prose").

+ *Reimplementation and refinement.* We reimplement the Harmony Search-based Tetris
  optimizer in Rust for improved performance. Of the original 19 feature functions, we retain
  16 that depend solely on the board state, removing three (removed rows, landing height,
  eroded pieces) that require game-context information beyond the current board configuration.

+ *Cross-Entropy Search as a comparative optimizer.* In addition to the Harmony Search
  algorithm, we implement a Cross-Entropy Search (CES) optimizer // TODO: check references
  @Szita2006NoisyCE @Thiery2009CEImprovements, enabling direct comparison between the two
  metaheuristic approaches under identical feature sets and simulation conditions.

+ *Benchmarking and parameter sweep framework.* We provide a benchmarking binary with
  parameter sweep support, enabling systematic exploration of hyperparameter sensitivity for
  both optimizers.
