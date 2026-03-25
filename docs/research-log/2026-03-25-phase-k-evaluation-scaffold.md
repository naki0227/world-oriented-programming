# 2026-03-25 Phase K Evaluation Scaffold

Started Phase K by fixing a small comparison corpus and a reproducible metric script.

Added:

- imperative reference baselines for:
  - `bounce`
  - `two_body_collision`
  - `candidate_velocity`
  - `clamped_region`
  - `candidate_velocity_deferred`
- `scripts/spec_metrics.py`
- Phase J and Phase K working notes

The current goal is not to claim final superiority, but to make the evaluation phase executable.
The project now has a concrete place to measure logical LOC, token load, control density,
state-assignment density, and declarative density across a small paired corpus.

The corpus was later widened to include:

- a repair-oriented Phase F scenario (`clamped_region`)
- a deferred Phase I scenario (`candidate_velocity_deferred`)
