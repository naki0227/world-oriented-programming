# 2026-03-27 Phase J Visibility Occlusion Dynamics

## Summary

Extended the first visibility pillar from static branching to observation-time dynamics.

## Changes

- added `examples/visibility_deferred_becomes_visible.sk`
- added `examples/visibility_deferred_becomes_occluded.sk`
- added tests for deferred worlds that later resolve through changing line of sight
- exposed both cases in the viewer sample selector and visibility comparison panel
- updated Phase J and viewer documentation
- added a short paper note that visibility now affects later convergence at observation frontiers

## Why It Matters

This slice turns visibility from a static yes/no condition into a temporal geometry condition.
A world can remain unresolved at one observation frontier and then converge later because line of sight changes as entities move.
