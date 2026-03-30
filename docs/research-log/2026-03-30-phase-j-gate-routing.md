# 2026-03-30: Phase J gate routing

We extended the connected-space geometry family from single-gate branching to multi-gate routing.

## What changed

- Added gate-specific action directives:
  - `prefer_candidate_if_gate_open(A, enter_left, left_door)`
  - `prefer_candidate_if_gate_open(A, enter_right, right_door)`
- Added route examples:
  - `surface_gate_route_left.sk`
  - `surface_gate_route_right.sk`
- Added an imperative baseline for the routed case:
  - `surface_gate_route_right.py`
- Added viewer samples for both route directions.

## Why it matters

This turns connected-space geometry into a routing medium rather than a single boundary exception. The world can now keep multiple exits explicit and let gate state choose which continuation becomes admissible at a later frontier.
