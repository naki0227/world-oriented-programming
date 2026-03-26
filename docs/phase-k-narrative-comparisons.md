# Phase K Narrative Comparisons

This note complements the structural metrics with short scenario-level comparisons.
The goal is to make explicit what the paired baselines suggest about representational fit.

## Bounce

`sekai` expresses the scene mostly as object declarations plus two laws:
reflection on contact and a speed bound.
The imperative baseline must instead encode a stepping procedure, detect floor contact,
repair penetration, and repeatedly clamp speed during progression.

Narrative reading:

- `sekai` spends most of its budget on world description.
- the imperative baseline spends more of its budget on explicit progression mechanics.

## Two-Body Collision

`sekai` states the world with two spheres and one interaction law.
The imperative baseline must still implement stepping, pairwise collision detection,
and velocity exchange as explicit procedural structure.

Narrative reading:

- `sekai` makes the interaction law central.
- the imperative baseline makes event mechanics central.

## Candidate Velocity

`sekai` can say that an entity has scored candidate velocities and one hard speed law.
The imperative baseline must explicitly enumerate candidates, filter them, sort them,
and then choose one.

Narrative reading:

- `sekai` treats admissibility as part of the world.
- the imperative baseline treats admissibility as a control-flow problem.

## Clamped Region

This pair is useful because it exercises repair rather than only selection.
`sekai` states a forbidden region and a clamp policy directly as a world law.
The imperative baseline must encode region membership tests, choose a nearest boundary,
and manually zero the repaired velocity component.

Narrative reading:

- `sekai` foregrounds the law and the repair policy.
- the imperative baseline foregrounds the repair algorithm.

## Candidate Velocity Deferred

This pair is useful because it exercises underdetermination.
`sekai` can state equally scored candidates together with `defer_on_ambiguous_top(A)`.
The imperative baseline must compute the tie explicitly and preserve unresolved status
through procedural bookkeeping.

Narrative reading:

- `sekai` can treat non-resolution as a first-class world state.
- the imperative baseline must simulate that non-resolution with extra machinery.

## Visibility Occluded

This pair is useful because it begins the move from collision-only geometry toward
view-dependent world conditions.
`sekai` can state `visible(A, B)` as a world law and let contradiction emerge when an
occluding region blocks the line of sight.
The imperative baseline must instead encode the segment-versus-box test directly and route
the failure through explicit procedural control flow.

Narrative reading:

- `sekai` foregrounds visibility as a world condition.
- the imperative baseline foregrounds the occlusion algorithm and failure plumbing.

## Current Evaluation Story

Taken together, the current corpus supports a modest but already meaningful claim:

- the `sekai` programs spend more of their surface form on world-level content
- the imperative baselines spend more of their surface form on execution management

This is not yet a user study or a definitive complexity argument, but it is enough to
support the project's current evaluation stance: the paradigm is already showing a
different representational center of gravity.
