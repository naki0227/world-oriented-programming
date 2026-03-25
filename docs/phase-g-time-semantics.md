# Phase G Time Semantics

## Purpose

This document begins the semantic core of `sekai`.
Its role is to define the time model used by the runtime and to prepare the ground for event ordering, local synchronization, and failure semantics.

Phase G1 is not yet a full operational semantics.
It is the constitutional layer for the later semantics.

## Central Claim

`sekai` time is not only a numeric clock.
It is a structure that orders world evolution, observation, interaction, and admissibility checks.

The runtime uses numeric timestamps, but the semantics must describe more than arithmetic progression.

## Time Domain

Let `T` be a dense totally ordered time domain.

For the prototype:

- `T` is modeled as non-negative real-valued time
- the runtime approximates this with floating-point values

Semantically, we use:

- `t, t', t_obs ∈ T`
- `t <= t'` for temporal precedence

## World Configuration

At semantic time `t`, a world configuration is:

`W_t = (E, sigma, C, A, t)`

where:

- `E` is the set of entities
- `sigma` is the per-entity state map
- `C` is the set of declared world laws
- `A` is the admissibility policy induced by those laws
- `t` is the current semantic frontier of the world execution

## Object-Local Progress

Each entity `e ∈ E` has a local progress timestamp:

`tau(e) ∈ T`

with the invariant:

`tau(e) <= t`

Interpretation:

- `t` is the world's current semantic frontier
- `tau(e)` is the latest time at which the runtime has materialized entity `e`

This allows the semantics to represent globally asynchronous storage with locally synchronized interaction.

## Evolution Operator

For each entity `e`, define an unconstrained evolution operator:

`Advance(e, t1, t2)`

for `t1 <= t2`.

In the current prototype, for constant velocity motion:

`position_e(t2) = position_e(t1) + velocity_e(t1) * (t2 - t1)`

unless an event or enforcement rule changes the trajectory at an intermediate time.

## Observation Operator

Observation is an explicit semantic operator:

`Obs(W, t_obs) -> Snapshot`

subject to `t <= t_obs`.

Its meaning is:

1. bring all observation-relevant entities to a common observation frontier
2. resolve any interaction or admissibility work needed before `t_obs`
3. emit a coherent snapshot of the resulting world

The prototype currently treats all declared spheres as observation-relevant.

## Stable Snapshot Predicate

Define:

`Stable(W, t_obs)`

to mean that the world admits a coherent observation at time `t_obs` after all required event and enforcement work at or before `t_obs` has been resolved.

A snapshot is semantically valid only if:

`Stable(W, t_obs)` holds.

This distinguishes:

- a merely requested observation time
- a semantically admissible observation

## Event-Relevant Time

Not every entity must be materialized at every global time.
Instead, materialization happens when time becomes semantically relevant.

Time becomes relevant for an entity when:

- the entity is observed
- the entity participates in a candidate event
- a law requires admissibility enforcement involving that entity

This is the semantic basis of lazy advancement.

## Local Synchronization Principle

`sekai` is not globally lockstep.

Instead:

- unrelated entities may remain stale relative to a requested observation frontier
- interacting entities are aligned when an event or admissibility dependency requires it

This principle can be summarized as:

global asynchrony with local synchronization.

Phase G3 will formalize the synchronization scope more precisely.

## Snapshot Determinism Goal

For a fixed world declaration and fixed semantic ordering rules, `Obs(W, t_obs)` should be deterministic.

This requires:

- deterministic event selection
- deterministic tie-breaking when events are simultaneous
- deterministic enforcement policy application

Phase G2 and G4 refine this requirement.

## Failure At Time

If no admissible configuration exists at time `t`, the world reaches contradiction at `t`.

Write:

`W @ t -> contradiction`

The prototype already records this operationally as a failed report with law activity and partial stable snapshots.

Phase G4 will formalize the relationship between:

- event firing
- repair
- contradiction

## Runtime Interpretation

Current runtime variables map to the semantic layer as follows:

- `global_time` corresponds to the world frontier `t`
- `last_update_time` corresponds to `tau(e)`
- snapshots correspond to `Obs(W, t_obs)` when `Stable(W, t_obs)` holds

This does not mean the current runtime fully realizes the final semantics.
It means the runtime already exposes the right semantic hooks.

## What G1 Fixes

By the end of G1, the project should have stable definitions for:

- time domain `T`
- world frontier `t`
- local progress `tau(e)`
- observation operator `Obs`
- stable snapshot predicate `Stable`

These definitions are the basis for:

- G2 event ordering
- G3 local synchronization scope
- G4 transition semantics
- G5 semantic consolidation
