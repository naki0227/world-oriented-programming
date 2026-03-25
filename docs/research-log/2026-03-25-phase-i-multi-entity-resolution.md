# 2026-03-25 Phase I Multi-Entity Resolution

## Summary

Extended the initial Phase I convergence slice from one candidate-bearing entity to multiple entities.

## What Changed

The runtime now groups action candidates by entity and resolves them in deterministic entity-name order.
Each entity still uses the same local rule:

- sort candidates by score, then label
- reject candidates blocked by hard laws
- select the highest-scoring admissible continuation
- keep repairs performed by the hard law layer

Structured reports now expose `candidate_resolutions` as a list rather than a single summary.

## Files Updated

- `orbis/src/world.rs`
- `viewer/app.js`
- `docs/output-format.md`
- `docs/phase-i-possibility-and-convergence.md`
- `examples/candidate_velocity_two_entity.sk`

## Why This Matters

This is the first point where Phase I stops looking like a one-off special case.
The prototype still does not support general global convergence, but it now shows that underdetermined-world resolution can be applied to more than one entity within the same world.
