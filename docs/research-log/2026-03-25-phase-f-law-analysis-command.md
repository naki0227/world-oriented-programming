# 2026-03-25 Phase F Law Analysis Command

## Summary

Phase F now includes a static law-analysis entry point.
The new `sekai analyze` command reports declared laws and aggregated law analytics without running time evolution.

## Changes

- added `analyze_program(...)` to `orbis`
- added `sekai analyze <file.sk>` to the CLI
- reused the shared law-summary and analytics model for static inspection
- documented the command in the output-format and viewer notes

## Research Value

This makes Phase F usable both before and after execution.
Users can now inspect the shape of a world-law configuration without requiring a simulation run.

That strengthens Phase F as a language-layer contribution, not only a runtime-layer contribution.
