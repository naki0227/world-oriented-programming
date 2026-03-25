# Roadmap

## Goal

Build a world-description language driven by 3D space, continuous time, and logical constraints.

## Phase 0. Philosophy Lock-In

Target: 1 to 2 weeks

Outputs:

- one-page philosophy statement
- core concept glossary
- initial success criteria

Key concepts:

- point / vector
- rigid body
- velocity
- constraint
- event
- time

Definition of done:

- the project can be explained without talking about implementation details first
- the non-goals are explicit

## Phase 1. Minimal World Simulator

Target: 1 to 2 months

Outputs:

- global world time
- per-object last-update time
- positions and velocities
- simple collision handling
- lazy time advancement on access
- coherent snapshot rendering

Definition of done:

- the world appears to move on its own
- observation can produce a consistent snapshot without a user-defined update loop

## Phase 2. Constraint System

Target: 1 to 2 months

Outputs:

- logical constraint engine
- forbidden regions
- velocity limits
- collision rules
- immediate contradiction detection
- local synchronization on interaction

Definition of done:

- invalid world states are rejected or repaired according to explicit rules
- interaction semantics are clearer than ad hoc event code

## Phase 3. Descriptive DSL

Target: 2 to 3 months

Outputs:

- minimal textual DSL
- declarations for objects and surfaces
- velocity and constraint syntax
- runtime bridge from DSL to world engine

Example target:

```text
sphere A
plane floor

velocity(A) = (1, 0, 0)

constraint:
    reflect_on_collision(A, floor)
```

Definition of done:

- a first prototype can describe a moving constrained world declaratively

## Phase 4. Diagram Interface

Target: 3 to 6 months

Outputs:

- 3D editor for placing points, surfaces, and regions
- extraction of candidate relations from diagrams
- human approval flow for generated logic

Definition of done:

- drawing and logical modeling are connected by a shared world model

## Phase 5. Action Candidate Model

Target: 6 months and beyond

Outputs:

- weighted action candidates
- hard constraint filtering
- time-varying decision scoring

Definition of done:

- agents can exhibit natural behavior without explicit procedural behavior trees in user space

## Phase 6. Optimization Layer

Research stage

Topics:

- GPU parallelization
- faster constraint solving
- event scheduling
- symbolic simplification

## Near-Term Milestone

Within six months, achieve:

Declaratively simulate a bouncing sphere on a floor with no user-written `update` logic.
