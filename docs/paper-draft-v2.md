# World-Oriented Programming:
# A Programming Paradigm for Executable World Description Through Space, Time, and Constraints

## Abstract

This paper proposes World-Oriented Programming, a programming paradigm in which executable systems are described as worlds rather than as instruction sequences.
The proposal is motivated by a persistent mismatch between human spatial-temporal reasoning and conventional programming practice, especially in domains such as simulation, geometry, interactive environments, and world-based game logic.
In the proposed model, entities, spatial relations, temporal development, and logical constraints are represented directly, while execution is treated as the evolution of a constrained world over time.

The contribution of this paper is not a claim of mature generality.
Rather, it is a claim that this representational shift is already coherent enough to be instantiated in a small executable system.
The current prototype combines declarative world description, update-free temporal progression, constraint-first validity handling, and local synchronization at interaction points.
Three executable scenarios are used as worked examples: a bouncing-sphere world, a forbidden-region world, and a two-body collision world.
Together they show that simple spatial-temporal systems can already be expressed without user-authored update loops, that contradictions can be modeled as violations of world laws, and that independently evolving entities can synchronize only when interaction occurs.
The paper argues that this direction is promising as a basis for future programming systems that unify spatial intuition, logical structure, diagrams, and execution.

## 1. Introduction

Programming languages are usually organized around procedures, instruction order, and explicit state updates.
This design has proved effective for a vast range of software, but it sits awkwardly with problem domains that humans understand primarily in terms of space, relation, motion, and constraint.
When developers build simulations, game worlds, geometric systems, or interactive spatial models, they often begin from a mental picture of a world and its laws.
Yet mainstream implementations typically force that picture into control flow, frame loops, and manual state mutation.

This paper begins from the claim that this is not only an ergonomic inconvenience.
It is a representational mismatch.
In many settings, the programmer's real intention is not best described as a procedure.
It is better described as a world: what exists, how it may evolve, what interactions matter, and what must never become true.

We therefore explore an alternative paradigm: World-Oriented Programming.
In this model, a program is an executable world description.
The programmer specifies entities, spatial structure, temporal development, and logical constraints directly.
Execution is understood not as running an instruction list, but as evolving a world while preserving the laws that define admissible development.

The long-term vision is ambitious.
Ultimately, the project aims toward a computational environment in which diagrams, formulas, logical constraints, and execution all refer to the same underlying world model.
This paper, however, makes a narrower contribution.
It presents the paradigm, gives a first prototype, and asks whether three central ideas are already executable in a nontrivial but still deliberately small system:

1. worlds can evolve without user-authored update loops
2. constraints can be expressed as world laws rather than imperative exception handlers
3. interaction can be localized to synchronization points rather than imposed globally

This framing makes the paper a better fit for a venue such as Onward!, where the key question is not whether a new idea has already reached mature generality, but whether it is conceptually meaningful, well argued, and supported by enough implementation to justify taking it seriously.

## 2. Related Work

World-Oriented Programming is not unprecedented in every dimension.
Its importance lies in the attempted integration of concerns that are usually split across separate communities.

Constraint Logic Programming provides one major antecedent because it treats constraints as part of the computational core rather than as post hoc checks (Jaffar & Lassez, 1987).
This is directly relevant to the present idea that admissibility should be described as part of a world's definition.
However, classic CLP is not in itself a model of executable spatial-temporal worlds.

Functional Reactive Programming provides a major precedent on the temporal axis (Hudak, 1999; Elliott & Hudak, 1997).
FRP demonstrates that time-varying behavior and event-based change can be treated as first-class semantic concerns rather than simulated by explicit loops.
The present project shares that motivation, but adds an explicit emphasis on geometric entities, interaction rules, and world constraints.

Modelica is perhaps the closest existing large-scale modeling relative (Modelica Association, 2023).
It shows that dynamic systems can be described declaratively and executed without manually specifying variable-solving order.
That makes it especially important here.
The present work differs in its intended representational center: not primarily engineering system equations, but executable world descriptions that may eventually be built, inspected, and constrained through visual and logical means.

The actor model is relevant for the asynchronous part of the proposal because it offers a classic alternative to globally sequential control (Hewitt et al., 1973).
Still, the present work is not fundamentally a message-passing model.
Its concern is a shared evolving world whose synchronization burden should be local rather than global.

GeoGebra and LabVIEW are useful practical precedents because both show that executable meaning can live in representations other than plain sequential text (GeoGebra, n.d.; National Instruments, 2026).
GeoGebra is relevant to the long-term visual-logical ambition of the project, while LabVIEW demonstrates that execution can be organized by structure rather than by textual control flow.
Neither, however, combines first-class spatial entities, temporal world evolution, constraint-first admissibility, and local synchronization in the way attempted here.

## 3. World-Oriented Programming Model

### 3.1 Programs As World Descriptions

The central shift is to treat a program as a world definition rather than as a procedure.
In the proposed model, the programmer describes:

- what entities exist
- how they are situated in space
- how they may evolve in time
- what constraints define admissible world development
- which interactions require shared interpretation

This changes the main question of programming from "how do I update the state?" to "what kind of world is being defined?"

### 3.2 First-Class Space, Time, And Constraint

The model assumes that space, time, and constraint should be part of the semantic core rather than merely library conventions.
Even in the current prototype, the primary language objects are spheres, planes, regions, motion descriptors, and collision or admissibility constraints.

Time is not authored through explicit repeated updates.
Instead, the runtime advances the world in response to observation and interaction demands.
This does not eliminate implementation complexity, but it moves temporal bookkeeping from user code into the execution model itself.

Constraints are not framed as secondary checks.
They are treated as laws of admissible world development.
This distinction is philosophically important and practically useful: it makes the boundary between valid and invalid evolution part of the world definition rather than part of imperative repair code.

### 3.3 Global Asynchrony And Local Synchronization

The execution model aims for global asynchrony with local synchronization.
Independent entities should not need to remain in permanent global lockstep.
They only need synchronized interpretation when the world's semantics require it.

In the current prototype, this idea appears in a simple but meaningful form.
Worlds are observed through snapshots at requested times, while multi-object interaction is only resolved when a declared interaction constraint becomes relevant.
This is still a narrow prototype formulation, but it is enough to make local synchronization an executable claim rather than a purely philosophical one.

## 4. Prototype Architecture

The prototype consists of four connected layers:

1. a small textual DSL
2. a runtime core
3. structured snapshot export
4. static figure generation

The DSL is implemented in `sekai`, and the runtime core is implemented in `orbis`.
The current system supports spheres, one static plane, an axis-aligned region model, velocity limits, forbidden-region constraints, and pairwise elastic collision constraints.

Execution begins by constructing a world from declarations.
The runtime then advances the relevant state to requested observation times, while resolving interaction events only when they become semantically relevant.
Users do not write an explicit frame loop.

JSON snapshot export provides a stable interface for later tools, including viewers and analysis scripts.
Those snapshots are already sufficient to generate paper-ready static figures directly from executable examples.

## 5. Prototype Evaluation

This evaluation is intentionally modest.
It is not an attempt to prove that World-Oriented Programming is already superior to mature programming paradigms.
Rather, it asks whether the paradigm is coherent enough to support a first executable system whose behavior genuinely reflects the ideas it claims to embody.

The evaluation is organized around three questions:

1. Can a minimal world evolve without a user-authored update loop?
2. Can constraints be expressed as world laws rather than imperative exception handlers?
3. Can independent entities progress separately while synchronizing only at interaction points?

### 5.1 Minimal Declarative Evolution

The first scenario is a bouncing sphere described in `examples/bounce.sk`.
The program declares a sphere, a floor plane, initial position and velocity, and a reflective collision law.
No user-authored update function is present.

Figure 1 shows the resulting snapshot sequence.
The sphere first moves downward and then reflects at floor contact, reversing its vertical velocity while preserving the horizontal component.
This is a small example, but it demonstrates that world evolution can already be derived from declarations rather than from an exposed update loop.

Figure 1. Declarative bouncing-sphere scenario in `sekai`. A single sphere evolves without any user-authored update loop. The panels show snapshots at `t=0.000`, `t=1.000`, `t=3.000`, and `t=4.000`; the downward trajectory is reflected at the floor, after which the vertical velocity reverses while the horizontal component is preserved.

### 5.2 Constraint-First Contradiction Handling

The second scenario is a forbidden-region world described in `examples/forbidden_region.sk`.
The user does not write imperative guard code around each movement step.
Instead, the world definition states that a sphere may not enter a specified region.

When the sphere reaches that region, the runtime reports the violation as a contradiction in world development.
This does not yet amount to a full constraint-solving framework, but it does show that invalid evolution can already be described declaratively and detected as a property of the modeled world.

### 5.3 Local Synchronization

The third scenario is a two-body collision world described in `examples/two_body_collision.sk`.
Two spheres move independently until an explicit elastic-collision rule becomes relevant.
Before contact, they follow separate trajectories.
At contact, a local synchronization event is resolved, after which the spheres continue with exchanged velocities.

Figure 2 shows this behavior.
The important point is not merely that a collision occurs.
It is that synchronization is localized to the interaction boundary.
That provides an executable example of the claim that a world can remain globally non-lockstep while still producing coherent interaction where meaning requires it.

Figure 2. Local synchronization in a two-body collision scenario. Two spheres advance independently until contact, at which point an explicit elastic-collision constraint is applied. The panels at `t=0.000`, `t=1.000`, and `t=3.000` show that interaction is localized to the collision event, after which the spheres separate with exchanged velocities.

### 5.4 What These Results Do And Do Not Show

The current prototype supports the paper's central conceptual claims.
It shows that declarative world description can drive execution, that admissibility can be modeled as a world law, and that interaction can be localized rather than globally synchronized.

At the same time, the prototype does not yet establish broad practical superiority.
It does not yet include a rich geometry model, a general solver, a mature visual interface, or comparative evaluation against imperative baselines.
This paper therefore should be read as a principled argument supported by a worked-out executable prototype, not as the final empirical validation of a mature programming ecosystem.

## 6. Limitations And Future Work

The prototype remains intentionally narrow.
Its object vocabulary is small, region handling is simplified, simultaneous-event semantics are underspecified, and the visualization pipeline is currently based on static 2D figures rather than a true interactive editor.

These limitations point directly to the next research steps.
One path is implementation-driven: richer geometry, multiple surfaces, better simultaneous-event handling, and a stronger viewer or diagrammatic interface.
Another path is evaluation-driven: comparisons against imperative baselines, analysis of specification size and clarity, and studies of whether the model actually improves representational alignment for human users.

The larger research promise remains the same.
If programs can be written as executable worlds, then diagrams, formulas, constraints, and runtime behavior may eventually become views over one shared structure rather than separate artifacts stitched together by translation.

## References Placeholder

See `docs/bibliography-draft.md` for the current formatted references and `docs/references-notes.md` for citation planning notes.
