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

Imperative reference baselines live in:

- `benchmarks/imperative/bounce.py`
- `benchmarks/imperative/two_body_collision.py`
- `benchmarks/imperative/candidate_velocity.py`
- `benchmarks/imperative/clamped_region.py`
- `benchmarks/imperative/candidate_velocity_deferred.py`

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

## Current Hypotheses

- `sekai` should require fewer control-flow sites for the same world behavior
- `sekai` should show higher declarative density than imperative baselines
- imperative baselines should spend more of their specification budget on explicit state progression

## Next Steps

- extend the corpus as Phase J adds richer geometry
- add scenario-specific narrative comparisons for each pair
- connect these structural metrics to cognitive-load hypotheses
- decide whether a lightweight user study is feasible
