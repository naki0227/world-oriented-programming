# 2026-03-25 Phase I Observational Equivalence Slice

## Summary

Added a minimal observational-equivalence signal for tied Phase I candidates.

## What Changed

The runtime now compares admissible top-score candidates against the selected continuation and records:

- `equivalent_top_labels`
- `observationally_equivalent_tie`

This remains a very small slice of the larger research problem, but it gives the prototype its first executable hook for the idea that different candidates may remain distinct internally while collapsing to the same observed result.

## Files Updated

- `orbis/src/world.rs`
- `viewer/app.js`
- `viewer/index.html`
- `docs/output-format.md`
- `docs/viewer.md`
- `examples/candidate_velocity_equivalent_tie.sk`

## Why This Matters

Phase I already had candidate inventory, resolution summaries, repaired selection, and tie handling.
This pass adds the first concrete step toward the roadmap's observational-stability goal.
