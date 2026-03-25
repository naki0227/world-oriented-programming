# Manuscript Skeleton Draft

## Title

World-Oriented Programming:
A Programming Paradigm for Executable World Description Through Space, Time, and Constraints

## Provisional Venue

Onward! at SPLASH

## Abstract

This paper proposes World-Oriented Programming, a programming paradigm in which executable systems are described as worlds rather than instruction sequences.
The approach is motivated by the mismatch between human spatial-temporal reasoning and conventional procedural programming, especially for domains such as simulation, geometry, and interactive environments.
In the proposed model, entities, spatial relations, temporal development, and logical constraints are represented directly, while execution is handled as the evolution of a constrained world over time.
The prototype combines declarative world description, continuous-time progression, constraint-first validity handling, and local synchronization at interaction points.
We report initial executable scenarios that demonstrate update-free world evolution, contradiction detection through world laws, and localized interaction between independently progressing objects.

## 1. Introduction

- problem: conventional languages center procedures and explicit state updates
- motivation: human reasoning is often spatial, relational, and constraint-driven
- proposal: describe worlds directly and execute them as constrained temporal systems
- contribution summary

## 2. Background And Motivation

- mismatch between mathematical or visual reasoning and imperative implementation
- need for first-class space, time, and constraints
- limitations of frame-loop-centered design

## 3. World-Oriented Programming Model

- world as executable description
- entities, relations, constraints, and time
- global asynchrony with local synchronization
- constraints as world laws

References:

- `docs/philosophy.md`
- `docs/model.md`
- `docs/syntax.md`

## 4. Prototype Architecture

- `sekai` DSL surface
- `orbis` runtime
- structured snapshot export
- figure-generation workflow

References:

- `docs/phase1-prototype.md`
- `docs/output-format.md`
- `docs/figure-generation.md`

## 5. Prototype Evaluation

Use and adapt:

- `docs/paper-evaluation-draft.md`
- `docs/paper-results-draft.md`

### 5.1 Minimal Declarative Evolution

Insert Figure 1 and caption from `docs/paper-figures-captions.md`.

### 5.2 Constraint-First Contradiction Handling

Discuss `examples/forbidden_region.sk`.

### 5.3 Local Synchronization

Insert Figure 2 and caption from `docs/paper-figures-captions.md`.

## 6. Discussion

- what is already validated
- what remains underspecified
- limitations of the current runtime
- implications for language design and visual computing

## 7. Future Work

- richer geometry and more entity types
- simultaneous-event semantics
- visual editor and diagram-to-logic translation
- comparative evaluation against imperative baselines

## Figure Placement Guide

- Figure 1: `figures/bounce-xy.png`
- Figure 2: `figures/two_body_collision-xy.png`

See also:

- `docs/paper-assets.md`
- `docs/paper-figures-captions.md`
- `docs/paper-draft-v1.md`
- `docs/paper-draft-v2.md`
