# 2026-03-26 — Phase J Visibility Pursuit Slice

## Summary

Added the next Phase J geometry-behavior slice:

- `prefer_candidate_if_visible(A, pursue, B)`

## What Changed

- extended action directives with a visibility-conditioned preference
- connected visibility to Phase I candidate selection
- added two executable examples:
  - `examples/visibility_pursuit_clear.sk`
  - `examples/visibility_pursuit_occluded.sk`
- updated Phase J docs, roadmap, and paper notes

## Why It Matters

The first visibility slice treated line of sight as a world-law contradiction.
This new slice makes visibility influence how the world evolves, which is a stronger geometry claim.
It is the first step from “visibility as condition” to “visibility-constrained worlds.”
