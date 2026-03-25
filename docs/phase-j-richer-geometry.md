# Phase J. Richer Geometry And Space

Phase J expands `sekai` beyond moving spheres, planes, and axis-aligned regions.

The goal is not only to add more geometry primitives, but to preserve the language's
world-oriented semantics while the spatial vocabulary becomes richer.

## Research Questions

- Which geometric primitives should become first-class before the model becomes too diffuse?
- How should richer geometry interact with world laws, event ordering, and local synchronization?
- Which new spatial relations matter most for later evaluation in Phase K?

## Near-Term Targets

- richer surfaces beyond a single plane
- non-axis-aligned regions and bounded volumes
- paths and trajectory primitives
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
