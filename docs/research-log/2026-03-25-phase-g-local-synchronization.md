# 2026-03-25 Phase G Local Synchronization

## Summary

Phase G3 has begun with a first semantic account of synchronization scope.
This pass defines `Sync(ev)`, `deps(ev)`, and the minimal consistency idea that separates local synchronization from global lockstep.

## Changes

- added a dedicated G3 section to the time-semantics document
- defined synchronization scope as participants plus dependency closure
- clarified the distinction between observation-wide synchronization and event-local synchronization
- treated admissibility as a source of synchronization, not only direct interaction
- updated the older core-model document with G3 terminology

## Research Value

This is the point where `sekai`'s claim of global asynchrony with local synchronization becomes a semantic statement rather than only a runtime intuition.
It creates a formal bridge from the time model to later transition and consistency semantics.
