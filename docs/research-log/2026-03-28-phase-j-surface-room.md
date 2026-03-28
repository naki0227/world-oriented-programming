# 2026-03-28 Phase J Surface Room

This pass extends the new multi-surface geometry family from contact channels and
two-plane wedges to a first bounded surface room.

The new `inside_planes(...)` law lets a sphere stay inside an admissible pocket defined
by several declared planes at once. This matters because it moves the runtime one step
closer to non-axis-aligned bounded geometry without introducing a separate region type or
a dedicated solver.

The immediate slice is intentionally modest:

- one sphere
- several planes
- `reject` or `clamp`
- one repaired bounded room example

Even so, it already demonstrates that Phase J can express bounded geometry as an explicit
world law rather than as imperative signed-distance repair logic.
