# 2026-03-25 Phase I Candidate Comparison Viewer

## Summary

Added a dedicated Phase I comparison panel to the viewer.

## What Changed

The viewer now exposes `Candidate Comparison` alongside `Candidate Resolution`.
It provides quick switching between two complementary Phase I samples:

- `candidate_velocity.json` for fallback selection after hard-law rejection
- `candidate_velocity_clamped.json` for repaired selection after hard-law enforcement

## Files Updated

- `viewer/index.html`
- `viewer/app.js`
- `docs/viewer.md`

## Why This Matters

Phase I is easier to understand when its two minimal convergence patterns can be compared directly:

- reject the highest-scoring candidate and fall back
- select the highest-scoring candidate and repair it into admissibility

This makes the first underdetermined-world prototype easier to demo, debug, and eventually discuss in the paper.
