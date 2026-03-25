# 2026-03-25 Paper Semantics Figure

## Summary

Added a semantics pipeline figure directly to the LaTeX paper.

## What Was Added

- a paper-native figure in `paper/main.tex`
- a compact pipeline view:
  - `W_t`
  - `Next(W_t)`
  - `Sync(ev)`
  - `W_t^{ev}`
  - admissible continuation `W_t'`
  - contradiction `W_t^{X}`
  - observation frontier `Obs(W, t_{obs})`

## Why This Form

The goal was to make the semantics section visually legible without adding a heavy external figure-generation dependency.
This figure is simple, stable under LaTeX compilation, and close to the staged operational story developed in Phase G.

## Next Step

If the paper later needs a richer semantics illustration, this compact pipeline can be replaced by a more formal diagram.

## Follow-up

Adjusted the evaluation figures so that the later paper figures form a more consistent visual family.
In particular, the contradiction, collision, and round-trip figures are now all treated as wide showcase figures rather than mixing full-width and single-column sizing.
