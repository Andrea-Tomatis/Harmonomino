#import "@preview/touying:0.6.1": *
#import "@preview/clear-tub:0.2.0": *
#import "../constants.typ": *

= Conclusion

== Findings

#slide[
  == Research Questions & Findings

  #grid(
    columns: (1fr, 1fr, 1fr),
    column-gutter: 1em,
    
    // RQ1 Block
    block(
      fill: blue.lighten(95%),
      stroke: 0.5pt + blue.lighten(50%),
      inset: 12pt,
      radius: 6pt,
      height: 90%,
      [
        === RQ1: HSA
        *How effective is HSA at weight optimization?*
        #v(0.5em)
        #set text(size: 18pt)
        *Verdict:* #text(fill: orange.darken(20%))[Suboptimal but Effective.] 
        Outperforms random weights and hand-tuned baselines significantly.
      ]
    ),

    // RQ2 Block
    block(
      fill: green.lighten(95%),
      stroke: 0.5pt + green.lighten(50%),
      inset: 12pt,
      radius: 6pt,
      height: 90%,
      [
        === RQ2: CES
        *How does CES compare to HSA?*
        #v(0.5em)
        #set text(size: 18pt)
        *Verdict:* #text(fill: green.darken(20%))[Superior Performance.] 
        Outperforms HSA in convergence speed and score, despite higher per-iteration costs.
      ]
    ),

    // RQ3 Block
    block(
      fill: purple.lighten(95%),
      stroke: 0.5pt + purple.lighten(50%),
      inset: 12pt,
      radius: 6pt,
      height: 90%,
      [
        === RQ3: Stability
        *Do optimized weights converge to stable values?*
        #v(0.5em)
        #set text(size: 18pt)
        *Verdict:* #text(fill: purple.darken(20%))[Partial Convergence.] 
        Core weights are stable; secondary weights vary due to landscape stochasticity.
      ]
    )
  )
]

#slide()[
  === Limitations
  - Single-piece lookahead only
  - Fixed game length caps observable performance
  - No T-spin or hold-piece strategies
]

== Future Work

- Multi-piece lookahead and hold-piece integration
- Hybrid algorithms combining HSA exploration with CES exploitation
- Neural-network evaluation functions trained on optimized weights
- Transfer learning across board sizes and rule variants

