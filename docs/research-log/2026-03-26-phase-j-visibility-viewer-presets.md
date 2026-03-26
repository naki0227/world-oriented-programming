# 2026-03-26 Phase J Visibility Viewer Presets

Added one-click visibility presets to the viewer draft editor.

## What changed

- added `Load Clear World` and `Load Occluded World` buttons to the draft editor
- each preset now configures:
  - two spheres
  - an occluding region
  - the visibility-pursuit world candidate already accepted
- `Run Draft` can therefore reproduce the branching visibility demo without manual placement

## Why it matters

The visibility pillar is now easier to demo live.
Instead of manually rebuilding the same scene, the viewer can jump directly to the clear
or occluded geometry case and show how the world branches toward pursuit or search.
