#import "@preview/zero:0.6.1": num

// Experiment parameters (from config.toml via params.json)
#let params = json("data/params.json")

#let n-features = params.n_features

/// Parse a CSV file into a dictionary keyed by the first column.
/// `transform` receives a dictionary of {column_name: value} for each row.
#let csv-to-dict(path, transform) = {
  let raw = csv(path)
  let headers = raw.first()
  let d = (:)
  for row in raw.slice(1) {
    let entry = (:)
    for (i, h) in headers.enumerate() {
      entry.insert(h, row.at(i))
    }
    let (key, val) = transform(entry)
    d.insert(key, val)
  }
  d
}

// Summary statistics lookup: summary.ces.mean, summary.hsa.std, etc.
#let summary = csv-to-dict("data/summary.csv", entry => (
  entry.method, (
    n:      num(entry.n),
    mean:   num(entry.mean),
    median: num(entry.median),
    std:    num(entry.std),
    ci95:   num(entry.ci95),
  )
))

// Weight statistics lookup: ws.w9.mean, ws.w9.std, etc.
#let ws = csv-to-dict("data/weight_stats.csv", entry => (
  entry.weight, (
    feature:       entry.feature_name,
    mean:          num(entry.mean),
    std:           num(entry.std),
    rank:          int(entry.stability_rank),
    high-variance: entry.high_variance == "true",
  )
))

// Derived: top-3 most stable weights (lowest std)
#let stable-weights = {
  ws.pairs()
    .filter(p => p.at(1).rank <= 3)
    .sorted(key: p => p.at(1).rank)
}

// Derived: high-variance weights (std > threshold)
#let hv-weights = {
  ws.pairs().filter(p => p.at(1).high-variance)
}

/// Format a list of weights as inline content.
/// `weights` is an array of (key, stats-dict) pairs.
/// When `show-mean` is true, appends the mean value.
#let fmt-weights(weights, show-mean: false) = {
  weights.map(p => {
    let idx = p.at(0).slice(1)
    let s = p.at(1)
    if show-mean {
      [$w_#idx$ (#s.feature, mean $approx$ #s.mean)]
    } else {
      [$w_#idx$ (#s.feature)]
    }
  }).join([, ], last: [, and ])
}
