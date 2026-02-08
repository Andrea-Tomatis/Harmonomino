#import "@preview/elsearticle:2.0.0": *

#import "@preview/zero:0.6.1": *
#import "@preview/booktabs:0.0.4": *
#import "@preview/equate:0.3.2": equate

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
  format: "5p",
  numcol: 2,
)

#set text(font: "New Computer Modern")
#show: booktabs-default-table-style
#show: equate.with(breakable: true, sub-numbering: true, number-mode: "label")
#set math.equation(numbering: "(1.a)")
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
#set figure(placement: auto)
#show selector(<nonumber>): set heading(numbering: none)

// NOTE: This shouldn't be necessary, maybe the template does something weird?
#set ref(supplement: it => {
  if it.func() == figure and it.kind == table {
    [Tab.]
  } else if it.func() == figure and it.kind == image {
    [Fig.]
  } else if it.func() == math.equation {
    [Eq.]
  } else {
    it.supplement
  }
})


// Alphanumeric citation labels
#set cite(style: "alphanumeric")

// Prose citations: "Surname et al. [LABEL]" with author black, label blue
#show cite.where(form: "prose"): it => {
  // Match author text before "[" and shorten to surname (+ et al. if multi-author)
  show regex(".+\\["): m => {
    let s = m.text
    let authors = s.trim().trim("[").trim()
    if "," in authors {
      let first = authors.split(",").first().trim()
      let surname = first.split(" ").last()
      [#surname et al. \[]
    } else if " and " in authors {
      let parts = authors.split(" and ")
      let s1 = parts.first().trim().split(" ").last()
      let s2 = parts.last().trim().split(" ").last()
      [#s1 and #s2 \[]
    } else {
      let surname = authors.split(" ").last()
      [#surname \[]
    }
  }
  set text(fill: rgb("#00F"))
  it
}

#include "sections/introduction.typ"
#include "sections/methodology.typ"
#include "sections/results.typ"
#include "sections/conclusion_and_discussion.typ"

#bibliography("refs.bib", style: "alpha-ieee.csl")

#v(2em)
#line(length: 100%, stroke: 0.5pt + gray)
#text(size: 0.9em, style: "italic")[
  = Declaration of Generative AI in Scientific Writing <nonumber>
  During the preparation of this work, the authors used generative AI tools in order to improve readability and ensure a correct use of the English language. After using these tools, the authors reviewed and edited the content as needed and take full responsibility for the content of the report.
]
