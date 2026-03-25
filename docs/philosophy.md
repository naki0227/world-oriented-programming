# Philosophy

## Core Claim

Programming should not be limited to writing step-by-step procedures.
For many classes of problems, especially geometry, simulation, games, robotics, and mathematical modeling, humans think in terms of worlds, relations, constraints, and evolving structure.

`sekai` is an attempt to make that mode of thought executable without forcing it to collapse into manual update logic.

## Design Principles

### 1. Write Worlds, Not Instructions

The primary act of programming is world definition.
Users describe entities, space, motion, relations, and constraints.
They should not be required to manually encode frame-by-frame state transitions.

### 2. No Central `update` Loop In User Space

The user should not need to write a conventional `update` loop to keep the world alive.
Time evolution belongs to the runtime and world model.

### 3. Continuous Time Is Fundamental

The model treats time as a built-in axis of the world, not as an afterthought layered over sequential instructions.

### 4. Global Asynchrony, Local Synchrony

Independent entities may progress without forced global synchronization.
Only interactions, observations, and coupling points require synchronized interpretation.

### 5. Constraints Are World Laws

Forbidden regions, collision responses, velocity limits, and impossibility conditions should be expressed as constraints that define valid worlds.

### 6. Spatial Concepts Are First-Class

Points, vectors, surfaces, regions, visibility, contact, trajectories, and rigid bodies are not library-level conveniences.
They are part of the core language model.

### 7. Diagrams, Logic, and Equations Must Converge

The same world should be accessible through visual placement, symbolic formulas, logical constraints, and runtime behavior.

### 8. Human Thought Admits Partiality

The language should allow partially specified worlds and candidate behaviors, then narrow them through constraints rather than requiring premature procedural commitment.

## One-Page Language Philosophy

`sekai` is a world-oriented programming language.
It treats space, time, relation, and constraint as first-class concepts.
Programs do not primarily describe how to update state.
They describe what exists, how it may evolve, and what must remain true.
Execution is the time development of a constrained world.
