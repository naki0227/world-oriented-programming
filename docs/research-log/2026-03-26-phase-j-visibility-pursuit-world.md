# 2026-03-26 Phase J Visibility Pursuit World

Extended the first visibility-conditioned pursuit slice into a small world-level behavior switch.

## What changed

- added `prefer_candidate_if_occluded(A, search, B)` to action directives
- allowed visibility-conditioned preference directives to coexist per entity
- added examples:
  - `examples/visibility_pursuit_world_clear.sk`
  - `examples/visibility_pursuit_world_occluded.sk`
- added a test that verifies the same world switches from `pursue` to `search`
  depending on whether line of sight is clear or occluded

## Why it matters

The earlier visibility slice showed that geometry could trigger contradiction.
The first pursuit slice showed that geometry could bias candidate selection.
This new slice is stronger: geometry now branches world evolution between two
equally scored continuations without introducing an explicit update procedure.

That makes visibility look more like a genuine world-language pillar and less
like a one-off law.
