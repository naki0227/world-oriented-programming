# 2026-03-29 - Phase J surface-gate geometry

Added a first gate/door geometry slice to extend the multi-surface family from bounded rooms
to connected spaces.

## What changed

- added `through_gate(A, wall, door[, policy])` as a boundary law
- treated the named `door` region as a gate aperture rather than a full 3D occupancy box
- added `surface_gate_clear` and `surface_gate_clamped` examples
- added an imperative baseline for the clamped gate scenario
- added viewer samples and sample-selector entries
- updated Phase J / Phase K docs and the paper

## Why it matters

This moves the geometry story past closed bounded spaces.
The prototype can now express that one wall remains active as a boundary except at a named
opening, which is closer to room-to-room transition than to simple containment.

## Current status

- `cargo test`: passed
- `python3 scripts/spec_metrics.py`: passed
- `node --check viewer/app.js`: passed
- `tectonic paper/main.tex`: passed
