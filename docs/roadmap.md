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

## Phase G. Time And Event Semantics

Research stage

Outputs:

- formal time domain for `sekai`
- observation operator and stable snapshot definition
- deterministic event ordering rule
- separation between event firing, enforcement, and contradiction
- semantics draft usable in the paper

Definition of done:

- the runtime's notion of time is mathematically documented
- simultaneous or competing events have an explicit resolution rule
- local synchronization has a formal scope
- snapshot and failure semantics are stated precisely enough for a semantics section

### Phase G1. Time Model

Outputs:

- global timeline definition
- object-local progress function
- observation operator
- stable snapshot predicate

Definition of done:

- `sekai` time is defined as a world-ordering structure, not only a numeric clock

### Phase G2. Event Ordering

Outputs:

- simultaneous-event rule
- semantic priority order
- deterministic tie-breaker

Definition of done:

- the same world configuration yields the same event order under the same runtime semantics

### Phase G3. Local Synchronization

Outputs:

- synchronization scope definition
- dependency closure for interacting entities
- minimal consistency set for event resolution

Definition of done:

- local synchronization is formalized rather than treated as an implementation detail

### Phase G4. Event / Enforcement Semantics

Outputs:

- transition-system account of event firing
- repair transition
- contradiction transition

Definition of done:

- fired, repaired, and contradicted laws are defined at the semantic level

### Phase G5. Semantic Consolidation

Outputs:

- operational semantics draft
- snapshot semantics draft
- failure semantics draft
- shared notation spanning G1-G4

Definition of done:

- `sekai` has a semantics section that can anchor the next paper draft

## Phase H. Diagram-To-Logic Extraction

Research stage

Outputs:

- richer candidate extraction from diagrams
- explanation of why a candidate relation was proposed
- editing loop between accepted logic and world execution

Definition of done:

- the visual interface proposes structured logic instead of acting only as a scene editor

## Phase I. Possibility And Convergence

Research stage

Outputs:

- candidate-state or candidate-action representation
- separation between hard constraints and soft preferences
- convergence rule for partially determined worlds

Definition of done:

- `sekai` can express underdetermined worlds that converge under constraints

## Phase J. Richer Geometry And Space

Research stage

Outputs:

- richer surfaces and volumes
- path and visibility primitives
- stronger 3D geometry support

Definition of done:

- the language can model more than moving spheres and axis-aligned forbidden regions

## Phase K. Comparative Evaluation

Research stage

Outputs:

- comparisons against imperative baselines
- specification-size comparisons
- qualitative evaluation or user-study planning

Definition of done:

- the project has evidence for why this model is better than an imperative alternative for its target tasks

## Near-Term Milestone

Within six months, achieve:

Declaratively simulate a bouncing sphere on a floor with no user-written `update` logic.
