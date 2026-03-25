# 2026-03-25 Phase I Tie-Break Metadata

## Summary

Upgraded Phase I candidate reporting so it captures top-score ties and skipped candidates explicitly.

## What Changed

Candidate resolution is no longer reconstructed only from activity logs.
The runtime now stores structured per-entity resolution summaries that include:

- total candidates
- rejected candidates
- skipped candidates
- selected candidate
- selected score
- top score
- labels tied at that top score
- whether repair happened after selection

## Why This Matters

Phase I already relied on deterministic tie-breaking after score ordering, but the report did not expose where a tie actually existed.
This pass makes that semantic choice visible, which is important for later work on underdetermined worlds and observational equivalence.
