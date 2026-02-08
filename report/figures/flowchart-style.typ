#import "@preview/fletcher:0.5.8" as fletcher: diagram, edge, node
#import fletcher.shapes: diamond

#let process-fill = blue.lighten(85%)
#let decision-fill = yellow.lighten(82%)

#let flowchart(body) = {
  set text(7pt)
  diagram(
    node-stroke: 0.7pt,
    edge-stroke: 0.7pt,
    node-corner-radius: 3pt,
    edge-corner-radius: 6pt,
    mark-scale: 80%,
    spacing: (8mm, 4mm),
    body,
  )
}
