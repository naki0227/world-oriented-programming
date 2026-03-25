# 2026-03-25 Phase F Law Outcomes

## Summary

Phase F now records a run-level outcome for each law.
Instead of inferring behavior only from counts, reports can now say directly whether a law stayed idle, fired, repaired the world, or ended in contradiction.

## Changes

- added `outcome` and `contradicted_count` to constraint summaries
- recorded explicit `contradicted` activity when a law rejects the world
- updated the viewer to display law outcome directly
- updated the output-format and phase notes for the richer law model

## Research Value

This makes the constraint layer easier to analyze as a semantic object.
Each law is now not only declared and traced, but also summarized by how it concluded during execution.

That is a strong bridge from Phase F toward later semantic work in Phase G.
