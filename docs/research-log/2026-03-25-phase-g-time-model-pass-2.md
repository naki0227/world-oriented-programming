# 2026-03-25 Phase G Time Model Pass 2

## Summary

The G1 time-semantics document has been strengthened with more explicit semantic machinery.
This second pass introduces candidate events, observation frontier, synchronization carrier, and time monotonicity invariants.

## Changes

- added world-frontier and local-progress notation
- added monotonicity invariants for time progression
- added a candidate-event set `Ev(W_t)`
- defined the observation frontier and stable snapshot conditions more explicitly
- related the runtime scheduler to the semantic event set
- linked the older core-model document to the new G1 terminology

## Research Value

This pass makes G1 much closer to a language-semantics substrate rather than an implementation note.
It now gives G2 and G4 a clearer formal base for event ordering and transition semantics.
