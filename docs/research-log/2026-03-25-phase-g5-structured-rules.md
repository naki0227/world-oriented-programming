# 2026-03-25 Phase G5 Structured Rules

## Summary

Pushed the G5 semantics one step closer to a paper-facing operational semantics by adding structured rule blocks.

## What Was Added

- structured blocks for:
  - `Select`
  - `Sync`
  - `Fire`
  - `Enforce`
  - `Contradict`
  - `Observe`
- an explicit statement that these blocks are now close to an inference-rule presentation
- a short paper-side note that the semantics is now staged as explicit rules rather than only prose

## Why It Matters

This pass narrows the gap between the current semantics notes and a true operational semantics section.
The project is still not claiming a finished formal semantics, but the shape of one is now much clearer.

## Next Step

Either:

- translate these structured blocks into a more formal inference-rule notation, or
- merge this pass and stabilize the paper text before continuing deeper theory work.
