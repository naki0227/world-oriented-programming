# Phase J. Richer Geometry And Space

Phase J expands `sekai` beyond moving spheres, a single plane, and axis-aligned region exclusion.

The goal is not only to add more geometry primitives, but to preserve the language's
world-oriented semantics while the spatial vocabulary becomes richer.

## Research Questions

- Which geometric primitives should become first-class before the model becomes too diffuse?
- How should richer geometry interact with world laws, event ordering, and local synchronization?
- Which new spatial relations matter most for later evaluation in Phase K?

## First Executable Slice

The first Phase J slice is a visibility law:

- `visible(A, B)`

In the current prototype, the law is evaluated against declared axis-aligned occluding `region`s.
If the line segment between `A` and `B` intersects one or more of those regions at an observation frontier,
the world reports contradiction.

This slice is intentionally small, but strategically important:

- it moves the project beyond collision-only geometry
- it gives Phase K a more compelling future comparison target
- it makes visibility a world-level condition instead of scattered update logic

The next incremental step is multi-occluder visibility.
The runtime can now keep line of sight admissible across several declared blocking regions and report the blocking set when visibility fails.
This now supports a first corridor-style world, where side walls preserve a narrow visibility channel and an inserted blocker closes it.
It also supports a first time-varying corridor slice, where a moving target can later enter or leave that channel and thereby resolve a deferred world.

## Next Geometry-Behavior Slice

The next Phase J slice connects visibility to world evolution:

- `prefer_candidate_if_visible(A, pursue, B)`
- `prefer_candidate_if_occluded(A, search, B)`

This matters because:

- visibility stops being only a contradiction trigger
- line of sight begins to influence candidate-world convergence
- Phase J starts to connect directly to Phase I underdetermined worlds
- a world can switch between pursuit-like and search-like continuations without exposing update logic
- a corridor-shaped world can now branch differently when its visibility channel is preserved or blocked
- a deferred corridor world can now resolve differently when line of sight changes at a later observation frontier

## Near-Term Targets

- richer surfaces beyond a single plane
- non-axis-aligned regions and bounded volumes
- path and trajectory primitives
- visibility and line-of-sight predicates
- stronger 3D editing support in the viewer

## Evaluation-Relevant Geometry Tasks

These are the first geometry scenarios that should be expressible before Phase J is considered mature:

- corridor navigation with forbidden volumes
- view-dependent interaction through line-of-sight
- path following with spatial constraints
- multiple surfaces with contact rules

## Design Principles

- add primitives only when they fit the current law/event model
- prefer geometric relations that can appear in both DSL and viewer
- preserve deterministic observation semantics from Phase G
- make new geometry measurable in Phase K through representative tasks

## Expected Outputs

- a small but meaningful richer-geometry corpus
- updated spatial/event semantics where needed
- viewer affordances for at least one new geometry family
- evaluation-ready scenarios that compare `sekai` against imperative baselines
