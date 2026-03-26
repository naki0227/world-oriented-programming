# Phase J. Richer Geometry And Space

Phase J expands `sekai` beyond moving spheres, a single plane, and axis-aligned region exclusion.

The goal is not just to add more primitives.
It is to identify which geometry features most clearly benefit from a world-oriented model
instead of imperative update logic.

## First Executable Slice

The first Phase J slice is a visibility law:

- `visible(A, B)`

In the current prototype, the law is evaluated against a declared axis-aligned occluding `region`.
If the line segment between `A` and `B` intersects that region at an observation frontier,
the world reports contradiction.

This slice is intentionally small, but strategically important:

- it moves the project beyond collision-only geometry
- it gives Phase K a more compelling future comparison target
- it makes visibility a world-level condition instead of scattered update logic

## Next Geometry Targets

- oriented or non-axis-aligned occluders
- path primitives
- visibility-sensitive action scenarios
- stronger 3D editing support in the viewer
