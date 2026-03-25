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

We also write:

- `sigma_t(e)` for the materialized state of entity `e` at world frontier `t`
- `tau_t(e)` for the local progress timestamp of entity `e` inside `W_t`

## Object-Local Progress

Each entity `e ∈ E` has a local progress timestamp:

`tau(e) ∈ T`

with the invariant:

`tau(e) <= t`

Interpretation:

- `t` is the world's current semantic frontier
- `tau(e)` is the latest time at which the runtime has materialized entity `e`

This allows the semantics to represent globally asynchronous storage with locally synchronized interaction.

## Time Monotonicity Invariants

The prototype and the intended semantics both rely on monotonic time advancement.

For every reachable world configuration:

1. world-frontier monotonicity:
   `t` never decreases
2. local monotonicity:
   for every entity `e`, `tau(e)` never decreases
3. local boundedness:
   for every entity `e`, `tau(e) <= t`

These invariants ensure that lazy materialization does not imply temporal rollback.

## Evolution Operator

For each entity `e`, define an unconstrained evolution operator:

`Advance(e, t1, t2)`

for `t1 <= t2`.

In the current prototype, for constant velocity motion:

`position_e(t2) = position_e(t1) + velocity_e(t1) * (t2 - t1)`

unless an event or enforcement rule changes the trajectory at an intermediate time.

For a set of entities `X ⊆ E`, define the joint advancement operator:

`Advance_X(W_t, t')`

which advances only entities in `X` from their current local timestamps toward `t'`.

This operator is partial:
it is only semantically valid when no unresolved earlier event or contradiction blocks advancement.

## Candidate Events

Let `Ev(W_t)` be the set of candidate events visible from world configuration `W_t`.

Each candidate event `ev ∈ Ev(W_t)` has:

- a scheduled time `time(ev) ∈ T`
- a participant set `part(ev) ⊆ E`
- a generating law `law(ev) ∈ C`

The intended meaning is:

`ev` is the next semantically relevant interaction or admissibility boundary proposed by the current world state.

For the current prototype, candidate events include:

- sphere-plane collision
- sphere-sphere collision
- forbidden-region entry

Phase G2 will define how `Ev(W_t)` is ordered when multiple candidates share the same time.

## Observation Operator

Observation is an explicit semantic operator:

`Obs(W, t_obs) -> Snapshot`

subject to `t <= t_obs`.

Its meaning is:

1. bring all observation-relevant entities to a common observation frontier
2. resolve any interaction or admissibility work needed before `t_obs`
3. emit a coherent snapshot of the resulting world

The prototype currently treats all declared spheres as observation-relevant.

## Observation Frontier

Define the observation frontier induced by a request at time `t_obs` as:

`Frontier(W, t_obs) = { e ∈ E | e is observation-relevant at t_obs }`

For the current prototype:

`Frontier(W, t_obs) = E_spheres`

In later phases, this frontier may be narrowed by dependency or view locality.

The important semantic point is that observation is not merely a read.
It is a request to construct a coherent world boundary at `t_obs`.

## Stable Snapshot Predicate

Define:

`Stable(W, t_obs)`

to mean that the world admits a coherent observation at time `t_obs` after all required event and enforcement work at or before `t_obs` has been resolved.

A snapshot is semantically valid only if:

`Stable(W, t_obs)` holds.

This distinguishes:

- a merely requested observation time
- a semantically admissible observation

More explicitly, `Stable(W, t_obs)` requires:

1. every entity in `Frontier(W, t_obs)` has been materialized to `t_obs`
2. no unresolved candidate event exists with time `< t_obs`
3. all required admissibility enforcement at or before `t_obs` has been applied
4. no contradiction has been reached at or before `t_obs`

This predicate is what turns a runtime snapshot into a semantic snapshot.

## Event-Relevant Time

Not every entity must be materialized at every global time.
Instead, materialization happens when time becomes semantically relevant.

Time becomes relevant for an entity when:

- the entity is observed
- the entity participates in a candidate event
- a law requires admissibility enforcement involving that entity

This is the semantic basis of lazy advancement.

Equivalently:

time is not forced to become relevant uniformly across the world.
It becomes relevant through observation, interaction, and admissibility demand.

## Local Synchronization Principle

`sekai` is not globally lockstep.

Instead:

- unrelated entities may remain stale relative to a requested observation frontier
- interacting entities are aligned when an event or admissibility dependency requires it

This principle can be summarized as:

global asynchrony with local synchronization.

Phase G3 will formalize the synchronization scope more precisely.

At the G1 level, we can already describe the intended synchronization carrier:

for a candidate event `ev`, define its minimal synchronization carrier as:

`Sync(ev) = part(ev) ∪ deps(ev)`

where `deps(ev)` is the smallest dependency closure needed to evaluate the event coherently.

G3 will define `deps(ev)` more rigorously.

## Snapshot Determinism Goal

For a fixed world declaration and fixed semantic ordering rules, `Obs(W, t_obs)` should be deterministic.

This requires:

- deterministic event selection
- deterministic tie-breaking when events are simultaneous
- deterministic enforcement policy application

Equivalently:

if `W`, `t_obs`, and the semantic ordering rules are fixed, then `Obs(W, t_obs)` must denote a unique snapshot or a unique contradiction outcome.

Phase G2 and G4 refine this requirement.

## Failure At Time

If no admissible configuration exists at time `t`, the world reaches contradiction at `t`.

Write:

`W @ t -> contradiction`

Contradiction is therefore a temporal outcome, not a timeless error condition.
It states that no admissible continuation exists at semantic time `t` under the current law set and enforcement rules.

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
- candidate events in the scheduler correspond to elements of `Ev(W_t)`

This does not mean the current runtime fully realizes the final semantics.
It means the runtime already exposes the right semantic hooks.

## Why G1 Matters Before G2

G2 cannot be written cleanly without G1 because event ordering presupposes:

- a time domain
- a notion of current world frontier
- a notion of candidate event time
- a definition of what counts as a stable observable world

So G1 is not an optional preface.
It is the semantic substrate that makes event ordering meaningful.

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

## Phase G2 Event Ordering

G2 defines how candidate events are selected when several are simultaneously or nearly simultaneously eligible.

### Ordering Goal

For each reachable configuration `W_t`, define a deterministic selection operator:

`Next(W_t) -> ev or none`

such that:

- if a semantically relevant event exists, `Next(W_t)` returns a unique event
- if no event exists before the next observation frontier, `Next(W_t) = none`

### Earliest-Time Principle

Primary ordering is temporal.

For candidate events `ev1, ev2 ∈ Ev(W_t)`:

`ev1 <_time ev2` iff `time(ev1) < time(ev2)`

The first selection rule is:

choose only events with minimal event time.

Formally, define:

`MinEv(W_t) = { ev ∈ Ev(W_t) | ∀ev' ∈ Ev(W_t), time(ev) <= time(ev') }`

If `MinEv(W_t)` has one element, selection is immediate.

### Simultaneous Events

If `|MinEv(W_t)| > 1`, the semantics must resolve simultaneity.

The current semantic direction is:

1. preserve semantic causality first
2. apply a deterministic priority rule second
3. apply a deterministic tie-breaker last

This avoids treating simultaneity as arbitrary scheduler accident.

### Event Categories For Ordering

For G2, each candidate event belongs to one of the following semantic classes:

- boundary-contact event
- boundary-entry event
- interaction event

Current prototype mapping:

- sphere-plane collision -> boundary-contact
- forbidden-region entry -> boundary-entry
- sphere-sphere collision -> interaction

### Priority Lattice

Within the same event time, G2 currently adopts a semantic priority lattice:

`boundary-entry > boundary-contact > interaction`

Interpretation:

- boundary-entry events are resolved first because they directly threaten admissibility
- boundary-contact events are resolved next because they alter immediate motion at a boundary
- interaction events are resolved after boundary conditions have been normalized

This is a semantic proposal, not yet the final runtime law.
It is the current intended formal direction.

### Deterministic Tie-Breaker

If two candidate events have:

- the same event time, and
- the same semantic priority

then selection falls back to a deterministic tie-breaker.

The current prototype-compatible proposal is:

1. compare the generating law kind in a fixed lexical order
2. compare participant identifiers in sorted lexical order
3. compare an implementation-stable fallback index if still needed

This keeps event ordering deterministic without pretending that all simultaneous events are semantically identical.

### Causality Preservation Requirement

The purpose of the ordering rule is not merely determinism.
It is to preserve semantic causality.

The intended requirement is:

if `ev1` changes the admissibility or participant state on which `ev2` depends, then `ev1` must not be ordered after `ev2`.

Later work can express this through a causality graph:

`ev1 -> ev2`

meaning that `ev2` is semantically downstream from `ev1`.

### Selection Schema

The full intended event-selection schema is therefore:

1. construct `Ev(W_t)`
2. restrict to `MinEv(W_t)`
3. apply semantic priority within `MinEv(W_t)`
4. apply deterministic tie-breaker if needed
5. produce `Next(W_t)`

This is the bridge between G1 time semantics and G4 transition semantics.

### Relationship To Observation

Observation must respect event ordering.

For `Obs(W, t_obs)`:

- if an event `ev` exists with `time(ev) < t_obs`, it must be resolved before snapshot construction
- if several events exist at the same earliest time, the G2 ordering rule determines which transition is taken first

So observation determinism depends directly on event determinism.

### G2 Scope Boundary

G2 does not yet define the full repair or contradiction transition.
It only defines which event is selected next and why.

That selected event will later feed:

- G3 synchronization scope
- G4 event / repair / contradiction transition rules
