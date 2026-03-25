# 2026-03-25 Phase G5 Operational Rules Draft

## Summary

Turned the G5 consolidation notes into a compact operational-rules draft.

## What Was Added

- a lightweight rule family for:
  - `Select`
  - `Sync`
  - `Fire`
  - `Enforce`
  - `Contradict`
  - `Observe`
- a composed staged execution view:
  `select -> sync -> fire -> enforce / contradict`
- a stronger snapshot-determinism statement tied to G2 and G3

## Why It Matters

This is the first draft in the project where the semantics can be read as a staged execution system instead of only prose.
That makes it much easier to move the material into the paper and to compare the prototype runtime against the intended formal model.

## Next Step

Refine the compact rules into a paper-facing semantics section, likely with either:

- structured rule blocks, or
- a more inference-rule-like notation

depending on how formal the next paper revision should be.
