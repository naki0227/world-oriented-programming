# 2026-03-25 Phase I Candidate Resolution

## Summary

Added a compact `candidate_resolution` summary to structured simulation reports and exposed it in the viewer.

## Why This Matters

The first executable Phase I slice already emitted `candidate_velocity` events through the generic activity log.
That was enough for debugging, but it still left the Phase I story too implicit.

This pass makes candidate convergence visible as its own report concept:

- which entity carried the candidates
- how many candidates were considered
- how many were rejected by hard laws
- which candidate was selected
- whether the selected candidate required repair before becoming admissible

## Files Updated

- `orbis/src/world.rs`
- `viewer/index.html`
- `viewer/app.js`
- `docs/output-format.md`
- `docs/viewer.md`
- `docs/phase-i-possibility-and-convergence.md`

## Verification

- `cargo test`
- `cargo run -p sekai-cli -- simulate-report examples/candidate_velocity.sk`

## Interpretation

This is a small but important step for Phase I.
Underdetermined worlds are no longer visible only through low-level activity traces; they now have an explicit convergence summary that can be inspected in the viewer and cited in later paper revisions.
