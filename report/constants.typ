#import "@preview/zero:0.6.1": num

// Experiment parameters (from config.toml via params.json)
#let params = json("data/params.json")

#let n-features = params.n_features

// Summary statistics lookup: summary.ces.mean, summary.hsa.std, etc.
#let summary = {
  let raw = csv("data/summary.csv")
  let d = (:)
  for i in range(1, raw.len()) {
    let row = raw.at(i)
    d.insert(row.at(0), (
      n:      num(row.at(1)),
      mean:   num(row.at(2)),
      median: num(row.at(3)),
      std:    num(row.at(4)),
      ci95:   num(row.at(5)),
    ))
  }
  d
}

// Weight statistics lookup: ws.w9.mean, ws.w9.std, etc.
#let ws = {
  let raw = csv("data/weight_stats.csv")
  let d = (:)
  for i in range(1, raw.len()) {
    let row = raw.at(i)
    d.insert(row.at(0), (
      feature:       row.at(1),
      mean:          num(row.at(2)),
      std:           num(row.at(3)),
      rank:          int(row.at(4)),
      high-variance: row.at(5) == "true",
    ))
  }
  d
}

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
