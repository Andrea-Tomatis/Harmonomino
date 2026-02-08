#import "flowchart-style.typ": *

#flowchart({
  // ── Main flow (y=0) ──
  node((1, -1), [*Start*], extrude: (0, 3), name: <start>)
  node((1, 0), align(center)[Initialize\ $mu = 0$, $sigma = sigma_0$], fill: process-fill)
  node((2, 0), align(center)[Build $cal(N)(mu_i, sigma_i)$\ distributions], fill: process-fill)
  node((3, 0), align(center)[Sample $N$\ candidates], fill: process-fill)
  node((4, 0), align(center)[Evaluate, sort\ & update best], fill: process-fill)
  node((5, 0), align(center)[Select top $N_"elite"$,\ update $mu$, $sigma$], fill: process-fill)
  node((6, 0), align(center)[Terminate?], shape: diamond, fill: decision-fill, name: <term>)
  node((6, -1), [*Return best*], extrude: (0, 3), name: <end>)

  // ── Edges ──
  edge(<start>, (1, 0), "-|>")
  edge((1, 0), (2, 0), "-|>")
  edge((2, 0), (3, 0), "-|>")
  edge((3, 0), (4, 0), "-|>")
  edge((4, 0), (5, 0), "-|>")
  edge((5, 0), <term>, "-|>")
  edge(<term>, <end>, "-|>", [Yes], label-pos: 0.3, label-side: left)

  // ── Loopback ──
  edge(<term>, "d,llll,u", "-|>", [No], label-pos: 0.3, label-side: left)
})
