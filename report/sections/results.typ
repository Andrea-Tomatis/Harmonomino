#import "@preview/zero:0.6.1": num, ztable
#import "@preview/booktabs:0.0.4": *
#import "@preview/pillar:0.3.3": cols
#import "../constants.typ": fmt-weights, hv-weights, params, stable-weights, summary, ws


= Results
// TODO: small introduction to results section (not: "in this section...")

== Agent Performance

@tbl-summary summarizes the evaluation results across all methods.
Each method is evaluated over $n$ independent games using fixed seeds.
$sigma$ denotes the sample standard deviation and _CI~95%_ is the 95% confidence interval
for the mean, computed as $1.96 dot sigma slash sqrt(n)$.

Both optimized methods dramatically outperform the baseline:
CES achieves a mean of #summary.ces.mean cleared rows (median #summary.ces.median) and HSA a mean of #summary.hsa.mean (median #summary.hsa.median),
compared to #summary.random.mean for random weights.

// TODO: make zero table work
#figure(
  placement: auto,
  scope: "parent",
  ztable(
    ..cols("c|CCCCC"),
    toprule(),
    table.header[*Method*][*n*][*Mean*][*Median*][*σ*][*CI~95%*],
    midrule(),
    [CES],
    [#summary.ces.n],
    [#summary.ces.mean],
    [#summary.ces.median],
    [#summary.ces.std],
    [#summary.ces.ci95],
    [HSA],
    [#summary.hsa.n],
    [#summary.hsa.mean],
    [#summary.hsa.median],
    [#summary.hsa.std],
    [#summary.hsa.ci95],
    [Random],
    [#summary.random.n],
    [#summary.random.mean],
    [#summary.random.median],
    [#summary.random.std],

    [#summary.random.ci95],

    bottomrule(),
  ),
  caption: [Evaluation statistics for each method (rows cleared over 30 fixed seeds).],
) <tbl-summary>

@fig-dist shows the full distribution of cleared rows.
The box plots confirm that both optimized methods exhibit substantial variance (standard deviation #summary.ces.std for CES and #summary.hsa.std for HSA),
yet their lower quartiles still comfortably exceed the best baseline outcomes.
CES edges out HSA in both mean and median, though the distributions overlap considerably.

#figure(
  image("../figures/rows_cleared_distribution.pdf"),
  caption: [Distribution of rows cleared across #params.eval_seeds evaluation seeds for each method.],
) <fig-dist>

== Convergence Properties

#figure(
  placement: auto,
  scope: "parent",
  image("../figures/fitness_over_iter.pdf"),
  caption: [
    Convergence of best fitness for HSA and CES, showing the mean, standard deviation ribbon,
    and min--max envelope across #params.training_seeds training seeds.
  ],
) <fig-conv>

@fig-conv traces the best and mean fitness of each optimizer over up to #params.hsa_iterations iterations.
CES converges rapidly, quickly narrowing its sampling distribution around high-fitness regions.
HSA follows a more gradual trajectory, with steady improvements throughout the run as the harmony memory slowly replaces weaker candidates.

// NOTE: Stopping iterations figure removed: the convergence plot already shows
// that CES converges early and HSA uses its full budget.

In practice, CES employs early stopping with a fitness target of #num(params.ces_early_stop_target)
and consistently converges well before exhausting its #params.ces_iterations\-iteration budget,
typically finishing within 5 iterations.
HSA does not use early stopping and always runs for the full #params.hsa_iterations iterations.
This confirms that CES is dramatically more efficient for this problem:
it finds near-optimal weights in an order of magnitude fewer iterations than the allotted budget,
while HSA requires the entire budget and still shows gradual improvements through the final iterations.

== Weight Distribution and Analysis

// NOTE: this plot is great, maybe we should stress more that ces is much more constistent
#figure(
  image("../figures/weights_distribution.pdf"),
  caption: [Distribution of learned weights for each evaluation function under HSA and CES.],
) <fig-violin>

@fig-violin and @fig-cat[] reveal the structure of the learned weight space.
The most consistent weights, those with low standard deviation across seeds, include #fmt-weights(stable-weights, show-mean: true).
These features are assigned decisive, stable values regardless of the optimization seed,
suggesting they capture the most important aspects of board quality.

In contrast, several weights show high variance:
#fmt-weights(hv-weights) all have standard deviations exceeding #num(params.high_variance_threshold).
This indicates that multiple weight configurations achieve similar performance,
and that the solution landscape admits a family of good solutions rather than a single optimum.

#figure(
  image("../figures/weight_correlation.pdf"),
  caption: [Pairwise Pearson correlation between learned weights across all #params.mass_optimize_count optimization runs.],
) <fig-corr>

@fig-corr shows the pairwise Pearson correlation between learned weights across all optimized runs.
Most off-diagonal correlations are weak, indicating that the features capture largely independent aspects of board quality.
However, notable positive correlations exist between height-related features,
specifically Blocks Above Highest and Pile Height, as well as between Holes and Connected Holes.
Conversely, several features show near-zero or slightly negative correlations, such as Max Well Depth relative to Row Transitions,
suggesting the optimization process successfully distinguishes between internal board structures and surface-level instability.

The learned weight distributions demonstrate clear directional trends for specific game-state features.
As shown in the weight histograms @fig-hist, features like Row Transitions and Col Transitions consistently gravitate toward negative values,
while others are aggregated by category to show their mean impact.

#figure(
  placement: auto,
  scope: "parent",
  image("../figures/weight_histograms.pdf"),
  caption: [Per-feature histograms of learned weight values across all #params.mass_optimize_count optimization runs.],
) <fig-hist>

#figure(
  placement: auto,
  scope: "parent",
  image("../figures/weight_categories.pdf"),
  caption: [Mean learned weight values grouped by feature category.],
) <fig-cat>

The consistency of these results is validated through clustering.
The k-distance plot in @fig-cluster identifies an elbow at approximately 1.35, providing a principled epsilon value for DBSCAN.
Using this parameter, the stability heatmap reveals that the majority of optimization runs converge into a single primary cluster.

#figure(
  grid(
    align: horizon,
    columns: (1fr, 1fr),
    gutter: 10pt,
    image("../figures/k_distance_elbow.pdf"), image("../figures/dbscan_stability.pdf"),
  ),
  caption: [K-distance elbow plot (left) and DBSCAN cluster stability across epsilon values (right).],
) <fig-cluster>

== Consistency Analysis

The consistency of the simulation environment was evaluated by comparing empirical results against a theoretical performance model across varying game lengths.
As the game length increases, the simulation results initially follow the theoretical maximum closely but begin to plateau after a length of approximately 500.

#figure(
  image("../figures/consistency_test.pdf"),
  caption: [Agent score versus theoretical maximum across increasing game lengths.],
) <fig-consistency-test>

The divergence between the theoretical expectation and the actual agent performance is further detailed in the absolute error analysis shown in @fig-consistency-error.
While the error remains near zero for shorter durations (up to a game length of 500),
it grows significantly as game length exceeds 750, eventually reaching an absolute error of over 1750 at a game length of 5000.

#figure(
  image("../figures/consistency_error.pdf"),
  caption: [Absolute error between theoretical maximum and agent score as a function of game length.],
) <fig-consistency-error>

As illustrated in @fig-consistency-test, the agent's performance becomes decoupled from the theoretical maximum as the simulation progresses.
This trend indicates that while the agent is highly consistent in short-term scenarios,
long-term performance is capped by constraints that the theoretical model does not account for.

== Parameter Sensitivity

To assess hyperparameter sensitivity for Harmony Search, we sweep three key parameters,
namely pitch-adjustment bandwidth and rate and maximum iterations, while holding the others fixed.

#figure(
  image("../figures/benchmark_bandwidth.pdf"),
  caption: [Effect of pitch-adjustment bandwidth on agent score.],
) <fig-bw>


#figure(
  image("../figures/benchmark_iterations.pdf"),
  caption: [Effect of maximum iterations on agent score.],
) <fig-iter>


#figure(
  image("../figures/benchmark_pitch_adj_rate.pdf"),
  caption: [Effect of pitch-adjustment rate on agent score.],
) <fig-par>

@fig-bw shows that bandwidth has a moderate effect:
too-small values restrict exploration, while excessively large values introduce disruptive perturbations.
@fig-iter confirms diminishing returns beyond roughly 170 iterations, consistent with the convergence analysis above.
@fig-par indicates that the pitch-adjustment rate has little effect as well on final performance.
