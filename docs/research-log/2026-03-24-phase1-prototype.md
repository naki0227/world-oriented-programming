# Research Log: 2026-03-24 Phase 1 Prototype Start

## Session Goal

Turn the conceptual documents into a minimal executable prototype.

## Context

The previous session established the philosophy, roadmap, requirements, and initial formal model for `sekai`.
The next step is to validate the first milestone with code.

## Actions Taken

- selected Rust for the first runtime prototype
- created a Cargo workspace with `orbis` and `sekai`
- implemented a minimal parser for `.sk` files
- implemented a narrow world model for one sphere and one plane
- implemented elastic reflection on plane collision
- added `examples/bounce.sk`

## Observations

- a very small DSL is enough to validate the no-`update` claim
- local synchronization appears naturally at collision time
- the parser should remain intentionally small until the runtime semantics stabilize
- the reference scenario executed successfully and showed reflection after floor contact

## Verification Result

Command used:

```text
cargo run -p sekai-cli -- simulate examples/bounce.sk
```

Observed snapshots:

- `t=0.000` position `(0.000, 10.000, 0.000)` velocity `(1.000, -3.000, 0.000)`
- `t=1.000` position `(1.000, 7.000, 0.000)` velocity `(1.000, -3.000, 0.000)`
- `t=3.000` position `(3.000, 1.000, 0.000)` velocity `(1.000, 3.000, 0.000)`
- `t=4.000` position `(4.000, 4.000, 0.000)` velocity `(1.000, 3.000, 0.000)`

This confirms that the sphere can be declared once, evolved internally by the runtime, and observed without a user-authored update loop.

## Decisions Recorded

- the first executable prototype will prefer semantic clarity over generality
- v0.1 parser support is intentionally restricted to milestone scenarios

## Open Questions

1. Should gravity be part of the first public prototype or remain out of scope?
2. Should snapshots be printed as text, JSON, or both?
3. Should the next extension focus on multiple objects or stronger constraints?

## Next Recommended Step

Run the reference scenario, verify output, and then choose the next semantic expansion.
