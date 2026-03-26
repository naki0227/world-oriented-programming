# 2026-03-26 Paper Visibility Figure Integration

## Summary

Integrated the new visibility screenshot set into the paper as an executable geometry figure.

## Changes

- Added a new `Visibility-Conditioned World Branching` subsection to `paper/main.tex`.
- Inserted a three-panel figure using:
  - `figures/viewer-visibility-01-clear-run-paper.png`
  - `figures/viewer-visibility-02-occluded-preset-paper.png`
  - `figures/viewer-visibility-03-occluded-run-paper.png`
- Framed the figure as a geometry-conditioned world pipeline:
  clear visibility -> pursuit-oriented continuation,
  editor-side world candidate,
  occluded visibility -> search-oriented continuation.

## Verification

- `tectonic paper/main.tex`

The paper now contains a concrete visibility geometry figure rather than only textual references to the Phase J slice.
