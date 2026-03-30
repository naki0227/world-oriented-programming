# 2026-03-30 - Phase J gate-conditioned branching

This pass took the new gate family one step beyond time-varying repair and turned it into a
geometry-conditioned branching world.

What changed:

- added `prefer_candidate_if_gate_open(...)`
- added `prefer_candidate_if_gate_closed(...)`
- added open/closed branching examples over the same wall and door geometry
- propagated the new pair to viewer samples and Phase K evaluation scaffolding
- extended static analysis hints so gate-conditioned continuation rules show up before simulation

Why it matters:

- gates are no longer only passive admissibility boundaries
- connected-space geometry can now participate directly in convergence
- the gate family now mirrors the earlier visibility family: law, time-varying law, and branching world

Current reading:

The prototype can now state that one doorway remains a world condition on branching:
if the gate is open, one continuation family is preferred; if it is closed, another is preferred.
