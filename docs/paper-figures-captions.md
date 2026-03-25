# Paper Figure Captions Draft

## Figure 1

Declarative bouncing-sphere scenario in `sekai`. A single sphere evolves without any user-authored update loop. The panels show snapshots at `t=0.000`, `t=1.000`, `t=3.000`, and `t=4.000`; the downward trajectory is reflected at the floor, after which the vertical velocity reverses while the horizontal component is preserved.

## Figure 2

Local synchronization in a two-body collision scenario. Two spheres advance independently until contact, at which point an explicit elastic-collision constraint is applied. The panels at `t=0.000`, `t=1.000`, and `t=3.000` show that interaction is localized to the collision event, after which the spheres separate with exchanged velocities.

## Legacy Optional Constraint Figure

Forbidden-region constraint example. The world evolves until a sphere reaches a prohibited spatial region; this contradiction is reported as a violation of world consistency rather than handled through imperative exception code.

## Figure 3

Interactive viewer overview for executable world inspection. The `two_body_collision` example is shown in the browser-based `sekai` viewer, where the same world report can be inspected spatially, temporally, and structurally without a viewer-specific authoring pipeline.

## Figure 4

Diagram-to-logic candidate generation in the draft editor. Two placed spheres and a forbidden region give rise to proposed world laws such as floor reflection, forbidden-region exclusion, and pairwise collision, illustrating that diagrams can generate logical structure rather than merely display it.

## Figure 5

Round-trip execution from diagram editing to world simulation. A draft scene is executed through `Run Draft`, after which the returned world report is immediately displayed in the same interface. This demonstrates the prototype cycle of diagram construction, candidate adoption, execution, and inspection.

## Figure 6

Constraint violation as a world contradiction in the viewer. After enabling `not inside(S1, forbidden_zone)`, execution fails with an explicit contradiction report. The resulting screen shows that invalid evolution is represented as a law violation in the world model rather than as imperative exception-handling code.
