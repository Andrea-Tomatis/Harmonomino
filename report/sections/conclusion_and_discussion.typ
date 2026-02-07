#import "@preview/zero:0.6.1": num
#import "../constants.typ": hv-weights, params, stable-weights, summary, ws

= Conclusion and Discussion

We presented _Harmonomino_, a Rust-based system that optimizes Tetris-playing agents via
Harmony Search and Cross-Entropy Search over #params.n_features board-state features. Both optimizers
converge to weight vectors that dramatically outperform baselines: CES achieves a mean of
#summary.ces.mean cleared rows and HSA #summary.hsa.mean, compared to #summary.random.mean for random weights. CES reaches
higher mean and median scores than HSA while converging faster in early iterations.
These results affirm that metaheuristic optimization over a linear heuristic suffices to produce competent Tetris
agents from board-state features alone (#link(<rq1>)[RQ1]), and that CES holds a moderate advantage over
HSA under identical conditions (#link(<rq2>)[RQ2]).

Analysis of the learned weight distributions reveals some structure in the solution
space (#link(<rq3>)[RQ3]). Certain features, such as
#stable-weights.map(p => {
  let idx = p.at(0).slice(1)
  let s = p.at(1)
  [$w_#idx$ (#s.feature)]
}).join([, ], last: [, and ]),
receive consistent values across seeds, with
standard deviations below #num(params.low_variance_threshold).
Conversely, weights for
#hv-weights.map(p => {
  let idx = p.at(0).slice(1)
  let s = p.at(1)
  [$w_#idx$ (#s.feature)]
}).join([, ], last: [, and ]) vary widely, suggesting that the loss landscape admits a family of
similarly-performing solutions rather than a single optimum.

== Limitations

Several design choices constrain the generality of these findings. The simulator uses a
uniform random piece generator rather than the 7-bag system used in modern Tetris
implementations, which might change statistical properties of piece sequences. The agent
operates without lookahead, evaluating only the current piece, which limits its ability to
plan for future placements. The high variance across evaluation seeds (standard deviation
#summary.ces.std for CES and #summary.hsa.std for HSA) indicates substantial sensitivity to the random piece
sequence, and different seeds can yield performance ranging from near-baseline to several
hundred cleared rows. Finally, the reduced feature set, #params.n_features of the original 19 features from #cite(<Romero2011TetrisHarmonySearch>, form: "prose") excludes game-context heuristics such as landing height and eroded pieces, which may limit the agent's performance ceiling.

== Future Work

Promising extensions include adopting the 7-bag piece generator for closer fidelity to
standard Tetris, introducing limited lookahead (one or two pieces) to enable planning, and
reintroducing the three excluded game-context features. Hybrid optimizers that combine
Cross-Entropy sampling with local search refinement may further improve convergence speed.
Larger-scale parameter sweeps across both optimizers, together with longer simulation
lengths, would provide more robust sensitivity analyses and tighter confidence intervals.

