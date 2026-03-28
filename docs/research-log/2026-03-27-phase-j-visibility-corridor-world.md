# 2026-03-27: Phase J Visibility Corridor World

This pass turns the multi-occluder visibility slice into a more recognizable geometry scenario: a corridor world.

The scenario remains intentionally small, but the geometry now has a clearer spatial role:

- two declared regions act as corridor walls,
- an optional third region acts as a blocker, and
- the same pursuit/search world changes continuation depending on whether the corridor preserves line of sight.

This strengthens the visibility pillar in two ways.

First, it moves the work from a predicate-oriented example toward a domain-style geometry situation.
Second, it strengthens Phase K by making the imperative baseline manage both multi-box visibility checks and explicit continuation routing.

The resulting claim is sharper:
`sekai` can keep corridor geometry and visibility-conditioned world branching in one description, instead of scattering both across update logic.
