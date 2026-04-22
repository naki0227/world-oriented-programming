# Roadmap

## Goal

Build a world-description language driven by 3D space, continuous time, and logical constraints.

## Long-Term Vision

The long-term goal is not to replace perception AI with one larger model.
It is to build a world-semantic runtime that can take imperfect perceptual outputs and turn them into safe, explainable, executable action decisions.

In that architecture:

- perception models interpret camera, depth, audio, map, or other sensor inputs
- those observations are translated into a world-oriented representation
- constraints, candidate actions, uncertainty, contradiction, and deferred resolution are handled by the language/runtime layer
- execution outputs are then produced as guidance, control, or robot actions

This means the system should not require a full AI deliberation pass for every small decision update.
Once observations have been grounded into the world model, safety and action selection should be enforced by semantic rules, explicit constraints, and incremental state updates.

That separation is one of the core intended advantages of the project:

- lower latency through incremental updates instead of repeated end-to-end inference
- stronger safety guarantees through explicit hard constraints
- clearer correctness conditions because actions are tied to world state and law evaluation
- better explainability because stops, detours, contradictions, and deferred choices remain part of the runtime record

One high-value future application domain is assistive physical AI, including navigation support for visually impaired users.
That is not the current claim of the prototype, but it is an important long-term target because it stresses exactly the properties this research aims to provide: partial observation, safety-critical action selection, uncertainty management, and actionable explanations.

## Language Positioning

`sekai` is not currently intended to become a general-purpose programming language in the same sense as Python, Rust, or JavaScript.
The stronger direction is to make it a first-class language for world modeling, constraint-governed execution, observation, and action selection.

In practice, that means:

- general-purpose host languages can still handle surrounding systems, UI, storage, networking, and device integration
- `sekai` should own the world description, law evaluation, contradiction handling, deferred resolution, and execution-facing decision layer
- the project should optimize for semantic clarity, runtime accountability, and executable world structure rather than for arbitrary application programming

The goal is therefore not "replace ordinary programming languages."
The goal is "make world-constrained reasoning and execution programmable as a language-level activity."

## Semantic Focus

The research should not be framed as merely integrating existing modules.
If `sekai` is only described as "perception + planner + safety + logging in one place," the contribution will sound shallow.

The stronger claim is that `sekai` makes certain semantic objects first-class and executable in one shared model.
These are not only implementation concerns; they are the real subject matter of the language.

Primary first-class semantic objects:

- world frontiers where observation, enforcement, convergence, and contradiction have precise meaning
- explicit laws with outcomes such as idle, fired, repaired, and contradicted
- underdetermined candidate continuations rather than only one already-collapsed action
- deferred decisions that remain valid runtime states instead of being treated as temporary failure
- observation statuses such as determinate, representative, and unresolved
- convergence histories that explain how a world moved from ambiguity toward a selected or still-open continuation
- contradiction as a semantic result of world evolution, not only as an implementation error

Why this is stronger than "system integration":

- the same source-level objects should appear in the runtime, report, viewer, and evaluation
- uncertainty is not only a numeric confidence value; it can remain structurally unresolved in the world model
- the language can represent "wait", "defer", "contradict", and "ask for confirmation" as world-execution outcomes
- the research claim is about executable semantics for incomplete worlds, not only about plumbing modules together

This gives the project a sharper novelty target:

- weak framing: integrate fragmented robotics/AI components
- stronger framing: unify fragmented semantic concerns under one executable world model
- strongest framing: make underdetermination, observation frontier, contradiction, and convergence language-level objects that existing stacks usually leave implicit or scatter across subsystems

Related note:

- `docs/first-class-semantics-comparison.md`
- `docs/semantic-first-class-audit.md`

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
- Phase I: in progress
- Phase J: in progress
- Phase K: in progress

Latest milestone:

- the flagship visibility-coordination scenario is now executable, tested, viewable, and figure-ready
- the viewer can play the flagship smoothly while preserving discrete semantic observation frontiers
- the paper now has a flagship convergence figure showing `unresolved -> unresolved -> determinate`

The next research-hardening plan is tracked in:

- `docs/research-strategy-and-task-plan.md`

That plan should guide near-term work whenever implementation, paper writing, and evaluation priorities compete.

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
- Phase K should evaluate not only behavior, but also representational structure. Cognitive-load claims remain hypotheses unless they are studied directly.

## Capabilities By Stage

This section states the roadmap in terms of what a user or researcher should actually be able to do at each stage.

### Stage A. Executable World Description

What becomes possible:

- describe a small physical world declaratively in `.sk`
- run it without writing a manual update loop
- inspect stable snapshots of motion and interaction
- detect simple contradictions from explicit laws

Representative use:

- bouncing objects, collisions, forbidden regions, and simple constrained motion

Current status:

- already possible in prototype form

Next build tasks:

- expand the executable example set beyond core motion and collision cases
- keep the CLI/report surface stable enough for repeated paper and benchmark use
- strengthen regression coverage for baseline physical laws and contradiction cases

### Stage B. Constraint-Aware Scenario Authoring

What becomes possible:

- write reusable constrained scenarios directly in the DSL
- attach explicit hard laws and repair policies
- produce structured reports instead of only raw simulation output
- inspect why a world succeeded, repaired, or contradicted

Representative use:

- research examples, teaching demos, and compact executable papers

Current status:

- largely possible now, but still being hardened

Next build tasks:

- broaden law coverage while keeping the report schema coherent
- improve source-level diagnostics so authoring errors are easier to fix
- tighten the connection between authored laws, runtime traces, and exported evidence

### Stage C. Observation-Centered Worlds

What becomes possible:

- request explicit observation frontiers instead of assuming dense continuous logging
- distinguish semantic observation from visual interpolation
- inspect world state at meaningful times for explanation and evaluation
- reason about failure or ambiguity at observation time rather than only at low-level runtime steps

Representative use:

- paper figures, viewer playback, and explanation-oriented execution traces

Current status:

- possible now in a first strong form

Next build tasks:

- refine observation-timeline reporting so frontier changes are easier to compare
- add more figure-ready and demo-ready observation scenarios
- make observation semantics easier to inspect directly from viewer and paper assets

### Stage D. Underdetermined Decision Worlds

What becomes possible:

- define candidate actions inside the world description
- separate hard admissibility from soft preference
- defer ambiguous choices and resolve them at later frontiers
- report why a branch was selected, skipped, repaired, or left unresolved

Representative use:

- visibility-conditioned pursuit, handoff, coordination, and staged convergence examples

Current status:

- possible now in an important but still early form

Next build tasks:

- expand the candidate-action model beyond the current flagship family
- add richer defer, resolve, and contradiction combinations as named scenarios
- strengthen analytics and explanation fields for branch choice and unresolved states
- compare the same decision worlds against fair host-language baselines

### Stage E. Diagram-To-Execution Authoring

What becomes possible:

- sketch or edit a world visually
- extract candidate logic from geometry
- confirm or revise proposed laws before execution
- move between diagram, DSL, report, and playback as one workflow

Representative use:

- interactive prototyping for researchers, designers, and non-expert scenario authors

Current status:

- partially possible through the current viewer, but not yet mature

Next build tasks:

- improve true 3D authoring rather than relying mainly on inspection and draft editing
- expose candidate-law suggestions with clearer rationale and approval controls
- make round-trip editing between diagram, DSL, and report less brittle

### Stage F. Narrow Practical Pilots

What becomes possible:

- connect the runtime to a bounded decision problem rather than a paper-only demo
- run low-speed, safety-first guidance logic in a limited environment
- keep explicit stop, wait, defer, and contradiction behavior in the execution layer
- integrate host-language code around `sekai` for sensors, UI, or device outputs

Representative use:

- indoor navigation prototypes, guided mobility demos, low-speed robot routing, and constrained assistive prompts

Current status:

- not yet reached; this is the first serious practicality milestone

Next build tasks:

- define one bounded pilot domain with explicit safety constraints and success criteria
- create a host-language integration layer for feeding observations into `sekai`
- add execution outputs suitable for prompts, guidance, or low-speed control loops
- validate stop/defer/contradict behavior under noisy or partial observations

### Stage G. Perception-Grounded World Runtime

What becomes possible:

- translate perception-model outputs into world entities, relations, and uncertainty markers
- update only changed parts of the world model rather than rerunning full deliberation
- let semantic constraints govern execution after perception grounding
- record explainable decision traces tied to changing observations

Representative use:

- camera or multimodal perception feeding a world-semantic runtime for real-time guidance support

Current status:

- vision only; architecture direction is clear, implementation path still ahead

Next build tasks:

- define the intermediate representation between perception outputs and world facts
- support partial-confidence observations without collapsing immediately into false precision
- implement incremental world updates driven by changed observations only
- keep decision traces explainable after perception-grounding and update cycles

### Stage H. High-Trust Physical AI Support

What becomes possible:

- support safety-critical embodied decision systems with explicit runtime accountability
- preserve hard safety constraints even under partial observation
- expose why the system proceeded, stopped, deferred, or asked for confirmation
- serve as a decision layer for assistive or robotic systems that need explainable world-grounded behavior

Representative use:

- assistive navigation for visually impaired users, high-reliability mobility aids, or other safety-critical embodied guidance systems

Current status:

- long-term target, far beyond the current prototype

Next build tasks:

- define safety cases and failure modes for assistive and embodied deployment settings
- design confirmation, stop, and fallback protocols as first-class runtime behaviors
- plan evaluation with human factors, reliability, and accountability requirements in view

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

- not started

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

- in progress
- underdetermined worlds and convergence behavior are part of the core intellectual identity of `sekai`
- flagship convergence now demonstrates staggered resolution across observation frontiers

## Phase J. Richer Geometry And Space

Research stage

Outputs:

- richer surfaces and volumes
- path and visibility primitives
- stronger 3D geometry support

Definition of done:

- the language can model more than moving spheres and axis-aligned forbidden regions

Status:

- in progress
- first executable slice is a visibility law over an occluding region
- next executable slice is a visibility-conditioned pursuit preference over candidate actions
- current slice: visibility can branch between pursuit, search, handoff, and multi-entity coordination continuations
- flagship scenario now ties visibility-conditioned preferences to staggered underdetermined-world convergence

## Phase K. Comparative Evaluation

Research stage

Outputs:

- specification complexity metrics
- representational-structure hypotheses
- world-content and mechanics density measures
- comparisons against imperative baselines
- specification-size comparisons
- qualitative evaluation or user-study planning

Definition of done:

- the project has evidence for how this model shifts representational burden compared with imperative alternatives for its target tasks

Status:

- in progress
- initial paired corpus, imperative baselines, metric script, and narrative comparisons now exist
- event-driven and library-style baselines now exist for fairer comparison
- next work should manually code the flagship scenario against the library-style baseline, because line counts alone cannot capture report integration or semantic identity

## Near-Term Milestone

Within six months, achieve:

Declaratively simulate a bouncing sphere on a floor with no user-written `update` logic.
