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
It now also supports a first multi-target visibility handoff, where the same deferred world can resolve toward one of several pursuit continuations depending on which target later becomes visible.
It now also supports a first multi-agent visibility coordination slice, where one visibility change can resolve more than one candidate-bearing entity at the same frontier.
It now also supports a first visibility network slice, where the same geometry assigns several agents to target-specific roles rather than only flipping one local branch.
It now also supports a first staggered visibility network slice, where different agents resolve at different observation frontiers as the visibility graph changes over time.
Alongside that visibility pillar, the runtime now also supports a first richer-surface slice, where one world can declare several planes and bind collision laws to each of them independently.
It now also supports a first plane-bounded wedge slice, where a sphere is kept inside a slanted admissible channel by a law over two declared planes rather than by an axis-aligned region.
It now also supports a first bounded surface room slice, where several declared planes jointly define an admissible pocket and a sphere can be repaired back inside that non-axis-aligned space.
It now also supports a first reflective surface room slice, where one sphere can bounce between several declared planar boundaries rather than between only a floor and a ceiling.
It now also supports a first gated-surface slice, where one declared plane becomes traversable only through a named door aperture rather than as a uniform boundary.
It now also supports a first time-varying gate slice, where that same doorway can open only after a named observation frontier and thereby change whether a deferred continuation later passes through the boundary or is repaired back to the allowed side.
It now also supports a first gate-conditioned branching slice, where one deferred world can later prefer `enter` when the gate is open or `wait` when the gate remains closed.

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
- a multi-target world can now hand off convergence between several pursuit continuations as visibility changes
- a shared visibility change can now coordinate several candidate-bearing entities at once
- a visibility network can now assign target-specific roles across several agents
- a visibility network can now reconfigure role assignment across observation frontiers
- a first multi-surface world can now evolve inside a declared channel rather than against a single floor
- a first plane-bounded wedge can now repair motion inside a non-axis-aligned admissible space
- a first bounded surface room can now keep a sphere inside a small polyhedral pocket built from several planes
- a first reflective surface room can now express contact-rich bounded motion over several declared surfaces
- a first gate law can now connect bounded spaces through one explicit geometric aperture
- a first time-varying gate law can now make that aperture open or remain closed across observation frontiers
- a first gate-conditioned branch can now let room connectivity choose between several continuation families

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
- connected rooms and gates

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
