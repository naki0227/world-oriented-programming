# 2026-03-25 Phase G Event Ordering

## Summary

Phase G2 has begun with a first formal pass over event ordering.
This pass defines earliest-event selection, simultaneous-event resolution, a semantic priority lattice, and a deterministic tie-breaker strategy.

## Changes

- extended the G1 semantics document with a dedicated G2 event-ordering section
- defined `Next(W_t)` as the semantic event-selection operator
- introduced `MinEv(W_t)` for earliest-time event selection
- proposed a current semantic priority ordering across boundary-entry, boundary-contact, and interaction events
- linked the older core-model document to the new ordering terminology

## Research Value

This is the first explicit answer to the project's long-standing simultaneous-event question.
It does not yet finish the transition semantics, but it fixes the selection problem that later transition semantics must consume.

That makes G2 the determinism bridge between the time model and the future operational semantics.
