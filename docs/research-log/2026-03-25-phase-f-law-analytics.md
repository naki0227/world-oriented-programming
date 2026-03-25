# 2026-03-25 Phase F Law Analytics

## Summary

Phase F now includes report-level law analytics.
In addition to per-law summaries, each run exposes aggregate counts over law categories and law outcomes.

## Changes

- added `analytics` to simulation reports and failure envelopes
- counted total laws by category: invariant, boundary, interaction
- counted total laws by outcome: idle, fired, repaired, contradicted
- added a `Law Analytics` panel to the viewer
- updated output-format and viewer documentation

## Research Value

This pushes Phase F from local inspection toward comparative analysis.
The runtime now supports both micro-level explanation of individual laws and macro-level explanation of whole runs.

That will make later comparisons and semantic evaluation easier to write up.
