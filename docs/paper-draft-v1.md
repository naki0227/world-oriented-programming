# World-Oriented Programming:
# A Programming Paradigm for Executable World Description Through Space, Time, and Constraints

## Abstract

This paper proposes World-Oriented Programming, a programming paradigm in which executable systems are described as worlds rather than as instruction sequences.
The proposal is motivated by the mismatch between human spatial-temporal reasoning and conventional procedural programming, especially in domains such as simulation, geometry, and interactive environments.
In the proposed model, entities, spatial relations, temporal development, and logical constraints are represented directly, while execution is handled as the evolution of a constrained world over time.
The current prototype combines declarative world description, continuous-time progression, constraint-first validity handling, and local synchronization at interaction points.
Three executable scenarios are used as initial validation targets: a bouncing-sphere world, a forbidden-region constraint world, and a two-body collision world.
These scenarios show that simple spatial-temporal systems can already be expressed without explicit user-authored update loops, that contradictions can be described as violations of world laws, and that independently evolving entities can synchronize only when interaction occurs.
The paper argues that this direction is promising as a basis for future programming systems that unify spatial intuition, logical structure, and execution.

Author note: the current draft is being shaped toward an Onward!-style submission, and should therefore emphasize conceptual clarity, representational originality, and honest prototype scope rather than overclaiming mature empirical validation.

## 1. Introduction

Programming languages are typically organized around procedures, instruction order, and explicit state updates.
This structure has been highly successful for many classes of software, but it is often poorly aligned with problems that humans naturally understand through space, relation, time, and constraint.
In areas such as geometry, simulation, game worlds, robotics, and interactive modeling, developers frequently begin with a mental image of a world and its laws, but are forced to translate that image into control flow, frame loops, and mutable update logic.

This paper starts from the claim that the translation cost is not merely an implementation inconvenience.
It reflects a representational mismatch between how humans often reason and how mainstream programming systems demand that executable meaning be expressed.
When the target problem is fundamentally about what exists, how it moves, and what relations or restrictions must hold, a step-by-step instruction sequence may not be the most natural primary representation.

We therefore explore an alternative paradigm: World-Oriented Programming.
In this model, programs are treated as executable world descriptions.
The programmer specifies entities, spatial structure, temporal development, and logical constraints directly.
Execution is then understood not as running an instruction list, but as evolving a world state over time while preserving the declared laws of that world.

The long-term vision is broader than a small DSL.
The intended direction is a computational environment in which diagrams, formulas, logical constraints, and executable behavior all refer to the same underlying world model.
However, the contribution of this paper is deliberately narrower and more concrete.
We present the conceptual model, describe a first prototype, and evaluate whether three central ideas are already executable in a small but functioning system:

1. world evolution without a user-authored update loop
2. constraints as world laws rather than imperative exception handlers
3. local synchronization only at interaction points

The rest of the paper is organized as follows.
Section 2 positions the work against related paradigms.
Section 3 describes the core model of World-Oriented Programming.
Section 4 presents the prototype architecture.
Section 5 evaluates the prototype through three executable scenarios.
Section 6 discusses limitations and future directions.

## 2. Related Work

World-Oriented Programming is adjacent to several well-established traditions, but is not reducible to any one of them.
Constraint Logic Programming provides an important conceptual precedent because it shows how constraints can act as a declarative basis for computation rather than as after-the-fact validation logic (Jaffar & Lassez, 1987).
This is closely aligned with the present goal of expressing admissible and inadmissible world developments directly.
However, classic CLP is not by itself a model of executable spatial-temporal worlds.

Functional Reactive Programming is an equally important precedent on the temporal axis (Hudak, 1999; Elliott & Hudak, 1997).
FRP treats time-varying behavior and event-based reactivity as first-class concepts, making it highly relevant to the present rejection of explicit user-authored update loops.
Yet FRP does not, by itself, establish a first-class spatial world model with explicit collision and admissibility laws over geometric entities.

Modelica is perhaps the closest large-scale modeling relative of the current work (Modelica Association, 2023).
It demonstrates that complex dynamic systems can be described through equations and executed without manually specifying variable-solving order.
That makes it a major comparison point for declarative dynamics.
The present project differs in its stronger emphasis on spatial world description, interaction semantics, and future visual-logical integration.

The actor model is relevant for the asynchronous dimension of the proposal (Hewitt et al., 1973).
It provides a classic account of decentralized progression without global sequential control.
The current work adopts a similar instinct against unnecessary global synchronization, but remains oriented toward a shared evolving world rather than toward purely communicating agents.

Finally, systems such as GeoGebra and LabVIEW show that nontraditional representations can already be valuable in practice (GeoGebra, n.d.; National Instruments, 2026).
GeoGebra demonstrates the power of tightly linking visual and mathematical structure, while LabVIEW demonstrates that executable meaning need not be organized around plain textual control flow.
Neither system, however, offers the same combination of first-class space, temporal world evolution, constraint-first admissibility, and local synchronization that the present work aims to unify.

## 3. World-Oriented Programming Model

### 3.1 Programs As World Descriptions

The primary conceptual shift is to treat a program as a world definition rather than as a procedure.
In the proposed model, the user describes:

- what entities exist
- where they are situated
- how they may move or evolve
- what constraints or laws must remain true
- when and how interactions should be interpreted

This orientation differs from imperative programming, where the dominant question is usually how to update state from one step to the next.
World-Oriented Programming instead emphasizes what the world is and how valid world development should be constrained.

### 3.2 First-Class Space, Time, And Constraint

The model assumes that spatial and temporal structure should not be treated merely as library-level conventions.
Even in the current small prototype, the core objects of expression are not generic records and loops, but entities such as spheres, planes, and regions, together with motion descriptors and constraints.

Time is not expressed through user-authored repeated updates.
Instead, the runtime advances the world in response to observation and interaction demands.
This creates a practical separation between the programmer's description of a world and the runtime's responsibility for temporal evolution.

Constraints are treated as laws of admissible world development.
For example, a sphere may be forbidden from entering a region, or a collision may be required to reflect a trajectory.
This framing matters because it shifts exceptional or boundary behavior from imperative patch logic into declarative world definition.

### 3.3 Global Asynchrony And Local Synchronization

The execution model aims to support globally asynchronous development with local synchronization at interaction points.
The intuition is that independent entities do not need to be semantically synchronized at all times.
They must only be brought into a shared interaction state when the meaning of the world requires it, such as at collision time.

In the current prototype, this idea appears in two ways.
First, worlds are observed through snapshots requested at particular times rather than by exposing a frame loop.
Second, multi-object interaction is resolved only when an explicit elastic-collision constraint becomes relevant.
This is still a simplified form of the broader model, but it is sufficient to make the synchronization idea executable.

## 4. Prototype Architecture

The prototype consists of a small textual DSL, a runtime core, structured snapshot export, and a figure-generation workflow.

The DSL is implemented in `sekai`, while the runtime core is implemented in `orbis`.
Programs define entities and properties such as position, velocity, radius, and collision behavior.
The current runtime supports spheres, a static plane, an axis-aligned region model, velocity limits, forbidden-region constraints, and pairwise elastic collision constraints.

Execution proceeds by constructing a world from declarations, advancing the relevant state to requested observation times, and resolving interaction events when they become relevant.
The user does not write an explicit frame loop.
Instead, the runtime determines when world advancement and synchronization are required.

Snapshots can be exported as JSON.
This serves two purposes.
First, it creates a stable structured interface for later viewers and analysis tools.
Second, it makes it possible to generate paper-ready static figures directly from executable examples.

## 5. Prototype Evaluation

The current evaluation does not attempt to claim large-scale superiority over existing programming models.
Instead, it asks whether the proposed paradigm is already coherent and executable in a narrow but meaningful prototype.
Three evaluation questions guide this initial study:

1. Can a minimal world evolve without a user-authored update loop?
2. Can constraints be expressed as world laws rather than imperative exception handlers?
3. Can independent entities progress separately while synchronizing only at interaction points?

### 5.1 Minimal Declarative Evolution

The first executable scenario is a bouncing sphere described in `examples/bounce.sk`.
The program declares a sphere, a floor plane, initial position and velocity, and a reflective collision law.
No user-authored update function is present.

Figure 1 shows the resulting snapshot sequence.
The sphere first moves downward and then reflects at floor contact, reversing its vertical velocity while maintaining the horizontal component.
This is a small example, but it directly supports the claim that an executable world can be specified without exposing a frame-loop-centered programming style to the user.

Figure 1. Declarative bouncing-sphere scenario in `sekai`. A single sphere evolves without any user-authored update loop. The panels show snapshots at `t=0.000`, `t=1.000`, `t=3.000`, and `t=4.000`; the downward trajectory is reflected at the floor, after which the vertical velocity reverses while the horizontal component is preserved.

### 5.2 Constraint-First Contradiction Handling

The second scenario is a forbidden-region world described in `examples/forbidden_region.sk`.
In this case, the user does not write guard code to prevent illegal movement.
Instead, the world definition states that a sphere may not enter a specified region.

When the sphere trajectory reaches that region, the runtime reports the violation as a contradiction in world development.
This matters conceptually because it shows how a world-oriented system can treat invalid evolution as a property of the modeled world rather than as an imperative error-handling branch surrounding each update step.

Although this example is currently simpler than the collision scenarios, it already demonstrates a distinct semantic advantage of the paradigm: admissibility can be described declaratively.

### 5.3 Local Synchronization

The third scenario is a two-body collision world described in `examples/two_body_collision.sk`.
Two spheres move independently until a declared elastic-collision rule becomes relevant.
Before contact, the spheres follow separate trajectories.
At contact, a local synchronization event is resolved, after which the spheres continue with exchanged velocities.

Figure 2 shows this interaction.
The figure supports the claim that synchronization can be localized to interaction points rather than enforced globally as a constant condition of execution.
This is particularly important because it suggests how larger worlds might eventually avoid unnecessary synchronization while preserving coherent interaction semantics where needed.

Figure 2. Local synchronization in a two-body collision scenario. Two spheres advance independently until contact, at which point an explicit elastic-collision constraint is applied. The panels at `t=0.000`, `t=1.000`, and `t=3.000` show that interaction is localized to the collision event, after which the spheres separate with exchanged velocities.

### 5.4 Interpretation

Taken together, the three scenarios support the current core claims of the project.
They show that declarative world description can already drive execution, that constraints can serve as laws of admissible world development, and that interaction can be organized around local synchronization rather than permanent global lockstep.

At the same time, these results should be interpreted carefully.
The prototype remains intentionally narrow.
Its significance lies less in scale than in representational alignment.
The programmer describes entities, motion, interaction, and validity conditions directly, while the runtime assumes responsibility for time development and event handling.

## 6. Limitations And Future Work

The prototype is still far from the full research vision.
Its object vocabulary is narrow, region handling is simplified, simultaneous-event semantics remain underspecified, and the visualization pipeline is based on static 2D figures rather than an interactive visual editor.

These limitations point directly to future work.
The next steps include richer geometry, more expressive interaction rules, multiple surfaces, better simultaneous-event policies, and comparative evaluation against imperative baseline implementations.
Beyond runtime improvements, the longer-term research goal remains the integration of diagram-driven modeling and logic extraction over a shared world representation.

Even in its current form, however, the prototype provides a useful result: the core intuition of World-Oriented Programming is not merely philosophical.
It can already be instantiated in an executable system with observable behavior, structured outputs, and paper-ready research artifacts.

## References Placeholder

See `docs/bibliography-draft.md` for the current formatted references and `docs/references-notes.md` for citation planning notes.
