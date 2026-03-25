# 2026-03-25 Phase G5 Notation Pass

## Summary

Extended the G5 scaffold with a shared notation block and named transition rules.

## What Was Added

- compact shared notation for:
  - `W_t`
  - `Ev(W_t)`
  - `Next(W_t)`
  - `part(ev)`
  - `deps(ev)`
  - `Sync(ev)`
  - `Obs(W, t_obs)`
  - `W_t^ev`
  - `W_t'`
  - `W_t^X`
- named semantic rules:
  - `Select`
  - `Sync`
  - `Fire`
  - `Enforce`
  - `Contradict`
  - `Observe`
- an explicit snapshot-determinism statement
- a prototype determinism assumption note

## Why It Matters

This pass turns G5 from a structural placeholder into a real semantics-writing surface.
The semantics can now be moved into the paper with much less rewriting.

## Next Step

Write the first compact operational semantics draft in a notation close to inference rules or structured semantic rules.
