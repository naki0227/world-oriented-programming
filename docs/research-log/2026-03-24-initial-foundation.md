# Research Log: 2026-03-24

## Session Goal

Create the initial research foundation for the `sekai` project so that future design and implementation work can be recorded systematically and later reused for a paper.

## Context

The project begins from the idea that existing programming languages are too centered on procedure, sequence, and explicit updates.
The desired alternative is a world-description language where users define space, time, relation, and constraint directly.

## Key Ideas Captured In This Session

### 1. Project identity

- Language name: `sekai`
- Core engine name: `orbis`
- research paradigm name: World-Oriented Programming
- proposed extension: `.sk`

### 2. Core philosophy

- write worlds, not instructions
- avoid user-authored central update loops
- treat 3D space as first-class
- treat time as fundamental
- prefer global asynchrony with local synchrony
- express exceptions as constraints

### 3. Development strategy

Use a staged roadmap so the project can accumulate real wins:

- philosophy first
- then a minimal world simulator
- then a constraint system
- then a DSL
- then a diagram interface

### 4. First milestone

Declaratively simulate a sphere bouncing on a floor without a user-written `update` loop.

## Decisions Recorded

- The repository will maintain dated research logs from the beginning.
- Stable conceptual choices should also become decision records.
- The first documents to maintain are philosophy, roadmap, requirements, and paper outline.
- The next specification layer should include a minimal formal model and a minimal DSL draft.

## Open Questions

1. What is the smallest formal world state model that can support continuous time and lazy updates?
2. Should the first runtime use exact continuous mathematics, discrete timesteps with lazy evaluation, or a hybrid approximation?
3. What should the first DSL grammar include and exclude?
4. What is the contradiction policy for constraints in v0.1?

## Actions Taken

- created the initial repository documentation structure
- recorded philosophy, roadmap, requirements, and paper outline
- fixed the initial project naming decision
- drafted a first formal world model
- drafted a minimal DSL surface syntax

## Next Recommended Step

Begin Phase 1 prototype planning:

- choose implementation language for `orbis`
- define runtime module boundaries
- specify the bouncing sphere reference scenario precisely
