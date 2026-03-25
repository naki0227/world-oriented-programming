# 2026-03-25 Phase I Candidate Inventory Analysis

## Summary

Extended `sekai analyze` so Phase I candidate declarations are visible without running simulation.

## What Changed

Static law analysis now includes a `candidate_inventory` section when a program declares action candidates.

The inventory currently reports:

- candidate-bearing entity
- total number of declared candidates
- sorted candidate labels
- highest declared soft score
- labels tied at that highest score

## Files Updated

- `orbis/src/world.rs`
- `docs/output-format.md`
- `docs/phase-i-possibility-and-convergence.md`

## Why This Matters

Phase I is not only about runtime convergence.
It is also about recognizing when a world is intentionally underdetermined before time evolution begins.

Adding candidate inventory to `sekai analyze` makes that underdetermination visible in the same static analysis path already used for laws.
