# 2026-03-25 Phase I Repaired Selection Sample

## Summary

Added a repaired-selection example and viewer sample for Phase I.

## Why This Matters

The first Phase I sample showed only one pattern:

- a high-score candidate is rejected by hard laws
- a lower-score candidate is selected unchanged

That was enough to demonstrate underdetermined-world convergence, but it did not yet show the second important case:

- a high-score candidate is selected
- the hard law layer repairs it into admissibility

This pass makes that case easy to inspect in the viewer and easy to cite in later paper updates.

## Files Updated

- `examples/candidate_velocity_clamped.sk`
- `viewer/index.html`
- `docs/viewer.md`
- `docs/phase-i-possibility-and-convergence.md`

## Verification

- `cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_clamped.sk`
- generated `viewer/samples/candidate_velocity_clamped.json`

## Interpretation

Phase I now has two small but complementary executable examples:

- hard-law rejection followed by fallback selection
- hard-law repair of the highest-scoring candidate
