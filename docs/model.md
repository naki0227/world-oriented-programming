# Core Model v0.1

## Purpose

This document defines the smallest formal model needed to begin implementing `orbis`, the runtime core for `sekai`.

The goal is not full realism.
The goal is to define a world model that supports:

- continuous or quasi-continuous time
- object-local advancement
- coherent observation
- constraint-driven behavior
- no user-authored central update loop

## World State

At time `t`, the world is modeled as:

`W(t) = (E, S(t), C, R)`

where:

- `E` is the set of entities
- `S(t)` is the time-indexed state of entities
- `C` is the set of constraints
- `R` is the set of evolution rules

## Entity State

Each entity `e` has:

`state(e) = (shape, pose, velocity, last_update_time, attributes)`

Minimum required fields for v0.1:

- `shape`
- `position`
- `velocity`
- `last_update_time`

For rigid translation-only motion:

`position_e(t2) = position_e(t1) + velocity_e(t1) * (t2 - t1)`

unless a constraint-triggered interaction changes the trajectory.

## Time Model

The runtime maintains:

- `global_time`
- object-local `last_update_time`

Phase G refines this into:

- a world frontier `t`
- object-local progress `tau(e)`
- an observation operator `Obs(W, t_obs)`
- a stable snapshot predicate `Stable(W, t_obs)`

### Advancement Rule

Entities are advanced lazily.
An entity is only brought forward when:

- it is observed
- it participates in an interaction
- a constraint requires evaluation at a later time

This supports the intended feeling that the world evolves on its own, while computation happens only when needed.

The semantic reading is:

- time advances monotonically
- entities need not be materialized uniformly
- coherent observation requires a semantically justified frontier rather than a naive global read

## Observation Semantics

An observation request asks for a coherent snapshot at time `t_obs`.

`snapshot(W, t_obs)`:

1. identify entities relevant to the observation
2. advance each relevant entity to `t_obs`
3. resolve required local interactions
4. produce a consistent snapshot view

The runtime may internally keep entities stale outside the observed or interacting region.

## Interaction Semantics

The default model is:

- globally asynchronous
- locally synchronized on contact or dependency

When entities become interaction-relevant, the runtime aligns them to a common interaction time and applies interaction logic there.

For v0.1, interaction relevance may be limited to:

- collision with static planes
- entering forbidden regions

## Constraint Model

A constraint is a predicate over world states and trajectories.

`c : W(t) -> {valid, invalid, repairable}`

Initial categories:

- spatial exclusion
- velocity bound
- collision response rule

Examples:

- `not inside(A, forbidden_zone)`
- `speed(A) <= vmax`
- `reflect_on_collision(A, floor)`

Current prototype interpretation:

- forbidden regions are axis-aligned boxes
- `speed(A) <= vmax` is enforced as a hard invariant
- entering a forbidden region halts the scenario with a contradiction report
- sphere-sphere interaction is opt-in through explicit elastic collision constraints

## Contradiction Policy

v0.1 should make contradiction handling explicit.
Three candidate outcomes exist:

1. reject the world state
2. repair by applying a rule
3. stop the affected local evolution

Recommended initial policy:

- invalid initial declarations: reject
- collision constraints with explicit response: repair
- unsupported contradictions: report and halt the affected scenario

## Minimal Runtime Loop Without User `update`

The runtime still has an internal scheduler, but it is not exposed as a user-authored frame loop.

Conceptually:

1. accept declarations
2. construct initial world state
3. answer observation and interaction demands
4. advance only what is needed
5. apply constraints and interaction rules

With multiple spheres, the runtime searches for the next relevant local event:

- sphere-plane collision
- sphere-region contradiction
- sphere-sphere collision

Only the entities participating in the event change interaction state there, which is the current practical form of local synchronization.

<<<<<<< feature/phase-g-local-synchronization
Phase G2 refines this operational intuition into:

- earliest-event selection
- simultaneous-event resolution
- semantic priority plus deterministic tie-breaking
=======
Phase G3 refines this operational intuition into:

- synchronization scope `Sync(ev)`
- dependency closure `deps(ev)`
- minimal consistency conditions for local event interpretation
>>>>>>> local

## First Implementation Scope

To keep Phase 1 realistic, support only:

- spheres
- static planes
- constant velocity between interactions
- elastic reflection on collision
- snapshot observation at requested time

This is enough to validate the central claim that a world can evolve without user-written update logic.
