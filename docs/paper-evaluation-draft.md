# Paper Evaluation Draft

## Section Title Candidate

Prototype Evaluation

## Section Goal

The goal of this section is not to claim large-scale empirical superiority yet.
At the current stage, the evaluation demonstrates that the proposed model is executable, semantically coherent, and capable of expressing key world-oriented behaviors without explicit user-authored update loops.

## Evaluation Setup

We evaluate the prototype through three executable scenarios:

1. a bouncing-sphere world
2. a forbidden-region constraint world
3. a two-body collision world

All scenarios are described in the `sekai` DSL and executed through the same `orbis` runtime.
Snapshots are exported as structured JSON and rendered into static figures for inspection.

## Evaluation Questions

### EQ-1

Can a minimal world evolve without a user-authored update loop?

### EQ-2

Can constraints be expressed as world laws rather than imperative exception handlers?

### EQ-3

Can independent entities progress asynchronously while synchronizing only at interaction points?

## Results

### R-1 Minimal Declarative Evolution

Figure 1 shows a bouncing-sphere scenario specified entirely through entity declarations, initial properties, and a collision constraint.
No explicit frame loop or mutable update procedure is written by the user.
The world nevertheless evolves coherently over time, and a floor interaction changes the sphere trajectory in a physically interpretable way.

### R-2 Constraint-First Contradiction Handling

The forbidden-region scenario demonstrates that invalid world evolution can be described declaratively and reported as a contradiction in world consistency.
Rather than encoding a procedural guard around each motion step, the user states that a sphere may not enter a specified region.
The runtime then detects the violating event and halts the scenario with a direct contradiction report.

### R-3 Local Synchronization

Figure 2 shows two spheres moving independently until they interact.
Before contact, each sphere follows its own trajectory.
At collision time, an explicit elastic-collision rule is applied, after which the objects separate with exchanged velocities.
This behavior illustrates the intended model of global asynchrony with local synchronization at interaction boundaries.

## Interpretation

These results do not yet establish broad usability or performance advantages.
However, they do validate three central claims of the proposed paradigm:

1. executable worlds can be defined declaratively
2. constraints can function as world laws
3. interaction semantics can be localized rather than globally synchronized

## Limitations

- the runtime currently supports a narrow object vocabulary
- the current region model is axis-aligned and simplified
- simultaneous-event resolution is still underspecified
- figure generation uses 2D projections rather than a full interactive viewer

## Next Evaluation Steps

1. compare equivalent scenarios against imperative baseline implementations
2. measure specification size and structural clarity
3. add more complex multi-object and multi-constraint worlds
4. evaluate diagram-assisted world construction once the viewer exists
