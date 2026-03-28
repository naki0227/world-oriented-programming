# 2026-03-27: Phase J Visibility Handoff

This pass extends the visibility pillar from binary branching into multi-target handoff.

The key step is small but important:
a deferred world no longer has to resolve only between `pursue` and `search`.
It can now resolve toward one of several target-specific pursuit continuations,
depending on which target later becomes visible through the corridor geometry.

This strengthens the Phase J claim in two ways:

- visibility now selects between multiple world continuations, not just a single preferred continuation, and
- geometry begins to look more like a routing condition over possible world developments.

The resulting examples are still compact, but they better match the intended research direction:
world geometry can hand off admissible development between several live continuations without exposing imperative update logic.
