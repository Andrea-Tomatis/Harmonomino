#import "@preview/touying:0.6.1": *
#import "@preview/clear-tub:0.2.0": *
#import "constants.typ": *

#show: tub-theme.with(
  aspect-ratio: "16-9",
  department: [Faculty of Electrical Engineering and Computer Science],
  logo: image("assets/logos/tu_berlin.svg"),
  progress-bar: true,
  config-info(
    title: [Harmonomino: Tetris Agent Optimization],
    subtitle: [Using Harmony Search & Cross-Entropy Methods],
    author: [Ezra Cerpac ⋅ Andrea Tomatis],
    date: datetime.today(),
    institution: [Technische Universität Berlin],
  ),
)

// NOTE: best to choose alignment on a per-slide basis
// #set align(horizon)

#title-slide()

// I'm not a fan of outlines, but let me know what you think
// #outline-slide()

#include "sections/background.typ"
#include "sections/results.typ"
#include "sections/conclusion.typ"

#ending-slide(title: [Thank You!])[
  #text(size: 0.9em)[Ezra Cerpac ⋅ Andrea Tomatis]

  #v(0.3cm)

  #text(size: 0.75em, fill: tub-gray)[
    Scientific Computing \
    Technische Universität Berlin
  ]

  #v(0.5cm)

  #text(size: 1em)[_Questions?_]
]

