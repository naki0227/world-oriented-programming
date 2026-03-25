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

Implementation-near mapping to the current runtime event kinds:

- `EventKind::PlaneCollision` -> boundary-contact
- `EventKind::ForbiddenRegionEntry` -> boundary-entry
- `EventKind::SphereCollision` -> interaction

### Priority Lattice

Within the same event time, G2 currently adopts a semantic priority lattice:

`boundary-entry > boundary-contact > interaction`

Interpretation:

- boundary-entry events are resolved first because they directly threaten admissibility
- boundary-contact events are resolved next because they alter immediate motion at a boundary
- interaction events are resolved after boundary conditions have been normalized

This is a semantic proposal, not yet the final runtime law.
It is the current intended formal direction.

### Prototype-Compatible Ordering Key

To connect the semantics to the present implementation, define the ordering key:

`Key(ev) = (time(ev), priority(ev), law_key(ev), participant_key(ev), index(ev))`

where:

- `time(ev)` is the scheduled event time
- `priority(ev)` is induced by the priority lattice
- `law_key(ev)` is a fixed lexical name for the generating law kind
- `participant_key(ev)` is the sorted lexical tuple of participant identifiers
- `index(ev)` is an implementation-stable fallback index

Candidate events are selected by lexicographic minimization over `Key(ev)`.

This gives a concrete deterministic tie-breaker without collapsing the semantic distinction between time, priority, and implementation convenience.

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

### Near-Simultaneity

The runtime currently uses floating-point time, so semantic simultaneity must tolerate numerical approximation.

For G2, two events are treated as simultaneous when:

`|time(ev1) - time(ev2)| <= epsilon_t`

for a fixed temporal tolerance `epsilon_t`.

The semantics should speak in exact time, but the implementation may use `epsilon_t` to approximate equality in candidate selection.

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

## Phase G4 Event / Enforcement Semantics

G4 defines what happens after `Next(W_t)` selects an event.

The current semantic objective is to separate:

- event firing
- admissibility enforcement
- contradiction

instead of treating them as one undifferentiated runtime step.

### Transition Layers

Let `ev = Next(W_t)`.

The intended transition layers are:

1. event transition
2. enforcement transition
3. contradiction transition, if enforcement fails

We write these as distinct semantic relations.

### Event Transition

If `ev` is selected and its participants are synchronized to `time(ev)`, then:

`(W_t, ev) ->event W_t^ev`

Interpretation:

- the world advances to the event time
- the participants of `ev` are materialized there
- the event law applies its immediate state transformation

Examples in the current prototype:

- plane collision reflects motion at a boundary
- sphere-sphere collision exchanges velocity through elastic interaction
- forbidden-region entry marks a boundary crossing that must be checked by enforcement

### Enforcement Transition

After event firing, the world must be checked for admissibility under the declared law set.

Write:

`W_t^ev ->enforce W_t'`

when all required post-event enforcement succeeds.

This relation may:

- leave the state unchanged
- repair the state according to a law policy

Current prototype examples:

- `clamp speed(A) <= vmax`
- `clamp not inside(A, zone)`
- `reflect not inside(A, zone)`

### Contradiction Transition

If enforcement cannot produce an admissible continuation, the world enters contradiction.

Write:

`W_t^ev ->contradiction W_t^X`

where `W_t^X` denotes semantic failure at time `t`.

The important point is that contradiction is downstream from event and enforcement semantics.
It is not merely a parser error or a detached exception.

### Composite Step Schema

The intended G4 composite step is:

1. select `ev = Next(W_t)`
2. apply event transition to obtain `W_t^ev`
3. apply enforcement to obtain either:
   - an admissible world `W_t'`, or
   - contradiction at time `t`

In compact form:

`W_t ->event W_t^ev ->enforce W_t'`

or

`W_t ->event W_t^ev ->contradiction W_t^X`

### Relationship To Runtime Activity Labels

The current runtime already exposes the labels:

- `fired`
- `repaired`
- `contradicted`

These labels now admit a semantic reading:

- `fired` corresponds to the event transition being taken
- `repaired` corresponds to successful enforcement after that event
- `contradicted` corresponds to failed enforcement at that event frontier

This means the runtime trace is already a partial observable of the intended transition system.

### Why G4 Depends On G2

G4 presupposes G2 because the transition system needs a unique next event.

Without `Next(W_t)`, one cannot define:

- which event fires first
- which participant set synchronizes first
- which enforcement step is semantically downstream

So G2 gives the selection rule and G4 gives the transition rule that consumes it.

## Phase G3 Local Synchronization

G3 defines which part of the world must be synchronized when an event or admissibility boundary becomes semantically relevant.

The core question is:

when a world event must be interpreted, which entities and static structures must be brought to a common semantic frontier?

## Synchronization Scope

For a semantically relevant event `ev`, define its synchronization scope:

`Sync(ev) = part(ev) ∪ deps(ev)`

where:

- `part(ev)` is the participant set of the event
- `deps(ev)` is the smallest dependency closure required for coherent interpretation of `ev`

The purpose of G3 is to define `deps(ev)` without collapsing back into global lockstep execution.

## Dependency Closure

`deps(ev)` contains world elements whose state is necessary to determine:

- whether `ev` is semantically valid
- how `ev` changes the world
- whether admissibility is preserved after `ev`

These dependencies may include:

- static geometry
- constraint context
- region bounds
- later, visibility or path context

So `deps(ev)` is not restricted to moving entities.

## Prototype-Level Synchronization Cases

In the current prototype, the intended scopes are:

- plane collision:
  moving sphere plus plane context
- forbidden-region entry:
  moving sphere plus region context
- sphere-sphere collision:
  the two colliding spheres

This means the current runtime already behaves as if synchronization were local, even though the scope is not yet fully formalized.

## Minimal Consistency Set

`Sync(ev)` should be minimal.

Define the minimal consistency condition:

`Sync(ev)` is acceptable only if excluding any element of `Sync(ev)` would make the interpretation of `ev` semantically ambiguous or semantically incorrect.

Equivalently:

entities outside `Sync(ev)` may remain stale only when their exclusion cannot change:

- event validity
- event effect
- admissibility outcome

## Synchronization Frontier

If an event is interpreted at semantic time `t_ev`, every synchronized entity must be materialized to that frontier:

`forall e in part(ev), tau(e) = t_ev`

and every required dependency in `deps(ev)` must be interpreted at the same semantic frontier.

Entities outside `Sync(ev)` are not required to satisfy this condition.

## Observation Versus Synchronization

Observation and synchronization are related but distinct.

- observation constructs a coherent snapshot frontier
- synchronization constructs a coherent event frontier

They coincide only when the observation request itself forces the same semantic carrier as the selected event.

This distinction is important because `sekai` should support:

- coherent observation
- non-lockstep world execution

without forcing the whole world to advance uniformly.

## Admissibility-Driven Synchronization

Some synchronization is required not because two entities interact directly, but because admissibility must be evaluated locally.

Examples:

- entering a forbidden region requires region context to determine contradiction, clamp, or reflection behavior
- a local repair step may require the same synchronized carrier even if no further interaction event is introduced

This motivates the semantic rule:

local synchronization may be induced by admissibility dependence, not only by collision or contact.

## Locality Criterion

G3 adopts the following locality criterion:

do not synchronize an entity or dependency unless excluding it would change the semantic interpretation of the currently relevant event.

This gives a principled alternative to:

- permanent global synchronization
- ad hoc implementation-only heuristics

## Why G3 Matters

G3 gives formal meaning to the claim:

global asynchrony with local synchronization.

Without G3, that claim remains philosophical.
With G3, it becomes a bounded semantic operation over a minimal consistency set.
