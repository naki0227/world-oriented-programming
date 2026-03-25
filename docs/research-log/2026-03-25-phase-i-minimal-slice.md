# 2026-03-25 Phase I Minimal Slice

## Summary

Implemented the first executable Phase I slice.

## What Was Added

- `action:` block support in the parser
- `candidate_velocity(entity, label) = (x, y, z) score n` syntax
- initial candidate resolution at frontier `t = 0`
- hard-law filtering through the existing law layer
- soft-score selection with deterministic tie-breaking by label
- activity-log entries for:
  - `selected`
  - `rejected_by_hard_law`

## Example

The initial executable example is:

- `examples/candidate_velocity.sk`

This example demonstrates:

- one entity
- two velocity candidates
- rejection of the higher-scoring invalid candidate
- selection of the lower-scoring admissible candidate

## Verification

- `cargo test`
- `cargo run -p sekai-cli -- simulate examples/candidate_velocity.sk`
- `cargo run -p sekai-cli -- simulate-report examples/candidate_velocity.sk`

## Why It Matters

This is the first point where Phase I stops being only a semantic proposal and becomes an executable capability.
