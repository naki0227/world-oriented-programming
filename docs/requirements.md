# Requirements v0.1

## Vision Requirement

The system should allow a user to define an executable world directly through diagrams, formulas, and logical constraints.

## Core Functional Requirements

### FR-1 World Definition

The language must represent entities, spatial structure, and temporal evolution as part of a shared world model.

### FR-2 Time Model

The runtime must support continuous or quasi-continuous time progression independent of a user-authored update loop.

### FR-3 Observation

The system must produce coherent snapshots of the world at requested observation times.

### FR-4 Spatial First-Class Support

The core model must include first-class support for:

- points
- vectors
- rigid bodies
- planes or surfaces
- regions
- motion descriptors

### FR-5 Constraint Expression

Users must be able to express:

- forbidden regions
- velocity limits
- collision behavior
- impossibility conditions

### FR-6 Constraint Handling

The runtime must detect contradictions and define how to respond to them.
Candidate policies include rejection, repair, or prioritized resolution.

### FR-7 Interaction Semantics

The runtime must support local synchronization at interaction boundaries without requiring global lockstep execution.

### FR-8 Declarative Surface Syntax

The first DSL version must support a purely declarative description style for core examples.

### FR-9 Visual-Logical Unification

The long-term interface must allow diagrams, formulas, and logical constraints to refer to the same world objects.

## Research Requirements

### RR-1 Human Thought Alignment

The project should evaluate whether world description better matches human reasoning than stepwise imperative code for selected tasks.

### RR-2 Comparative Expressiveness

The project should compare the resulting notation against imperative game loops, FRP-style models, and constraint-based systems.

### RR-3 Prototype Validation

At least one prototype scenario should demonstrate that the absence of an explicit `update` loop does not prevent expressive world behavior.

## Evaluation Criteria

The project is successful if it can show that:

1. users can define a simple world without manual update logic
2. diagrams, formulas, and constraints address the same objects
3. asynchronous progression still yields coherent observation
4. exceptions can be written as world laws
5. small games or geometric models feel more natural than in baseline imperative code
