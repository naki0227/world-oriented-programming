# 2026-03-25 Phase G Transition Semantics

## Summary

The G branch now connects event ordering to transition semantics.
This pass adds a first G4 account of event firing, enforcement, and contradiction as distinct semantic relations.

## Changes

- linked G2 more explicitly to current runtime event kinds
- defined a prototype-compatible ordering key for deterministic event selection
- introduced a semantic notion of near-simultaneity for floating-point runtimes
- added G4 transition layers: `event`, `enforce`, and `contradiction`
- connected runtime activity labels to the emerging semantic transition system

## Research Value

This is the first point where `sekai` starts looking like an operational semantics rather than a design manifesto.
The project now has:

- a time model
- an event-selection rule
- a transition-layer vocabulary

That is the core triangle needed to turn the prototype into a real language semantics.
