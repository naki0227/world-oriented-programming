# 2026-03-25 Paper Semantics Section Pass

## Summary

Refactored the paper's new semantics material from a short subsection into an independent section.

## What Changed

- promoted the semantics discussion into `\section{Toward an Operational Semantics}`
- split the section into four focused parts:
  - time frontiers and observation
  - event ordering and local synchronization
  - event, enforcement, and contradiction
  - deterministic snapshots without global lockstep

## Why It Matters

This pass makes the semantics contribution look less like an appended note and more like a genuine part of the paper's argument.
It also aligns the paper structure more closely with the current G-phase research structure.

## Next Step

Either keep refining this section in place, or later split it into a fuller `Semantics` section if the paper grows enough to support that structure cleanly.
