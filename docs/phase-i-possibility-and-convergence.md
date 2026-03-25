# Phase I Possibility And Convergence

## Purpose

Phase I extends `sekai` from a language of fully specified worlds to a language that can also express underdetermined worlds.

The goal is not to introduce unconstrained nondeterminism.
It is to let the programmer write worlds whose next admissible state is not fully fixed yet, while still allowing the runtime to converge toward valid behavior under explicit laws and preferences.

This phase is central to the long-term identity of the project because it connects:

- world description
- constraint semantics
- agent-like choice
- convergence under partial information

## Core Intuition

An underdetermined world is a world where some future state, action, or relation is not uniquely determined by the current hard law set alone.

Instead of forcing the user to proceduralize that uncertainty, `sekai` should allow the world to contain:

- candidate states
- candidate actions
- candidate relations

which remain unresolved until more information, more time, or more constraints become relevant.

The key design rule is:

`sekai` should represent uncertainty as constrained world structure, not as arbitrary branching execution.

## Minimal Semantic Shift

Up to Phase G, the semantics centered on a single world at frontier `t`:

`W_t = (E, G, L, tau, sigma, t)`

Phase I introduces a candidate-bearing extension:

`W_t^? = (E, G, L_h, L_s, C, tau, sigma, t)`

where:

- `L_h` is the hard law set
- `L_s` is the soft preference or scoring set
- `C` is the candidate structure still under resolution

The important distinction is:

- hard laws determine admissibility
- soft laws influence convergence among admissible candidates

## Candidate Structure

The first useful abstraction is a candidate structure:

`C = {c_1, c_2, ..., c_n}`

where each `c_i` is a candidate continuation compatible with the current world frontier to some degree.

At this stage, candidates may refer to:

- action choices
- next-state tendencies
- local interaction outcomes

The candidate structure should not be read as a full powerset of possible worlds.
That would be too expensive and too semantically loose.

Instead, the intended interpretation is:

`C` is a compact representation of still-admissible unresolved continuations.

## Hard Constraints And Soft Preferences

Phase I depends on a strict separation:

### Hard Laws

Hard laws are world-validity requirements.
They may:

- reject a candidate
- repair a candidate
- terminate the world in contradiction

### Soft Preferences

Soft preferences do not make a world invalid.
They instead rank or bias candidates.

Examples of future soft preferences:

- prefer shorter paths
- prefer safer motion
- prefer pursuit over waiting

The design principle is:

hard laws prune the search space;
soft preferences steer convergence within the remaining admissible space.

## Convergence Operator

The core semantic object of Phase I is a convergence operator:

`Conv(W_t^?) -> W_t' or W_t^X`

Interpretation:

- start from an underdetermined world
- eliminate candidates violating hard laws
- apply repair where the law system allows it
- score or rank the remaining admissible candidates
- commit to one admissible continuation, or reach contradiction if none survive

This operator does not need to be globally optimal in the first prototype.
It only needs to be semantically disciplined.

## Observation Under Underdetermination

Observation now becomes more subtle.

For a requested observation time `t_obs`, three states become possible:

1. the world has already converged to a unique admissible continuation
2. the world remains underdetermined but observationally equivalent across remaining candidates
3. the world remains observably ambiguous

This motivates a refined observation idea:

`Obs?(W^?, t_obs) -> S or U or X`

where:

- `S` is a stable snapshot
- `U` is unresolved observational ambiguity
- `X` is contradiction

The long-term goal is to minimize unnecessary ambiguity while still preserving semantically meaningful underdetermination.

## Initial Proof Obligations

Phase I introduces new theory obligations beyond Phase G.

The first important ones are:

- admissibility preservation under convergence
- convergence termination for the supported candidate model
- observational stability when different candidates become indistinguishable at the requested frontier

These are not yet solved, but they define the research target clearly.

## Prototype Direction

The first prototype for Phase I should stay small.

Recommended first scope:

- candidate actions for a small finite set of entities
- hard constraints from the existing law layer
- soft preference scores attached to those actions
- deterministic tie-breaking after scoring

This keeps the work continuous with Phase F and Phase G instead of opening an entirely separate subsystem.

The first concrete syntax target is:

```text
action:
    candidate_velocity(A, wait) = (0, 0, 0) score 1
    candidate_velocity(A, move) = (3, 0, 0) score 5
```

where the runtime:

- tests each candidate against the hard law layer
- keeps repaired candidates if they remain admissible
- chooses the highest-scoring admissible continuation
- logs which candidates were rejected and which one was selected
- exposes a compact candidate-resolution summary in the structured report
- can apply the same initial resolution pass to multiple entities in a deterministic order
- can expose top-score ties and skipped candidates after early deterministic selection

The next minimal demonstration after pure rejection filtering is a repaired selection:

```text
action:
    candidate_velocity(A, fast) = (6, 0, 0) score 5
    candidate_velocity(A, safe) = (3, 0, 0) score 2

constraint:
    clamp speed(A) <= 4
```

In that case the runtime may still choose `fast`, provided the hard law layer repairs it into an admissible continuation.

The static analysis path should also expose candidate inventory before simulation, so that underdetermined worlds can be inspected without executing convergence:

- which entities carry candidates
- how many candidates they declare
- which labels are tied for the top soft score
- which entities explicitly request deferred handling for ambiguous top choices
- which static resolution hint applies before runtime (`single_top_candidate`, `deterministic_tie_break`, `deferred_on_ambiguous_top`, or a deferred preference hint such as `defer_then_prefer_beta_at_1.000`)

The runtime report should also expose a convergence summary after selection, so that underdetermined worlds are not reduced to a single opaque winner:

- whether a branch was chosen directly, by fallback, or after repair
- whether symbolic underdetermination remained after score ordering
- whether any remaining underdetermination was still visible at the observation layer
- whether the observation should be read as determinate, representative, or still ambiguous
- aggregate totals across all candidate-bearing entities in the run

In the current prototype this is also summarized at run level as:
- `determinate`
- `representative`
- `unresolved`

This is the first executable stand-in for `Obs?(W^?, t_obs) = U`.

The current prototype can also defer an ambiguous top-score tie explicitly instead of choosing a representative branch immediately.
This creates the first small executable case where the world remains unresolved at observation time by design.

The next useful case is a mixed world where one entity stays deferred while another still converges.
That makes partial convergence visible instead of forcing the whole world into a fully determinate or fully unresolved reading.

After that, the next important executable step is persistence across frontiers:
a deferred entity should be able to remain unresolved across more than one observation while other entities continue to evolve.

The next step after persistence is controlled re-convergence:
the world should be able to defer a top-score ambiguity at one frontier and resolve it later when an explicit convergence trigger becomes active.

The current prototype now also includes a small preference-triggered version of that step:
`prefer_candidate_at(A, beta, 1)` makes a later frontier prefer `beta` among an otherwise tied top-score set, so the world can move from deferred ambiguity to a determinate branch for a semantic reason stronger than alphabetical tie-breaking.

It now also includes a score-update version:
`rescore_candidate_at(A, beta, 1, 1)` changes the soft ranking itself at a later frontier, making it possible for re-convergence to follow newly active information rather than only a fixed score table declared at time zero.

It now also includes a law-update version:
`update_speed_limit_at(A, 1, 6)` changes admissibility itself at a later frontier, so a branch that was previously blocked by a hard law can later become selectable without rewriting the whole scene imperatively.

Once controlled re-convergence exists for one entity, the next useful case is staged world-level convergence:
different entities may resolve at different frontiers, so observation status should be able to move from unresolved to determinate over time.

## Definition Of Success

Phase I is successful when:

- `sekai` can represent at least one underdetermined world without falling back to imperative user code
- the runtime can converge that world using hard laws and soft preferences
- the resulting semantics can explain when ambiguity is preserved, when it is resolved, and when it becomes contradiction
