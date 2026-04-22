# Phase K. Comparative Evaluation

Phase K asks whether `sekai` is a better representational fit than imperative alternatives
for the kinds of worlds it targets.

This phase is not only about performance. Its core question is whether the same scenario
can be specified with less accidental mechanism and with clearer world-level intent.

## Evaluation Goals

- compare representational complexity rather than benchmark speed alone
- measure how much of a program describes the world versus how much manages execution
- ground later representational-structure claims in a reproducible corpus

## Initial Scenario Corpus

- `examples/bounce.sk`
- `examples/two_body_collision.sk`
- `examples/candidate_velocity.sk`
- `examples/clamped_region.sk`
- `examples/candidate_velocity_deferred.sk`
- `examples/visibility_occluded.sk`
- `examples/visibility_pursuit_occluded.sk`
- `examples/visibility_pursuit_world_occluded.sk`
- `examples/visibility_corridor_world_occluded.sk`
- `examples/visibility_coordination_flagship.sk`

Imperative reference baselines live in:

- `benchmarks/imperative/bounce.py`
- `benchmarks/imperative/two_body_collision.py`
- `benchmarks/imperative/candidate_velocity.py`
- `benchmarks/imperative/clamped_region.py`
- `benchmarks/imperative/candidate_velocity_deferred.py`
- `benchmarks/imperative/visibility_occluded.py`
- `benchmarks/imperative/visibility_pursuit_occluded.py`
- `benchmarks/imperative/visibility_pursuit_world_occluded.py`
- `benchmarks/imperative/visibility_corridor_world_occluded.py`

Event-driven reference baselines live in:

- `benchmarks/event_driven/bounce.py`
- `benchmarks/event_driven/two_body_collision.py`
- `benchmarks/event_driven/candidate_velocity.py`
- `benchmarks/event_driven/clamped_region.py`
- `benchmarks/event_driven/candidate_velocity_deferred.py`
- `benchmarks/event_driven/visibility_occluded.py`
- `benchmarks/event_driven/visibility_pursuit_occluded.py`
- `benchmarks/event_driven/visibility_pursuit_world_occluded.py`
- `benchmarks/event_driven/visibility_corridor_world_occluded.py`

Library-style reference baselines live in:

- `benchmarks/library_style/bounce.py`
- `benchmarks/library_style/two_body_collision.py`
- `benchmarks/library_style/candidate_velocity.py`
- `benchmarks/library_style/clamped_region.py`
- `benchmarks/library_style/candidate_velocity_deferred.py`
- `benchmarks/library_style/visibility_occluded.py`
- `benchmarks/library_style/visibility_pursuit_occluded.py`
- `benchmarks/library_style/visibility_pursuit_world_occluded.py`
- `benchmarks/library_style/visibility_corridor_world_occluded.py`
- `benchmarks/library_style/visibility_coordination_flagship.py`

## Initial Metrics

The first pass uses simple structural metrics. They are intentionally modest, but they make
the later evaluation phase concrete and reproducible.

- logical LOC: non-empty, non-comment lines
- token count: coarse lexical load
- control density: `if` / `for` / `while` load
- state-assignment density: explicit update sites in imperative code
- world-content density: fraction of lines devoted to object, law, action, and observation content
- mechanics density: fraction of lines devoted to update, event-detection, repair, and selection mechanics

The current coding rules are tracked in:

- `docs/evaluation-coding-manual.md`

The first manual flagship comparison is tracked in:

- `docs/flagship-comparison.md`

## Current Script

`scripts/spec_metrics.py` computes the current scaffold metrics for the corpus above.

Example:

```bash
python3 scripts/spec_metrics.py
```

The first recorded output is summarized in:

- `docs/phase-k-baseline-metrics.md`
- `docs/phase-k-narrative-comparisons.md`

## Current Hypotheses

- `sekai` should require fewer control-flow sites for the same world behavior
- `sekai` should show higher world-content density than imperative baselines
- imperative baselines should show higher mechanics density than `sekai` specifications
- imperative baselines should spend more of their specification budget on explicit state progression

## Next Steps

- extend the corpus as Phase J adds richer geometry
- use visibility as the first geometry-forward comparison scenario
- extend that visibility slice into a behavior-level comparison where geometry changes candidate selection
- extend that same visibility line into a branching world comparison where geometry changes which continuation family is taken
- add scenario-specific narrative comparisons for each pair
- extend the manual flagship comparison into a compact paper table, because line counts alone cannot capture source identity, report structure, or viewer integration
- keep cognitive-load claims as hypotheses unless a user study is actually run
- decide whether a lightweight user study is feasible
