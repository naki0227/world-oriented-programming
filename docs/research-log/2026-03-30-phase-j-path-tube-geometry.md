# 2026-03-30 - Phase J path-tube geometry

Today I added a first path / trajectory geometry family to Phase J.

## What changed

- added `path` as a declared world object with `start(...)`, `end(...)`, and `width(...)`
- added `inside_tube(A, lane)` as a new geometry law
- supported `reject` and `clamp` policies for path tubes
- exposed `path_inventory` in simulation and analysis reports
- rendered declared paths in the viewer and added first path-tube samples
- added an imperative baseline for `path_tube_clamped`

## Why it matters

This is the first geometry family in the prototype that is neither:

- visibility / occlusion
- surface / bounded-space contact

Instead, it makes trajectory admissibility explicit at the world-description level.
That means a motion corridor can now be stated as geometry plus law, rather than as procedural closest-point projection logic.

## Immediate result

Phase J now has three visibly distinct geometry families:

- visibility and occlusion
- multi-surface and connected-space geometry
- path / trajectory corridors

That widens the geometry story in both the paper and the Phase K comparative scaffold.
