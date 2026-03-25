# 2026-03-25 Phase F Constraint Classification

## Summary

Phase F now exposes a more explicit law model in the runtime report.
Each constraint is classified and annotated with its supported repair policies, making the report closer to a language-level description of world laws rather than a minimal execution trace.

## Changes

- added constraint categories: `invariant`, `boundary`, and `interaction`
- added supported-policy metadata to constraint summaries
- moved unsupported-policy rejection to constraint build time
- updated the viewer so law cards show category and supported policies
- updated output-format and phase documentation to reflect the richer report model

## Research Value

This makes Phase F more legible as a design contribution.
The prototype no longer reports only that a law existed or fired.
It now also reports what kind of law it was and what enforcement styles belonged to that law.

That is important for later work on semantics, solver integration, and visual explanation.
