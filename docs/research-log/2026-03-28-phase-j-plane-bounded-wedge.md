# 2026-03-28 — Phase J Plane-Bounded Wedge

I extended the new multi-surface geometry family with a first plane-bounded volume law:

- `between_planes(A, floor, ceiling, clamp)`

This matters because Phase J now has a geometry slice that is not only:

- visibility through axis-aligned regions, or
- collision against several declared planes,

but also:

- admissibility inside a non-axis-aligned bounded space defined by planes.

The first example is a slanted wedge, where a sphere would leave the admissible channel
unless the `between_planes(...)` law clamps it back into the allowed space.

This is still a very small slice, but it is the first clear step from
axis-aligned `region` exclusion toward richer bounded geometry.
