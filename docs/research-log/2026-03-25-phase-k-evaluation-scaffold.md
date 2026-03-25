# 2026-03-25 Phase K Evaluation Scaffold

Started Phase K by fixing a small comparison corpus and a reproducible metric script.

Added:

- imperative reference baselines for:
  - `bounce`
  - `two_body_collision`
  - `candidate_velocity`
- `scripts/spec_metrics.py`
- Phase J and Phase K working notes

The current goal is not to claim final superiority, but to make the evaluation phase executable.
The project now has a concrete place to measure logical LOC, token load, control density,
state-assignment density, and declarative density across a small paired corpus.
