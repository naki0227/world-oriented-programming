# 2026-03-25 Paper Semantics Integration

## Summary

Integrated the current Phase G semantics work into the LaTeX paper.

## What Changed

- merged `feature/phase-g-local-synchronization` into `main`
- added a new `Toward an Operational Semantics` subsection to `paper/main.tex`
- connected the paper's model section to the current G-phase semantics work:
  - time frontier
  - event ordering
  - local synchronization
  - event / enforcement split
  - contradiction as semantic failure
  - deterministic snapshots via deterministic selection plus coherent synchronization

## Why It Matters

Before this pass, the paper described the execution model mainly as a prototype design claim.
After this pass, the manuscript starts to present `sekai` as a language with an emerging operational semantics.

## Next Step

Refine the new subsection into a fuller semantics section or split it into a separate section once the paper structure is revised again.
