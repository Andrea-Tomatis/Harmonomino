#import "flowchart-style.typ": *

#flowchart({
  // ── Main column ──
  node((-0.8, 1), [*Start*], extrude: (0, 3), name: <start>)
  node((0, 1), align(center)[Initialize &\ evaluate HM], fill: process-fill)
  node((0, 2), align(center)[For each\ weight $w_i$], fill: process-fill)
  node((0, 3), align(center)[$"rand" < r_"accept"$?], shape: diamond, fill: decision-fill)

  // ── Branches from r_accept ──
  node((-1, 4), align(center)[Memory\ consideration], fill: process-fill)
  node((1, 4), align(center)[Random\ selection], fill: process-fill)
  node((-1, 5), align(center)[$"rand" < r_"pa"$?], shape: diamond, fill: decision-fill)
  node((-1, 6), align(center)[Pitch\ adjustment], fill: process-fill)

  // ── Merge & evaluate ──
  node((0, 6), align(center)[Evaluate new\ harmony], fill: process-fill)

  // ── Selection & termination ──
  node((0, 7), align(center)[Improves worst?], shape: diamond, fill: decision-fill)
  node((1, 7), align(center)[Replace\ worst], fill: process-fill)
  node((0, 8), align(center)[Terminate?], shape: diamond, fill: decision-fill)
  node((0, 9.3), [*Return best*], extrude: (0, 3), name: <end>)

  // ── Main flow ──
  edge(<start>, (0, 1), "-|>")
  edge((0, 1), (0, 2), "-|>")
  edge((0, 2), (0, 3), "-|>")
  edge((0, 6), (0, 7), "-|>")

  // ── Accept branching ──
  edge((0, 3), (-1, 4), "-|>", [Yes], label-side: left)
  edge((0, 3), (1, 4), "-|>", [No], label-side: right)

  // ── Memory → r_pa ──
  edge((-1, 4), (-1, 5), "-|>")

  // ── r_pa branching ──
  edge((-1, 5), (-1, 6), "-|>", [Yes], label-side: left, label-pos: 0.4)

  // ── Merge at evaluate ──
  edge((-1, 6), (0, 6), "-|>")
  edge((-1, 5), (0, 6), "-|>", [No], label-side: right, bend: 20deg)
  edge((1, 4), "dd,l", "-|>")

  // ── Fitness ──
  edge((0, 7), (1, 7), "-|>", [Yes], label-side: right, label-pos: 0.2)
  edge((0, 7), (0, 8), "-|>", [No], label-side: left)
  edge((1, 7), "d,l", "-|>")

  // ── Termination ──
  edge((0, 8), <end>, "-|>", [Yes], label-side: left, label-pos: 0.3)

  // ── Loopback ──
  edge((0, 8), "ll,uuuuuu,rr", "-|>", [No], label-pos: 0.1, label-side: right)
})
