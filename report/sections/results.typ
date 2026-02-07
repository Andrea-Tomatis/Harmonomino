#import "@preview/zero:0.6.1": num, ztable
#import "@preview/booktabs:0.0.4": *
#import "@preview/pillar:0.3.3": cols
#import "../constants.typ": hv-weights, params, stable-weights, summary, ws

// TODO: remove titles from plots and make them smaller (so the font is bigger)

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
// NOTE: this is awesome right?
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

@fig-corr shows the pairwise Pearson correlation between learned weights across all
optimized runs. Most off-diagonal correlations are weak, indicating that the features
capture largely independent aspects of board quality.
// TODO: write better
However, something something about opisite features in threes not being mapped.
// TODO: comment on strong correlations in blocks above highest and pile height, and others.

// TODO: reference consistency_test.pdf and consistency_error.pdf in a consistency section
// TODO: reference weight_histograms.pdf and weight_categories.pdf in the weight analysis section
// TODO: reference dbscan_stability.pdf, k_distance_elbow.pdf, and pca_weights.pdf in the weight analysis section

== Parameter Sensitivity

To assess hyperparameter sensitivity for Harmony Search, we sweep three key parameters
while holding the others fixed.

// TODO: adjust ylim to go from 193 to 202, like @fig-iter
#figure(
  image("../figures/benchmark_bandwidth.pdf"),
  caption: [Effect of pitch-adjustment bandwidth on agent score.],
) <fig-bw>

// TODO: This shows nothing, run until 200 iters instead, with more intermediate values.
#figure(
  image("../figures/benchmark_iterations.pdf"),
  caption: [Effect of maximum iterations on agent score.],
) <fig-iter>

// TODO: also make consistent y-lims
#figure(
  image("../figures/benchmark_pitch_adj_rate.pdf"),
  caption: [Effect of pitch-adjustment rate on agent score.],
) <fig-par>

// NOTE: These results are pretty disapointing, hope everything went well.
@fig-bw shows that bandwidth has a moderate effect: too-small values restrict exploration,
while excessively large values introduce disruptive perturbations. @fig-iter confirms
// TODO: 200 needs to be updated after re-running @fig-iter
diminishing returns beyond roughly 200 iterations, consistent with the convergence analysis
above. @fig-par indicates that the pitch-adjustment rate has little effect as well on final
performance.
