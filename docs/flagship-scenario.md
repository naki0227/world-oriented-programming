# Flagship Scenario: Visibility Coordination

Last updated: 2026-04-20

## Purpose

The flagship scenario is the first compact example intended to carry the full research argument.
It is not just another benchmark case.
It combines several features that should be understood as one world-oriented model rather than as separate demos.

Files:

- `examples/visibility_coordination_flagship.sk`
- `examples/visibility_coordination_flagship_contradiction.sk`
- `viewer/samples/visibility_coordination_flagship.json`
- `viewer/samples/visibility_coordination_flagship_contradiction.json`
- `benchmarks/library_style/visibility_coordination_flagship.py`

## Scenario

Two coordinating entities, `A` and `D`, begin with ambiguous candidate actions.
Two moving targets, `B` and `C`, pass through a corridor-like visibility structure.
The world contains top and bottom occluding wall regions.

At the initial observation frontier, both `A` and `D` are intentionally unresolved.
At `t=1`, `A` resolves after a visibility-conditioned preference becomes meaningful.
At `t=2`, `D` resolves after a later visibility-conditioned preference becomes meaningful.

The report therefore exhibits:

- candidate-bearing worlds
- hard speed laws
- soft visibility-conditioned preferences
- deferred ambiguity
- staggered convergence
- observation states changing from unresolved to determinate

## Why This Is The Flagship

The scenario exercises the project's central thesis:

> a program can define an observable spatial-temporal world whose entities, laws, events, contradictions, local synchronization, and underdetermined continuations share one executable semantic model.

The clean example now has a separate contradiction-bearing variant, so the paper can keep the main figure focused on convergence while still showing how the same world model reports failed laws.

## Observed Runtime Behavior

Running:

```sh
cargo run -p sekai-cli -- simulate-report examples/visibility_coordination_flagship.sk
```

produces:

- `status: ok`
- `candidate_entities: 2`
- `preference_resolved_entities: 2`
- observation timeline:
  - `t=0`: unresolved with two ambiguous entities
  - `t=1`: unresolved with one ambiguous entity
  - `t=2`: determinate
- selected branches:
  - `A -> pursue_b` at `t=1`
  - `D -> support_c` at `t=2`

This is the first paper-worthy example where world-level observation status changes across frontiers rather than resolving everything at the initial step.

## Contradiction Variant

The contradiction variant keeps the same core scene and adds an observer `S` with a hard law:

```text
visible(S, C)
```

At `t=0`, `S` can see `C`.
After `C` moves, the lower wall blocks the line of sight, so the runtime reports:

- `status: error`
- contradicted interaction constraint: `visible(S, C)`
- blocker: `wall_bottom`
- contradiction time: `t=1.000`

This variant is intentionally separate from the clean flagship figure.
It prevents the flagship from becoming visually overloaded while giving the research argument a compact falsification case: the same executable world language can show both deferred convergence and a hard-law contradiction.

## Viewer Reading

The viewer now loads both reports as first-class samples and can play the clean convergence report smoothly.
That playback is a visual interpolation between semantic frontiers, not an extra source of observations.

This distinction matters:

- `t=0`, `t=1`, and `t=2` are the actual reported observation frontiers
- candidate ambiguity and resolution are evaluated only at those frontiers
- the in-between frames make the motion legible for talks, demos, and screen capture
- denser semantic observation still requires adding more explicit snapshot requests to the `.sk` source
- the contradiction sample stops at the failing frontier because it is a runtime error report, not a completed convergence run

## Library-Style Baseline Reading

The library-style baseline intentionally gives Python a favorable world-building API:

```python
world.sphere(...)
world.law(...)
world.candidate_velocity(...)
world.prefer_if_visible(...)
world.observe(...)
```

This baseline can hide most mechanics.
That is useful because it prevents the paper from relying only on "imperative code is verbose."

The remaining `sekai` claim must therefore focus on:

- laws as language-level source constructs
- contradiction and convergence reports tied to world laws
- stable report fields for activities and candidate resolutions
- viewer continuity from source world to observed frontiers

## Next Required Work

- Add an imperative or event-driven flagship baseline only if it clarifies the narrative rather than bloating the corpus.
- Capture a paper figure or short demo clip showing `unresolved -> unresolved -> determinate`.
- Decide whether the contradiction variant should appear as a paper sidebar, appendix example, or viewer-only artifact.
