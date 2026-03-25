# Related Work Draft

## Positioning

World-Oriented Programming overlaps with several existing traditions, but is not identical to any of them.
Its nearest neighbors appear in at least five areas:

1. constraint and logic programming
2. functional reactive programming
3. equation-based systems modeling
4. actor and asynchronous computation models
5. dynamic geometry and visual mathematics systems

The current prototype should therefore be presented not as a replacement for all of these traditions, but as an attempt to combine concerns that are usually separated across them.

## Constraint And Logic Programming

Constraint Logic Programming (CLP) is a major antecedent because it demonstrates how constraints can serve as a declarative basis for computation rather than as after-the-fact validation logic.
Jaffar and Lassez describe CLP as a family of highly declarative languages in which reasoning is performed directly over constrained domains rather than by encoding everything into primitive symbolic structure.
This is closely aligned with the present project's desire to express admissible world states and invalid world developments directly.

However, the focus is different.
Classic CLP is primarily concerned with symbolic reasoning and constraint solving over chosen domains.
By contrast, the current project is centered on executable spatial-temporal worlds.
Its core challenge is not only solving constraints, but integrating constraints with continuous or quasi-continuous time, observation semantics, and interaction in a shared world model.

This makes CLP a conceptual foundation for the constraint side of the language, but not a full model for world execution.

## Functional Reactive Programming

Functional Reactive Programming (FRP) is a key related paradigm because it treats time-varying behavior and event-based change as first-class programming concerns.
Hudak characterizes FRP around continuous time-varying behaviors and event-based reactivity, which makes it one of the clearest precedents for treating time as a core semantic dimension rather than as an artifact of an explicit loop.

The present work shares this interest in temporal structure, but differs in emphasis.
FRP is principally a semantic and programming model for reactive behavior over time.
World-Oriented Programming, as proposed here, is more explicitly concerned with space, object interaction, and constraints over admissible world development.
In other words, FRP is highly relevant for the time axis, but does not by itself provide a first-class spatial world model or a direct notion of world laws as constraints over geometric entities.

## Equation-Based And Physical Modeling Languages

Modelica is one of the most important related systems for this research.
Its language specification presents it as an object-oriented language for modeling complex heterogeneous systems using differential, algebraic, and discrete equations, and explicitly emphasizes that no variable must be manually chosen as the one to solve for.
This makes Modelica especially important because it already moves away from imperative update logic toward model-based execution.

The difference is again one of target representation and scope.
Modelica is designed primarily for engineering-oriented multi-domain physical system modeling.
The present project instead aims toward a more general executable world description framework that treats spatial intuition, interaction, and eventually diagrammatic reasoning as part of the programming model itself.
Modelica is therefore both a close relative and a strong comparison point, especially for discussions of declarative dynamics and model execution without manual update loops.

## Actor And Asynchronous Computation

The actor model is important background for the asynchronous dimension of this work.
Hewitt, Bishop, and Steiger's actor formalism provides an early and influential account of decentralized computation through independently progressing units that interact through message-like coordination rather than through a single global sequential control structure.

The current project shares the desire to avoid unnecessary global synchronization.
However, it is not primarily a message-passing model.
Its goal is to represent a shared evolving world in which synchronization is required only at semantically meaningful interaction boundaries, such as collision.
The actor model therefore informs the asynchrony side of the language, but the proposed system remains grounded in a common world state model rather than purely in communicating computational agents.

## Dynamic Geometry And Visual Mathematics Systems

Dynamic mathematics systems such as GeoGebra are relevant because they already demonstrate a practical union of visual construction and mathematical relation.
GeoGebra describes itself as dynamic mathematics software that brings together geometry, algebra, graphing, statistics, and calculus in one engine.
This is especially relevant to the long-term goal of connecting diagrams, formulas, and executable semantics through a shared underlying representation.

At the same time, dynamic geometry systems are not general executable world-description languages.
They excel at linked visual and mathematical exploration, but they are not usually designed as general-purpose spatial-temporal programming systems with explicit synchronization, declarative collision rules, or generalized world constraints.
They are therefore best viewed as an important precedent for the visual-logical side of the project.

## Graphical Dataflow Systems

Graphical dataflow systems such as LabVIEW are also relevant because they reduce the dominance of textual control flow and make execution depend on availability of data rather than on sequential instruction order.
NI's LabVIEW documentation explicitly contrasts dataflow execution with conventional control flow, explaining that nodes execute when required input data is available and that the flow of data determines execution order.

This is related to the current project in two ways.
First, it reinforces the idea that not all executable meaning must be expressed as sequential text.
Second, it shows a practical path toward execution models with explicit parallel structure.
Still, LabVIEW remains a dataflow language rather than a world-description language.
It does not treat space, object interaction, or continuous-time world constraints as first-class semantic objects.

## Summary Of Distinction

The clearest claim the paper can make is therefore not that World-Oriented Programming is unprecedented in every dimension.
Rather, its novelty lies in the attempted integration of:

- first-class spatial entities
- temporal world evolution without user-authored update loops
- constraint-first admissibility
- local synchronization at interaction points
- future visual-logical unification around a shared world model

This is the level at which the present work most clearly diverges from the prior traditions above.
