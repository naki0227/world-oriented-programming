# Positioning Matrix

Last updated: 2026-04-18

## Purpose

This note defines the nearest-neighbor landscape for `sekai` and World-Oriented Programming.
Its job is not to claim that every part of the project is unprecedented.
Its job is to make the specific synthesis legible.

The working thesis is:

> `sekai` is an executable world-description language in which spatial entities, world laws, event-frontier time, contradiction, local synchronization, and underdetermined continuation share one semantic model.

## Comparison Axes

| Axis | Meaning in `sekai` |
| --- | --- |
| First-class space | spatial entities and geometric relations are language-level objects |
| Event-frontier time | execution is organized around observation and event frontiers rather than user-authored frame loops |
| Laws as admissibility | constraints describe valid world development, not only checks inside procedures |
| Contradiction as result | invalid development is reported as a world-level semantic outcome |
| Local synchronization | interacting entities synchronize only when an event or dependency requires it |
| Underdetermined continuation | multiple admissible continuations can remain unresolved or converge under laws and preferences |
| Diagram-to-logic continuity | visual editing, generated laws, execution, and reports point to the same world model |

## Matrix

| System family | Overlap | Main difference from `sekai` |
| --- | --- | --- |
| Modelica / equation-based modeling | Strong precedent for declarative model execution and avoiding manual solve order. | Primarily equation-based engineering modeling; does not center contradiction reports, local synchronization scope, underdetermined continuation, or diagram-to-law execution as one language story. |
| Simulink / block simulation | Strong precedent for visual modeling and simulation. | Execution is usually block/dataflow oriented; spatial world entities and admissibility laws are not the central semantic objects. |
| Functional Reactive Programming | Strong precedent for first-class time, behaviors, and events. | FRP focuses on time-varying values and reactive composition; `sekai` focuses on spatial worlds, world laws, event-frontier observation, and contradiction. |
| Constraint Logic Programming | Strong precedent for constraints as computational content. | CLP is primarily symbolic/domain reasoning; `sekai` asks how constraints govern evolving spatial-temporal worlds and observations. |
| Game engines / ECS | Strong precedent for entities, worlds, physics, and interaction. | World rules are often encoded in systems, callbacks, scripts, or engine APIs; `sekai` aims to make laws and admissibility direct source-level constructs. |
| Physics engines | Strong precedent for collision, integration, and spatial dynamics. | Physics engines solve a domain problem; `sekai` is a language experiment about expressing world laws, contradiction, observation, and candidate continuation. |
| Actor systems | Strong precedent for independent progress and local communication. | Actors coordinate through messages; `sekai` keeps a shared world semantics with synchronization only at semantically relevant world events. |
| GeoGebra / dynamic geometry | Strong precedent for linked visual and mathematical objects. | Dynamic geometry systems are not usually general executable world languages with event-frontier time, admissibility laws, or local synchronization. |
| LabVIEW / graphical dataflow | Strong precedent for non-textual execution and data-dependent ordering. | LabVIEW centers dataflow between nodes; `sekai` centers spatial-temporal world state and laws over admissible development. |
| Constraint-based animation | Strong precedent for declarative spatial constraints and motion. | Usually focused on animation solving; `sekai` tries to integrate constraints with language-level events, contradiction reports, local synchronization, and observation semantics. |

## Strongest Differentiation Claim

The strongest claim is not that `sekai` invented declarative modeling, constraints, time, geometry, or visual programming.
Each has deep prior art.

The claim is that these concerns can be organized around a single executable object:

`world + laws + event frontiers + observation + contradiction + convergence`

That is the point to defend in the language design, implementation, viewer, and evaluation.

## Hostile Questions To Be Ready For

### Is this just Modelica with different syntax?

No, if the paper demonstrates contradiction as a semantic outcome, local synchronization scope, visibility-conditioned candidate convergence, and viewer-level law inspection.
Yes, or too close, if the paper only shows declarative equations or update-free motion.

### Is this just a game engine script with declarative sugar?

No, if laws are inspectable source-level objects with stable report identities and semantic categories.
Yes, or too close, if all interesting behavior is still hidden in built-in runtime procedures with no law-level explanation.

### Is this just FRP for geometry?

No, if admissibility, contradiction, and synchronization are central.
Yes, or too close, if time-varying positions are the only real semantic idea.

### Is this just CLP over spatial predicates?

No, if event-frontier time, observation, and local interactions are essential.
Yes, or too close, if the system becomes only a finite constraint selector.

## Implications For The Next Paper

- Do not sell the project as unprecedented in every part.
- Emphasize the integration boundary: what is one semantic object in `sekai` but split across tools elsewhere.
- Use one flagship scenario that requires at least four axes from the matrix.
- Make every related-work paragraph answer a concrete comparison rather than naming a tradition in passing.

