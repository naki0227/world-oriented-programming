# Semantics Core

Last updated: 2026-04-18

## Purpose

This document consolidates the current operational semantics trajectory for `sekai`.
It is intentionally small.
The goal is to define the semantic objects that the prototype, paper, viewer, and evaluation should all refer to consistently.

This is not yet a full formal proof development.
It is the minimum stable core needed to stop World-Oriented Programming from being only a metaphor.

## Core Thesis

Execution in `sekai` is not described as a user-authored instruction sequence.
It is described as world evolution under explicit laws.

The runtime may use ordinary algorithms internally, but the source-level semantic account is:

```text
world frontier -> next event -> local synchronization -> event firing -> law enforcement -> observation or contradiction
```

For underdetermined worlds, a convergence step can resolve candidate continuations before or during observation.

## World Frontier

At semantic frontier `t`, a world is modeled as:

```text
W_t = (E, G, L_h, L_s, tau, sigma, A, t)
```

where:

- `E` is the finite set of entities.
- `G` is the static or slowly changing geometry known to the world.
- `L_h` is the hard law set.
- `L_s` is the soft preference set.
- `tau : E -> Time` records object-local progress.
- `sigma` assigns each entity its current semantic state.
- `A` is the activity history relevant to reports and observation.
- `t` is the current resolved world frontier.

The current prototype supports a small instance of this model:

- entities are spheres
- geometry includes one static plane and axis-aligned regions
- hard laws include collision response, speed bounds, region exclusion, visibility, and elastic pairwise collision
- soft preferences are currently attached to candidate velocities and visibility-conditioned preferences

## Entity State

For the current prototype, an entity state is:

```text
state(e) = (shape, position, velocity, radius, attributes)
```

Between interaction frontiers, rigid translation is interpreted as:

```text
position_e(t2) = position_e(t1) + velocity_e(t1) * (t2 - t1)
```

unless a law-triggered event or repair changes the continuation.

## Laws

A hard law is an admissibility condition over a world frontier or trajectory segment:

```text
l_h(W_t) -> valid | repairable | invalid
```

Hard laws may:

- permit the frontier
- fire an event
- repair the frontier into an admissible continuation
- produce contradiction

A soft law or preference does not determine validity:

```text
l_s(W_t, C) -> ranking adjustment over candidates
```

Soft laws only influence convergence among candidates that remain admissible under hard laws.

## Events

An event is a semantic object, not only an implementation callback:

```text
ev = (kind, participants, dependencies, frontier, law)
```

Examples in the current prototype include:

- sphere-plane contact
- sphere-region contradiction or repair
- sphere-sphere elastic collision
- visibility failure
- candidate resolution

## Event Selection

The next event operator is:

```text
Next(W_t) -> ev | none
```

The intended ordering is:

1. earliest event frontier
2. semantic priority among event families
3. deterministic tie-breaker

This is the basis for the determinism claim.
The current supported claim should be limited to the prototype event set until simultaneous-event behavior is fully specified and tested.

## Local Synchronization

Local synchronization identifies the smallest carrier that must be brought to a common frontier to interpret an event:

```text
Sync(ev, W_t) -> S
```

where `S` contains:

- event participants
- geometry needed to evaluate the event
- dependency context needed for the relevant law

This is the semantic meaning of "global asynchrony with local synchronization".
Entities outside `S` need not be materialized at the event frontier unless observation or another dependency requires them.

## Event Firing

Event firing produces a post-event world at the same frontier:

```text
Fire(W_t, ev) -> W_t^ev
```

Firing records the event in the activity history and may update participant state.
Firing alone does not guarantee admissibility.
Admissibility is handled by enforcement.

## Enforcement

The enforcement step applies the hard law set:

```text
Enforce(W_t^ev, L_h) -> W_t' | W_t^X
```

where:

- `W_t'` is an admissible world frontier
- `W_t^X` is a contradiction frontier

Repair is a successful enforcement path:

```text
Repair(W_t^ev, l_h) -> W_t'
```

Only laws with explicit repair policies may repair.
Unsupported invalid frontiers become contradiction.

## Contradiction

Contradiction is a semantic result:

```text
W_t^X = (W_t, law, participants, failed_predicate, frontier)
```

It should not be treated as an out-of-band implementation exception in the research account.

Reports and viewer output should therefore expose:

- the law that failed
- the participants
- the frontier
- the policy
- the activity that led to contradiction

## Observation

Observation requests a coherent snapshot:

```text
Obs(W, t_obs) -> Snapshot | W_t^X
```

Observation is valid only after earlier required event frontiers have been resolved.
The runtime may lazily advance entities, but the snapshot must correspond to a coherent semantic frontier for the observed scope.

## Underdetermined Worlds

An underdetermined world carries a candidate structure:

```text
W_t^? = (W_t, C)
```

where `C` is a compact set of candidate continuations.
Candidates are not a full possible-world tree.
They are the unresolved continuations currently exposed by the program.

The convergence operator is:

```text
Conv(W_t^?) -> W_t' | W_t^? | W_t^X
```

It may:

- reject candidates that violate hard laws
- repair candidates when explicit repair policies allow it
- rank remaining candidates with soft preferences
- choose a determinate representative
- preserve unresolved ambiguity
- produce contradiction when no admissible candidate remains

## Observation Under Underdetermination

Observation of candidate-bearing worlds can produce three statuses:

```text
Obs?(W_t^?, t_obs) -> stable | representative | unresolved | contradiction
```

- `stable`: a unique admissible continuation is observed.
- `representative`: multiple symbolic continuations exist but are observationally equivalent or deterministically represented.
- `unresolved`: observable ambiguity remains.
- `contradiction`: no admissible observed frontier exists.

These names should match report fields and viewer labels wherever possible.

## Current Proof Obligations

The near-term theory should focus on limited, prototype-scoped claims:

1. **Deterministic event order**
   Given the supported event set, fixed input world, and fixed tie-breaker, `Next(W_t)` returns the same event sequence.

2. **Snapshot determinism**
   For deterministic laws and fixed observation times, `Obs(W, t_obs)` returns the same snapshot or contradiction.

3. **Finite candidate termination**
   For the current finite candidate model with deterministic ranking and no candidate-generating recursion, `Conv(W_t^?)` terminates.

4. **Admissibility preservation after repair**
   If a law repairs a frontier successfully, the resulting frontier satisfies the repaired law's admissibility predicate.

These are intentionally narrow.
The project should prove the small true claims before claiming a general semantics.

## Known Gaps

- simultaneous events need a fully tested priority and dependency rule
- repair policy interaction is not yet general
- geometry is still narrow
- candidate convergence is finite and prototype-scoped
- contradiction reports need stable law identifiers and source locations
- observation scope is not yet formalized for partial spatial views

## Paper Guidance

The seed paper should use this document to keep the semantics section disciplined.
It should avoid implying a complete theorem-level semantics.
The right claim is:

> The prototype has a coherent operational core with explicit proof obligations.

That claim is strong enough for a seed paper and honest enough to survive close reading.

