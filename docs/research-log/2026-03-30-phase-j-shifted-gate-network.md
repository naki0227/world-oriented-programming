# 2026-03-30 — Phase J shifted-gate network

This pass extends the moving-aperture geometry line from one doorway to a small connected-space network.

## What changed

- Added `surface_gate_network_shifted.sk` as the first room-network world where several entities resolve through different translated apertures.
- Added the imperative baseline `surface_gate_network_shifted.py`.
- Added a runtime test for staggered re-convergence through shifted gates.
- Added a viewer sample for the shifted-gate network.
- Extended the Phase J and Phase K documents so moving apertures are now represented as both single-entity and multi-entity connected-space geometry.

## Why it matters

The earlier shifted-gate slice showed that admissibility can change because a doorway itself moves.
The network slice shows that the same idea scales to connected-space worlds with more than one entity and more than one frontier.

This matters because it ties together:

- bounded rooms
- connected-space routing
- later aperture motion
- staggered convergence

inside one world description, without pushing control flow back into an imperative update loop.
