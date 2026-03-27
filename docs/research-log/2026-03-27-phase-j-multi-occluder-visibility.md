# 2026-03-27: Phase J Multi-Occluder Visibility

This pass extends the Phase J visibility slice from a single blocking region to a small set of declared occluding regions.

The key design choice is intentionally narrow:

- forbidden-region behavior remains anchored to the primary declared region, but
- visibility treats all declared regions as potential occluders.

This preserves the earlier constraint kernel while making the visibility pillar more representative of real geometry problems.

The resulting claim is stronger than the original first slice.
`visible(A, B)` is no longer only "line of sight across one wall".
It can now express line of sight across a small configuration of blockers and report which declared blocking set invalidates the current world.

That keeps the extension tightly aligned with the larger thesis of the project:
geometry should appear as explicit world structure and world law, not as scattered update logic.
