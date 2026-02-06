#import "@preview/intextual:0.1.1": *
#import "../constants.typ": n-features

#let score-eq = $V(s) = sum_(i=1)^#n-features w_i dot f_i (s)$

= Introduction

Tetris, the iconic puzzle video game created by Alexey Pajitnov in 1984, has attracted
substantial interest from the artificial intelligence and optimization research communities.
A Tetromino is a geometric shape composed of four squares connected orthogonally (i.e. at the edges and not the corners)  #cite(<Golomb1994Polyominoes>, form: "prose"), and in Tetris, a sequence of these pieces falls from the top of a $10 × 20$ game board.
The player must rotate and translate each piece to form complete horizontal rows, which are then cleared. The game
terminates when the accumulated pieces prevent new pieces from entering the board, making
the objective to clear as many rows as possible.

// TODO: change prose citation to use "... et al."
#cite(<Demaine2003TetrisHard>, form: "prose") proved that in the offline version of Tetris,
maximizing the number of cleared rows, maximizing the number of simultaneous four-row
clears ("Tetrises"), and minimizing the maximum height of occupied cells are all NP-complete
problems. Furthermore, these objectives are inapproximable to within a factor of
$p^(1-epsilon)$ for any $epsilon > 0$, where $p$ is the number of pieces in the sequence.
The immense complexity of Tetris is rooted in its state space, which encompasses approximately $7×2^20$ possible configurations for a standard 20×10 board, as stated in #cite(<Algorta2019TetrisSurvey>, form: "prose"). This vast state space, combined with the stochastic nature of piece generation, makes Tetris a challenging domain for both exact algorithms and heuristic approaches. As a result, researchers have turned to metaheuristic optimization techniques to develop agents capable of playing Tetris effectively, often by optimizing a set of heuristic weights that guide the agent's decision-making process.

== Research Questions

This work is guided by three research questions, which align with the broader goals of understanding and improving metaheuristic optimization for Tetris:

- *RQ1:* Can metaheuristic optimization converge to high-quality Tetris agents using only board-state features? <rq1>

- *RQ2:* What structure exists in the learned weight space; are certain features consistently emphasized? <rq2>

- *RQ3:* How does Harmony Search compare to Cross-Entropy Search under identical feature sets and simulation conditions? <rq3>

//moved on top of "related work" section, since in the instruction it's speficfied that the question we want to answer comes before the literature review.
== Related Work

// TODO: add reference(s) (maybe not Romero)
The dominant approach to building Tetris-playing agents relies on a _state-evaluation
function_: a linear combination of weighted board features that scores each possible
placement. Given $n$ feature functions $f_i (s)$ mapping a board state $s$ to a real value,
and corresponding weights $w_i$, the agent selects the move that maximizes

$ #score-eq . $ <eq-score>

The optimization problem then reduces to finding the weight vector $bold(w)$ that yields the
highest number of cleared rows #cite(<Romero2011TetrisHarmonySearch>, form: "prose").

A variety of metaheuristic and machine learning approaches have been applied to this weight
optimization problem. #cite(<Bohm2005Evolutionary>, form: "prose") used evolutionary algorithms,
evolving a population of weight vectors via selection, crossover, and mutation, and
demonstrated that relatively simple feature sets can produce competent agents.
#cite(<Chen2009AntColony>, form: "prose") applied ant colony optimization
(ACO) to Tetris using a set of feature functions, reporting results competitive with other
methods.

// TODO: maybe split into full section on ce? I think this introduction is already long enough, will see later.
An impressive result was achieved by #cite(<Szita2006NoisyCE>, form: "prose"), who applied the
noisy cross-entropy method to Tetris. By injecting noise into the cross-entropy update rule, they prevented
premature convergence of the sampling distribution.
// TODO: see
// #cite(<Thiery2009CEImprovements>, form: "prose")
// #cite(<Gabillon2013ADP>, form: "prose")
// #cite(<Langenhoven2010SwarmTetris>, form: "prose")
//
// At this point we are at half of the length of the paper
// so might want to keep this short.

== The Harmony Search Algorithm

The Harmony Search (HS) algorithm, introduced by #cite(<Geem2001HarmonySearch>, form: "prose") in
2001, is a metaheuristic optimization algorithm inspired by the improvisation process of
musicians. When musicians seek to create pleasing harmony, they may (1) play a known piece
from memory, (2) play something similar to a known piece with slight variations, or (3)
compose freely from random notes. These three strategies correspond to the three core
mechanisms of HS: harmony memory consideration, pitch adjustment, and randomization.

The Harmony Search (HS) algorithm maintains a harmony memory (HM), a population of solution vectors. Each iteration constructs a new solution by either copying a value from HM (probability $r_"accept"$) or sampling randomly, with optional perturbation (probability $r_"pa"$). If the candidate outperforms the worst solution in HM, it replaces it. HS considers all solutions in HM, unlike genetic algorithms, which recombine only two parents #cite(<Geem2001HarmonySearch>, form: "prose"), #cite(<Yang2009HSMetaheuristic>, form: "prose").

#cite(<Romero2011TetrisHarmonySearch>, form: "prose") were the first to apply Harmony Search to the
Tetris weight optimization problem. Using 19 board feature functions and a harmony memory of
size 5, their system demonstrated that HS can efficiently discover high-quality weight configurations, achieving a spawned-pieces-to-cleared-rows ratio approaching the theoretical optimum of 2.5. Our specific parameterization and implementation details are described in @sec-method-hsa.

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
  #cite(<Szita2006NoisyCE>, form: "prose") #cite(<Thiery2009CEImprovements>, form: "prose"), enabling direct comparison between the two
  metaheuristic approaches under identical feature sets and simulation conditions.

+ *Benchmarking and parameter sweep framework.* We provide a benchmarking binary with
  parameter sweep support, enabling systematic exploration of hyperparameter sensitivity for
  both optimizers.
