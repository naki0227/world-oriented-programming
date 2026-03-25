# Phase 1 Prototype Plan

## Objective

Build the smallest executable prototype that validates the central claim:

Users can describe a moving world without writing a manual `update` loop.

## Chosen Scope

The first prototype is intentionally narrow:

- one sphere
- one static plane
- constant velocity between interactions
- elastic reflection on collision
- observation through requested snapshots

## Why This Scope

This scenario is minimal but meaningful.
It already tests:

- world declaration
- time advancement
- lazy observation
- local synchronization at collision
- constraint-driven behavior

## Runtime Shape

### `orbis`

Rust library crate containing:

- vector math
- world state
- time advancement
- collision handling
- simulation report generation

### `sekai`

Rust CLI binary containing:

- `.sk` file loading
- parsing
- simulation execution
- text output of snapshots

## Reference Scenario

`examples/bounce.sk` defines:

- sphere `A`
- plane `floor`
- initial position `(0, 10, 0)`
- initial velocity `(1, -3, 0)`
- reflection on collision

Expected qualitative behavior:

- the sphere moves downward
- at first contact with the floor it reflects upward
- later snapshots show the post-collision direction

## Progress Update

The prototype has already been extended beyond the original minimum:

- multiple spheres are now supported
- forbidden regions are supported
- velocity-limit constraints are supported
- explicit pairwise elastic collision is supported

## Immediate Next Extensions

The next realistic additions are:

1. structured snapshot export
2. simultaneous-event policy
3. richer region semantics
4. multiple planes or surfaces
5. a basic renderer or viewer
