# 2026-03-25 Phase I Tie Sample Viewer Follow-Up

## Summary

Added explicit `tie_broken` metadata and wired the tie sample into the viewer.

## What Changed

- `candidate_resolutions` now report whether deterministic tie-breaking was actually used
- the viewer now displays that flag
- the candidate comparison panel now includes a dedicated tie sample

## Why This Matters

Phase I no longer treats tie-breaking as an invisible implementation detail.
The prototype can now show when equal top-score candidates existed, which labels were tied, and that one branch was chosen by deterministic ordering.
