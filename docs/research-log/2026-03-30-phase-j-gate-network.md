# 2026-03-30: Phase J gate network

We extended connected-space geometry from routed exits to a small staggered room network.

## What changed

- Added a multi-entity example:
  - `surface_gate_network_staggered.sk`
- Added an imperative baseline:
  - `surface_gate_network_staggered.py`
- Added a viewer sample:
  - `surface_gate_network_staggered.json`
- Added a runtime test that checks staggered resolution across two entities.

## Why it matters

This makes the gate family read less like a single doorway feature and more like a connected-space model. Geometry can now coordinate several deferred entities and resolve them through different exits at different frontiers.
