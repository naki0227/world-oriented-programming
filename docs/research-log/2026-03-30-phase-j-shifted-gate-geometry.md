# 2026-03-30 — Phase J shifted-gate geometry

This pass extends the connected-space geometry family from delayed openings to moving apertures.

## What changed

- Added `through_gate_shift_after(...)` as a gate law that can translate a named aperture after a chosen frontier.
- Added `surface_gate_shifted_open.sk` and `surface_gate_shifted_closed.sk` as the first moving-aperture room-transition examples.
- Added the imperative baseline `surface_gate_shifted_closed.py`.
- Added viewer samples for both shifted-gate worlds.
- Extended Phase K documents so moving apertures are part of the reproducible geometry corpus.

## Why it matters

The existing gate family could already express closed, open, delayed, and routed room transitions.
The shifted-gate slice adds a different temporal geometry behavior:
admissibility can now change because the aperture itself moves into or out of alignment.

This is useful because it connects:

- bounded-space geometry
- deferred convergence
- observation frontiers
- connected-space law

without reducing the world back to explicit update logic.
