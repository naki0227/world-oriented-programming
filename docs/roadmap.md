# Roadmap

## Goal

Build a world-description language driven by 3D space, continuous time, and logical constraints.

## Current Status

The project is no longer in the concept-only stage.
It currently has:

- a working prototype runtime and DSL
- a viewer and diagram-to-execution round-trip
- a paper draft with figures and a LaTeX build
- a semantics trajectory through Phase G

Current roadmap status by phase:

- Phase 0: completed
- Phase 1: completed
- Phase 2: completed in prototype form
- Phase 3: completed in prototype form
- Phase 4: partially completed
- Phase 5: not started
- Phase 6: not started
- Phase G: in progress, but already substantially advanced
- Phase H: partially started through the current viewer
- Phase I: not started
- Phase J: not started
- Phase K: not started

More specifically:

- G1: completed draft
- G2: completed draft
- G3: completed draft
- G4: completed draft
- G5: in progress, with a paper-facing scaffold and compact operational rules

This means the project is currently in a hybrid stage:

- implementation-wise: prototype validation and semantic strengthening
- paper-wise: beyond a pure vision paper, but not yet a full formal-semantics paper

The next research inflection points are now clearer than in the original version of the roadmap:

- Phase G is the semantic core of the language
- Phase I is no longer a distant add-on; it is a central follow-up to the current semantics work
- Phase K should evaluate not only behavior, but also representational complexity and cognitive load

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

Status:

- completed

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

Status:

- completed

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

Status:

- completed in prototype form
- extended beyond the original plan with repair policies, law metadata, activity traces, and static law analysis

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

Status:

- completed in prototype form

## Phase 4. Diagram Interface

Target: 3 to 6 months

Outputs:

- 3D editor for placing points, surfaces, and regions
- extraction of candidate relations from diagrams
- human approval flow for generated logic

Definition of done:

- drawing and logical modeling are connected by a shared world model

Status:

- partially completed
- current prototype supports a viewer, draft editing, candidate law suggestion, and round-trip execution
- still missing a richer true 3D authoring interface

## Phase 5. Action Candidate Model

Target: 6 months and beyond

Outputs:

- weighted action candidates
- hard constraint filtering
- time-varying decision scoring

Definition of done:

- agents can exhibit natural behavior without explicit procedural behavior trees in user space

Status:

- in progress
- initial semantics and research framing for underdetermined worlds now exist

## Phase 6. Optimization Layer

Research stage

Topics:

- GPU parallelization
- faster constraint solving
- event scheduling
- symbolic simplification

Status:

- in progress with a first executable visibility-law slice

## Phase G. Time And Event Semantics

Research stage

Outputs:

- formal time domain for `sekai`
- observation operator and stable snapshot definition
- deterministic event ordering rule
- separation between event firing, enforcement, and contradiction
- semantics draft usable in the paper
- explicit proof obligations for later theory work

Definition of done:

- the runtime's notion of time is mathematically documented
- simultaneous or competing events have an explicit resolution rule
- local synchronization has a formal scope
- snapshot and failure semantics are stated precisely enough for a semantics section
- the paper can name at least the main proof obligations, even if they are not yet proven

Status:

- in progress
- already advanced enough to support a dedicated semantics section in the paper

Future proof obligations:

- snapshot determinism
- causality preservation
- repair termination

### Phase G1. Time Model

Outputs:

- global timeline definition
- object-local progress function
- observation operator
- stable snapshot predicate

Definition of done:

- `sekai` time is defined as a world-ordering structure, not only a numeric clock

Status:

- completed draft

### Phase G2. Event Ordering

Outputs:

- simultaneous-event rule
- semantic priority order
- deterministic tie-breaker

Definition of done:

- the same world configuration yields the same event order under the same runtime semantics

Status:

- completed draft

### Phase G3. Local Synchronization

Outputs:

- synchronization scope definition
- dependency closure for interacting entities
- minimal consistency set for event resolution

Definition of done:

- local synchronization is formalized rather than treated as an implementation detail

Status:

- completed draft

### Phase G4. Event / Enforcement Semantics

Outputs:

- transition-system account of event firing
- repair transition
- contradiction transition

Definition of done:

- fired, repaired, and contradicted laws are defined at the semantic level

Status:

- completed draft

### Phase G5. Semantic Consolidation

Outputs:

- operational semantics draft
- snapshot semantics draft
- failure semantics draft
- shared notation spanning G1-G4

Definition of done:

- `sekai` has a semantics section that can anchor the next paper draft

Status:

- in progress
- shared notation, consolidation scaffold, and compact operational rules already exist

## Phase H. Diagram-To-Logic Extraction

Research stage

Outputs:

- richer candidate extraction from diagrams
- explanation of why a candidate relation was proposed
- editing loop between accepted logic and world execution

Definition of done:

- the visual interface proposes structured logic instead of acting only as a scene editor

Status:

- partially started
- current viewer already proposes candidate constraints and supports law selection before execution

## Phase I. Possibility And Convergence

Research stage

Rationale:

- this phase is now considered a central follow-up to Phase G rather than a distant optional extension
- underdetermined worlds and convergence behavior are part of the core intellectual identity of `sekai`

Outputs:

- candidate-state or candidate-action representation
- separation between hard constraints and soft preferences
- convergence rule for partially determined worlds

Definition of done:

- `sekai` can express underdetermined worlds that converge under constraints

Status:

- not started

## Phase J. Richer Geometry And Space

Research stage

Outputs:

- richer surfaces and volumes
- path and visibility primitives
- stronger 3D geometry support

Definition of done:

- the language can model more than moving spheres and axis-aligned forbidden regions

Status:

- not started

## Phase K. Comparative Evaluation

Research stage

Outputs:

- specification complexity metrics
- cognitive load hypotheses
- declarative density measures
- comparisons against imperative baselines
- specification-size comparisons
- qualitative evaluation or user-study planning

Definition of done:

- the project has evidence for why this model is better than an imperative alternative for its target tasks

## Near-Term Milestone

Within six months, achieve:

Declaratively simulate a bouncing sphere on a floor with no user-written `update` logic.
