#import "@preview/elsearticle:2.0.0": *

// See: https://isis.tu-berlin.de/pluginfile.php/3774882/mod_resource/content/1/Project-Report-Instruction.pdf

#let small_cite(url) = footnote([#link(url, url)])

#let abstract = lorem(100)

#show: elsearticle.with(
  title: "Harmonomino: Tetris Agent Optimization using Harmony Search Agent written in Rust",
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
  abstract: abstract,
  keywords: (
    // TODO: maybe to many keywords?
    "Harmony Search Algorithm",
    "Tetris",
    "Agentic Learning",
    "Artificial Intelligence",
    "Genetic Algorithms",
  ),
  format: "3p", // (review, 1p, 3p, 5p, final)
  numcol: 2,
  // line-numbering: true,
)

= Introduction

Based on work by #cite(<Romero2011TetrisHarmonySearch>, form: "prose").

#include "sections/introduction.typ"

= Conclusion

#include "sections/conclusion.typ"


#bibliography("refs.bib", style: "ieee")
