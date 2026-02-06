#import "@preview/elsearticle:2.0.0": *

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
  format: "3p", // (review, 1p, 3p, 5p, final)
  numcol: 1,
  // line-numbering: true,
)


#include "sections/introduction.typ"
#include "sections/methodology.typ"
#include "sections/results.typ"
#include "sections/conclusion_and_discussion.typ"

// TODO: make bracket link in shape [LH32]
#bibliography("refs.bib", style: "ieee")

// TODO: add disclaimer about AI tools
