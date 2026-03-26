# 2026-03-26 Phase J Visibility Pursuit World K Integration

Added the branching visibility-pursuit world to the Phase K corpus.

## What changed

- added imperative baseline:
  - `benchmarks/imperative/visibility_pursuit_world_occluded.py`
- extended `scripts/spec_metrics.py` with:
  - `visibility_pursuit_world_occluded`
- updated Phase K documents:
  - `docs/phase-k-comparative-evaluation.md`
  - `docs/phase-k-baseline-metrics.md`
  - `docs/phase-k-narrative-comparisons.md`
- updated `paper/main.tex` to mention the branching visibility world in the compact baseline discussion

## Why it matters

The earlier visibility pair showed contradiction.
The first pursuit pair showed geometry-conditioned preference.
This new corpus entry shows a stronger claim: geometry can switch the same world between
different continuation families, which makes visibility look more like a genuine expressive
pillar and less like a small auxiliary predicate.
