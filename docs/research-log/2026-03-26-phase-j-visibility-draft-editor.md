# 2026-03-26 Phase J Visibility Draft Editor

Extended the viewer draft editor so it can propose a small visibility-pursuit world.

## What changed

- broadened draft candidates from pure constraints to mixed world candidates
- kept the same checkbox-based acceptance flow
- when two spheres and an occluding region are present, the editor can now propose:
  - `visible(A, B)`
  - a small `action:` block with
    - `candidate_velocity(..., hold)`
    - `candidate_velocity(..., pursue)`
    - `candidate_velocity(..., search)`
    - `prefer_candidate_if_visible(...)`
    - `prefer_candidate_if_occluded(...)`

## Why it matters

This is the first step toward making visibility a real editor-level concept.
The draft UI no longer only proposes static world laws; it can now also propose a small
geometry-conditioned behavior world that round-trips through the same runtime.
