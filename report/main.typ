#import "@preview/elsearticle:2.0.0": *

#import "@preview/zero:0.6.1": *
#import "@preview/booktabs:0.0.4": *
#import "@preview/equate:0.3.2": *

// NOTE: See https://isis.tu-berlin.de/pluginfile.php/3774882/mod_resource/content/1/Project-Report-Instruction.pdf

#show: elsearticle.with(
  title: "Harmonomino: Tetris Agent Optimization using Stochastic Local Search Heuristics written in Rust",
  authors: (
    (
      name: "E. Cerpac",
      email: "e.cerpac@campus.tu-berlin.de",
      institutions: ("a",),
    ),
    (
      name: "A. Tomatis",
      email: "a.tomatis@campus.tu-berlin.de", // TODO: is this correct?
      institutions: ("a",),
    ),
  ),
  institutions: ("a": [Technische Universität Berlin, Berlin, Germany]),
  keywords: (
    // TODO: maybe to many keywords?
    "Harmony Search Algorithm",
    "Tetris",
    "Agentic Learning",
    "Artificial Intelligence",
    "Genetic Algorithms",
  ),
  // TODO: Choose between formats; I like two-column 5p, but we get much fewer pages out of it
  format: "5p", // (review, 1p, 3p, 5p, final)
  numcol: 2,
  // format: "3p", // (review, 1p, 3p, 5p, final)
  // numcol: 1,
  // line-numbering: true,
)

#set text(font: "New Computer Modern")
#show: booktabs-default-table-style
#set-round(
  mode: "figures",
  precision: 3,
  pad: false,
  direction: "nearest",
)
#show figure.caption: emph
#show figure.where(
  kind: table,
): set figure.caption(position: top)

#include "sections/introduction.typ"
#include "sections/methodology.typ"
#include "sections/results.typ"
#include "sections/conclusion_and_discussion.typ"

// TODO: make bracket link in shape [LH32]
#bibliography("refs.bib", style: "ieee")

// TODO: add disclaimer about AI tools
