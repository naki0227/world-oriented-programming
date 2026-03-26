# 2026-03-26 — Phase J Visibility Pursuit Integration With Viewer And K

## Summary

Extended the new visibility-pursuit slice into the viewer and Phase K corpus.

## What Changed

- added an imperative baseline for `visibility_pursuit_occluded`
- extended `scripts/spec_metrics.py` with the new geometry-behavior scenario
- updated Phase K comparison docs and narrative notes
- added viewer support and sample slots for:
  - `visibility_pursuit_clear`
  - `visibility_pursuit_occluded`
- updated the paper's evaluation language to mention the pursuit pair

## Why It Matters

This is the first Phase J example where geometry affects convergence rather than only contradiction.
It makes the visibility line stronger as a research pillar because the geometry now changes world evolution.
