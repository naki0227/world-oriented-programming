# Phase K. Comparative Evaluation

Phase K asks whether `sekai` is a better representational fit than imperative alternatives
for the kinds of worlds it targets.

This phase is not only about performance. Its core question is whether the same scenario
can be specified with less accidental mechanism and with clearer world-level intent.

## Evaluation Goals

- compare representational complexity rather than benchmark speed alone
- measure how much of a program describes the world versus how much manages execution
- ground later claims about cognitive load and declarative density in a reproducible corpus

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
- `examples/surface_channel.sk`
- `examples/surface_room_clamped.sk`
- `examples/surface_room_reflective.sk`
- `examples/surface_gate_clamped.sk`
- `examples/surface_gate_deferred_closed.sk`
- `examples/surface_gate_branch_closed.sk`
- `examples/surface_gate_route_right.sk`
- `examples/surface_gate_network_staggered.sk`
- `examples/surface_gate_shifted_closed.sk`
- `examples/surface_gate_network_shifted.sk`
- `examples/path_tube_clamped.sk`

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
- `benchmarks/imperative/surface_channel.py`
- `benchmarks/imperative/surface_room_clamped.py`
- `benchmarks/imperative/surface_room_reflective.py`
- `benchmarks/imperative/surface_gate_clamped.py`
- `benchmarks/imperative/surface_gate_deferred_closed.py`
- `benchmarks/imperative/surface_gate_branch_closed.py`
- `benchmarks/imperative/surface_gate_route_right.py`
- `benchmarks/imperative/surface_gate_network_staggered.py`
- `benchmarks/imperative/surface_gate_shifted_closed.py`
- `benchmarks/imperative/surface_gate_network_shifted.py`
- `benchmarks/imperative/path_tube_clamped.py`

## Initial Metrics

The first pass uses simple structural metrics. They are intentionally modest, but they make
the later evaluation phase concrete and reproducible.

- logical LOC: non-empty, non-comment lines
- token count: coarse lexical load
- control density: `if` / `for` / `while` load
- state-assignment density: explicit update sites in imperative code
- declarative density: fraction of lines devoted to object, law, action, and observation declarations

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
- `sekai` should show higher declarative density than imperative baselines
- imperative baselines should spend more of their specification budget on explicit state progression

## Next Steps

- extend the corpus as Phase J adds richer geometry
- use visibility as the first geometry-forward comparison scenario
- extend that visibility slice into a behavior-level comparison where geometry changes candidate selection
- extend that same visibility line into a branching world comparison where geometry changes which continuation family is taken
- add a non-visibility geometry pair so Phase J is not represented by occlusion alone
- add scenario-specific narrative comparisons for each pair
- add a gate / doorway pair so bounded spaces can be connected rather than only closed
- add a time-varying gate pair so geometry can open or close a room transition across frontiers
- extend that gate pair into a branching world where opening state changes which continuation the world prefers
- extend that connected-space line into multi-gate routing where geometry chooses which exit becomes active
- extend that connected-space line into staggered room networks where several entities resolve through different gates at different frontiers
- extend that connected-space line into shifted-gate worlds where the aperture itself can later move into or out of alignment
- extend that moving-aperture line into shifted-gate networks where several entities resolve through different translated apertures
- add a path / trajectory pair so geometry is not represented only by visibility and bounded surfaces
- connect these structural metrics to cognitive-load hypotheses
- decide whether a lightweight user study is feasible
