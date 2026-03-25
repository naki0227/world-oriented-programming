# Research Log: 2026-03-24 Constraint Extension

## Session Goal

Extend the first prototype so the world model can express more than collision behavior.

## Context

The bouncing sphere prototype validated declarative time evolution.
The next step is to show that logical constraints can actively restrict world behavior.

## Actions Taken

- added `speed(A) <= vmax` parsing as a velocity-limit constraint
- added `not inside(A, region)` parsing as a forbidden-region constraint
- implemented axis-aligned box regions through `min(region)` and `max(region)`
- enforced velocity limits as hard invariants
- detected forbidden-region entry during trajectory advancement
- added executable examples and unit tests

## Observations

- even simple constraints immediately sharpen the semantic character of the language
- contradiction reporting is already becoming part of the user experience
- axis-aligned regions are a good narrow starting point because they keep geometry simple while still expressing meaningful world laws

## Verification Result

Commands used:

```text
cargo run -p sekai-cli -- simulate examples/bounce.sk
cargo run -p sekai-cli -- simulate examples/forbidden_region.sk
cargo test
```

Observed outcomes:

- `bounce.sk` still simulates successfully and reflects on floor contact
- `forbidden_region.sk` stops with `sphere A entered forbidden region zone at t=2.000`
- unit tests passed for reflection, velocity-limit rejection, and forbidden-region contradiction

## Open Questions

1. Should forbidden-region violations always halt, or should repair policies become first-class?
2. Should region semantics account for sphere radius in v0.2?
3. How should multiple simultaneous constraints be prioritized?

## Next Recommended Step

Broaden the constraint system carefully:

- multiple entities
- explicit contradiction policy categories
- structured simulation output
