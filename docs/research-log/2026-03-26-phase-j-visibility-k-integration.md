# 2026-03-26 — Phase J Visibility Integration With Viewer And K

## Summary

Connected the first Phase J visibility law to the viewer and to the Phase K comparative-evaluation scaffold.

## What Changed

- added an imperative visibility baseline:
  - `benchmarks/imperative/visibility_occluded.py`
- extended `scripts/spec_metrics.py` with the `visibility_occluded` scenario pair
- updated Phase K docs with the visibility scenario, metrics, and narrative comparison
- added viewer samples and a `Visibility Comparison` panel for:
  - `visibility_clear`
  - `visibility_occluded`
- reflected the new visibility pair in the paper's evaluation discussion

## Why It Matters

This is the first geometry-forward evaluation pair in the corpus.
It starts moving the project away from collision-only examples and toward a stronger
claim that world-level spatial relations such as line-of-sight can be more natural in
`sekai` than in imperative update-oriented baselines.

## Verification

- `cargo run -p sekai-cli -- simulate-report examples/visibility_clear.sk > viewer/samples/visibility_clear.json`
- `cargo run -p sekai-cli -- simulate-report examples/visibility_occluded.sk > viewer/samples/visibility_occluded.json`
- `python3 scripts/spec_metrics.py`
- `cargo test`
- `tectonic paper/main.tex`
