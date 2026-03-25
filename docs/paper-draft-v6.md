# World-Oriented Programming:
# A Programming Paradigm for Executable World Description Through Space, Time, and Constraints

## Abstract

This paper proposes World-Oriented Programming, a programming paradigm in which executable systems are described as worlds rather than as instruction sequences.
The proposal is motivated by a persistent mismatch between human spatial-temporal reasoning and conventional programming practice, especially in domains such as simulation, geometry, interactive environments, and world-based game logic.
In the proposed model, entities, spatial relations, temporal development, and logical constraints are represented directly, while execution is treated as the evolution of a constrained world over time.

The contribution of this paper is not a claim of mature generality.
Rather, it is a claim that this representational shift is already coherent enough to be instantiated in a small executable system.
The current prototype combines declarative world description, update-free temporal progression, constraint-first validity handling, and local synchronization at interaction points.
Three executable scenarios and a minimal visual round-trip are used as worked examples: a bouncing-sphere world, a forbidden-region world, a two-body collision world, and a viewer path from diagrammatic drafting to execution.
Together they show that simple spatial-temporal systems can already be expressed without user-authored update loops, that contradictions can be modeled as violations of world laws, that independently evolving entities can synchronize only when interaction occurs, and that visual arrangement can already participate in the same execution pipeline.
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

The paper's concrete contributions are therefore:

1. a formulation of World-Oriented Programming as a paradigm centered on executable world description rather than procedure
2. a small but functioning prototype that combines declarative world description, constraint-first admissibility, and local synchronization
3. a first visual-logical interface in which diagrammatic arrangement, candidate world laws, execution, and contradiction reporting share a common report pipeline

This framing fits an Onward!-style submission, where the key question is not whether a new idea has already reached mature generality, but whether it is conceptually meaningful, well argued, and supported by enough implementation to justify taking it seriously.

## 2. Related Work

World-Oriented Programming is not unprecedented in every dimension.
Its importance lies in the attempted integration of concerns that are usually split across separate communities.

Constraint Logic Programming provides one major antecedent because it treats constraints as part of the computational core rather than as post hoc checks (Jaffar & Lassez, 1987).
This is directly relevant to the present idea that admissibility should be described as part of a world's definition.
However, classic CLP is not in itself a model of executable spatial-temporal worlds.
Its center of gravity is reasoning over constrained symbolic domains rather than the execution of shared evolving worlds.

Functional Reactive Programming provides a major precedent on the temporal axis (Hudak, 1999; Elliott & Hudak, 1997).
FRP demonstrates that time-varying behavior and event-based change can be treated as first-class semantic concerns rather than simulated by explicit loops.
The present project shares that motivation, but adds an explicit emphasis on geometric entities, interaction rules, and world constraints.
In that sense, FRP contributes an important temporal intuition but not the whole representational package this work is seeking.

Modelica is perhaps the closest existing large-scale modeling relative (Modelica Association, 2023).
It shows that dynamic systems can be described declaratively and executed without manually specifying variable-solving order.
That makes it especially important here.
The present work differs in its intended representational center: not primarily engineering system equations, but executable world descriptions that may eventually be built, inspected, and constrained through visual and logical means.

The actor model is relevant for the asynchronous part of the proposal because it offers a classic alternative to globally sequential control (Hewitt et al., 1973).
Still, the present work is not fundamentally a message-passing model.
Its concern is a shared evolving world whose synchronization burden should be local rather than global.
The commonality is therefore organizational rather than ontological.

GeoGebra and LabVIEW are useful practical precedents because both show that executable meaning can live in representations other than plain sequential text (GeoGebra, n.d.; National Instruments, 2026).
GeoGebra is relevant to the long-term visual-logical ambition of the project, while LabVIEW demonstrates that execution can be organized by structure rather than by textual control flow.
Neither, however, combines first-class spatial entities, temporal world evolution, constraint-first admissibility, and local synchronization in the way attempted here.

The novelty claim of this paper therefore should not be read as "nothing similar has ever existed."
The stronger and more defensible claim is that this work explores a new synthesis: spatial world description, temporal evolution, constraints as laws, and local synchronization brought together inside a single executable language model.

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

The prototype consists of five connected layers:

1. a small textual DSL
2. a runtime core
3. structured snapshot export
4. an interactive viewer and draft editor
5. static figure generation

The DSL is implemented in `sekai`, and the runtime core is implemented in `orbis`.
The current system supports spheres, one static plane, an axis-aligned region model, velocity limits, forbidden-region constraints, and pairwise elastic collision constraints.

Execution begins by constructing a world from declarations.
The runtime then advances the relevant state to requested observation times, while resolving interaction events only when they become semantically relevant.
Users do not write an explicit frame loop.

JSON snapshot export provides a stable interface for later tools, including viewers and analysis scripts.
Those snapshots are already sufficient to generate paper-ready static figures directly from executable examples.

The newer prototype layer is a browser-based viewer that reads the same structured reports and adds three capabilities that matter to the research argument.
First, it makes executable worlds directly inspectable through a unified spatial-temporal interface.
Second, it introduces a minimal diagram-aware editor that can propose logical candidates such as floor reflection, collision, speed bounds, and forbidden-region exclusion.
Third, it closes a first round-trip path in which a draft scene can be executed through the same runtime and immediately reloaded into the viewer.
This does not yet amount to the full envisioned diagrammatic programming environment, but it is enough to make the visual-logical ambition empirically visible.
The viewer is not presented as a polished end-user environment.
It is presented as evidence that the same world model can already support inspection, visual drafting, logical suggestion, execution, and contradiction display inside one prototype loop.

Figure 3 is inserted here: `figures/viewer-01-3d-two-body-paper.png`

Figure 4 is inserted here: `figures/viewer-02-draft-candidates-paper.png`

## 5. Prototype Evaluation

This evaluation is intentionally modest.
It is not an attempt to prove that World-Oriented Programming is already superior to mature programming paradigms.
Rather, it asks whether the paradigm is coherent enough to support a first executable system whose behavior genuinely reflects the ideas it claims to embody.

The evaluation is organized around three questions:

1. Can a minimal world evolve without a user-authored update loop?
2. Can constraints be expressed as world laws rather than imperative exception handlers?
3. Can independent entities progress separately while synchronizing only at interaction points?

### 5.1 Evaluation Setup

The prototype is evaluated through three executable scenarios written in the same DSL and executed by the same runtime:

1. `examples/bounce.sk`
2. `examples/forbidden_region.sk`
3. `examples/two_body_collision.sk`

For each scenario, the runtime produces snapshots at explicitly requested observation times.
Those snapshots are exported as JSON and rendered into static figures.
The aim of this setup is not statistical comparison, but semantic inspection: each example is chosen to expose one core claim of the paradigm in a form that can be directly executed and observed.

This example-driven method is deliberate.
At the present stage, the paper is not trying to claim benchmark superiority or broad usability.
It is trying to establish semantic coherence: whether the paradigm's core claims can survive contact with an executable artifact.
For that reason, the evaluation focuses on worked examples that each test a distinct representational promise.

### 5.2 Compact Imperative Baseline

The comparison target throughout this paper is not an optimized industrial engine, but the conventional style in which the same scenarios would be expressed through explicit update loops, manual state mutation, and distributed guard logic.
For the bounce example, that baseline would typically require a per-frame position update plus explicit collision handling.
For the forbidden-region example, it would require imperative checks around movement or collision steps.
For the two-body case, it would require manual event detection and collision response sequencing.
The present prototype does not yet prove that World-Oriented Programming is shorter or faster in every case.
It does, however, show that the center of expression shifts from step-by-step update procedure to world definition and admissibility law.

### 5.3 Minimal Declarative Evolution

The first scenario is a bouncing sphere described in `examples/bounce.sk`.
The program declares a sphere, a floor plane, initial position and velocity, and a reflective collision law.
No user-authored update function is present.

Figure 1 is inserted here: `figures/bounce-xy.png`

Figure 1 shows the resulting snapshot sequence.
The sphere first moves downward and then reflects at floor contact, reversing its vertical velocity while preserving the horizontal component.
This is a small example, but it demonstrates that world evolution can already be derived from declarations rather than from an exposed update loop.

Figure 1. Declarative bouncing-sphere scenario in `sekai`. A single sphere evolves without any user-authored update loop. The panels show snapshots at `t=0.000`, `t=1.000`, `t=3.000`, and `t=4.000`; the downward trajectory is reflected at the floor, after which the vertical velocity reverses while the horizontal component is preserved.

### 5.4 Constraint-First Contradiction Handling

The second scenario is a forbidden-region world described in `examples/forbidden_region.sk`.
The user does not write imperative guard code around each movement step.
Instead, the world definition states that a sphere may not enter a specified region.

When the sphere reaches that region, the runtime reports the violation as a contradiction in world development.
This does not yet amount to a full constraint-solving framework, but it does show that invalid evolution can already be described declaratively and detected as a property of the modeled world.

The value of this example is not geometric complexity.
Its value is semantic clarity.
It shows that exceptional behavior can be moved into the world's law structure itself rather than expressed as procedural control logic around every step of execution.

Figure 6 is inserted here: `figures/viewer-04-contradiction-report-paper.png`

The contradiction-viewer screen is especially useful because it ties together specification and outcome.
The adopted `not inside(...)` candidate is visible in the same interface that reports the resulting contradiction, making the law-like interpretation of invalid evolution easier to inspect and explain than a scattered imperative guard implementation would be.

### 5.5 Local Synchronization

The third scenario is a two-body collision world described in `examples/two_body_collision.sk`.
Two spheres move independently until an explicit elastic-collision rule becomes relevant.
Before contact, they follow separate trajectories.
At contact, a local synchronization event is resolved, after which the spheres continue with exchanged velocities.

Figure 2 is inserted here: `figures/two_body_collision-xy.png`

Figure 2 shows this behavior.
The important point is not merely that a collision occurs.
It is that synchronization is localized to the interaction boundary.
That provides an executable example of the claim that a world can remain globally non-lockstep while still producing coherent interaction where meaning requires it.

Figure 2. Local synchronization in a two-body collision scenario. Two spheres advance independently until contact, at which point an explicit elastic-collision constraint is applied. The panels at `t=0.000`, `t=1.000`, and `t=3.000` show that interaction is localized to the collision event, after which the spheres separate with exchanged velocities.

### 5.6 Visual-Logical Round-Trip

Beyond the three textual scenarios, the prototype now includes a minimal visual-logical round-trip.
A user can place spheres in the viewer, enable a floor or forbidden region, inspect automatically proposed constraints, and execute the generated draft directly through the same runtime.
The resulting report is then shown in the same interface.

Figure 5 is inserted here: `figures/viewer-03-roundtrip-result-paper.png`

This is still a small prototype feature, but it matters conceptually.
It provides the first implemented instance of the larger project thesis that diagrams, logic, and execution should not be isolated stages connected only by manual translation.
Instead, a visual arrangement can already act as a partial world description, produce candidate laws, and participate in execution.
The importance of this result is less about interface polish than about representational continuity.
The same world description can now be approached textually or visually, executed through the same runtime path, and interpreted through the same report structure.

### 5.7 What These Results Do And Do Not Show

The current prototype supports the paper's central conceptual claims.
It shows that declarative world description can drive execution, that admissibility can be modeled as a world law, that interaction can be localized rather than globally synchronized, and that a first visual-to-execution loop is already implementable.

At the same time, the prototype does not yet establish broad practical superiority.
It does not yet include a rich geometry model, a general solver, a mature visual interface, user studies, or comparative evaluation against imperative baselines.
This paper therefore should be read as a principled argument supported by a worked-out executable prototype, not as the final empirical validation of a mature programming ecosystem.

That limitation should be understood as part of the paper's genre rather than as a hidden weakness.
The work should be judged by the clarity of the problem it identifies, the coherence of the alternative it proposes, and the degree to which the prototype demonstrates that the proposal is more than a metaphor.

## 6. Discussion And Future Work

The prototype remains intentionally narrow.
Its object vocabulary is small, region handling is simplified, simultaneous-event semantics are underspecified, and the visualization pipeline is currently based on static 2D figures rather than a true interactive editor.

These limitations point directly to the next research steps.
One path is implementation-driven: richer geometry, multiple surfaces, better simultaneous-event handling, and a stronger viewer or diagrammatic interface.
Another path is evaluation-driven: comparisons against imperative baselines, analysis of specification size and clarity, and studies of whether the model actually improves representational alignment for human users.

More importantly, the prototype clarifies what is now plausible to attempt next.
The project no longer rests only on a philosophical intuition.
It now has a minimal executable language, a runtime model, figure-producing examples, and a manuscript argument that can be criticized, refined, and extended.
That is a meaningful threshold for a paradigm paper.

## 7. Current Remaining Work Before Submission

The manuscript is not yet submission-ready.
The main remaining tasks are:

1. replace placeholder figure-insertion notes with actual typeset figures
2. convert the reference list into the exact style of the target venue
3. strengthen the related-work section with a few more archival sources, especially around visual mathematics and language design
4. polish prose for concision and submission length
5. decide which of Figures 3-6 should remain in the main paper versus an appendix or talk deck

## References

Elliott, C., & Hudak, P. (1997). *Functional Reactive Animation*. In *Proceedings of the Second ACM SIGPLAN International Conference on Functional Programming (ICFP 1997)* (pp. 263-273). ACM. https://doi.org/10.1145/258948.258973

GeoGebra. (n.d.). *About GeoGebra*. https://stage.geogebra.org/about

Hewitt, C., Bishop, P. B., & Steiger, R. (1973). *A Universal Modular ACTOR Formalism for Artificial Intelligence*. In *Proceedings of the 3rd International Joint Conference on Artificial Intelligence (IJCAI 1973)*. IJCAI. https://www.ijcai.org/proceedings/1973

Hudak, P. (1999). *Functional Reactive Programming*. In S. D. Swierstra (Ed.), *Programming Languages and Systems* (ESOP 1999, LNCS 1576, p. 1). Springer. https://doi.org/10.1007/3-540-49099-X_1

Jaffar, J., & Lassez, J.-L. (1987). *Constraint Logic Programming*. In *Proceedings of the 14th ACM SIGACT-SIGPLAN Symposium on Principles of Programming Languages (POPL 1987)* (pp. 111-119). ACM. https://doi.org/10.1145/41625.41635

Modelica Association. (2023). *Modelica Language Specification, Version 3.6*. https://specification.modelica.org/maint/3.6/

National Instruments. (2026). *LabVIEW Overview*. https://www.ni.com/docs/en-IO/bundle/labview/page/labview-overview.html
