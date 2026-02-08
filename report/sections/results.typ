#import "@preview/zero:0.6.1": num, ztable
#import "@preview/booktabs:0.0.4": *
#import "@preview/pillar:0.3.3": cols
#import "../constants.typ": hv-weights, params, stable-weights, summary, ws


= Results

== Agent Performance

@tbl-summary summarizes the evaluation results across all methods.
Each method is evaluated over $n$ independent games using fixed seeds.
$sigma$ denotes the sample standard deviation and _CI~95%_ is the 95% confidence interval
for the mean, computed as $1.96 dot sigma slash sqrt(n)$.

Both optimized methods
dramatically outperform the baseline: CES achieves a mean of #summary.ces.mean cleared
rows (median #summary.ces.median) and HSA a mean of #summary.hsa.mean (median
#summary.hsa.median), compared to #summary.random.mean for random weights.

// TODO: make zero table work
#figure(
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

@fig-dist shows the full distribution of cleared rows. The box plots confirm that both
// NOTE: Why is this variance so high?
optimized methods exhibit substantial variance (standard deviation #summary.ces.std for CES
and #summary.hsa.std for HSA), yet their
lower quartiles still comfortably exceed the best baseline outcomes.
// NOTE: check in the end
CES edges out HSA in both mean and median, though the distributions overlap considerably.

#figure(
  image("../figures/rows_cleared_distribution.pdf"),
  caption: [Distribution of cleared rows for HSA, CES, and baselines.],
) <fig-dist>

== Convergence Properties

// TODO: update to standard min-max-std-mean plot or whatever it's called
// Also add baseline
#figure(
  scope: "parent",
  image("../figures/fitness_over_iter.pdf"),
  caption: [Mean and best fitness across iterations for HSA and CES.],
) <fig-conv>

@fig-conv traces the best and mean fitness of each optimizer over up to #params.hsa_iterations iterations. CES
converges rapidly, quickly narrowing its sampling
distribution around high-fitness regions. HSA follows a more gradual trajectory, with
steady improvements throughout the run as the harmony memory slowly replaces weaker
candidates.

== Early Stopping

#figure(
  image("../figures/stopping_iterations.pdf"),
  caption: [Iteration at which each seed's optimization terminated.],
) <fig-stop>

@fig-stop shows the actual number of iterations used by each seed before optimization
terminated. CES employs early stopping with a fitness target of #num(params.ces_early_stop_target) and consistently
converges well before exhausting its #params.ces_iterations\-iteration budget, typically
finishing within 5 iterations. HSA does not use early stopping and always runs for the
full #params.hsa_iterations iterations. This confirms that CES is dramatically more
efficient for this problem: it finds near-optimal weights in an order of magnitude
fewer iterations than the allotted budget, while HSA requires the entire budget and still
shows gradual improvements through the final iterations.


This difference in performance also shows up in the speed of execution, as shown in @fig-time, where CES runs much slower than HSA. A clear performance gap is observed between the two configurations, reflecting the different computational demands of each approach. The accurate HS method exhibits relatively stable execution times, generally ranging between 12 and 19 seconds per iteration. In contrast, the accurate CES method is significantly more computationally intensive, with processing times consistently exceeding those of HS and fluctuating between approximately 35 and 58 seconds.

// TODO: Please don't put in figures manually because they can easily change. Also make it pdf, if
// you use the script everything should work great.

#figure(
  [Placeholder],
  // FIX: This file doesn't exist, that breaks the entire compilation
  // image("../figures/speed_comparison.pdf"),
  caption: [Execution time comparison of the two algorithms (seconds).],
) <fig-time>


== Weight Distribution and Analysis

// NOTE: this plot is great, maybe we should stress more that ces is much more constistent
#figure(
  image("../figures/weights_distribution.pdf"),
  caption: [Violin plots of learned weight distributions for HSA and CES.],
) <fig-violin>

#figure(
  image("../figures/weight_mean_std.pdf"),
  caption: [Mean and standard deviation of each weight across all optimized runs.],
) <fig-mean-std>

@fig-violin and @fig-mean-std[] reveal the structure of the learned weight space. The most
consistent weights, those with low standard deviation across seeds, include
// NOTE: this is awesome right? Confirmed
#stable-weights.map(p => {
  let idx = p.at(0).slice(1)
  let s = p.at(1)
  [$w_#idx$ (#s.feature, mean $approx$ #s.mean)]
}).join([, ], last: [, and ]).
These features are assigned decisive, stable values regardless of the optimization seed, suggesting they
capture the most important aspects of board quality.

In contrast, several weights show high variance:
#hv-weights.map(p => {
  let idx = p.at(0).slice(1)
  let s = p.at(1)
  [$w_#idx$ (#s.feature)]
}).join([, ], last: [, and ]) all have standard
deviations exceeding #num(params.high_variance_threshold). This indicates that multiple weight configurations achieve
similar performance, and that the solution landscape admits a family of good solutions rather than a single optimum.

#figure(
  image("../figures/weight_correlation.pdf"),
  caption: [Pairwise Pearson correlation of learned weights across all optimized runs.],
) <fig-corr>

@fig-corr shows the pairwise Pearson correlation between learned weights across all optimized runs. Most off-diagonal correlations are weak, indicating that the features capture largely independent aspects of board quality. However, notable positive correlations exist between height-related features, specifically Blocks Above Highest and Pile Height, as well as between Holes and Connected Holes. Conversely, several features show near-zero or slightly negative correlations, such as Max Well Depth relative to Row Transitions, suggesting the optimization process successfully distinguishes between internal board structures and surface-level instability.


The learned weight distributions demonstrate clear directional trends for specific game-state features. As shown in the weight histograms @fig-hist, features like Row Transitions and Col Transitions consistently gravitate toward negative values, while others are aggregated by category to show their mean impact.

#figure(
  image("../figures/weight_histograms.pdf", width: 80%),
  caption: [Frequency distribution of learned weights across all optimization runs.],
)<fig-hist>

#figure(
  image("../figures/weight_categories.pdf", width: 80%),
  caption: [Mean weight values grouped by feature category showing relative importance.],
)<fig-cat>

The consistency of these results is validated through clustering. The k-distance plot in @fig-cluster identifies an elbow at approximately 1.35, providing a principled epsilon value for DBSCAN. Using this parameter, the stability heatmap reveals that the majority of optimization runs converge into a single primary cluster.

#figure(
  grid(
    columns: (1fr, 1.2fr),
    gutter: 10pt,
    image("../figures/k_distance_elbow.pdf", width: 100%), image("../figures/dbscan_stability.pdf", width: 100%),
  ),
  caption: [K-distance elbow plot and DBSCAN stability analysis.],
) <fig-cluster>

// TODO: reference consistency_test.pdf and consistency_error.pdf in a consistency section
== Consistency Analysis
The consistency of the simulation environment was evaluated by comparing empirical results against a theoretical performance model across varying game lengths. As the game length increases, the simulation results initially follow the theoretical maximum closely but begin to plateau after a length of approximately 500.

#figure(
  image("../figures/consistency_test.pdf", width: 90%),
  caption: [Comparison between simulation results and the theoretical maximum score across increasing game lengths.],
) <fig-consistency-test>

The divergence between the theoretical expectation and the actual agent performance is further detailed in the absolute error analysis shown in @fig-consistency-error. While the error remains near zero for shorter durations (up to a game length of 500), it grows significantly as game length exceeds 750, eventually reaching an absolute error of over 1750 at a game length of 5000.

#figure(
  image("../figures/consistency_error.pdf", width: 90%),
  caption: [Absolute error between theoretical and simulation results as a function of game length.],
) <fig-consistency-error>

As illustrated in @fig-consistency-test, the agent's performance becomes decoupled from the theoretical maximum as the simulation progresses. This trend indicates that while the agent is highly consistent in short-term scenarios, long-term performance is capped by constraints that the theoretical model does not account for.

== Parameter Sensitivity

To assess hyperparameter sensitivity for Harmony Search, we sweep three key parameters
while holding the others fixed.

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

// NOTE: These results are pretty disapointing, hope everything went well. Yeah technically one could argue that those graphs are useless because for example if you keep all the parameters fixed and you vary only pitch adjustment rate the only think that could reasonably change is the convergence speed and not the final peformance. While having bandwidth=0.1 or =1.0 doesn't change much as all the weight will be scaled by a factor of 10 but they will still mantain their relative importance.
@fig-bw shows that bandwidth has a moderate effect: too-small values restrict exploration,
while excessively large values introduce disruptive perturbations. @fig-iter confirms
diminishing returns beyond roughly 170 iterations, consistent with the convergence analysis
above. @fig-par indicates that the pitch-adjustment rate has little effect as well on final
performance.
