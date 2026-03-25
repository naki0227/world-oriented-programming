# 2026-03-25 Phase G5 Proof Obligations

## Summary

Made the current proof obligations explicit in both the semantics notes and the paper.

## What Was Added

- a `Proof Obligations` section in `docs/phase-g-time-semantics.md`
- explicit naming of:
  - snapshot determinism
  - causality preservation
  - repair termination
- a short paper-side paragraph explaining that these are the next theory targets rather than already-proven properties

## Why It Matters

This makes the semantics story more mature.
The paper no longer only says that the model seems coherent.
It now says which properties a later semantics paper should actually prove.

## Next Step

Either:

- merge this pass and continue toward a more formal theorem-style semantics, or
- use the named obligations to structure the next semantics-focused paper draft.
